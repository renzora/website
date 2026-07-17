use leptos::prelude::*;

#[component]
pub fn DonatePage() -> impl IntoView {
    view! {
        <section class="max-w-4xl mx-auto py-12 px-4">
            // Hero: title + live stats
            <div class="flex flex-col md:flex-row items-center md:items-start justify-between gap-8 mb-12">
                <div class="text-center md:text-left">
                    <h1 class="text-4xl font-extrabold tracking-tight text-zinc-100">"Support Renzora"</h1>
                    <p class="text-sm text-zinc-400 mt-3 max-w-md">"Become a member or sponsor and help us build the free and open source Renzora Engine. Memberships are paid monthly in credits from your balance."</p>
                </div>
                <div class="text-center md:text-right shrink-0">
                    <div class="pb-2 border-b border-zinc-700">
                        <span id="stat-monthly" class="text-4xl font-extrabold text-zinc-100">"..."</span>
                        <span class="text-sm text-zinc-400 ml-2">"Per Month"</span>
                    </div>
                    <div class="py-2 border-b border-zinc-700">
                        <span id="stat-members" class="text-3xl font-extrabold text-zinc-100">"..."</span>
                        <span class="text-sm text-zinc-400 ml-2">"Members"</span>
                    </div>
                    <div class="pt-2">
                        <span id="stat-sponsors" class="text-3xl font-extrabold text-zinc-100">"..."</span>
                        <span class="text-sm text-zinc-400 ml-2">"Sponsors"</span>
                    </div>
                </div>
            </div>

            // Membership tiers
            <h2 class="text-2xl font-bold text-center text-zinc-100 mb-6">"Membership"</h2>
            <div id="tier-grid" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3 mb-4"></div>

            // Corporate tiers (collapsible)
            <div class="text-center mb-12">
                <button onclick="toggleCorporate()" id="corp-toggle" class="px-5 py-2.5 rounded-xl text-sm font-medium bg-zinc-800/50 border border-zinc-700 text-zinc-200 hover:border-accent/50 transition-colors">"Corporate Tiers"</button>
                <div id="corp-grid" class="hidden grid grid-cols-1 sm:grid-cols-3 gap-3 mt-4 text-left"></div>
            </div>

            // Sponsor listing editor (shown to listed members)
            <div id="sponsor-editor" class="hidden bg-surface-card border border-zinc-800 rounded-2xl p-6 mb-12">
                <h2 class="text-base font-semibold text-zinc-200 mb-1">"Your Sponsor Listing"</h2>
                <p class="text-xs text-zinc-500 mb-4" id="sponsor-status"></p>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 mb-3">
                    <input id="sponsor-name" type="text" maxlength="64" placeholder="Display name (defaults to username)"
                        class="px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl text-sm text-zinc-200 placeholder:text-zinc-600 focus:outline-none focus:border-accent/50" />
                    <input id="sponsor-url" type="url" placeholder="Website link (https://...)"
                        class="px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl text-sm text-zinc-200 placeholder:text-zinc-600 focus:outline-none focus:border-accent/50" />
                </div>
                <div id="sponsor-logo-row" class="hidden items-center gap-3 mb-3">
                    <img id="sponsor-logo-preview" class="hidden w-12 h-12 rounded-lg object-contain bg-zinc-900 border border-zinc-800" />
                    <label class="inline-flex items-center gap-2 px-4 py-2 rounded-xl text-xs font-medium bg-zinc-800/50 border border-zinc-700 text-zinc-300 hover:border-accent/50 cursor-pointer transition-colors">
                        <i class="ph ph-image"></i>"Upload logo (max 1MB)"
                        <input id="sponsor-logo-file" type="file" accept="image/*" class="hidden" />
                    </label>
                </div>
                <div class="flex items-center justify-between">
                    <label class="flex items-center gap-2 text-xs text-zinc-400 cursor-pointer">
                        <input type="checkbox" id="sponsor-hidden" class="checkbox checkbox-sm checkbox-accent" />
                        "Hide me from the supporters wall"
                    </label>
                    <button id="sponsor-save" class="px-5 py-2 bg-accent hover:bg-accent-hover text-white text-xs font-medium rounded-xl transition-colors">"Save listing"</button>
                </div>
                <div id="sponsor-msg" class="hidden text-xs mt-2"></div>
            </div>

            // Supporters wall
            <h2 class="text-2xl font-bold text-center text-zinc-100 mb-8">"Renzora Supporters"</h2>
            <div id="sponsor-wall" class="mb-14">
                <div class="flex justify-center py-4"><span class="loading loading-spinner loading-sm text-accent"></span></div>
            </div>

            // One-off donations
            <div class="border-t border-zinc-800 pt-10">
                <div class="text-center mb-6">
                    <h2 class="text-xl font-bold text-zinc-100">"One-off Donations"</h2>
                    <p class="text-xs text-zinc-500 mt-1">"Prefer not to subscribe? Make a single donation of any size."</p>
                    <div class="mt-3 inline-flex items-center gap-2 px-4 py-2 bg-accent/10 border border-accent/20 rounded-xl">
                        <i class="ph ph-heart-fill text-accent"></i>
                        <span class="text-sm text-accent font-medium" id="total-donated">"Loading..."</span>
                        <span class="text-xs text-zinc-500">"credits donated"</span>
                    </div>
                </div>

                <div id="donate-form" class="hidden bg-surface-card border border-zinc-800 rounded-2xl p-6 mb-8">
                    <div class="grid grid-cols-4 gap-2 mb-4">
                        <button class="donate-preset px-4 py-3 bg-zinc-800/50 border border-zinc-700 rounded-xl text-sm text-zinc-300 hover:border-accent/50 hover:text-accent transition-colors" data-amount="10">"10"</button>
                        <button class="donate-preset px-4 py-3 bg-zinc-800/50 border border-zinc-700 rounded-xl text-sm text-zinc-300 hover:border-accent/50 hover:text-accent transition-colors" data-amount="50">"50"</button>
                        <button class="donate-preset px-4 py-3 bg-zinc-800/50 border border-zinc-700 rounded-xl text-sm text-zinc-300 hover:border-accent/50 hover:text-accent transition-colors" data-amount="100">"100"</button>
                        <button class="donate-preset px-4 py-3 bg-zinc-800/50 border border-zinc-700 rounded-xl text-sm text-zinc-300 hover:border-accent/50 hover:text-accent transition-colors" data-amount="500">"500"</button>
                    </div>
                    <div class="flex gap-3 mb-4">
                        <input id="donate-amount" type="number" min="1" class="flex-1 px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl text-sm text-zinc-200 placeholder:text-zinc-600 focus:outline-none focus:border-accent/50" placeholder="Custom amount" />
                        <input id="donate-message" type="text" class="flex-1 px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl text-sm text-zinc-200 placeholder:text-zinc-600 focus:outline-none focus:border-accent/50" placeholder="Message (optional)" />
                    </div>
                    <div class="flex items-center justify-between">
                        <label class="flex items-center gap-2 text-sm text-zinc-400 cursor-pointer">
                            <input type="checkbox" id="donate-anon" class="checkbox checkbox-sm checkbox-accent" />
                            "Donate anonymously"
                        </label>
                        <button id="donate-btn" class="px-6 py-2.5 bg-accent hover:bg-accent-hover text-white text-sm font-medium rounded-xl transition-colors">"Donate"</button>
                    </div>
                    <div id="donate-error" class="hidden text-xs text-red-400 mt-2"></div>
                    <div id="donate-success" class="hidden text-xs text-green-400 mt-2"></div>
                </div>

                <div id="donate-login" class="hidden text-center mb-8 p-6 bg-surface-card border border-zinc-800 rounded-2xl">
                    <p class="text-sm text-zinc-400">"Sign in to become a member or donate"</p>
                    <a href="/login" class="inline-block mt-3 px-6 py-2 bg-accent hover:bg-accent-hover text-white text-sm font-medium rounded-xl transition-colors">"Sign In"</a>
                </div>

                // Donor leaderboard
                <div class="bg-surface-card border border-zinc-800 rounded-2xl p-6">
                    <h2 class="text-base font-semibold text-zinc-200 mb-4">"Top Donors"</h2>
                    <div id="leaderboard" class="space-y-2">
                        <div class="flex justify-center py-4"><span class="loading loading-spinner loading-sm text-accent"></span></div>
                    </div>
                </div>

                // Donor badge info
                <div class="mt-8 grid grid-cols-4 gap-3">
                    <div class="text-center p-4 bg-surface-card border border-zinc-800 rounded-xl">
                        <div class="text-2xl mb-1" style="color: #cd7f32">"♥"</div>
                        <div class="text-xs font-medium text-zinc-300">"Bronze"</div>
                        <div class="text-[10px] text-zinc-500">"100+ donated"</div>
                    </div>
                    <div class="text-center p-4 bg-surface-card border border-zinc-800 rounded-xl">
                        <div class="text-2xl mb-1" style="color: #c0c0c0">"♥"</div>
                        <div class="text-xs font-medium text-zinc-300">"Silver"</div>
                        <div class="text-[10px] text-zinc-500">"500+ donated"</div>
                    </div>
                    <div class="text-center p-4 bg-surface-card border border-zinc-800 rounded-xl">
                        <div class="text-2xl mb-1" style="color: #ffd700">"♥"</div>
                        <div class="text-xs font-medium text-zinc-300">"Gold"</div>
                        <div class="text-[10px] text-zinc-500">"1000+ donated"</div>
                    </div>
                    <div class="text-center p-4 bg-surface-card border border-zinc-800 rounded-xl">
                        <div class="text-2xl mb-1" style="color: #e5e4e2">"♥"</div>
                        <div class="text-xs font-medium text-zinc-300">"Platinum"</div>
                        <div class="text-[10px] text-zinc-500">"5000+ donated"</div>
                    </div>
                </div>
            </div>
        </section>

        <script>
        r##"
        // Tier metadata — thresholds mirror the API constants.
        var TIERS = [
            { slug: 'bronze',   name: 'Bronze',   credits: 50,    color: '#cd7f32', perks: [] },
            { slug: 'silver',   name: 'Silver',   credits: 100,   color: '#c0c0c0', perks: [] },
            { slug: 'gold',     name: 'Gold',     credits: 250,   color: '#ffd700', perks: ['Name on wall'] },
            { slug: 'platinum', name: 'Platinum', credits: 500,   color: '#e5e4e2', perks: ['Name on wall'] },
            { slug: 'titanium', name: 'Titanium', credits: 1000,  color: '#9ba1a6', perks: ['Name on wall', 'Link'] },
            { slug: 'diamond',  name: 'Diamond',  credits: 2500,  color: '#7dd3fc', perks: ['Logo on wall', 'Link'] }
        ];
        var CORP_TIERS = [
            { slug: 'corp_bronze', name: 'Corporate Bronze', credits: 5000,  color: '#cd7f32', perks: ['Large logo', 'Link'] },
            { slug: 'corp_silver', name: 'Corporate Silver', credits: 10000, color: '#c0c0c0', perks: ['Large logo', 'Link'] },
            { slug: 'corp_gold',   name: 'Corporate Gold',   credits: 25000, color: '#ffd700', perks: ['Largest logo', 'Top billing', 'Link'] }
        ];
        var WALL_ORDER = ['corp_gold', 'corp_silver', 'corp_bronze', 'diamond', 'titanium', 'platinum', 'gold'];
        var TIER_NAMES = {};
        TIERS.concat(CORP_TIERS).forEach(function(t) { TIER_NAMES[t.slug] = t; });

        var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();

        function usd(credits) { return '$' + (credits * 0.1).toLocaleString(undefined, {maximumFractionDigits: 0}); }
        function esc(t) { var d = document.createElement('div'); d.textContent = t || ''; return d.innerHTML; }

        function tierCard(t, big) {
            var perkChips = t.perks.map(function(p) {
                return '<span class="px-2 py-0.5 rounded-full border border-zinc-700 text-[10px] text-zinc-400">' + p + '</span>';
            }).join(' ');
            return '<button onclick="joinTier(' + t.credits + ', \'' + t.name + '\')" class="text-left p-5 bg-white/[0.02] border border-zinc-800/70 rounded-2xl hover:border-accent/50 transition-all group">' +
                '<div class="flex items-center gap-3">' +
                    '<div class="w-10 h-10 rounded-xl flex items-center justify-center border" style="border-color:' + t.color + '40; background:' + t.color + '14">' +
                        '<i class="ph ph-medal text-xl" style="color:' + t.color + '"></i>' +
                    '</div>' +
                    '<div>' +
                        '<div class="text-lg font-bold" style="color:' + t.color + '">' + t.name + '</div>' +
                        '<div class="text-xs text-zinc-400">' + t.credits.toLocaleString() + ' credits / mo <span class="text-zinc-600">(' + usd(t.credits) + '/month)</span></div>' +
                    '</div>' +
                '</div>' +
                (perkChips ? '<div class="flex flex-wrap gap-1.5 mt-3">' + perkChips + '</div>' : '') +
            '</button>';
        }

        function toggleCorporate() {
            document.getElementById('corp-grid').classList.toggle('hidden');
        }

        function joinTier(credits, name) {
            if (!token) { window.location.href = '/login'; return; }
            if (!confirm('Become a ' + name + ' member for ' + credits.toLocaleString() + ' credits/month (' + usd(credits) + ')?\n\nThe first month is deducted from your credit balance now; it renews every 30 days while auto-renew is on.')) return;
            fetch('/api/subscriptions/subscribe', {
                method: 'POST',
                headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                body: JSON.stringify({ amount: credits, auto_renew: true })
            }).then(async function(res) {
                var data = await res.json();
                if (res.ok) { window.location.reload(); }
                else { alert(data.error || data.message || 'Failed to subscribe'); }
            }).catch(function(e) { alert('Error: ' + e.message); });
        }

        (function renderTiers() {
            document.getElementById('tier-grid').innerHTML = TIERS.map(function(t) { return tierCard(t); }).join('');
            document.getElementById('corp-grid').innerHTML = CORP_TIERS.map(function(t) { return tierCard(t, true); }).join('');
        })();

        (async function init() {
            // Sponsors + stats
            fetch('/api/credits/donate/sponsors').then(r => r.json()).then(data => {
                var stats = data.stats || {};
                document.getElementById('stat-monthly').textContent = usd(stats.credits_per_month || 0);
                document.getElementById('stat-members').textContent = (stats.members || 0).toLocaleString();
                document.getElementById('stat-sponsors').textContent = (stats.sponsors || 0).toLocaleString();
                renderWall(data.sponsors || []);
            }).catch(() => {
                document.getElementById('sponsor-wall').innerHTML = '<p class="text-sm text-zinc-500 text-center py-4">Failed to load supporters.</p>';
            });

            // Donation total
            fetch('/api/credits/donate/total').then(r => r.json()).then(data => {
                document.getElementById('total-donated').textContent = (data.total || 0).toLocaleString();
            }).catch(() => {});

            // Donor leaderboard
            fetch('/api/credits/donate/leaderboard').then(r => r.json()).then(data => {
                var el = document.getElementById('leaderboard');
                if (!Array.isArray(data) || data.length === 0) {
                    el.innerHTML = '<p class="text-sm text-zinc-500 text-center py-4">No donations yet. Be the first!</p>';
                    return;
                }
                el.innerHTML = data.map(function(d, i) {
                    var medal = i === 0 ? '🥇' : i === 1 ? '🥈' : i === 2 ? '🥉' : (i + 1) + '.';
                    var name = d.username || 'Anonymous';
                    return '<div class="flex items-center gap-3 py-2.5 px-3 ' + (i < 3 ? 'bg-accent/5 border border-accent/10' : 'bg-zinc-800/30') + ' rounded-lg">' +
                        '<span class="text-base w-8 text-center">' + medal + '</span>' +
                        '<div class="w-8 h-8 rounded-full bg-zinc-800 flex items-center justify-center text-xs font-bold text-zinc-400">' + esc(name[0].toUpperCase()) + '</div>' +
                        '<span class="flex-1 text-sm text-zinc-200">' + esc(name) + '</span>' +
                        '<span class="text-sm font-medium text-accent">' + (d.total || 0).toLocaleString() + ' credits</span>' +
                    '</div>';
                }).join('');
            }).catch(() => {});

            // Auth-dependent bits
            if (token) {
                document.getElementById('donate-form').classList.remove('hidden');
                loadSponsorEditor();
            } else {
                document.getElementById('donate-login').classList.remove('hidden');
            }

            // Donation presets + submit
            document.querySelectorAll('.donate-preset').forEach(function(btn) {
                btn.addEventListener('click', function() {
                    document.getElementById('donate-amount').value = btn.dataset.amount;
                    document.querySelectorAll('.donate-preset').forEach(b => b.classList.remove('border-accent', 'text-accent'));
                    btn.classList.add('border-accent', 'text-accent');
                });
            });

            document.getElementById('donate-btn')?.addEventListener('click', async function() {
                var amount = parseInt(document.getElementById('donate-amount').value);
                var message = document.getElementById('donate-message').value;
                var anonymous = document.getElementById('donate-anon').checked;
                var errorEl = document.getElementById('donate-error');
                var successEl = document.getElementById('donate-success');

                errorEl.classList.add('hidden');
                successEl.classList.add('hidden');

                if (!amount || amount < 1) { errorEl.textContent = 'Enter an amount'; errorEl.classList.remove('hidden'); return; }

                var res = await fetch('/api/credits/donate', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ amount: amount, message: message, anonymous: anonymous })
                });
                var data = await res.json();
                if (data.ok) {
                    var msg = 'Thank you! You donated ' + amount + ' credits. Total: ' + (data.total_donated || 0).toLocaleString();
                    if (data.new_badges && data.new_badges.length) {
                        msg += ' — 🏅 You earned the ' + data.new_badges.map(function(b) { return b.name; }).join(' and ') + ' badge' + (data.new_badges.length > 1 ? 's' : '') + '!';
                    }
                    successEl.textContent = msg;
                    successEl.classList.remove('hidden');
                    document.getElementById('donate-amount').value = '';
                    document.getElementById('donate-message').value = '';
                    fireConfetti();
                    setTimeout(function() { window.location.reload(); }, 2500);
                } else {
                    errorEl.textContent = data.error || 'Failed to donate';
                    errorEl.classList.remove('hidden');
                }
            });
        })();

        function fireConfetti() {
            var colors = ['#6366f1', '#818cf8', '#a78bfa', '#22c55e', '#f59e0b', '#ec4899', '#06b6d4'];
            var container = document.createElement('div');
            container.style.cssText = 'position:fixed;inset:0;pointer-events:none;z-index:9999;overflow:hidden';
            document.body.appendChild(container);
            for (var i = 0; i < 80; i++) {
                var p = document.createElement('div');
                var size = Math.random() * 8 + 4;
                var x = Math.random() * 100;
                var color = colors[Math.floor(Math.random() * colors.length)];
                var delay = Math.random() * 0.3;
                var drift = (Math.random() - 0.5) * 200;
                var shape = Math.random() > 0.5 ? '50%' : '0';
                p.style.cssText = 'position:absolute;top:-10px;left:' + x + '%;width:' + size + 'px;height:' + size + 'px;background:' + color + ';border-radius:' + shape + ';opacity:0.9;animation:confettiFall ' + (1.5 + Math.random()) + 's ease-out ' + delay + 's forwards';
                p.style.setProperty('--drift', drift + 'px');
                container.appendChild(p);
            }
            setTimeout(function() { container.remove(); }, 3000);
        }

        function renderWall(sponsors) {
            var el = document.getElementById('sponsor-wall');
            if (!sponsors.length) {
                el.innerHTML = '<p class="text-sm text-zinc-500 text-center py-4">No supporters at Gold or above yet — claim the first spot!</p>';
                return;
            }
            var byTier = {};
            sponsors.forEach(function(s) { (byTier[s.tier] = byTier[s.tier] || []).push(s); });

            var linkOpen = function(s, cls) {
                return s.url
                    ? '<a href="' + esc(s.url) + '" target="_blank" rel="noopener nofollow sponsored" class="' + cls + '">'
                    : '<a href="/profile/' + esc(s.username) + '" class="' + cls + '">';
            };

            var html = '';
            WALL_ORDER.forEach(function(slug) {
                var group = byTier[slug];
                if (!group) return;
                var t = TIER_NAMES[slug];
                var isCorp = slug.indexOf('corp_') === 0;

                html += '<div class="text-center mb-10">';
                html += '<h3 class="text-lg font-bold text-zinc-100">' + t.name + '</h3>';
                html += '<p class="text-xs text-zinc-500 mb-4">' + usd(t.credits) + ' / month</p>';

                if (isCorp) {
                    // Corporate: large logos stacked
                    html += '<div class="flex flex-wrap items-center justify-center gap-6">';
                    group.forEach(function(s) {
                        html += linkOpen(s, 'hover:opacity-80 transition-opacity');
                        if (s.logo_url) {
                            var h = slug === 'corp_gold' ? 'h-24 max-w-[420px]' : slug === 'corp_silver' ? 'h-20 max-w-[340px]' : 'h-16 max-w-[280px]';
                            html += '<img src="' + esc(s.logo_url) + '" alt="' + esc(s.name) + '" title="' + esc(s.name) + '" class="' + h + ' object-contain">';
                        } else {
                            html += '<span class="text-2xl font-bold text-zinc-100">' + esc(s.name) + '</span>';
                        }
                        html += '</a>';
                    });
                    html += '</div>';
                } else if (slug === 'diamond') {
                    // Diamond: logos and linked names mixed
                    html += '<div class="flex flex-wrap items-center justify-center gap-x-6 gap-y-4">';
                    group.forEach(function(s) {
                        html += linkOpen(s, 'hover:opacity-80 transition-opacity');
                        if (s.logo_url) html += '<img src="' + esc(s.logo_url) + '" alt="' + esc(s.name) + '" title="' + esc(s.name) + '" class="h-10 max-w-[180px] object-contain">';
                        else html += '<span class="text-base font-medium text-accent">' + esc(s.name) + '</span>';
                        html += '</a>';
                    });
                    html += '</div>';
                } else if (slug === 'titanium') {
                    // Titanium: linked names
                    html += '<div class="flex flex-wrap justify-center gap-x-5 gap-y-2">';
                    group.forEach(function(s) {
                        html += linkOpen(s, 'text-[15px] text-accent hover:text-accent-hover transition-colors') + esc(s.name) + '</a>';
                    });
                    html += '</div>';
                } else {
                    // Platinum & Gold: plain names, denser as the tier lowers
                    var cls = slug === 'platinum' ? 'text-[14px] text-zinc-300' : 'text-[13px] text-zinc-400';
                    html += '<div class="flex flex-wrap justify-center gap-x-4 gap-y-1.5 max-w-2xl mx-auto">';
                    group.forEach(function(s) {
                        html += '<span class="' + cls + '">' + esc(s.name) + '</span>';
                    });
                    html += '</div>';
                }
                html += '</div>';
            });

            el.innerHTML = html;
        }

        async function loadSponsorEditor() {
            try {
                var res = await fetch('/api/credits/donate/sponsor-profile', { headers: { 'Authorization': 'Bearer ' + token } });
                if (!res.ok) return;
                var data = await res.json();
                if (!data.listed) return; // below Gold — nothing to customize

                document.getElementById('sponsor-editor').classList.remove('hidden');
                var t = TIER_NAMES[data.tier];
                document.getElementById('sponsor-status').textContent =
                    'You are a ' + (t ? t.name : data.tier) + ' member (' + data.monthly_amount.toLocaleString() + ' credits/month). Customize how you appear on the supporters wall.' +
                    (data.link_eligible ? '' : ' Links unlock at Titanium; logos at Diamond.');

                if (data.profile) {
                    document.getElementById('sponsor-name').value = data.profile.display_name || '';
                    document.getElementById('sponsor-url').value = data.profile.website_url || '';
                    document.getElementById('sponsor-hidden').checked = !!data.profile.hidden;
                    if (data.profile.logo_url) {
                        var img = document.getElementById('sponsor-logo-preview');
                        img.src = data.profile.logo_url;
                        img.classList.remove('hidden');
                    }
                }
                if (data.logo_eligible) {
                    var row = document.getElementById('sponsor-logo-row');
                    row.classList.remove('hidden');
                    row.classList.add('flex');
                }
            } catch(e) {}
        }

        function sponsorMsg(ok, text) {
            var el = document.getElementById('sponsor-msg');
            el.textContent = text;
            el.className = 'text-xs mt-2 ' + (ok ? 'text-green-400' : 'text-red-400');
        }

        document.getElementById('sponsor-save')?.addEventListener('click', async function() {
            var res = await fetch('/api/credits/donate/sponsor-profile', {
                method: 'PUT',
                headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    display_name: document.getElementById('sponsor-name').value,
                    website_url: document.getElementById('sponsor-url').value,
                    hidden: document.getElementById('sponsor-hidden').checked
                })
            });
            var data = await res.json();
            if (res.ok) sponsorMsg(true, 'Saved! The wall updates immediately.');
            else sponsorMsg(false, data.error || 'Failed to save');
        });

        document.getElementById('sponsor-logo-file')?.addEventListener('change', async function() {
            var file = this.files[0];
            if (!file) return;
            if (file.size > 1024 * 1024) { sponsorMsg(false, 'Logo must be under 1MB'); return; }
            var fd = new FormData();
            fd.append('logo', file);
            var res = await fetch('/api/credits/donate/sponsor-logo', {
                method: 'PUT',
                headers: { 'Authorization': 'Bearer ' + token },
                body: fd
            });
            var data = await res.json();
            if (res.ok) {
                var img = document.getElementById('sponsor-logo-preview');
                img.src = data.logo_url;
                img.classList.remove('hidden');
                sponsorMsg(true, 'Logo uploaded!');
            } else {
                sponsorMsg(false, data.error || 'Upload failed');
            }
        });
        "##
        </script>
        <style>
            r#"
            @keyframes confettiFall {
                0% { transform: translateY(0) translateX(0) rotate(0deg); opacity: 1; }
                100% { transform: translateY(100vh) translateX(var(--drift, 0px)) rotate(720deg); opacity: 0; }
            }
            "#
        </style>
    }
}
