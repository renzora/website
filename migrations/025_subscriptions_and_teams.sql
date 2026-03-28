-- ── Subscription Plans (credit-based) ──
CREATE TABLE IF NOT EXISTS subscription_plans (
    id VARCHAR(32) PRIMARY KEY,
    name VARCHAR(64) NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    price_credits INT NOT NULL DEFAULT 0,
    -- Limits
    daily_api_limit INT NOT NULL DEFAULT 500,
    storage_mb INT NOT NULL DEFAULT 0,
    max_team_members INT NOT NULL DEFAULT 0,
    max_file_size_mb INT NOT NULL DEFAULT 500,
    -- Add-on pricing
    extra_seat_credits INT NOT NULL DEFAULT 0,
    extra_storage_credits_per_gb INT NOT NULL DEFAULT 0,
    -- Marketplace
    commission_percent INT NOT NULL DEFAULT 30,  -- platform take (seller gets 100 - this)
    library_assets_per_month INT NOT NULL DEFAULT 0,  -- 0 = no library access
    search_boost INT NOT NULL DEFAULT 0,  -- 0=none, 1=slight, 2=boosted
    asset_spotlights_per_month INT NOT NULL DEFAULT 0,
    -- Xbox porting
    xbox_builds_per_month INT NOT NULL DEFAULT 0,
    xbox_build_cost_credits INT NOT NULL DEFAULT 0,  -- 0 = not available
    xbox_submission_cost_credits INT NOT NULL DEFAULT 0,
    -- Profile
    profile_badge VARCHAR(32) NOT NULL DEFAULT '',
    profile_customization VARCHAR(16) NOT NULL DEFAULT 'basic',  -- basic, custom, verified
    -- Features
    features JSONB NOT NULL DEFAULT '[]',
    sort_order INT NOT NULL DEFAULT 0
);

INSERT INTO subscription_plans (
    id, name, description, price_credits,
    daily_api_limit, storage_mb, max_team_members, max_file_size_mb,
    extra_seat_credits, extra_storage_credits_per_gb,
    commission_percent, library_assets_per_month, search_boost, asset_spotlights_per_month,
    xbox_builds_per_month, xbox_build_cost_credits, xbox_submission_cost_credits,
    profile_badge, profile_customization,
    features, sort_order
) VALUES
    ('free', 'Free', 'For hobbyists getting started', 0,
     500, 0, 0, 500,
     0, 0,
     30, 0, 0, 0,
     0, 0, 0,
     '', 'basic',
     '["marketplace_access","community_support","basic_analytics"]', 1),

    ('pro', 'Pro', 'For solo creators and developers', 50,
     5000, 10240, 0, 2048,
     0, 1,
     20, 0, 0, 0,
     0, 200, 100,
     'pro', 'custom',
     '["marketplace_access","priority_support","full_analytics","custom_storefront","api_access","custom_profile","xbox_porting","early_access","scheduled_publishing","private_assets","priority_build_queue","premium_forums","cloud_engine","discord_role","creator_pool"]', 2),

    ('indie', 'Indie', 'For small teams and indie studios', 150,
     10000, 51200, 5, 5120,
     20, 1,
     15, 30, 1, 1,
     1, 200, 50,
     'indie', 'custom',
     '["marketplace_access","priority_support","full_analytics","custom_storefront","api_access","custom_profile","team_management","xbox_porting","early_access","scheduled_publishing","private_assets","priority_build_queue","premium_forums","cloud_engine","discord_role","team_library","asset_spotlight","creator_pool"]', 3),

    ('studio', 'Studio', 'For professional studios', 400,
     25000, 204800, 20, 10240,
     15, 1,
     10, 100, 2, 3,
     3, 150, 0,
     'studio', 'verified',
     '["marketplace_access","dedicated_support","full_analytics","custom_storefront","api_access","custom_profile","team_management","bulk_uploads","xbox_porting","early_access","beta_access","scheduled_publishing","private_assets","priority_build_queue","premium_forums","cloud_engine","discord_role","team_library","asset_spotlight","verified_badge","creator_pool"]', 4)
ON CONFLICT (id) DO NOTHING;

-- ── User Subscriptions (credit-based) ──
CREATE TABLE IF NOT EXISTS subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    plan_id VARCHAR(32) NOT NULL REFERENCES subscription_plans(id),
    status VARCHAR(16) NOT NULL DEFAULT 'active',  -- active, canceled, expired
    extra_seats INT NOT NULL DEFAULT 0,
    extra_storage_gb INT NOT NULL DEFAULT 0,
    current_period_start TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    current_period_end TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '30 days',
    cancel_at_period_end BOOLEAN NOT NULL DEFAULT false,
    auto_renew BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id)
);

CREATE INDEX idx_subscriptions_user ON subscriptions(user_id);
CREATE INDEX idx_subscriptions_status ON subscriptions(status);

-- ── Auto Top-Up Settings ──
CREATE TABLE IF NOT EXISTS auto_topup (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    enabled BOOLEAN NOT NULL DEFAULT false,
    threshold_credits INT NOT NULL DEFAULT 100,
    topup_amount_credits INT NOT NULL DEFAULT 500,
    stripe_payment_method_id VARCHAR(128),
    stripe_customer_id VARCHAR(128),
    last_topup_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ── Teams ──
CREATE TABLE IF NOT EXISTS teams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(128) NOT NULL,
    slug VARCHAR(128) NOT NULL UNIQUE,
    owner_id UUID NOT NULL REFERENCES users(id),
    avatar_url TEXT,
    description TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_teams_owner ON teams(owner_id);
CREATE INDEX idx_teams_slug ON teams(slug);

-- ── Team Members ──
CREATE TABLE IF NOT EXISTS team_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(16) NOT NULL DEFAULT 'member',  -- owner, admin, member
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(team_id, user_id)
);

CREATE INDEX idx_team_members_team ON team_members(team_id);
CREATE INDEX idx_team_members_user ON team_members(user_id);

-- ── Team Invites ──
CREATE TABLE IF NOT EXISTS team_invites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    invited_by UUID NOT NULL REFERENCES users(id),
    invited_user_id UUID REFERENCES users(id),   -- NULL if invited by email (not yet registered)
    invited_email VARCHAR(256),                    -- for email-based invites
    role VARCHAR(16) NOT NULL DEFAULT 'member',
    status VARCHAR(16) NOT NULL DEFAULT 'pending', -- pending, accepted, declined, expired
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '7 days'
);

CREATE INDEX idx_team_invites_team ON team_invites(team_id);
CREATE INDEX idx_team_invites_user ON team_invites(invited_user_id);
CREATE INDEX idx_team_invites_email ON team_invites(invited_email);

-- ── Daily API Usage Tracking ──
CREATE TABLE IF NOT EXISTS api_usage_daily (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    date DATE NOT NULL DEFAULT CURRENT_DATE,
    request_count INT NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, date)
);
