#!/usr/bin/env bash
# Put the engine's wasm build under ./engine, where the app serves it at /engine.
#
# Usage: scripts/fetch-engine-wasm.sh [tag]
#   tag  a renzora/engine release tag. Omitted, the newest release is used —
#        via the releases list, NOT /releases/latest, because nightlies are
#        prereleases and `latest` skips those. The newest release is almost
#        always a nightly, and a nightly is exactly what we want here.
#
# Runs on the droplet, from /opt/renzora-website. It is called two ways:
#   - by the normal deploy, so a fresh droplet ends up with a build at all;
#   - by the `engine-wasm` workflow, fired from the engine's own publish job,
#     so a nightly reaches the site the night it is cut rather than whenever
#     the website next happens to deploy.
#
# ── Why this is a download and not a checked-in directory ────────────────────
# `renzora-editor_bg.wasm` is ~100 MB and `renzora-runtime_bg.wasm` ~72 MB.
# The editor module is within a few MB of GitHub's 100 MiB per-file hard limit,
# so committing it would break the push outright before long, and until then
# would add ~170 MB of history per nightly that can never be pruned. The
# marketplace preview wasm has been fetched this way for the same reason.
#
# ── The layout, and why it has a symlink in it ───────────────────────────────
#   engine/builds/<tag>/    one unpacked build
#   engine/current -> builds/<tag>
# and the app serves `engine/current`. The indirection exists because `engine/`
# is a *bind mount*: replacing that directory with a freshly built one would
# leave the container pinned to the old, now-unlinked inode, serving a build
# nobody can see on the host (the same trap nginx.conf's single-file mount
# documents). The mount root therefore never moves; only the symlink inside it
# does, in one atomic rename, so no request ever sees a half-written tree.
#
# ── Why it pre-compresses ────────────────────────────────────────────────────
# Both nginx (`gzip_types` includes `application/wasm`) and the app's
# CompressionLayer would otherwise compress 100 MB on *every* request, on a
# two-core VPS. Writing `.br`/`.gz` beside each file once means ServeDir's
# `precompressed_*` hands the encoded file straight out with a
# `Content-Encoding` header, which both compressors then leave alone.
set -euo pipefail

REPO="${ENGINE_REPO:-renzora/engine}"
TAG="${1:-}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEST="$ROOT/engine"
ASSET="web-wasm32.zip"

if [ -z "$TAG" ]; then
    # Newest first, prereleases included. The first `tag_name` in the response
    # is the newest release; `jq` is not guaranteed to be on the droplet.
    TAG=$(curl -sfL -H 'Accept: application/vnd.github+json' \
            "https://api.github.com/repos/$REPO/releases?per_page=1" \
          | grep -m1 '"tag_name"' | cut -d'"' -f4 || true)
fi
if [ -z "$TAG" ]; then
    echo "ERROR: could not determine an engine release tag" >&2
    exit 1
fi

BUILD="$DEST/builds/$TAG"

# Skip a re-fetch of a build that is already live. The dispatch and a deploy can
# land within a minute of each other, and pulling 52 MB twice to write the same
# bytes is pure waste.
if [ "$(readlink "$DEST/current" 2>/dev/null || true)" = "builds/$TAG" ] && [ -d "$BUILD" ]; then
    echo "engine wasm already at $TAG — nothing to do"
    exit 0
fi

echo "Fetching $ASSET from $REPO@$TAG"
mkdir -p "$DEST/builds"

# Stage inside `builds/` rather than /tmp: the final step is a rename, which is
# only atomic within one filesystem, and /tmp is often a separate one. The `.new`
# prefix keeps a stalled run out of the prune below, which matches `builds/*`.
STAGE="$DEST/.new.$$"
rm -rf "$STAGE"
trap 'rm -rf "$STAGE"' EXIT
mkdir -p "$STAGE"

if ! curl -sfL "https://github.com/$REPO/releases/download/$TAG/$ASSET" -o "$STAGE/$ASSET"; then
    echo "ERROR: $REPO@$TAG has no $ASSET (a desktop-only build?) — keeping what is live" >&2
    exit 1
fi

unzip -q "$STAGE/$ASSET" -d "$STAGE/tree"
rm -f "$STAGE/$ASSET"

# The zip carries `renzora-editor.html` and `renzora-runtime.html` but no index,
# so bare /engine would 404. Point it at the editor, which is what a visitor
# means by "the engine in a browser"; the runtime shell stays one click away.
cat > "$STAGE/tree/index.html" <<'HTML'
<!doctype html>
<meta charset="utf-8">
<title>Renzora Editor</title>
<meta http-equiv="refresh" content="0; url=./renzora-editor.html">
<a href="./renzora-editor.html">Renzora Editor</a>
HTML

echo "$TAG" > "$STAGE/tree/BUILD_TAG"

# Pre-compress what is worth compressing. gzip is always present; brotli often
# is not, and its absence costs about a third of the transfer rather than
# breaking anything, so it stays optional.
while IFS= read -r -d '' f; do
    gzip -9 -k -f "$f"
    command -v brotli >/dev/null 2>&1 && brotli -q 5 -f -o "$f.br" "$f"
done < <(find "$STAGE/tree" \( -name '*.wasm' -o -name '*.js' -o -name '*.html' \) -type f -print0)

chmod -R a+rX "$STAGE/tree"
rm -rf "$BUILD"
mv "$STAGE/tree" "$BUILD"

# Atomic publish: `ln -T` onto a temp name, then rename it over `current`. A
# plain `ln -sfn` unlinks first, which leaves a window where /engine 404s.
ln -sfn "builds/$TAG" "$DEST/.current.$$"
mv -T "$DEST/.current.$$" "$DEST/current"

# One build is ~260 MB with its .gz/.br companions, so old ones do not stay.
for old in "$DEST"/builds/*; do
    [ -d "$old" ] || continue
    [ "$old" = "$BUILD" ] && continue
    echo "pruning $(basename "$old")"
    rm -rf "$old"
done

echo "engine wasm now at $TAG ($(du -sh "$BUILD" | cut -f1))"
