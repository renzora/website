-- 058: store a file's bytes once, however many releases contain them.
--
-- Every release uploaded its own copy of every file. `storage_key` put a fresh
-- UUID in the object name, so two releases of a plugin that differ in one line
-- stored two full copies of everything else, and republishing a 400 MB model
-- with a corrected README stored the model again.
--
-- Addressing a blob by the SHA-256 of its contents makes that impossible: the
-- same bytes are the same object, so a release costs only what is new in it.
--
-- This is the saving the "host releases in a git server" idea was after, and it
-- gets it for binaries too, which is where git gives none: git delta-compresses
-- text well and stores a changed PNG as a whole new object forever. Measured on
-- renzora/plugins, whose 2.7 MB of source and 7.2 MB of thumbnails had grown a
-- 65 MB .git in 25 commits.

-- ── The blobs ───────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS storage_blobs (
    -- Lowercase hex SHA-256 of the file's contents. The identity of the bytes,
    -- and the reason two uploads of the same file are one row.
    sha256       CHAR(64) PRIMARY KEY,
    -- Where the object actually lives. Kept as a column rather than derived
    -- from the hash so the layout can change without rewriting every row, and
    -- so pre-existing objects could be adopted in place if that is ever wanted.
    storage_key  TEXT NOT NULL,
    byte_size    BIGINT NOT NULL,
    content_type VARCHAR(128) NOT NULL DEFAULT 'application/octet-stream',
    -- NULL until the bytes are known to be in storage.
    --
    -- Two publishes of the same new file race: one inserts the row and starts a
    -- long upload, the other finds the row and would otherwise skip uploading
    -- and hand out a key to an object that is not there yet. A reader that
    -- finds this NULL uploads anyway, which is safe because the bytes are
    -- identical by construction.
    uploaded_at  TIMESTAMPTZ,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ── The reference ───────────────────────────────────────────────────────────
-- Nullable, and no backfill. Every existing row keeps its own object and its
-- own `file_key`, and a NULL here means "this file predates content
-- addressing": it is deleted the old way, on its own, because nothing else can
-- be sharing it. Hashing what is already stored would mean downloading the
-- entire bucket, and it buys nothing until those files are republished.
ALTER TABLE asset_files
    ADD COLUMN IF NOT EXISTS blob_sha CHAR(64) REFERENCES storage_blobs(sha256);

-- Deletion asks "does anything else still point at this blob?" on every removed
-- file, so that lookup has to be indexed.
CREATE INDEX IF NOT EXISTS idx_asset_files_blob ON asset_files(blob_sha);
