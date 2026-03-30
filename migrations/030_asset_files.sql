-- Multi-file asset support: individual files per asset with private storage keys
CREATE TABLE IF NOT EXISTS asset_files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id UUID NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    file_key TEXT NOT NULL,
    preview_key TEXT,
    original_filename TEXT NOT NULL,
    file_size BIGINT NOT NULL DEFAULT 0,
    mime_type VARCHAR(128) NOT NULL DEFAULT 'application/octet-stream',
    sort_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_asset_files_asset ON asset_files(asset_id);

-- Track multi-file vs single-file assets
ALTER TABLE assets ADD COLUMN IF NOT EXISTS multi_file BOOLEAN NOT NULL DEFAULT false;

-- Preserve original public URLs during migration to private storage
ALTER TABLE assets ADD COLUMN IF NOT EXISTS legacy_file_url TEXT;

-- Snapshot existing public URLs
UPDATE assets SET legacy_file_url = file_url WHERE file_url IS NOT NULL AND legacy_file_url IS NULL;
