use leptos::prelude::*;

#[component]
pub fn SubscriptionPage() -> impl IntoView {
    view! {
        <section class="py-12 px-6 min-h-screen">
            <div class="max-w-5xl mx-auto">
                <div class="mb-10">
                    <h1 class="text-3xl font-bold">"Subscription"</h1>
                    <p class="text-zinc-400 mt-2">"Pay with credits. No external billing. Renews monthly from your balance."</p>
                </div>
                <div id="sub-content">
                    <div class="text-center py-12">
                        <div class="inline-block animate-spin w-6 h-6 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                    </div>
                </div>
            </div>
        </section>
        <script>
            r##"
            (async function() {
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                var el = document.getElementById('sub-content');

                var plansRes = await fetch('/api/subscriptions/plans');
                var plans = plansRes.ok ? await plansRes.json() : [];

                var current = null;
                var usage = null;
                if (token) {
                    try {
                        var curRes = await fetch('/api/subscriptions/current', { headers: { 'Authorization': 'Bearer ' + token } });
                        if (curRes.ok) current = await curRes.json();
                        var usageRes = await fetch('/api/subscriptions/usage', { headers: { 'Authorization': 'Bearer ' + token } });
                        if (usageRes.ok) usage = await usageRes.json();
                    } catch(e) {}
                }

                var activePlan = (current && current.subscription && current.subscription.status === 'active'
                    && new Date(current.subscription.current_period_end) > new Date()) ? current.plan.id : 'free';

                var featureLabels = {
                    'marketplace_access': 'Marketplace access',
                    '5_tags': 'Up to 5 tags per asset',
                    'community_support': 'Community support',
                    'priority_support': 'Priority support',
                    'analytics': 'Analytics dashboard',
                    'custom_storefront': 'Custom storefront',
                    'api_access': 'Full API access',
                    'team_management': 'Team management',
                    'bulk_uploads': 'Bulk uploads',
                    'dedicated_support': 'Dedicated support',
                    '10gb_storage': '10GB cloud storage',
                    '50gb_storage': '50GB cloud storage',
                    '200gb_storage': '200GB cloud storage'
                };

                var html = '';

                // Current plan banner
                if (token && current) {
                    html += '<div class="mb-8 p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl">';
                    html += '<div class="flex items-center justify-between"><div>';
                    html += '<div class="flex items-center gap-2"><span class="text-lg font-semibold">' + current.plan.name + ' Plan</span>';
                    if (activePlan !== 'free') html += '<span class="px-2 py-0.5 rounded bg-accent/10 border border-accent/20 text-[10px] text-accent font-medium">ACTIVE</span>';
                    html += '</div>';
                    html += '<div class="flex items-center gap-4 mt-2 text-sm text-zinc-500">';
                    html += '<span><i class="ph ph-coins"></i> Balance: <strong class="text-zinc-300">' + current.credit_balance.toLocaleString() + ' credits</strong></span>';
                    if (current.monthly_cost > 0) html += '<span><i class="ph ph-repeat"></i> Monthly: <strong class="text-zinc-300">' + current.monthly_cost + ' credits</strong></span>';
                    if (usage) html += '<span><i class="ph ph-lightning"></i> API: ' + usage.daily_requests.toLocaleString() + ' / ' + usage.daily_limit.toLocaleString() + '</span>';
                    html += '</div>';
                    if (current.subscription && current.subscription.cancel_at_period_end) html += '<p class="text-sm text-amber-400 mt-2"><i class="ph ph-warning"></i> Cancels at end of billing period</p>';
                    if (current.subscription && current.subscription.extra_seats > 0) html += '<p class="text-xs text-zinc-600 mt-1">+ ' + current.subscription.extra_seats + ' extra seats</p>';
                    if (current.subscription && current.subscription.extra_storage_gb > 0) html += '<p class="text-xs text-zinc-600 mt-1">+ ' + current.subscription.extra_storage_gb + 'GB extra storage</p>';
                    html += '</div>';
                    if (activePlan !== 'free' && current.subscription && !current.subscription.cancel_at_period_end) {
                        html += '<button onclick="cancelSub()" class="px-4 py-2 rounded-lg text-sm text-red-400 hover:bg-red-950/30 border border-transparent hover:border-red-900/50 transition-all">Cancel</button>';
                    }
                    html += '</div></div>';
                }

                // Plans grid
                html += '<div class="grid grid-cols-1 md:grid-cols-4 gap-4">';
                plans.forEach(function(p) {
                    var isCurrent = p.id === activePlan;
                    var features = p.features || [];
                    var isPopular = p.id === 'pro';

                    html += '<div class="p-6 bg-white/[0.02] border ' + (isCurrent ? 'border-accent/50' : isPopular ? 'border-accent/30' : 'border-zinc-800/50') + ' rounded-2xl relative">';
                    if (isCurrent) html += '<div class="absolute -top-2.5 left-1/2 -translate-x-1/2 px-3 py-0.5 rounded-full bg-accent text-white text-[10px] font-bold">CURRENT</div>';
                    else if (isPopular) html += '<div class="absolute -top-2.5 left-1/2 -translate-x-1/2 px-3 py-0.5 rounded-full bg-accent/20 text-accent text-[10px] font-bold border border-accent/30">POPULAR</div>';

                    html += '<h3 class="text-xl font-bold mb-1">' + p.name + '</h3>';
                    html += '<p class="text-xs text-zinc-500 mb-4 min-h-[2rem]">' + p.description + '</p>';

                    // Price
                    html += '<div class="mb-5">';
                    if (p.price_credits === 0) {
                        html += '<span class="text-3xl font-bold">Free</span>';
                    } else {
                        html += '<span class="text-3xl font-bold">' + p.price_credits + '</span>';
                        html += '<span class="text-sm text-zinc-500"> credits/mo</span>';
                        html += '<div class="text-xs text-zinc-600 mt-0.5">$' + (p.price_credits * 0.1).toFixed(2) + '/month</div>';
                    }
                    html += '</div>';

                    // Perks list — only show what the plan includes
                    var row = function(icon, text) { return '<div class="flex items-center gap-2 text-zinc-400"><i class="ph ph-' + icon + ' text-accent text-xs"></i>' + text + '</div>'; };
                    var check = function(text) { return '<div class="flex items-center gap-2 text-zinc-400"><i class="ph ph-check text-green-400 text-xs"></i>' + text + '</div>'; };

                    html += '<div class="space-y-1.5 mb-6 text-[13px]">';

                    html += row('coins', '<strong>' + (100 - p.commission_percent) + '/' + p.commission_percent + '</strong> seller/platform split');

                    var uploadLabel = p.max_file_size_mb >= 1024 ? (p.max_file_size_mb/1024) + 'GB' : p.max_file_size_mb + 'MB';
                    html += row('file-arrow-up', uploadLabel + ' max upload');

                    html += row('lightning', p.daily_api_limit.toLocaleString() + ' API requests/day');

                    if (p.storage_mb > 0) {
                        var storageLabel = p.storage_mb >= 1024 ? (p.storage_mb/1024) + 'GB' : p.storage_mb + 'MB';
                        html += row('hard-drive', storageLabel + ' cloud storage');
                    }

                    if (p.max_team_members > 0) html += row('users-three', p.max_team_members + ' team members included');
                    if (p.library_assets_per_month > 0) html += row('books', p.library_assets_per_month + ' library assets/month');
                    if (p.xbox_builds_per_month > 0) html += row('game-controller', p.xbox_builds_per_month + ' Xbox build' + (p.xbox_builds_per_month > 1 ? 's' : '') + '/month');
                    if (p.xbox_submission_cost_credits === 0 && p.xbox_builds_per_month > 0) html += check('Free Xbox submissions');

                    if (p.profile_badge) {
                        var badgeColor = p.profile_badge === 'studio' ? 'text-amber-400' : p.profile_badge === 'indie' ? 'text-purple-400' : 'text-accent';
                        html += '<div class="flex items-center gap-2 text-zinc-400"><i class="ph ph-seal-check ' + badgeColor + ' text-xs"></i>' + p.name + ' profile badge</div>';
                    }
                    if (features.indexOf('discord_role') >= 0) html += check('Discord ' + p.name + ' role');
                    if (p.profile_customization === 'verified') html += check('Verified profile + custom colors & banner');
                    else if (p.profile_customization === 'custom') html += check('Custom profile colors & banner');

                    if (features.indexOf('creator_pool') >= 0) html += check('Creator reward pool earnings');
                    if (features.indexOf('cloud_engine') >= 0) html += check('Cloud engine access');
                    if (features.indexOf('team_library') >= 0) html += check('Shared team asset library');
                    if (features.indexOf('scheduled_publishing') >= 0) html += check('Scheduled publishing');
                    if (features.indexOf('private_assets') >= 0) html += check('Private/unlisted assets');
                    if (features.indexOf('priority_build_queue') >= 0) html += check('Priority build queue');
                    if (features.indexOf('premium_forums') >= 0) html += check('Premium forum access');

                    if (p.asset_spotlights_per_month > 0) html += check(p.asset_spotlights_per_month + ' asset spotlight' + (p.asset_spotlights_per_month > 1 ? 's' : '') + '/month');
                    if (p.search_boost === 2) html += check('Boosted search visibility');
                    else if (p.search_boost === 1) html += check('Slight search boost');

                    if (features.indexOf('custom_storefront') >= 0) html += check('Custom storefront');
                    if (features.indexOf('full_analytics') >= 0) html += check('Full analytics dashboard');
                    if (features.indexOf('dedicated_support') >= 0) html += check('Dedicated support');
                    else if (features.indexOf('priority_support') >= 0) html += check('Priority support');
                    if (features.indexOf('beta_access') >= 0) html += check('Early access + beta features');
                    else if (features.indexOf('early_access') >= 0) html += check('Early access to new features');

                    html += '</div>';

                    // Action button
                    if (isCurrent) {
                        html += '<div class="px-4 py-2.5 rounded-xl text-sm font-medium bg-white/[0.03] border border-zinc-800/50 text-zinc-500 text-center">Current plan</div>';
                    } else if (p.price_credits === 0) {
                        html += '<div class="px-4 py-2.5 rounded-xl text-sm text-zinc-600 text-center">Default</div>';
                    } else if (!token) {
                        html += '<a href="/login" class="block px-4 py-2.5 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all text-center">Sign in</a>';
                    } else {
                        html += '<button onclick="subscribePlan(\'' + p.id + '\')" class="w-full px-4 py-2.5 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all">' + (activePlan !== 'free' ? 'Switch to ' : 'Subscribe for ') + p.price_credits + ' credits/mo</button>';
                    }

                    html += '</div>';
                });
                html += '</div>';

                html += '<p class="text-xs text-zinc-600 text-center mt-6">Credits are deducted monthly from your balance. If your balance runs out, your plan downgrades to Free. <a href="/wallet" class="text-accent hover:text-accent-hover">Top up credits</a></p>';

                el.innerHTML = html;
            })();

            async function subscribePlan(planId) {
                if (!confirm('Subscribe to this plan? Credits will be deducted from your balance now and every 30 days.')) return;
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                try {
                    var res = await fetch('/api/subscriptions/subscribe', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({ plan_id: planId, extra_seats: 0, extra_storage_gb: 0 })
                    });
                    var data = await res.json();
                    if (res.ok) { window.location.reload(); }
                    else { alert(data.message || 'Failed to subscribe'); }
                } catch(e) { alert('Error: ' + e.message); }
            }

            async function cancelSub() {
                if (!confirm('Cancel your subscription? You keep access until the end of your billing period.')) return;
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                var res = await fetch('/api/subscriptions/cancel', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token }
                });
                if (res.ok) { window.location.reload(); }
                else { alert('Failed to cancel'); }
            }
            "##
        </script>
    }
}
