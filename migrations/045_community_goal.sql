-- ── Community goal (monthly donation target) ──
-- Stored in the existing site_settings key-value table so it can be tuned
-- from the admin API without a redeploy. Progress is measured against the
-- current calendar month's donation total.
INSERT INTO site_settings (key, value) VALUES
    ('community_goal_enabled', 'true'),
    ('community_goal_target', '5000'),
    ('community_goal_title', 'Monthly Community Goal'),
    ('community_goal_description', 'Help us fund Renzora development this month. Every donation counts toward the goal and lists you on the supporters wall.')
ON CONFLICT (key) DO NOTHING;
