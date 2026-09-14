-- 057: Which engine a release is for, on the release rather than the listing.
--
-- `min_engine_version` lived in `assets.metadata`, one value per LISTING. A
-- listing has many releases, so that value could only ever describe the latest
-- one, and the consequence was worse than it sounds.
--
-- Publish a plugin release that needs r1-alpha8 and the listing's floor moves to
-- r1-alpha8. An r1-alpha7 user is then told the update needs a newer engine --
-- correct -- but they are also cut off from the r1-alpha7 release that still
-- sits in `asset_releases` and works perfectly. A new r1-alpha7 user browsing
-- cannot install the plugin at all. The working version was there the whole
-- time and nothing could hand it to them.
--
-- Moving the value onto the release makes "which version do I get" a question
-- with a per-engine answer: the newest release built for an engine no newer than
-- yours. r1-alpha7 users keep getting r1-alpha7 releases; r1-alpha8 users get
-- r1-alpha8 ones; and the day somebody upgrades their engine, the newer release
-- becomes available without anything being republished.
--
-- No maximum. The ceiling on a release is implied by the next release that
-- declares a higher engine version, so it never has to be written down and can
-- never go stale. A plugin that has not been updated for the current engine
-- keeps being offered, on the assumption that it still works -- which is what
-- cargo and npm do, and the alternative is freezing the whole catalogue on every
-- engine release.

-- ── The engine versions themselves ──────────────────────────────────────────
-- A table rather than a convention, for three reasons that all needed one.
--
-- ORDERING. `r1-alpha10` sorts before `r1-alpha7` as a string, and every
-- comparison below is an ordering question. `ordinal` is the sort key and the
-- string is just a label, so a future `r2-beta1` slots in by being given a
-- number rather than by teaching a parser a new shape.
--
-- THE CLI. `renzora publish` already fetches a catalogue of engine versions and
-- validates a manifest's `min_engine_version` against it. That endpoint does not
-- exist yet, so the check currently passes anything, including a typo. This is
-- the table it should have been reading.
--
-- THE DROPDOWN. Browsing the marketplace as another engine version needs a list
-- of them from somewhere.
CREATE TABLE IF NOT EXISTS engine_versions (
    version     VARCHAR(32) PRIMARY KEY,
    -- What orders releases. Deliberately not derived from the string.
    ordinal     INTEGER NOT NULL UNIQUE,
    -- NULL for a version that exists but has not shipped.
    released_at TIMESTAMPTZ,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO engine_versions (version, ordinal) VALUES
    ('r1-alpha5', 5),
    ('r1-alpha6', 6),
    ('r1-alpha7', 7),
    ('r1-alpha8', 8)
ON CONFLICT (version) DO NOTHING;

-- ── The column ──────────────────────────────────────────────────────────────
-- NULL means "any engine", and that is the honest default rather than a floor
-- invented for it. Most listings predate the field entirely.
ALTER TABLE asset_releases
    ADD COLUMN IF NOT EXISTS min_engine_version VARCHAR(32)
    REFERENCES engine_versions(version);

-- Backfill from the listing's metadata: it is the only statement anybody has
-- made about these releases, and until now it applied to all of them. Only
-- values the catalogue recognises, so a typo in a manifest becomes NULL ("any")
-- rather than a foreign key violation that fails the whole migration.
UPDATE asset_releases r
SET min_engine_version = a.metadata->>'min_engine_version'
FROM assets a
WHERE a.id = r.asset_id
  AND r.min_engine_version IS NULL
  AND a.metadata->>'min_engine_version' IN (SELECT version FROM engine_versions);

-- Resolution filters on this and orders by version, so both are worth indexing.
CREATE INDEX IF NOT EXISTS idx_asset_releases_engine
    ON asset_releases(asset_id, min_engine_version);

-- ── Comparing plugin versions ───────────────────────────────────────────────
-- Resolution picks the newest release BY VERSION, not by date, and the
-- difference is the whole point of being able to maintain an old line: a fix
-- published to the r1-alpha7 branch today is newer in time and older in version
-- than the r1-alpha8 release from last month. Ordering by `created_at` would
-- hand r1-alpha8 users the r1-alpha7 code.
--
-- `1.10.0` must also sort above `1.9.0`, which a string compare gets wrong, so
-- the version becomes an integer array and is compared element by element.
--
-- Any pre-release suffix is trimmed rather than interpreted: `1.2.0-beta.1`
-- sorts as `1.2.0`. Ranking a pre-release against its own release needs the
-- whole of semver's precedence rules, and nothing in this marketplace has ever
-- published one.
CREATE OR REPLACE FUNCTION semver_key(v TEXT) RETURNS INTEGER[]
LANGUAGE sql IMMUTABLE STRICT PARALLEL SAFE AS $$
    SELECT COALESCE(
        string_to_array(regexp_replace(v, '[^0-9.].*$', ''), '.')::INTEGER[],
        ARRAY[0]
    );
$$;
