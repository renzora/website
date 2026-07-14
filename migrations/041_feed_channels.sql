-- Feed channels — topic rooms for the community feed (the forum is retired and
-- its categories become channels). Channels are NOT user-owned/user-moderated;
-- they only organise the feed. New channels are user-*suggested* (approved=false)
-- and go live once an admin approves them.
--
-- A channel `slug` is a single URL-safe token: lowercase letters, digits,
-- hyphens, and underscores (no spaces). Enforced by the CHECK below and the
-- suggest handler.

CREATE TABLE IF NOT EXISTS channels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(64) NOT NULL,
    slug VARCHAR(48) UNIQUE NOT NULL CHECK (slug ~ '^[a-z0-9_-]+$'),
    description TEXT NOT NULL DEFAULT '',
    icon VARCHAR(64) NOT NULL DEFAULT 'ph-hash',
    sort_order INT NOT NULL DEFAULT 100,
    -- Seeded/approved channels are live; user suggestions start unapproved.
    approved BOOLEAN NOT NULL DEFAULT true,
    suggested_by UUID REFERENCES users(id) ON DELETE SET NULL,
    post_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed the default channels (carried over from the former forum categories).
INSERT INTO channels (name, slug, description, icon, sort_order, approved) VALUES
    ('Announcements', 'announcements', 'Official news and updates', 'ph-megaphone', 1, true),
    ('Introductions', 'introductions', 'Say hello and introduce yourself', 'ph-hand-waving', 2, true),
    ('General', 'general', 'General chat about Renzora and game dev', 'ph-chat-circle', 3, true),
    ('Help & Support', 'help', 'Ask questions and get help with the engine', 'ph-question', 4, true),
    ('Scripting', 'scripting', 'Lua, Rhai, and visual blueprint discussion', 'ph-code', 5, true),
    ('Editor & Tools', 'editor', 'Editor features, workflows, and panel tips', 'ph-cube', 6, true),
    ('Materials & Shaders', 'shaders', 'Material graphs, WGSL shaders, and rendering', 'ph-drop', 7, true),
    ('Networking', 'networking', 'Multiplayer, servers, and replication', 'ph-wifi-high', 8, true),
    ('Showcase', 'showcase', 'Show off your games, prototypes, and experiments', 'ph-monitor-play', 9, true),
    ('Work In Progress', 'wip', 'Share what you are working on', 'ph-hammer', 10, true),
    ('Tutorials & Resources', 'tutorials', 'Community guides and learning resources', 'ph-graduation-cap', 11, true),
    ('Marketplace Discussion', 'marketplace-discuss', 'Asset feedback and seller support', 'ph-storefront', 12, true),
    ('Feature Requests', 'feature-requests', 'Suggest new engine features', 'ph-lightbulb', 13, true),
    ('Bug Reports', 'bugs', 'Report engine bugs', 'ph-bug', 14, true)
ON CONFLICT (slug) DO NOTHING;

-- Posts optionally belong to a channel.
ALTER TABLE posts ADD COLUMN IF NOT EXISTS channel_id UUID REFERENCES channels(id) ON DELETE SET NULL;
CREATE INDEX IF NOT EXISTS idx_posts_channel ON posts(channel_id);
