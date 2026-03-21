-- Withdrawal system: creators can withdraw credits to their bank via Stripe Connect.

-- Add Stripe Connect account ID to users
ALTER TABLE users ADD COLUMN IF NOT EXISTS stripe_connect_id VARCHAR(64);
ALTER TABLE users ADD COLUMN IF NOT EXISTS stripe_connect_onboarded BOOLEAN NOT NULL DEFAULT false;

-- Withdrawal requests
CREATE TABLE IF NOT EXISTS withdrawals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    amount_credits BIGINT NOT NULL,        -- credits being withdrawn
    amount_usd_cents BIGINT NOT NULL,      -- dollar amount in cents (credits * 10)
    status VARCHAR(16) NOT NULL DEFAULT 'pending',  -- pending, processing, completed, failed, rejected
    stripe_transfer_id VARCHAR(64),
    failure_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_withdrawals_user ON withdrawals(user_id);
CREATE INDEX IF NOT EXISTS idx_withdrawals_status ON withdrawals(status);
