-- Social connections on profiles
CREATE TABLE IF NOT EXISTS social_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    platform VARCHAR(32) NOT NULL,  -- 'discord', 'twitch', 'steam', 'xbox', 'playstation', 'epic', 'kick', 'youtube', 'twitter', 'github'
    platform_id VARCHAR(128),       -- OAuth verified ID (NULL if manual)
    platform_username VARCHAR(128) NOT NULL,
    platform_url VARCHAR(512),
    verified BOOLEAN NOT NULL DEFAULT false,  -- true if connected via OAuth
    connected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, platform)
);

CREATE INDEX IF NOT EXISTS idx_social_connections_user ON social_connections(user_id);

-- Gift cards (credit-based)
CREATE TABLE IF NOT EXISTS gift_cards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sender_id UUID NOT NULL REFERENCES users(id),
    recipient_id UUID REFERENCES users(id),  -- NULL if sent by code
    code VARCHAR(32) NOT NULL UNIQUE,
    amount BIGINT NOT NULL,
    message TEXT NOT NULL DEFAULT '',
    status VARCHAR(20) NOT NULL DEFAULT 'pending',  -- 'pending', 'redeemed', 'expired'
    redeemed_by UUID REFERENCES users(id),
    redeemed_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_gift_cards_sender ON gift_cards(sender_id);
CREATE INDEX IF NOT EXISTS idx_gift_cards_recipient ON gift_cards(recipient_id);
CREATE INDEX IF NOT EXISTS idx_gift_cards_code ON gift_cards(code);

-- Donations
CREATE TABLE IF NOT EXISTS donations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    amount BIGINT NOT NULL,
    message TEXT NOT NULL DEFAULT '',
    anonymous BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_donations_user ON donations(user_id);
CREATE INDEX IF NOT EXISTS idx_donations_created ON donations(created_at DESC);

-- Donation badges
INSERT INTO badges (id, name, slug, description, icon, color) VALUES
    (gen_random_uuid(), 'Bronze Donor', 'donor_bronze', 'Donated 100+ credits to Renzora', 'ph-heart', '#cd7f32'),
    (gen_random_uuid(), 'Silver Donor', 'donor_silver', 'Donated 500+ credits to Renzora', 'ph-heart', '#c0c0c0'),
    (gen_random_uuid(), 'Gold Donor', 'donor_gold', 'Donated 1000+ credits to Renzora', 'ph-heart', '#ffd700'),
    (gen_random_uuid(), 'Platinum Donor', 'donor_platinum', 'Donated 5000+ credits to Renzora', 'ph-heart', '#e5e4e2')
ON CONFLICT (slug) DO NOTHING;
