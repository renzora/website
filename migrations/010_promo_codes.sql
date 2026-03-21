-- Promo codes reduce the platform's distribution cut on purchases.
-- The standard platform cut is 20%. A promo code with discount_percent = 10
-- reduces it to 10%, meaning the creator gets 90% instead of 80%.

CREATE TABLE IF NOT EXISTS promo_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(32) UNIQUE NOT NULL,
    discount_percent INT NOT NULL DEFAULT 0,  -- how much to reduce the platform cut (0-20)
    max_uses INT,           -- NULL = unlimited
    times_used INT NOT NULL DEFAULT 0,
    active BOOLEAN NOT NULL DEFAULT true,
    expires_at TIMESTAMPTZ, -- NULL = never expires
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Track which user used which promo code on which asset
CREATE TABLE IF NOT EXISTS promo_code_uses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    promo_code_id UUID NOT NULL REFERENCES promo_codes(id),
    user_id UUID NOT NULL REFERENCES users(id),
    asset_id UUID NOT NULL REFERENCES assets(id),
    discount_applied INT NOT NULL,  -- the actual discount_percent applied
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_promo_codes_code ON promo_codes(code);
CREATE INDEX IF NOT EXISTS idx_promo_code_uses_user ON promo_code_uses(user_id);
