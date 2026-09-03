-- Retire the social, course and game-store halves of the site. renzora.com is
-- now the engine download page plus the asset marketplace, so the feed,
-- profiles, articles, courses, messaging, notifications, teams (and the team
-- library economy built on them), gift cards, the game store and the game
-- waiting list all go. Dropped in FK-dependency order; CASCADE clears the
-- foreign keys that surviving tables hold into these.
--
-- Kept on purpose: `friends`, `user_presence`, `leaderboards`,
-- `player_achievements` and `player_stats` back the game-services SDK that
-- developers call from their own games (see /developers), not the website's
-- social features. `social_connections` backs OAuth sign-in, not profile links.

-- ── Community feed ──
DROP TABLE IF EXISTS post_reactions CASCADE;
DROP TABLE IF EXISTS post_reports CASCADE;
DROP TABLE IF EXISTS post_likes CASCADE;
DROP TABLE IF EXISTS comment_likes CASCADE;
DROP TABLE IF EXISTS post_comments CASCADE;
DROP TABLE IF EXISTS posts CASCADE;
DROP TABLE IF EXISTS channels CASCADE;

-- ── Articles ──
DROP TABLE IF EXISTS article_likes CASCADE;
DROP TABLE IF EXISTS article_comments CASCADE;
DROP TABLE IF EXISTS articles CASCADE;

-- ── Courses ──
DROP TABLE IF EXISTS enrollments CASCADE;
DROP TABLE IF EXISTS course_reviews CASCADE;
DROP TABLE IF EXISTS course_chapters CASCADE;
DROP TABLE IF EXISTS courses CASCADE;

-- ── Social graph, messaging, notifications ──
DROP TABLE IF EXISTS follows CASCADE;
DROP TABLE IF EXISTS message_attachments CASCADE;
DROP TABLE IF EXISTS messages CASCADE;
DROP TABLE IF EXISTS conversation_participants CASCADE;
DROP TABLE IF EXISTS conversations CASCADE;
DROP TABLE IF EXISTS notifications CASCADE;
DROP TABLE IF EXISTS gift_cards CASCADE;

-- ── Teams and the team-library / creator-pool economy built on them ──
DROP TABLE IF EXISTS library_add_log CASCADE;
DROP TABLE IF EXISTS library_allowance CASCADE;
DROP TABLE IF EXISTS library_requests CASCADE;
DROP TABLE IF EXISTS team_library CASCADE;
DROP TABLE IF EXISTS team_role_permissions CASCADE;
DROP TABLE IF EXISTS team_invites CASCADE;
DROP TABLE IF EXISTS team_members CASCADE;
DROP TABLE IF EXISTS teams CASCADE;
DROP TABLE IF EXISTS license_grants CASCADE;
DROP TABLE IF EXISTS license_types CASCADE;
DROP TABLE IF EXISTS creator_pool_earnings CASCADE;
DROP TABLE IF EXISTS creator_pool_contributions CASCADE;
DROP TABLE IF EXISTS creator_pool CASCADE;

-- ── Game store + waiting list ──
DROP TABLE IF EXISTS wishlists CASCADE;
DROP TABLE IF EXISTS user_games CASCADE;
DROP TABLE IF EXISTS game_reviews CASCADE;
DROP TABLE IF EXISTS game_media CASCADE;
DROP TABLE IF EXISTS game_comments CASCADE;
DROP TABLE IF EXISTS games CASCADE;
DROP TABLE IF EXISTS game_categories CASCADE;
DROP TABLE IF EXISTS waitlist CASCADE;

-- ── User columns that only fed the removed features ──
-- `online_status_visible` stays: the game-services presence API reads it.
ALTER TABLE users DROP COLUMN IF EXISTS follower_count;
ALTER TABLE users DROP COLUMN IF EXISTS following_count;
ALTER TABLE users DROP COLUMN IF EXISTS post_count;
ALTER TABLE users DROP COLUMN IF EXISTS message_privacy;
ALTER TABLE users DROP COLUMN IF EXISTS profile_visibility;
