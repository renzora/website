-- Retire the 3D avatar system (removed from the app). Drops the tables created
-- in migration 036, in FK-dependency order. Note: `users.avatar_url` (a plain
-- uploaded profile picture) is unrelated to this system and is intentionally kept.
DROP TABLE IF EXISTS user_avatar_parts CASCADE;
DROP TABLE IF EXISTS user_avatars CASCADE;
DROP TABLE IF EXISTS avatar_parts CASCADE;
