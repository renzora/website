-- Gallery media for marketplace assets (screenshots and videos).
CREATE TABLE IF NOT EXISTS asset_media (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id UUID NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    media_type VARCHAR(8) NOT NULL DEFAULT 'image',  -- 'image' or 'video'
    url TEXT NOT NULL,
    thumbnail_url TEXT,  -- video poster / thumbnail override
    sort_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_asset_media_asset ON asset_media(asset_id);
