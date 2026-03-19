-- Asset categories (admin-managed instead of hardcoded strings)
CREATE TABLE IF NOT EXISTS categories (
    id UUID PRIMARY KEY,
    name VARCHAR(64) NOT NULL UNIQUE,
    slug VARCHAR(64) NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT '',
    icon VARCHAR(64) NOT NULL DEFAULT 'ph-folder',
    sort_order INT NOT NULL DEFAULT 0,
    max_file_size_mb INT NOT NULL DEFAULT 50,
    allowed_extensions TEXT[] NOT NULL DEFAULT '{zip,rar,7z}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed default categories
INSERT INTO categories (id, name, slug, description, icon, sort_order, allowed_extensions) VALUES
    (gen_random_uuid(), 'Plugin', 'plugin', 'Engine plugins and extensions', 'ph-puzzle-piece', 1, '{zip,rar,7z}'),
    (gen_random_uuid(), 'Theme', 'theme', 'Editor themes and color schemes', 'ph-palette', 2, '{zip,json}'),
    (gen_random_uuid(), 'Asset Pack', 'asset-pack', '3D models, textures, audio, and more', 'ph-package', 3, '{zip,rar,7z}'),
    (gen_random_uuid(), 'Script', 'script', 'Reusable Lua/Rhai scripts', 'ph-code', 4, '{zip,lua,rhai}'),
    (gen_random_uuid(), 'Material', 'material', 'Material graphs and shaders', 'ph-drop', 5, '{zip,material,wgsl}')
ON CONFLICT DO NOTHING;

-- Disputes / refund requests
CREATE TABLE IF NOT EXISTS disputes (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    asset_id UUID REFERENCES assets(id),
    transaction_id UUID REFERENCES transactions(id),
    reason TEXT NOT NULL,
    status VARCHAR(16) NOT NULL DEFAULT 'open',
    admin_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMPTZ
);

CREATE INDEX idx_disputes_status ON disputes(status);

-- Site settings (key-value config for admin)
CREATE TABLE IF NOT EXISTS site_settings (
    key VARCHAR(128) PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed default settings
INSERT INTO site_settings (key, value) VALUES
    ('max_upload_size_mb', '50'),
    ('platform_fee_percent', '20'),
    ('min_topup_credits', '100'),
    ('min_payout_credits', '1000')
ON CONFLICT DO NOTHING;
