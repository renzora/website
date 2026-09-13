-- 056: A claimable, immutable identity for every listing.
--
-- Until now a listing had three names and none of them were a handle. `id` is a
-- UUID nobody types; `slug` is unique but generated (`clouds-58fa79b5`), so it
-- cannot be chosen in advance; `name` is free text with no constraint at all,
-- which is why "CRT Fx", "Clouds Plugin" and "Film grain FX" all exist beside
-- crates called `crt`, `clouds` and `film_grain`. A tool publishing from a
-- directory had nothing stable to match on, and would happily create a second
-- listing next to the real one.
--
-- `marketplace_id` is that handle: chosen by the creator on first publish,
-- unique across the whole marketplace, and never changed afterwards. It is the
-- crates.io model — the name *is* the claim, first come first served.
ALTER TABLE assets ADD COLUMN IF NOT EXISTS marketplace_id VARCHAR(128);

-- ── Backfill 1: a plugin takes its crate name ──────────────────────────────
-- Plugins already carry a machine identity: the server records `crate_name`
-- from the uploaded zip's Cargo.toml, and the editor installs into a directory
-- named after it. Giving those listings `crt` rather than `crt-fx-93de3c0e`
-- means the CLI's natural default — the crate name — already matches, and no
-- existing plugin needs its manifest edited.
--
-- Only when the claim is unambiguous: the crate name must be held by exactly
-- one listing (case-insensitively), and must not be a slug that backfill 2 is
-- about to hand to somebody else.
WITH candidate AS (
    SELECT id, trim(metadata->>'crate_name') AS handle
    FROM assets
    WHERE marketplace_id IS NULL
      AND coalesce(trim(metadata->>'crate_name'), '') <> ''
),
unambiguous AS (
    SELECT lower(handle) AS folded
    FROM candidate
    GROUP BY lower(handle)
    HAVING count(*) = 1
)
UPDATE assets a
SET marketplace_id = c.handle
FROM candidate c
JOIN unambiguous u ON u.folded = lower(c.handle)
WHERE a.id = c.id
  AND NOT EXISTS (
      SELECT 1 FROM assets x WHERE lower(x.slug) = lower(c.handle)
  );

-- ── Backfill 2: everything else takes its slug ─────────────────────────────
-- `slug` is already UNIQUE and immutable (renaming a listing never regenerates
-- it), so it is the one existing value that can become a handle for 1,300-odd
-- listings without a collision to resolve. It is ugly — the UUID fragment is
-- right there in it — but it is correct, and a creator who wants a better one
-- can claim it on a listing they publish later.
UPDATE assets SET marketplace_id = slug WHERE marketplace_id IS NULL;

ALTER TABLE assets ALTER COLUMN marketplace_id SET NOT NULL;

-- Case-insensitive, because `Clouds` and `clouds` are the same claim and
-- allowing both would make "is this name taken?" unanswerable.
--
-- Note this index is the *only* rule applied to the values above: backfilled
-- handles are grandfathered as they are, including one slug that is Cyrillic.
-- The charset rule for a *new* claim lives in the API, which is the only place
-- that can tell a fresh claim from a historical one.
CREATE UNIQUE INDEX IF NOT EXISTS idx_assets_marketplace_id
    ON assets (lower(marketplace_id));
