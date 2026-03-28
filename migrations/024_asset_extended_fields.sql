-- Extended asset metadata for comprehensive marketplace listings.

-- Licence type (e.g. CC0, MIT, Standard, Extended)
ALTER TABLE assets ADD COLUMN IF NOT EXISTS licence VARCHAR(32) NOT NULL DEFAULT 'standard';

-- AI content declaration (true = contains AI-generated content)
ALTER TABLE assets ADD COLUMN IF NOT EXISTS ai_generated BOOLEAN NOT NULL DEFAULT false;

-- Material/shader specific metadata (JSON for flexibility)
-- e.g. {"render_pipeline":"pbr","texture_resolution":"2048x2048","poly_count":1500}
ALTER TABLE assets ADD COLUMN IF NOT EXISTS metadata JSONB NOT NULL DEFAULT '{}';

-- Extend asset_media to support audio type
-- media_type can now be 'image', 'video', or 'audio'
ALTER TABLE asset_media ALTER COLUMN media_type TYPE VARCHAR(16);

-- Add views column if somehow missing
ALTER TABLE assets ADD COLUMN IF NOT EXISTS views BIGINT NOT NULL DEFAULT 0;
