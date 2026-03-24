-- Games table (parallel to assets, but for publishable games)
CREATE TABLE IF NOT EXISTS games (
    id UUID PRIMARY KEY,
    creator_id UUID NOT NULL REFERENCES users(id),
    name VARCHAR(128) NOT NULL,
    slug VARCHAR(160) NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT '',
    category VARCHAR(32) NOT NULL DEFAULT 'other',
    price_credits BIGINT NOT NULL DEFAULT 0,
    file_url TEXT,
    thumbnail_url TEXT,
    version VARCHAR(32) NOT NULL DEFAULT '1.0.0',
    downloads BIGINT NOT NULL DEFAULT 0,
    published BOOLEAN NOT NULL DEFAULT false,
    rating_sum BIGINT NOT NULL DEFAULT 0,
    rating_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_games_creator ON games(creator_id);
CREATE INDEX idx_games_category ON games(category);
CREATE INDEX idx_games_published ON games(published);
CREATE INDEX idx_games_slug ON games(slug);

-- User game ownership (library)
CREATE TABLE IF NOT EXISTS user_games (
    user_id UUID NOT NULL REFERENCES users(id),
    game_id UUID NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    purchased_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, game_id)
);

-- Game reviews
CREATE TABLE IF NOT EXISTS game_reviews (
    id UUID PRIMARY KEY,
    game_id UUID NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id),
    rating INT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    title VARCHAR(128) NOT NULL DEFAULT '',
    content TEXT NOT NULL DEFAULT '',
    helpful_count INT NOT NULL DEFAULT 0,
    flagged BOOLEAN NOT NULL DEFAULT false,
    flag_reason TEXT,
    hidden BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(game_id, author_id)
);

CREATE INDEX idx_game_reviews_game ON game_reviews(game_id);

-- Game comments
CREATE TABLE IF NOT EXISTS game_comments (
    id UUID PRIMARY KEY,
    game_id UUID NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_game_comments_game ON game_comments(game_id);

-- Game media (screenshots, videos)
CREATE TABLE IF NOT EXISTS game_media (
    id UUID PRIMARY KEY,
    game_id UUID NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    media_type VARCHAR(8) NOT NULL DEFAULT 'image',
    url TEXT NOT NULL,
    thumbnail_url TEXT,
    sort_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_game_media_game ON game_media(game_id);

-- Game categories
CREATE TABLE IF NOT EXISTS game_categories (
    id UUID PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    slug VARCHAR(64) NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT '',
    icon VARCHAR(32) NOT NULL DEFAULT '',
    sort_order INT NOT NULL DEFAULT 0,
    max_file_size_mb INT NOT NULL DEFAULT 2048,
    allowed_extensions TEXT[] NOT NULL DEFAULT ARRAY['zip', 'exe', 'tar.gz', 'dmg', 'appimage'],
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed default game categories
INSERT INTO game_categories (id, name, slug, description, icon, sort_order) VALUES
    (gen_random_uuid(), 'Action', 'action', 'Fast-paced action games', 'ph-sword', 1),
    (gen_random_uuid(), 'Adventure', 'adventure', 'Story-driven adventure games', 'ph-compass', 2),
    (gen_random_uuid(), 'RPG', 'rpg', 'Role-playing games', 'ph-shield', 3),
    (gen_random_uuid(), 'Puzzle', 'puzzle', 'Brain teasers and puzzles', 'ph-puzzle-piece', 4),
    (gen_random_uuid(), 'Simulation', 'simulation', 'Simulation and management games', 'ph-buildings', 5),
    (gen_random_uuid(), 'Strategy', 'strategy', 'Strategic planning games', 'ph-chess-rook', 6),
    (gen_random_uuid(), 'Platformer', 'platformer', 'Jump and run platformers', 'ph-person-simple-run', 7),
    (gen_random_uuid(), 'Multiplayer', 'multiplayer', 'Online multiplayer games', 'ph-users-three', 8),
    (gen_random_uuid(), 'Sandbox', 'sandbox', 'Open world sandbox games', 'ph-cube', 9),
    (gen_random_uuid(), 'Other', 'other', 'Other games', 'ph-game-controller', 10)
ON CONFLICT DO NOTHING;
