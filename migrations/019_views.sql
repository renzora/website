-- Add views column to assets and games tables
ALTER TABLE assets ADD COLUMN IF NOT EXISTS views BIGINT NOT NULL DEFAULT 0;
ALTER TABLE games ADD COLUMN IF NOT EXISTS views BIGINT NOT NULL DEFAULT 0;

-- Track unique views to prevent refresh spam
CREATE TABLE IF NOT EXISTS page_views (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_type TEXT NOT NULL,       -- 'asset' or 'game'
    entity_id UUID NOT NULL,
    ip_hash TEXT NOT NULL,           -- hashed IP for privacy
    user_id UUID,                    -- optional, if logged in
    viewed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- One view per IP per entity within the cooldown window
CREATE UNIQUE INDEX IF NOT EXISTS idx_page_views_unique
    ON page_views (entity_type, entity_id, ip_hash);

CREATE INDEX IF NOT EXISTS idx_page_views_entity
    ON page_views (entity_type, entity_id);
