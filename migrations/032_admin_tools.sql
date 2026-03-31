-- Admin credit tracking on transactions
ALTER TABLE transactions ADD COLUMN IF NOT EXISTS reason TEXT;
ALTER TABLE transactions ADD COLUMN IF NOT EXISTS admin_id UUID REFERENCES users(id);

-- Voucher system (distinct from promo codes which reduce commission)
CREATE TABLE IF NOT EXISTS vouchers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(32) NOT NULL UNIQUE,
    voucher_type VARCHAR(20) NOT NULL, -- 'credit', 'asset_discount'
    credit_amount BIGINT,              -- for credit vouchers: amount of credits to grant
    discount_percent INT,              -- for asset_discount: 0-100
    max_asset_price BIGINT,            -- max asset price this applies to (NULL = any)
    specific_asset_id UUID REFERENCES assets(id), -- NULL = any asset
    max_uses INT,                      -- NULL = unlimited
    max_uses_per_user INT NOT NULL DEFAULT 1,
    times_used INT NOT NULL DEFAULT 0,
    active BOOLEAN NOT NULL DEFAULT true,
    expires_at TIMESTAMPTZ,
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS voucher_uses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    voucher_id UUID NOT NULL REFERENCES vouchers(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id),
    credit_amount BIGINT,
    asset_id UUID REFERENCES assets(id),
    used_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_vouchers_code ON vouchers(code);
CREATE INDEX IF NOT EXISTS idx_voucher_uses_voucher ON voucher_uses(voucher_id);
CREATE INDEX IF NOT EXISTS idx_voucher_uses_user ON voucher_uses(user_id);

-- Launcher download tracking
CREATE TABLE IF NOT EXISTS launcher_downloads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    platform VARCHAR(20) NOT NULL,
    version VARCHAR(20),
    ip_address TEXT,
    user_agent TEXT,
    user_id UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_launcher_downloads_created ON launcher_downloads(created_at);
CREATE INDEX IF NOT EXISTS idx_launcher_downloads_platform ON launcher_downloads(platform);
