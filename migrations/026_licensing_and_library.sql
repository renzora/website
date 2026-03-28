-- ── License Types ──
CREATE TABLE IF NOT EXISTS license_types (
    id VARCHAR(16) PRIMARY KEY,  -- personal, commercial, team, enterprise
    name VARCHAR(32) NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    price_multiplier NUMERIC(3,1) NOT NULL DEFAULT 1.0,
    max_users INT NOT NULL DEFAULT 1,  -- 0 = unlimited
    max_projects INT NOT NULL DEFAULT 1,  -- 0 = unlimited
    commercial_use BOOLEAN NOT NULL DEFAULT false,
    sort_order INT NOT NULL DEFAULT 0
);

INSERT INTO license_types (id, name, description, price_multiplier, max_users, max_projects, commercial_use, sort_order) VALUES
    ('personal', 'Personal', 'Use in 1 project, 1 person, non-commercial', 1.0, 1, 1, false, 1),
    ('commercial', 'Commercial', 'Use in 1 commercial project, 1 person', 2.0, 1, 1, true, 2),
    ('team', 'Team', 'Use in 1 project, entire team', 3.0, 0, 1, true, 3),
    ('enterprise', 'Enterprise', 'Unlimited projects, entire organization', 5.0, 0, 0, true, 4)
ON CONFLICT (id) DO NOTHING;

-- ── License Grants (tracks who has what license) ──
CREATE TABLE IF NOT EXISTS license_grants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id UUID NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,       -- individual license
    team_id UUID REFERENCES teams(id) ON DELETE CASCADE,       -- team/enterprise license
    license_type VARCHAR(16) NOT NULL REFERENCES license_types(id),
    source VARCHAR(16) NOT NULL DEFAULT 'purchase',  -- purchase, library, grant
    credits_paid INT NOT NULL DEFAULT 0,
    granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ  -- NULL = perpetual, set for library assets on unsub
);

CREATE INDEX idx_license_grants_asset ON license_grants(asset_id);
CREATE INDEX idx_license_grants_user ON license_grants(user_id);
CREATE INDEX idx_license_grants_team ON license_grants(team_id);

-- ── Team Cloud Storage (assets added to team library) ──
CREATE TABLE IF NOT EXISTS team_library (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    asset_id UUID NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    added_by UUID NOT NULL REFERENCES users(id),
    license_grant_id UUID REFERENCES license_grants(id),
    size_bytes BIGINT NOT NULL DEFAULT 0,
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(team_id, asset_id)
);

CREATE INDEX idx_team_library_team ON team_library(team_id);

-- ── Team Roles (expanded) ──
-- Update existing team_members to support more granular roles
-- Roles: owner, manager, lead, designer, programmer, viewer
-- (the role column already exists as VARCHAR(16), just use new values)

-- ── Team Role Permissions ──
CREATE TABLE IF NOT EXISTS team_role_permissions (
    role VARCHAR(16) PRIMARY KEY,
    can_browse_library BOOLEAN NOT NULL DEFAULT true,
    can_add_to_library BOOLEAN NOT NULL DEFAULT false,
    can_request_assets BOOLEAN NOT NULL DEFAULT false,
    can_remove_from_library BOOLEAN NOT NULL DEFAULT false,
    can_manage_budget BOOLEAN NOT NULL DEFAULT false,
    can_invite_members BOOLEAN NOT NULL DEFAULT false,
    can_manage_roles BOOLEAN NOT NULL DEFAULT false
);

INSERT INTO team_role_permissions (role, can_browse_library, can_add_to_library, can_request_assets, can_remove_from_library, can_manage_budget, can_invite_members, can_manage_roles) VALUES
    ('owner', true, true, true, true, true, true, true),
    ('manager', true, true, true, true, true, true, false),
    ('lead', true, true, true, false, false, false, false),
    ('designer', true, false, true, false, false, false, false),
    ('programmer', true, false, true, false, false, false, false),
    ('viewer', true, false, false, false, false, false, false)
ON CONFLICT (role) DO NOTHING;

-- ── Asset Library Requests (for roles that need approval) ──
CREATE TABLE IF NOT EXISTS library_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    asset_id UUID NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    requested_by UUID NOT NULL REFERENCES users(id),
    status VARCHAR(16) NOT NULL DEFAULT 'pending',  -- pending, approved, denied
    reviewed_by UUID REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_library_requests_team ON library_requests(team_id);

-- ── Creator Pool ──
CREATE TABLE IF NOT EXISTS creator_pool (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    month DATE NOT NULL,  -- first day of month
    total_credits BIGINT NOT NULL DEFAULT 0,
    total_library_adds BIGINT NOT NULL DEFAULT 0,
    credits_per_add NUMERIC(10,4) NOT NULL DEFAULT 0,
    distributed BOOLEAN NOT NULL DEFAULT false,
    distributed_at TIMESTAMPTZ,
    UNIQUE(month)
);

-- ── Creator Pool Contributions (tracks each sub's contribution) ──
CREATE TABLE IF NOT EXISTS creator_pool_contributions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pool_month DATE NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    credits_contributed INT NOT NULL DEFAULT 0,
    contributed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_pool_contributions_month ON creator_pool_contributions(pool_month);

-- ── Creator Pool Earnings (per creator per month) ──
CREATE TABLE IF NOT EXISTS creator_pool_earnings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pool_month DATE NOT NULL,
    creator_id UUID NOT NULL REFERENCES users(id),
    library_adds INT NOT NULL DEFAULT 0,
    credits_earned BIGINT NOT NULL DEFAULT 0,
    paid_at TIMESTAMPTZ,
    UNIQUE(pool_month, creator_id)
);

CREATE INDEX idx_pool_earnings_creator ON creator_pool_earnings(creator_id);

-- ── Library Add Log (tracks every time an asset is added to a team library) ──
CREATE TABLE IF NOT EXISTS library_add_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id UUID NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    creator_id UUID NOT NULL,  -- denormalized for fast pool calc
    team_id UUID NOT NULL REFERENCES teams(id) ON DELETE CASCADE,
    added_by UUID NOT NULL REFERENCES users(id),
    pool_month DATE NOT NULL DEFAULT date_trunc('month', CURRENT_DATE)::date,
    added_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_library_add_log_month ON library_add_log(pool_month);
CREATE INDEX idx_library_add_log_creator ON library_add_log(creator_id);

-- ── Subscriber monthly allowance tracking ──
CREATE TABLE IF NOT EXISTS library_allowance (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    month DATE NOT NULL DEFAULT date_trunc('month', CURRENT_DATE)::date,
    assets_added INT NOT NULL DEFAULT 0,
    max_assets INT NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, month)
);
