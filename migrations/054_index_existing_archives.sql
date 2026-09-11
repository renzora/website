-- 054: Make archive indexing idempotent, so it can be filled in lazily.
--
-- Migration 053 gave existing assets a release and a path, but it could not
-- index what is *inside* an already-uploaded zip: the bytes live in object
-- storage, which SQL can't reach. Those releases are indexed on demand instead,
-- the first time someone opens the file browser.
--
-- Two readers can race on that, so the indexed rows get a uniqueness rule and
-- the insert becomes ON CONFLICT DO NOTHING. Indexing the same archive twice
-- is then harmless rather than duplicating every entry.
CREATE UNIQUE INDEX IF NOT EXISTS idx_asset_files_indexed_unique
    ON asset_files(release_id, path) WHERE archived;
