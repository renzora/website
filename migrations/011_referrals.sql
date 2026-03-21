-- Referral system: users get a unique referral code, earn 5% of their referrals' purchases
-- from the platform's cut (not from the creator's share).

-- Add referral columns to users
ALTER TABLE users ADD COLUMN IF NOT EXISTS referral_code VARCHAR(16) UNIQUE;
ALTER TABLE users ADD COLUMN IF NOT EXISTS referred_by UUID REFERENCES users(id);

-- Generate referral codes for existing users
UPDATE users SET referral_code = UPPER(SUBSTRING(id::text FROM 1 FOR 8)) WHERE referral_code IS NULL;

-- Track referral earnings per purchase
CREATE TABLE IF NOT EXISTS referral_earnings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    referrer_id UUID NOT NULL REFERENCES users(id),
    referee_id UUID NOT NULL REFERENCES users(id),
    asset_id UUID NOT NULL REFERENCES assets(id),
    purchase_amount BIGINT NOT NULL,
    referral_amount BIGINT NOT NULL,  -- credits earned by referrer
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_referral_earnings_referrer ON referral_earnings(referrer_id);
CREATE INDEX IF NOT EXISTS idx_referral_earnings_referee ON referral_earnings(referee_id);
CREATE INDEX IF NOT EXISTS idx_users_referral_code ON users(referral_code);
CREATE INDEX IF NOT EXISTS idx_users_referred_by ON users(referred_by);
