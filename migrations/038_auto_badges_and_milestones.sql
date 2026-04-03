-- New milestone badges with auto_rule + auto_threshold
-- Level milestones
INSERT INTO badges (id, slug, name, description, icon, color, auto_rule, auto_threshold) VALUES
(gen_random_uuid(), 'level-5', 'Adventurer', 'Reached level 5', 'ph-sword', '#3b82f6', 'level', 5),
(gen_random_uuid(), 'level-10', 'Veteran', 'Reached level 10', 'ph-shield-star', '#8b5cf6', 'level', 10),
(gen_random_uuid(), 'level-25', 'Legend', 'Reached level 25', 'ph-crown', '#f59e0b', 'level', 25),
(gen_random_uuid(), 'level-50', 'Mythic', 'Reached level 50', 'ph-fire', '#ef4444', 'level', 50)
ON CONFLICT (slug) DO NOTHING;

-- XP milestones
INSERT INTO badges (id, slug, name, description, icon, color, auto_rule, auto_threshold) VALUES
(gen_random_uuid(), 'xp-1000', 'XP Grinder', 'Earned 1,000 XP', 'ph-lightning', '#14b8a6', 'total_xp', 1000),
(gen_random_uuid(), 'xp-10000', 'XP Machine', 'Earned 10,000 XP', 'ph-lightning-slash', '#06b6d4', 'total_xp', 10000)
ON CONFLICT (slug) DO NOTHING;

-- Seller milestones
INSERT INTO badges (id, slug, name, description, icon, color, auto_rule, auto_threshold) VALUES
(gen_random_uuid(), 'seller-bronze', 'Bronze Seller', 'Reached Bronze seller level', 'ph-storefront', '#CD7F32', 'seller_level', 1),
(gen_random_uuid(), 'seller-silver', 'Silver Seller', 'Reached Silver seller level', 'ph-storefront', '#C0C0C0', 'seller_level', 2),
(gen_random_uuid(), 'seller-gold', 'Gold Seller', 'Reached Gold seller level', 'ph-storefront', '#FFD700', 'seller_level', 3),
(gen_random_uuid(), 'seller-platinum', 'Platinum Seller', 'Reached Platinum seller level', 'ph-storefront', '#E5E4E2', 'seller_level', 4),
(gen_random_uuid(), 'seller-diamond', 'Diamond Seller', 'Reached Diamond seller level', 'ph-diamond', '#B9F2FF', 'seller_level', 5)
ON CONFLICT (slug) DO NOTHING;

-- Download milestones
INSERT INTO badges (id, slug, name, description, icon, color, auto_rule, auto_threshold) VALUES
(gen_random_uuid(), 'downloads-100', 'Popular Creator', '100 total downloads', 'ph-download-simple', '#10b981', 'total_downloads', 100),
(gen_random_uuid(), 'downloads-1000', 'Trending Creator', '1,000 total downloads', 'ph-trend-up', '#059669', 'total_downloads', 1000),
(gen_random_uuid(), 'downloads-10000', 'Viral Creator', '10,000 total downloads', 'ph-rocket-launch', '#047857', 'total_downloads', 10000)
ON CONFLICT (slug) DO NOTHING;

-- Social milestones
INSERT INTO badges (id, slug, name, description, icon, color, auto_rule, auto_threshold) VALUES
(gen_random_uuid(), 'followers-10', 'Rising Star', '10 followers', 'ph-users', '#8b5cf6', 'follower_count', 10),
(gen_random_uuid(), 'followers-100', 'Influencer', '100 followers', 'ph-megaphone', '#7c3aed', 'follower_count', 100),
(gen_random_uuid(), 'followers-1000', 'Celebrity', '1,000 followers', 'ph-star-four', '#6d28d9', 'follower_count', 1000)
ON CONFLICT (slug) DO NOTHING;

-- Forum milestones
INSERT INTO badges (id, slug, name, description, icon, color, auto_rule, auto_threshold) VALUES
(gen_random_uuid(), 'posts-10', 'Chatterbox', '10 forum posts', 'ph-chat-circle-text', '#ec4899', 'post_count', 10),
(gen_random_uuid(), 'posts-500', 'Forum Legend', '500 forum posts', 'ph-chat-circle-text', '#be185d', 'post_count', 500)
ON CONFLICT (slug) DO NOTHING;

-- Update existing badges with auto_rules
UPDATE badges SET auto_rule = 'first_purchase', auto_threshold = 1 WHERE slug = 'first-purchase' AND auto_rule IS NULL;
UPDATE badges SET auto_rule = 'purchase_count', auto_threshold = 10 WHERE slug = 'big-spender' AND auto_rule IS NULL;
UPDATE badges SET auto_rule = 'first_upload', auto_threshold = 1 WHERE slug = 'creator' AND auto_rule IS NULL;
UPDATE badges SET auto_rule = 'sale_count', auto_threshold = 10 WHERE slug = 'top-seller' AND auto_rule IS NULL;
UPDATE badges SET auto_rule = 'post_count', auto_threshold = 100 WHERE slug = 'prolific-poster' AND auto_rule IS NULL;
UPDATE badges SET auto_rule = 'article_count', auto_threshold = 5 WHERE slug = 'writer' AND auto_rule IS NULL;
