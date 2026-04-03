-- XP and Level system
ALTER TABLE users ADD COLUMN IF NOT EXISTS total_xp BIGINT NOT NULL DEFAULT 0;
ALTER TABLE users ADD COLUMN IF NOT EXISTS level INT NOT NULL DEFAULT 1;
ALTER TABLE users ADD COLUMN IF NOT EXISTS seller_level INT NOT NULL DEFAULT 0;
ALTER TABLE users ADD COLUMN IF NOT EXISTS seller_xp BIGINT NOT NULL DEFAULT 0;

CREATE INDEX IF NOT EXISTS idx_users_level ON users(level);
CREATE INDEX IF NOT EXISTS idx_users_seller_level ON users(seller_level);

-- XP history log
CREATE TABLE xp_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount BIGINT NOT NULL,
    reason VARCHAR(64) NOT NULL,
    source_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_xp_events_user ON xp_events(user_id);

-- Seller level definitions
CREATE TABLE seller_levels (
    level INT PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    min_seller_xp BIGINT NOT NULL DEFAULT 0,
    search_boost REAL NOT NULL DEFAULT 1.0,
    badge_color VARCHAR(7) NOT NULL DEFAULT '#6366f1',
    perks TEXT NOT NULL DEFAULT ''
);

-- Seller tasks — milestones for each level
CREATE TABLE seller_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    seller_level INT NOT NULL REFERENCES seller_levels(level),
    description TEXT NOT NULL,
    task_type VARCHAR(32) NOT NULL,
    target_value BIGINT NOT NULL,
    xp_reward BIGINT NOT NULL DEFAULT 0,
    sort_order INT NOT NULL DEFAULT 0
);

-- Seed seller levels
INSERT INTO seller_levels (level, name, min_seller_xp, search_boost, badge_color, perks) VALUES
(0, 'New Seller', 0, 1.0, '#71717a', 'Basic listing'),
(1, 'Bronze Seller', 100, 1.1, '#CD7F32', '10% search boost, seller badge'),
(2, 'Silver Seller', 500, 1.25, '#C0C0C0', '25% search boost, featured eligible'),
(3, 'Gold Seller', 2000, 1.5, '#FFD700', '50% search boost, priority support'),
(4, 'Platinum Seller', 5000, 2.0, '#E5E4E2', '2x search boost, homepage featured'),
(5, 'Diamond Seller', 15000, 3.0, '#B9F2FF', '3x search boost, verified badge, custom storefront');

-- Seed seller tasks
INSERT INTO seller_tasks (seller_level, description, task_type, target_value, xp_reward, sort_order) VALUES
-- Level 0 → 1
(0, 'Upload your first asset', 'assets_uploaded', 1, 25, 0),
(0, 'Make your first sale', 'total_sales', 1, 50, 1),
(0, 'Earn 50 credits from sales', 'total_revenue', 50, 25, 2),
-- Level 1 → 2
(1, 'Upload 5 assets', 'assets_uploaded', 5, 50, 0),
(1, 'Make 10 sales', 'total_sales', 10, 100, 1),
(1, 'Earn 500 credits from sales', 'total_revenue', 500, 100, 2),
(1, 'Get 5 positive reviews', 'positive_reviews', 5, 50, 3),
-- Level 2 → 3
(2, 'Upload 15 assets', 'assets_uploaded', 15, 100, 0),
(2, 'Make 50 sales', 'total_sales', 50, 200, 1),
(2, 'Earn 2000 credits from sales', 'total_revenue', 2000, 200, 2),
(2, 'Get 25 positive reviews', 'positive_reviews', 25, 100, 3),
(2, 'Have an asset with 100+ downloads', 'top_asset_downloads', 100, 150, 4),
-- Level 3 → 4
(3, 'Upload 30 assets', 'assets_uploaded', 30, 200, 0),
(3, 'Make 200 sales', 'total_sales', 200, 400, 1),
(3, 'Earn 10000 credits from sales', 'total_revenue', 10000, 500, 2),
(3, 'Get 100 positive reviews', 'positive_reviews', 100, 300, 3),
-- Level 4 → 5
(4, 'Make 500 sales', 'total_sales', 500, 1000, 0),
(4, 'Earn 50000 credits from sales', 'total_revenue', 50000, 1500, 1),
(4, 'Get 500 positive reviews', 'positive_reviews', 500, 500, 2),
(4, 'Have an asset with 1000+ downloads', 'top_asset_downloads', 1000, 500, 3);

-- XP level thresholds (computed in code, but here for reference):
-- Level 1: 0 XP
-- Level 2: 100 XP
-- Level 3: 300 XP
-- Level 4: 600 XP
-- Level 5: 1000 XP
-- Level N: N*(N-1)*50 XP
-- (formula: required_xp_for_level(n) = n*(n-1)*50)
