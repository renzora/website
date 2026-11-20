-- 060: whether each plugin release compiled against each official engine build.
--
-- The engine's CI compiles every marketplace plugin, third-party ones included,
-- against every nightly and stable build, from the source its creator uploaded
-- and the official SDK (never the creator's own engine, which could be anything).
-- A plugin that compiles also gets a prebuilt binary (`prebuilt_plugins`); this
-- table records every attempt, and above all the ones that failed, so a creator
-- finds out that a new engine build broke their plugin from its dashboard rather
-- than from their users.
CREATE TABLE IF NOT EXISTS plugin_builds (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id       UUID NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    release_id     UUID REFERENCES asset_releases(id) ON DELETE SET NULL,
    plugin_version VARCHAR(64) NOT NULL,
    platform       VARCHAR(32) NOT NULL,
    engine_stamp   VARCHAR(160) NOT NULL,
    engine_tag     VARCHAR(80) NOT NULL DEFAULT '',
    ok             BOOLEAN NOT NULL,
    -- The first line of the compiler's complaint, for lists.
    summary        TEXT NOT NULL DEFAULT '',
    -- The compiler's output, capped, for the creator to read.
    log            TEXT NOT NULL DEFAULT '',
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- One result per plugin, platform and engine build; a re-run replaces it.
    UNIQUE (asset_id, platform, engine_stamp)
);

CREATE INDEX IF NOT EXISTS idx_plugin_builds_recent
    ON plugin_builds(asset_id, created_at DESC);
