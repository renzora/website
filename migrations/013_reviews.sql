-- Rating and review system for marketplace assets.
-- Only asset owners (purchasers) can leave a review. One review per user per asset.

CREATE TABLE IF NOT EXISTS asset_reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id UUID NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id),
    rating INT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    title VARCHAR(128) NOT NULL DEFAULT '',
    content TEXT NOT NULL DEFAULT '',
    helpful_count INT NOT NULL DEFAULT 0,
    flagged BOOLEAN NOT NULL DEFAULT false,       -- flagged for moderation
    flag_reason TEXT,
    hidden BOOLEAN NOT NULL DEFAULT false,        -- hidden by admin
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(asset_id, author_id)                   -- one review per user per asset
);

-- Add rating summary columns to assets
ALTER TABLE assets ADD COLUMN IF NOT EXISTS rating_sum BIGINT NOT NULL DEFAULT 0;
ALTER TABLE assets ADD COLUMN IF NOT EXISTS rating_count INT NOT NULL DEFAULT 0;

CREATE INDEX IF NOT EXISTS idx_asset_reviews_asset ON asset_reviews(asset_id);
CREATE INDEX IF NOT EXISTS idx_asset_reviews_author ON asset_reviews(author_id);
CREATE INDEX IF NOT EXISTS idx_asset_reviews_flagged ON asset_reviews(flagged) WHERE flagged = true;
