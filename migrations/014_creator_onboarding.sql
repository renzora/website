-- Track whether a creator has accepted the marketplace seller policy.
ALTER TABLE users ADD COLUMN IF NOT EXISTS creator_policy_accepted_at TIMESTAMPTZ;
