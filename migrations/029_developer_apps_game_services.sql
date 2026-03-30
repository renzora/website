-- 029: Developer apps, OAuth-style grants, game services (achievements, leaderboards, stats, friends)

-- ── Developer Apps ─────────────────────────────────────────────────────────
-- Developers register their game/service here before accessing user data.
CREATE TABLE IF NOT EXISTS developer_apps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    slug VARCHAR(128) NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT '',
    website_url VARCHAR(512) NOT NULL DEFAULT '',
    redirect_uri VARCHAR(512) NOT NULL DEFAULT '',
    client_id VARCHAR(64) NOT NULL UNIQUE,
    client_secret_hash VARCHAR(128) NOT NULL,
    icon_url VARCHAR(512),
    approved BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_developer_apps_owner ON developer_apps(owner_id);
CREATE INDEX IF NOT EXISTS idx_developer_apps_client_id ON developer_apps(client_id);

-- App-scoped API tokens — created by the app developer, scoped to specific permissions
CREATE TABLE IF NOT EXISTS app_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    app_id UUID NOT NULL REFERENCES developer_apps(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    token_hash VARCHAR(128) NOT NULL UNIQUE,
    prefix VARCHAR(16) NOT NULL,
    scopes TEXT[] NOT NULL DEFAULT '{}',
    expires_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_app_tokens_hash ON app_tokens(token_hash);
CREATE INDEX IF NOT EXISTS idx_app_tokens_app ON app_tokens(app_id);

-- ── User Grants ────────────────────────────────────────────────────────────
-- Users grant specific scopes to specific apps. No grant = no access.
CREATE TABLE IF NOT EXISTS app_user_grants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    app_id UUID NOT NULL REFERENCES developer_apps(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    scopes_granted TEXT[] NOT NULL DEFAULT '{}',
    granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (app_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_app_user_grants_user ON app_user_grants(user_id);
CREATE INDEX IF NOT EXISTS idx_app_user_grants_app ON app_user_grants(app_id);

-- ── Friends ────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS friends (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    friend_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status VARCHAR(16) NOT NULL DEFAULT 'pending', -- pending, accepted, blocked
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, friend_id)
);

CREATE INDEX IF NOT EXISTS idx_friends_user ON friends(user_id);
CREATE INDEX IF NOT EXISTS idx_friends_friend ON friends(friend_id);
CREATE INDEX IF NOT EXISTS idx_friends_status ON friends(status);

-- ── Achievements ───────────────────────────────────────────────────────────
-- Defined by the app developer
CREATE TABLE IF NOT EXISTS app_achievements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    app_id UUID NOT NULL REFERENCES developer_apps(id) ON DELETE CASCADE,
    achievement_key VARCHAR(128) NOT NULL,
    name VARCHAR(256) NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    icon_url VARCHAR(512),
    points INT NOT NULL DEFAULT 0,
    hidden BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (app_id, achievement_key)
);

CREATE INDEX IF NOT EXISTS idx_app_achievements_app ON app_achievements(app_id);

-- Unlocked by players
CREATE TABLE IF NOT EXISTS player_achievements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    app_id UUID NOT NULL REFERENCES developer_apps(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    achievement_id UUID NOT NULL REFERENCES app_achievements(id) ON DELETE CASCADE,
    unlocked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metadata JSONB NOT NULL DEFAULT '{}',
    UNIQUE (user_id, achievement_id)
);

CREATE INDEX IF NOT EXISTS idx_player_achievements_user ON player_achievements(user_id);
CREATE INDEX IF NOT EXISTS idx_player_achievements_app ON player_achievements(app_id);

-- ── Player Stats ───────────────────────────────────────────────────────────
-- Generic key-value stats per player per app (play time, kills, wins, etc.)
CREATE TABLE IF NOT EXISTS player_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    app_id UUID NOT NULL REFERENCES developer_apps(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    stat_key VARCHAR(128) NOT NULL,
    value_int BIGINT NOT NULL DEFAULT 0,
    value_float DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (app_id, user_id, stat_key)
);

CREATE INDEX IF NOT EXISTS idx_player_stats_app_user ON player_stats(app_id, user_id);

-- ── Leaderboards ───────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS leaderboards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    app_id UUID NOT NULL REFERENCES developer_apps(id) ON DELETE CASCADE,
    leaderboard_key VARCHAR(128) NOT NULL,
    name VARCHAR(256) NOT NULL,
    sort_order VARCHAR(4) NOT NULL DEFAULT 'desc', -- asc or desc
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (app_id, leaderboard_key)
);

CREATE INDEX IF NOT EXISTS idx_leaderboards_app ON leaderboards(app_id);

CREATE TABLE IF NOT EXISTS leaderboard_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    leaderboard_id UUID NOT NULL REFERENCES leaderboards(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    score BIGINT NOT NULL DEFAULT 0,
    metadata JSONB NOT NULL DEFAULT '{}',
    submitted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (leaderboard_id, user_id)
);

CREATE INDEX IF NOT EXISTS idx_leaderboard_entries_board ON leaderboard_entries(leaderboard_id);
CREATE INDEX IF NOT EXISTS idx_leaderboard_entries_score ON leaderboard_entries(leaderboard_id, score);
