-- Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username VARCHAR(32) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    avatar_url TEXT,
    role VARCHAR(16) NOT NULL DEFAULT 'user',
    credit_balance BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);

-- Assets table (marketplace)
CREATE TABLE IF NOT EXISTS assets (
    id UUID PRIMARY KEY,
    creator_id UUID NOT NULL REFERENCES users(id),
    name VARCHAR(128) NOT NULL,
    slug VARCHAR(128) NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT '',
    category VARCHAR(16) NOT NULL,
    price_credits BIGINT NOT NULL DEFAULT 0,
    file_url TEXT,
    thumbnail_url TEXT,
    version VARCHAR(32) NOT NULL DEFAULT '1.0.0',
    downloads BIGINT NOT NULL DEFAULT 0,
    published BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_assets_creator ON assets(creator_id);
CREATE INDEX idx_assets_category ON assets(category);
CREATE INDEX idx_assets_slug ON assets(slug);

-- Transactions table (credits)
CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    type VARCHAR(16) NOT NULL,
    amount BIGINT NOT NULL,
    asset_id UUID REFERENCES assets(id),
    stripe_payment_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_transactions_user ON transactions(user_id);

-- User asset ownership
CREATE TABLE IF NOT EXISTS user_assets (
    user_id UUID NOT NULL REFERENCES users(id),
    asset_id UUID NOT NULL REFERENCES assets(id),
    purchased_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, asset_id)
);
