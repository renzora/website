-- One-time real-money donations from people without an account, captured via
-- Stripe Checkout and recorded from the webhook. The amount is stored as its
-- credit-equivalent (1 credit = $0.10) so that when the donor later signs in
-- with the same email, the donation drops straight into the credit-denominated
-- supporters wall. `claimed_by` is set once it has been attached to an account.
CREATE TABLE IF NOT EXISTS guest_donations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL,
    amount_credits BIGINT NOT NULL,
    message TEXT NOT NULL DEFAULT '',
    anonymous BOOLEAN NOT NULL DEFAULT false,
    stripe_session_id TEXT NOT NULL UNIQUE,
    claimed_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Fast lookup of a donor's unclaimed donations when they sign in / register.
CREATE INDEX IF NOT EXISTS idx_guest_donations_unclaimed_email
    ON guest_donations (LOWER(email)) WHERE claimed_by IS NULL;
