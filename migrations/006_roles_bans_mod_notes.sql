-- Roles with granular permissions
CREATE TABLE IF NOT EXISTS roles (
    id UUID PRIMARY KEY,
    name VARCHAR(32) NOT NULL UNIQUE,
    color VARCHAR(16) NOT NULL DEFAULT '#a1a1aa',
    is_staff BOOLEAN NOT NULL DEFAULT false,
    permissions JSONB NOT NULL DEFAULT '{}',
    sort_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed default roles
INSERT INTO roles (id, name, color, is_staff, permissions, sort_order) VALUES
    (gen_random_uuid(), 'admin', '#ef4444', true, '{
        "manage_users": true, "manage_roles": true, "manage_bans": true,
        "manage_assets": true, "manage_categories": true, "manage_docs": true,
        "manage_forum": true, "manage_disputes": true, "manage_settings": true,
        "manage_badges": true, "mod_notes": true, "view_admin": true
    }', 1),
    (gen_random_uuid(), 'moderator', '#f59e0b', true, '{
        "manage_bans": true, "manage_forum": true, "manage_assets": true,
        "mod_notes": true, "view_admin": true
    }', 2),
    (gen_random_uuid(), 'editor', '#6366f1', true, '{
        "manage_docs": true, "manage_assets": true, "view_admin": true
    }', 3),
    (gen_random_uuid(), 'creator', '#8b5cf6', false, '{}', 4),
    (gen_random_uuid(), 'user', '#a1a1aa', false, '{}', 5)
ON CONFLICT DO NOTHING;

-- User role assignment (many-to-many, users can have multiple roles)
CREATE TABLE IF NOT EXISTS user_roles (
    user_id UUID NOT NULL REFERENCES users(id),
    role_id UUID NOT NULL REFERENCES roles(id),
    granted_by UUID REFERENCES users(id),
    granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, role_id)
);

-- Bans
CREATE TABLE IF NOT EXISTS bans (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    banned_by UUID NOT NULL REFERENCES users(id),
    reason TEXT NOT NULL,
    type VARCHAR(16) NOT NULL DEFAULT 'full',
    expires_at TIMESTAMPTZ,
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_bans_user ON bans(user_id, active);

-- Mod notes (staff-only notes on user profiles)
CREATE TABLE IF NOT EXISTS mod_notes (
    id UUID PRIMARY KEY,
    target_user_id UUID NOT NULL REFERENCES users(id),
    author_id UUID NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_mod_notes_target ON mod_notes(target_user_id);
