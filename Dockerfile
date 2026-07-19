# Stage 1: Build
FROM rust:latest AS builder

WORKDIR /app

# Copy manifests first for dependency layer caching
COPY Cargo.toml Cargo.lock* ./
COPY crates/server/Cargo.toml crates/server/Cargo.toml
COPY crates/api/Cargo.toml crates/api/Cargo.toml
COPY crates/models/Cargo.toml crates/models/Cargo.toml
COPY crates/web/Cargo.toml crates/web/Cargo.toml
COPY crates/common/Cargo.toml crates/common/Cargo.toml

# Create dummy source files for dependency caching
RUN mkdir -p crates/server/src crates/api/src crates/models/src crates/web/src crates/common/src && \
    echo "fn main() {}" > crates/server/src/main.rs && \
    echo "" > crates/api/src/lib.rs && \
    echo "" > crates/models/src/lib.rs && \
    echo "" > crates/web/src/lib.rs && \
    echo "" > crates/common/src/lib.rs

# Limit codegen parallelism to reduce peak memory usage on small VPS
ENV SQLX_OFFLINE=true
ENV CARGO_BUILD_JOBS=2

# Build dependencies only (cached layer)
RUN cargo build --release 2>/dev/null || true

# Copy real source code + migrations
COPY crates/ crates/
COPY migrations/ migrations/

# Touch to invalidate cache for source changes
RUN touch crates/server/src/main.rs crates/api/src/lib.rs crates/models/src/lib.rs crates/web/src/lib.rs crates/common/src/lib.rs

# Build the application
RUN cargo build --release

# Stage 1b: Front-end assets — compile Tailwind CSS and subset the Phosphor icon
# font from source, so main.css and the icon subset are always in sync. Runs in
# parallel with the Rust build (BuildKit).
FROM node:20-alpine AS css
WORKDIR /app
# Install deps first (cached unless package.json changes)
COPY package.json package-lock.json* ./
RUN npm install --no-audit --no-fund
# Inputs the Tailwind scan + icon subsetter need (scan crates + migrations for
# both class names and ph-* icon tokens, incl. DB-seeded ones)
COPY tailwind.config.js ./
COPY assets/style/input.css ./assets/style/input.css
# Preview image sources: committed PNG originals + the canonical .webp masters
# (some hand-cropped, e.g. inspector). optimize:images regenerates the .avif
# companions and responsive srcset variants from the .webp so the deployed set
# always matches source, the same way main.css is rebuilt here.
COPY assets/previews ./assets/previews
COPY scripts ./scripts
COPY migrations ./migrations
COPY crates ./crates
RUN npm run build:icons && npm run build:css && npm run optimize:images

# Stage 2: Runtime (must match builder's glibc — rust:latest uses Trixie)
FROM debian:trixie-slim

RUN apt-get update && \
    apt-get install -y ca-certificates curl ffmpeg && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/renzora-server /app/renzora-server
COPY --from=builder /app/migrations /app/migrations
COPY assets/ /app/assets/
# Override committed assets with the freshly built ones (always up to date)
COPY --from=css /app/assets/style/main.css /app/assets/style/main.css
COPY --from=css /app/assets/style/phosphor.css /app/assets/style/phosphor.css
COPY --from=css /app/assets/fonts /app/assets/fonts
# Override the context-copied previews with the freshly generated avif/variants.
COPY --from=css /app/assets/previews /app/assets/previews
COPY docs/ /app/docs/

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

CMD ["/app/renzora-server"]
