-- 028: Credit/attribution for assets from other creators
-- When set, the asset is forced to be free (price_credits = 0).
ALTER TABLE assets ADD COLUMN IF NOT EXISTS credit_name VARCHAR(255) NOT NULL DEFAULT '';
ALTER TABLE assets ADD COLUMN IF NOT EXISTS credit_url VARCHAR(512) NOT NULL DEFAULT '';
