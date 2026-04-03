-- 027: Download filename, proper tags table, subcategories

-- ── Download filename ──────────────────────────────────────────────────────
-- Stores the human-readable filename for downloads instead of using the UUID key.
ALTER TABLE assets ADD COLUMN IF NOT EXISTS download_filename VARCHAR(255) NOT NULL DEFAULT '';

-- ── Tags table ─────────────────────────────────────────────────────────────
-- Replaces the TEXT[] column on assets with a proper normalized table.
-- Tags can be approved (usable by everyone) or pending review.
CREATE TABLE IF NOT EXISTS tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(64) NOT NULL,
    slug VARCHAR(64) NOT NULL UNIQUE,
    approved BOOLEAN NOT NULL DEFAULT false,
    submitted_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_tags_slug ON tags(slug);
CREATE INDEX IF NOT EXISTS idx_tags_approved ON tags(approved);
CREATE INDEX IF NOT EXISTS idx_tags_name_trgm ON tags USING gin (name gin_trgm_ops);

-- Junction table for asset <-> tag many-to-many
CREATE TABLE IF NOT EXISTS asset_tags (
    asset_id UUID NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (asset_id, tag_id)
);

-- ── Subcategories ──────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS subcategories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    category_id UUID NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    slug VARCHAR(128) NOT NULL,
    approved BOOLEAN NOT NULL DEFAULT false,
    submitted_by UUID REFERENCES users(id) ON DELETE SET NULL,
    sort_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (category_id, slug)
);

CREATE INDEX IF NOT EXISTS idx_subcategories_category ON subcategories(category_id);
CREATE INDEX IF NOT EXISTS idx_subcategories_approved ON subcategories(approved);

-- Add subcategory reference to assets
ALTER TABLE assets ADD COLUMN IF NOT EXISTS subcategory VARCHAR(128) NOT NULL DEFAULT '';

-- Enable trigram extension for fuzzy tag search (idempotent)
CREATE EXTENSION IF NOT EXISTS pg_trgm;
