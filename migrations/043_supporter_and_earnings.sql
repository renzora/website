-- ── Earnings balance ──
-- Creator income (sales, referrals) accrues here instead of the spendable
-- credit balance. Earnings can be withdrawn to a bank or converted into
-- spendable credits; purchased credits can never be withdrawn.
ALTER TABLE users ADD COLUMN IF NOT EXISTS earnings_balance BIGINT NOT NULL DEFAULT 0;

-- ── Supporter subscription ──
-- The tiered plans (Pro/Indie/Studio) are replaced by a single
-- pay-what-you-want Supporter plan (minimum 10 credits/month).
ALTER TABLE subscriptions ADD COLUMN IF NOT EXISTS monthly_amount BIGINT NOT NULL DEFAULT 0;

INSERT INTO subscription_plans (
    id, name, description, price_credits,
    daily_api_limit, storage_mb, max_team_members, max_file_size_mb,
    extra_seat_credits, extra_storage_credits_per_gb,
    commission_percent, library_assets_per_month, search_boost, asset_spotlights_per_month,
    xbox_builds_per_month, xbox_build_cost_credits, xbox_submission_cost_credits,
    profile_badge, profile_customization,
    features, sort_order
) VALUES
    ('supporter', 'Supporter', 'Support Renzora with a monthly amount you choose', 10,
     5000, 10240, 5, 2048,
     0, 0,
     20, 0, 0, 0,
     0, 0, 0,
     'supporter', 'custom',
     '["supporter_badge","discord_role","custom_profile"]', 2)
ON CONFLICT (id) DO NOTHING;

-- Move any existing tiered subscribers onto the Supporter plan at their old
-- price, then remove the old plans.
UPDATE subscriptions s SET plan_id = 'supporter',
    monthly_amount = GREATEST(10, (SELECT p.price_credits FROM subscription_plans p WHERE p.id = s.plan_id))
    WHERE s.plan_id IN ('pro', 'indie', 'studio');
DELETE FROM subscription_plans WHERE id IN ('pro', 'indie', 'studio');
