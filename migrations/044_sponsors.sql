-- ── Sponsor profiles ──
-- Public sponsor-wall presence for donors. Tier is derived from the user's
-- cumulative non-anonymous donation total; this table only stores how they
-- want to appear (display name, link, logo for the top tiers).
CREATE TABLE IF NOT EXISTS sponsor_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    display_name VARCHAR(64) NOT NULL DEFAULT '',
    website_url TEXT NOT NULL DEFAULT '',
    logo_url TEXT,
    hidden BOOLEAN NOT NULL DEFAULT false,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
