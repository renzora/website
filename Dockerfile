# Stage 1: Build
FROM rust:1.88-bookworm AS builder

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

# Build dependencies only (cached layer)
ENV SQLX_OFFLINE=true
RUN cargo build --release 2>/dev/null || true

# Copy real source code + migrations
COPY crates/ crates/
COPY migrations/ migrations/

# Touch to invalidate cache for source changes
RUN touch crates/server/src/main.rs crates/api/src/lib.rs crates/models/src/lib.rs crates/web/src/lib.rs crates/common/src/lib.rs

# Build the application
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/renzora-server /app/renzora-server
COPY --from=builder /app/migrations /app/migrations
COPY assets/ /app/assets/

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

CMD ["/app/renzora-server"]
