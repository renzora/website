-- Badge auto-award rules
ALTER TABLE badges ADD COLUMN IF NOT EXISTS auto_rule TEXT;
ALTER TABLE badges ADD COLUMN IF NOT EXISTS auto_threshold BIGINT;

-- Wishlists for game store
CREATE TABLE IF NOT EXISTS wishlists (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    game_id UUID NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, game_id)
);
CREATE INDEX IF NOT EXISTS idx_wishlists_user ON wishlists(user_id);

-- Articles improvements for WYSIWYG
ALTER TABLE articles ADD COLUMN IF NOT EXISTS content_html TEXT NOT NULL DEFAULT '';
