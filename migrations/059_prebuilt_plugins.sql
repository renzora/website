-- 059: plugins compiled by the engine's CI for one exact engine build.
--
-- A native plugin is listed as source and compiled on the machine that installs
-- it. For an official nightly or release that compile has the same result on
-- every machine, since every copy of that build has the same engine images, so
-- CI compiles each first-party plugin once per platform and uploads the result
-- here. An editor of that build downloads the binary instead of needing the
-- 1.9 GB SDK and a Rust toolchain.
--
-- `engine_stamp` is the key, not the engine version. The stamp is a content
-- hash of the engine images a plugin links (the SDK's build id plus rustc), so
-- it changes with every build, and two builds of the same `r1-alpha8` are
-- different stamps whose binaries do not interchange. A local source build has a
-- stamp no row carries, so its editor finds nothing here and installs from
-- source, which is the correct answer for it.
--
-- `signature` is CI's Ed25519 signature over the binary's statement (id,
-- version, platform, engine stamp, SHA-256, size). The site stores it and does
-- not vouch for it: the editor checks it against keys compiled into the engine,
-- so a binary that did not come from CI is refused even if this table is wrong.
CREATE TABLE IF NOT EXISTS prebuilt_plugins (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id       UUID NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    -- The release built from, when its version is one the listing has. NULL
    -- when CI built a version that has not been published as a release yet.
    release_id     UUID REFERENCES asset_releases(id) ON DELETE SET NULL,
    plugin_version VARCHAR(64) NOT NULL,
    -- `windows-x64`, `macos-arm64`, ...: the engine's dist directory names.
    platform       VARCHAR(32) NOT NULL,
    engine_stamp   VARCHAR(160) NOT NULL,
    -- The engine release or nightly the stamp belongs to (`r1-alpha8-nightly-…`),
    -- for people reading a listing; nothing resolves by it.
    engine_tag     VARCHAR(80) NOT NULL DEFAULT '',
    engine_version VARCHAR(32),
    file_name      VARCHAR(255) NOT NULL,
    blob_sha       CHAR(64) NOT NULL REFERENCES storage_blobs(sha256),
    file_key       TEXT NOT NULL,
    byte_size      BIGINT NOT NULL,
    sha256         CHAR(64) NOT NULL,
    signature      CHAR(128) NOT NULL,
    downloads      BIGINT NOT NULL DEFAULT 0,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- One binary per plugin, platform and engine build. A re-run of the same
    -- nightly replaces it rather than adding a second.
    UNIQUE (asset_id, platform, engine_stamp)
);

-- The editor's lookup: this plugin, this build, this platform.
CREATE INDEX IF NOT EXISTS idx_prebuilt_plugins_lookup
    ON prebuilt_plugins(asset_id, engine_stamp, platform);

-- The listing page's: the newest binaries of a plugin.
CREATE INDEX IF NOT EXISTS idx_prebuilt_plugins_recent
    ON prebuilt_plugins(asset_id, created_at DESC);
