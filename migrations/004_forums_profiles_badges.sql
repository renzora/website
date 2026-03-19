-- Forum categories (admin-managed)
CREATE TABLE IF NOT EXISTS forum_categories (
    id UUID PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    slug VARCHAR(64) NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT '',
    icon VARCHAR(64) NOT NULL DEFAULT 'ph-chat-circle',
    sort_order INT NOT NULL DEFAULT 0,
    thread_count INT NOT NULL DEFAULT 0,
    post_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed default forum categories
INSERT INTO forum_categories (id, name, slug, description, icon, sort_order) VALUES
    (gen_random_uuid(), 'General', 'general', 'General discussion about Renzora', 'ph-chat-circle', 1),
    (gen_random_uuid(), 'Help & Support', 'help', 'Ask questions and get help', 'ph-question', 2),
    (gen_random_uuid(), 'Showcase', 'showcase', 'Show off your projects and creations', 'ph-image', 3),
    (gen_random_uuid(), 'Tutorials', 'tutorials', 'Share and find community tutorials', 'ph-graduation-cap', 4),
    (gen_random_uuid(), 'Plugins & Assets', 'plugins', 'Discussion about marketplace items', 'ph-puzzle-piece', 5),
    (gen_random_uuid(), 'Feature Requests', 'feature-requests', 'Suggest new engine features', 'ph-lightbulb', 6),
    (gen_random_uuid(), 'Bug Reports', 'bugs', 'Report engine bugs', 'ph-bug', 7)
ON CONFLICT DO NOTHING;

-- Forum threads
CREATE TABLE IF NOT EXISTS forum_threads (
    id UUID PRIMARY KEY,
    category_id UUID NOT NULL REFERENCES forum_categories(id),
    author_id UUID NOT NULL REFERENCES users(id),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    pinned BOOLEAN NOT NULL DEFAULT false,
    locked BOOLEAN NOT NULL DEFAULT false,
    post_count INT NOT NULL DEFAULT 1,
    last_post_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_post_by UUID REFERENCES users(id),
    views INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_threads_category ON forum_threads(category_id);
CREATE INDEX idx_threads_author ON forum_threads(author_id);

-- Forum posts (first post = thread body, rest = replies)
CREATE TABLE IF NOT EXISTS forum_posts (
    id UUID PRIMARY KEY,
    thread_id UUID NOT NULL REFERENCES forum_threads(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    is_first_post BOOLEAN NOT NULL DEFAULT false,
    edited BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_posts_thread ON forum_posts(thread_id);

-- Notifications
CREATE TABLE IF NOT EXISTS notifications (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    type VARCHAR(32) NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL DEFAULT '',
    link TEXT,
    read BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_notifications_user ON notifications(user_id, read);

-- Follows
CREATE TABLE IF NOT EXISTS follows (
    follower_id UUID NOT NULL REFERENCES users(id),
    following_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (follower_id, following_id)
);

CREATE INDEX idx_follows_following ON follows(following_id);

-- Badges
CREATE TABLE IF NOT EXISTS badges (
    id UUID PRIMARY KEY,
    slug VARCHAR(64) NOT NULL UNIQUE,
    name VARCHAR(64) NOT NULL,
    description TEXT NOT NULL,
    icon VARCHAR(64) NOT NULL DEFAULT 'ph-medal',
    color VARCHAR(16) NOT NULL DEFAULT '#6366f1',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed badges
INSERT INTO badges (id, slug, name, description, icon, color) VALUES
    (gen_random_uuid(), 'early-adopter', 'Early Adopter', 'Joined during alpha', 'ph-rocket', '#f59e0b'),
    (gen_random_uuid(), 'one-year', '1 Year', 'Member for 1 year', 'ph-calendar', '#6366f1'),
    (gen_random_uuid(), 'first-purchase', 'First Purchase', 'Bought your first asset', 'ph-shopping-cart', '#10b981'),
    (gen_random_uuid(), 'big-spender', 'Big Spender', 'Bought 10+ assets', 'ph-crown', '#f59e0b'),
    (gen_random_uuid(), 'creator', 'Creator', 'Published your first asset', 'ph-paint-brush', '#8b5cf6'),
    (gen_random_uuid(), 'top-seller', 'Top Seller', 'Sold 10+ assets', 'ph-star', '#ef4444'),
    (gen_random_uuid(), 'contributor', 'Contributor', 'Contributed to the engine', 'ph-git-pull-request', '#06b6d4'),
    (gen_random_uuid(), 'prolific-poster', 'Prolific Poster', '100+ forum posts', 'ph-chat-circle-text', '#ec4899'),
    (gen_random_uuid(), 'helpful', 'Helpful', '10+ replies marked as helpful', 'ph-heart', '#f43f5e'),
    (gen_random_uuid(), 'writer', 'Writer', 'Published 5+ community articles', 'ph-pencil-line', '#14b8a6')
ON CONFLICT DO NOTHING;

-- User badges
CREATE TABLE IF NOT EXISTS user_badges (
    user_id UUID NOT NULL REFERENCES users(id),
    badge_id UUID NOT NULL REFERENCES badges(id),
    awarded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, badge_id)
);

-- Add profile fields to users
ALTER TABLE users ADD COLUMN IF NOT EXISTS bio TEXT NOT NULL DEFAULT '';
ALTER TABLE users ADD COLUMN IF NOT EXISTS website TEXT NOT NULL DEFAULT '';
ALTER TABLE users ADD COLUMN IF NOT EXISTS follower_count INT NOT NULL DEFAULT 0;
ALTER TABLE users ADD COLUMN IF NOT EXISTS following_count INT NOT NULL DEFAULT 0;
ALTER TABLE users ADD COLUMN IF NOT EXISTS post_count INT NOT NULL DEFAULT 0;
