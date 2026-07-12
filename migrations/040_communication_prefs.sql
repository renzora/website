-- Communication / email notification preferences + profile cover photo,
-- plus the foreign-key cleanup that makes account deletion possible.

-- Per-user email notification toggles (default on so existing users keep
-- receiving mail until they opt out).
ALTER TABLE users ADD COLUMN IF NOT EXISTS email_product_updates BOOLEAN NOT NULL DEFAULT TRUE;
ALTER TABLE users ADD COLUMN IF NOT EXISTS email_marketplace BOOLEAN NOT NULL DEFAULT TRUE;
ALTER TABLE users ADD COLUMN IF NOT EXISTS email_comments BOOLEAN NOT NULL DEFAULT TRUE;
ALTER TABLE users ADD COLUMN IF NOT EXISTS email_security BOOLEAN NOT NULL DEFAULT TRUE;

-- Cover/banner photo. Distinct from the existing banner_color solid-color field;
-- nullable so "no photo" is the default and can be cleared again.
ALTER TABLE users ADD COLUMN IF NOT EXISTS banner_url TEXT;

-- Account deletion. The older tables (migrations 001–034) reference users(id)
-- with the default NO ACTION, so `DELETE FROM users` would fail against them.
-- Convert those FKs so a user delete cleans up after itself: NOT NULL columns
-- (the referencing row belongs to the user and is meaningless without them)
-- cascade, while nullable columns (attribution / audit "actor" references on
-- rows that belong to someone else) null out so the row survives. Constraints
-- that already declare CASCADE or SET NULL are left untouched, so the deliberate
-- choices in the newer migrations are preserved.
DO $$
DECLARE
    r RECORD;
    action TEXT;
BEGIN
    FOR r IN
        SELECT con.conname,
               con.conrelid::regclass::text AS tbl,
               pg_get_constraintdef(con.oid) AS def,
               att.attnotnull AS notnull
        FROM pg_constraint con
        JOIN pg_class ref ON ref.oid = con.confrelid
        JOIN pg_attribute att ON att.attrelid = con.conrelid AND att.attnum = con.conkey[1]
        WHERE con.contype = 'f'
          AND ref.relname = 'users'
          AND con.confdeltype = 'a'            -- only the default NO ACTION FKs
          AND array_length(con.conkey, 1) = 1  -- single-column FKs only
    LOOP
        action := CASE WHEN r.notnull THEN 'CASCADE' ELSE 'SET NULL' END;
        EXECUTE format('ALTER TABLE %s DROP CONSTRAINT %I', r.tbl, r.conname);
        EXECUTE format('ALTER TABLE %s ADD CONSTRAINT %I %s ON DELETE %s',
                       r.tbl, r.conname, r.def, action);
    END LOOP;
END $$;
