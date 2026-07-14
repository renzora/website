-- Network-wide, report-driven moderation for feed posts.
--
-- A post auto-hides once enough *distinct* users report it (threshold lives in
-- the API as a tunable constant). A hidden post disappears from the feed; its
-- author can request a manual staff review. Network-wide volunteer moderators
-- (the `moderator` role) clear the queue — there is no per-channel moderation.

ALTER TABLE posts ADD COLUMN IF NOT EXISTS hidden BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE posts ADD COLUMN IF NOT EXISTS report_count INT NOT NULL DEFAULT 0;
ALTER TABLE posts ADD COLUMN IF NOT EXISTS review_requested BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE posts ADD COLUMN IF NOT EXISTS hidden_at TIMESTAMPTZ;

-- One report per (post, reporter). `reason` is a short free-text category.
CREATE TABLE IF NOT EXISTS post_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    reporter_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reason TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (post_id, reporter_id)
);

-- The moderator review queue = hidden posts (esp. those whose author asked for
-- review). Partial index keeps that lookup cheap.
CREATE INDEX IF NOT EXISTS idx_posts_hidden ON posts(hidden_at) WHERE hidden = true;
CREATE INDEX IF NOT EXISTS idx_post_reports_post ON post_reports(post_id);
