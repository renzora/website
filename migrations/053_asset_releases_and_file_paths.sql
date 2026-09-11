-- 050: Asset releases (version history) and archive-relative file paths.
--
-- Two changes that work together:
--   1. `asset_releases` gives every asset a version history. Files belong to a
--      release rather than directly to the asset, so shipping v1.2.0 no longer
--      destroys v1.1.0 — buyers can still download what they bought.
--   2. `asset_files.path` keeps the file's path *inside* the uploaded archive
--      (`docs/install.md`, not `install.md`), which is what the marketplace
--      file tree and the README/docs viewer are built on.

-- ── Releases ───────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS asset_releases (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id UUID NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    version VARCHAR(32) NOT NULL,
    -- Release notes, markdown. Seeded from a CHANGELOG.md in the archive when
    -- the creator doesn't write any.
    notes TEXT NOT NULL DEFAULT '',
    is_current BOOLEAN NOT NULL DEFAULT false,
    downloads BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (asset_id, version)
);

CREATE INDEX IF NOT EXISTS idx_asset_releases_asset ON asset_releases(asset_id, created_at DESC);
-- At most one current release per asset, enforced by the database rather than
-- by whoever remembers to clear the old flag first.
CREATE UNIQUE INDEX IF NOT EXISTS idx_asset_releases_current
    ON asset_releases(asset_id) WHERE is_current;

-- ── Files belong to a release, and remember where they sat in the archive ──
ALTER TABLE asset_files ADD COLUMN IF NOT EXISTS release_id UUID
    REFERENCES asset_releases(id) ON DELETE CASCADE;
ALTER TABLE asset_files ADD COLUMN IF NOT EXISTS path TEXT NOT NULL DEFAULT '';

CREATE INDEX IF NOT EXISTS idx_asset_files_release ON asset_files(release_id);
CREATE INDEX IF NOT EXISTS idx_asset_files_release_path ON asset_files(release_id, path);

-- ── Backfill ───────────────────────────────────────────────────────────────
-- Every asset that already has files gets one release, at the asset's current
-- version, holding them. Assets with no files (legacy single-URL uploads) get
-- nothing — the download path still falls back to `legacy_file_url` for those.
INSERT INTO asset_releases (asset_id, version, is_current, created_at)
SELECT a.id, a.version, true, a.created_at
FROM assets a
WHERE EXISTS (SELECT 1 FROM asset_files f WHERE f.asset_id = a.id)
ON CONFLICT (asset_id, version) DO NOTHING;

UPDATE asset_files f
SET release_id = r.id
FROM asset_releases r
WHERE r.asset_id = f.asset_id AND r.is_current AND f.release_id IS NULL;

-- Pre-existing rows were stored flattened (the zip extractor threw the
-- directories away), so the path is just the filename.
UPDATE asset_files SET path = original_filename WHERE path = '';

-- ── Cached doc text ────────────────────────────────────────────────────────
-- Markdown and licence files are public (that's the documentation system), so
-- their text is kept alongside the row. The asset page can then render a
-- README straight from the database instead of round-tripping to object
-- storage on every view. NULL means "not cached yet" — pre-existing rows are
-- filled in lazily the first time they're read.
ALTER TABLE asset_files ADD COLUMN IF NOT EXISTS text_content TEXT;

-- ── Files indexed inside a stored archive ──────────────────────────────────
-- A plugin ships as one zip of buildable source, and those bytes must survive
-- the round trip untouched — the editor extracts the archive into
-- `plugins/<crate>/` and the SDK builds it. So the zip stays the single stored
-- object, and its contents are *indexed* rather than stored separately: an
-- archived row's `file_key` points at the containing zip and its `path` is the
-- entry inside it.
--
-- Archived rows are for browsing and documentation only. Downloads, previews
-- and the legacy `file_url` all look at deliverables (archived = false).
ALTER TABLE asset_files ADD COLUMN IF NOT EXISTS archived BOOLEAN NOT NULL DEFAULT false;

CREATE INDEX IF NOT EXISTS idx_asset_files_release_deliverables
    ON asset_files(release_id) WHERE NOT archived;
