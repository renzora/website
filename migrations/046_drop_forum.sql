-- Retire the forum entirely. Channel-based feed posts (migration 041) fully
-- replace forum categories/threads/posts. Dropped in FK-dependency order.
-- Note: `post_reactions` (the feed's reactions, migration 039) is intentionally
-- kept — only the forum-specific `forum_post_reactions` is dropped here.
DROP TABLE IF EXISTS forum_post_reactions CASCADE;
DROP TABLE IF EXISTS forum_posts CASCADE;
DROP TABLE IF EXISTS forum_threads CASCADE;
DROP TABLE IF EXISTS forum_categories CASCADE;
