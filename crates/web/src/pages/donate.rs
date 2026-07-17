use leptos::prelude::*;

#[component]
pub fn DonatePage() -> impl IntoView {
    view! {
        <section class="max-w-3xl mx-auto py-12 px-4">
            <div class="text-center mb-8">
                <h1 class="text-2xl font-bold text-zinc-100">"Support Renzora"</h1>
                <p class="text-sm text-zinc-400 mt-2">"Your donations help us keep the platform running and fund new features."</p>
                <div class="mt-4 inline-flex items-center gap-2 px-4 py-2 bg-accent/10 border border-accent/20 rounded-xl">
                    <i class="ph ph-heart-fill text-accent"></i>
                    <span class="text-sm text-accent font-medium" id="total-donated">"Loading..."</span>
                    <span class="text-xs text-zinc-500">"credits donated"</span>
                </div>
            </div>

            // Donate form (auth required)
            <div id="donate-form" class="hidden bg-surface-card border border-zinc-800 rounded-2xl p-6 mb-8">
                <h2 class="text-base font-semibold text-zinc-200 mb-4">"Make a Donation"</h2>
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
                <p class="text-sm text-zinc-400">"Sign in to make a donation"</p>
                <a href="/login" class="inline-block mt-3 px-6 py-2 bg-accent hover:bg-accent-hover text-white text-sm font-medium rounded-xl transition-colors">"Sign In"</a>
            </div>

            // Sponsor wall
            <div class="bg-surface-card border border-zinc-800 rounded-2xl p-6 mb-8">
                <h2 class="text-base font-semibold text-zinc-200 mb-1">"Our Sponsors"</h2>
                <p class="text-xs text-zinc-500 mb-5">"Thank you to everyone supporting Renzora. Sponsor tiers are based on your total donations."</p>
                <div id="sponsor-wall">
                    <div class="flex justify-center py-4"><span class="loading loading-spinner loading-sm text-accent"></span></div>
                </div>

                // Tier explainer
                <div class="mt-6 pt-5 border-t border-zinc-800/50">
                    <h3 class="text-xs font-semibold text-zinc-400 uppercase tracking-widest mb-3">"Sponsor tiers"</h3>
                    <div class="grid grid-cols-1 sm:grid-cols-5 gap-2 text-center">
                        <div class="p-3 bg-zinc-800/30 rounded-xl">
                            <div class="text-xs font-semibold" style="color:#cd7f32">"Bronze"</div>
                            <div class="text-[10px] text-zinc-500 mt-0.5">"100+ credits"</div>
                            <div class="text-[10px] text-zinc-600 mt-1">"Name listed"</div>
                        </div>
                        <div class="p-3 bg-zinc-800/30 rounded-xl">
                            <div class="text-xs font-semibold" style="color:#c0c0c0">"Silver"</div>
                            <div class="text-[10px] text-zinc-500 mt-0.5">"500+ credits"</div>
                            <div class="text-[10px] text-zinc-600 mt-1">"Name + link"</div>
                        </div>
                        <div class="p-3 bg-zinc-800/30 rounded-xl">
                            <div class="text-xs font-semibold" style="color:#ffd700">"Gold"</div>
                            <div class="text-[10px] text-zinc-500 mt-0.5">"1,000+ credits"</div>
                            <div class="text-[10px] text-zinc-600 mt-1">"Highlighted + link"</div>
                        </div>
                        <div class="p-3 bg-zinc-800/30 rounded-xl border border-zinc-700">
                            <div class="text-xs font-semibold" style="color:#e5e4e2">"Platinum"</div>
                            <div class="text-[10px] text-zinc-500 mt-0.5">"5,000+ credits"</div>
                            <div class="text-[10px] text-zinc-600 mt-1">"Logo + link"</div>
                        </div>
                        <div class="p-3 bg-accent/10 rounded-xl border border-accent/30">
                            <div class="text-xs font-semibold text-accent">"Corporate"</div>
                            <div class="text-[10px] text-zinc-500 mt-0.5">"25,000+ credits"</div>
                            <div class="text-[10px] text-zinc-600 mt-1">"Large logo, top billing"</div>
                        </div>
                    </div>
                </div>
            </div>

            // Sponsor profile editor (shown to signed-in donors)
            <div id="sponsor-editor" class="hidden bg-surface-card border border-zinc-800 rounded-2xl p-6 mb-8">
                <h2 class="text-base font-semibold text-zinc-200 mb-1">"Your Sponsor Listing"</h2>
                <p class="text-xs text-zinc-500 mb-4" id="sponsor-status"></p>
                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 mb-3">
                    <input id="sponsor-name" type="text" maxlength="64" placeholder="Display name (defaults to username)"
                        class="px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl text-sm text-zinc-200 placeholder:text-zinc-600 focus:outline-none focus:border-accent/50" />
                    <input id="sponsor-url" type="url" placeholder="Website link (https://...)"
                        class="px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl text-sm text-zinc-200 placeholder:text-zinc-600 focus:outline-none focus:border-accent/50" />
                </div>
                <div id="sponsor-logo-row" class="hidden flex items-center gap-3 mb-3">
                    <img id="sponsor-logo-preview" class="hidden w-12 h-12 rounded-lg object-contain bg-zinc-900 border border-zinc-800" />
                    <label class="inline-flex items-center gap-2 px-4 py-2 rounded-xl text-xs font-medium bg-zinc-800/50 border border-zinc-700 text-zinc-300 hover:border-accent/50 cursor-pointer transition-colors">
                        <i class="ph ph-image"></i>"Upload logo (max 1MB)"
                        <input id="sponsor-logo-file" type="file" accept="image/*" class="hidden" />
                    </label>
                </div>
                <div class="flex items-center justify-between">
                    <label class="flex items-center gap-2 text-xs text-zinc-400 cursor-pointer">
                        <input type="checkbox" id="sponsor-hidden" class="checkbox checkbox-sm checkbox-accent" />
                        "Hide me from the sponsor wall"
                    </label>
                    <button id="sponsor-save" class="px-5 py-2 bg-accent hover:bg-accent-hover text-white text-xs font-medium rounded-xl transition-colors">"Save listing"</button>
                </div>
                <div id="sponsor-msg" class="hidden text-xs mt-2"></div>
            </div>

            // Leaderboard
            <div class="bg-surface-card border border-zinc-800 rounded-2xl p-6">
                <h2 class="text-base font-semibold text-zinc-200 mb-4">"Donation Leaderboard"</h2>
                <div id="leaderboard" class="space-y-2">
                    <div class="flex justify-center py-4"><span class="loading loading-spinner loading-sm text-accent"></span></div>
                </div>
            </div>

            // Badge info
            <div class="mt-8 grid grid-cols-4 gap-3">
                <div class="text-center p-4 bg-surface-card border border-zinc-800 rounded-xl">
                    <div class="text-2xl mb-1" style="color: #cd7f32">"♥"</div>
                    <div class="text-xs font-medium text-zinc-300">"Bronze"</div>
                    <div class="text-[10px] text-zinc-500">"100+ credits"</div>
                </div>
                <div class="text-center p-4 bg-surface-card border border-zinc-800 rounded-xl">
                    <div class="text-2xl mb-1" style="color: #c0c0c0">"♥"</div>
                    <div class="text-xs font-medium text-zinc-300">"Silver"</div>
                    <div class="text-[10px] text-zinc-500">"500+ credits"</div>
                </div>
                <div class="text-center p-4 bg-surface-card border border-zinc-800 rounded-xl">
                    <div class="text-2xl mb-1" style="color: #ffd700">"♥"</div>
                    <div class="text-xs font-medium text-zinc-300">"Gold"</div>
                    <div class="text-[10px] text-zinc-500">"1000+ credits"</div>
                </div>
                <div class="text-center p-4 bg-surface-card border border-zinc-800 rounded-xl">
                    <div class="text-2xl mb-1" style="color: #e5e4e2">"♥"</div>
                    <div class="text-xs font-medium text-zinc-300">"Platinum"</div>
                    <div class="text-[10px] text-zinc-500">"5000+ credits"</div>
                </div>
            </div>
        </section>

        <script>
        r##"
        (async function() {
            var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();

            // Load total
            fetch('/api/credits/donate/total').then(r => r.json()).then(data => {
                document.getElementById('total-donated').textContent = (data.total || 0).toLocaleString();
            }).catch(() => {});

            // Load leaderboard
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
                        '<div class="w-8 h-8 rounded-full bg-zinc-800 flex items-center justify-center text-xs font-bold text-zinc-400">' + name[0].toUpperCase() + '</div>' +
                        '<span class="flex-1 text-sm text-zinc-200">' + name + '</span>' +
                        '<span class="text-sm font-medium text-accent">' + (d.total || 0).toLocaleString() + ' credits</span>' +
                    '</div>';
                }).join('');
            }).catch(() => {});

            // Load sponsor wall
            fetch('/api/credits/donate/sponsors').then(r => r.json()).then(data => {
                var el = document.getElementById('sponsor-wall');
                var sponsors = (data && data.sponsors) || [];
                if (!sponsors.length) {
                    el.innerHTML = '<p class="text-sm text-zinc-500 text-center py-4">No sponsors yet — donate to claim the first spot!</p>';
                    return;
                }
                var byTier = {};
                sponsors.forEach(function(s) { (byTier[s.tier] = byTier[s.tier] || []).push(s); });

                var esc = function(t) { var d = document.createElement('div'); d.textContent = t || ''; return d.innerHTML; };
                var linkOpen = function(s) { return s.url ? '<a href="' + esc(s.url) + '" target="_blank" rel="noopener nofollow sponsored" class="hover:opacity-80 transition-opacity">' : '<a href="/profile/' + esc(s.username) + '" class="hover:opacity-80 transition-opacity">'; };

                var html = '';

                // Corporate: large logos, top billing
                if (byTier.corporate) {
                    html += '<div class="mb-5"><h3 class="text-[10px] font-semibold text-accent uppercase tracking-widest mb-3">Corporate</h3><div class="flex flex-wrap items-center gap-4">';
                    byTier.corporate.forEach(function(s) {
                        html += linkOpen(s);
                        if (s.logo_url) html += '<img src="' + esc(s.logo_url) + '" alt="' + esc(s.name) + '" title="' + esc(s.name) + '" class="h-16 max-w-[220px] object-contain rounded-lg bg-white/[0.03] border border-accent/20 p-2">';
                        else html += '<span class="px-5 py-3 rounded-xl bg-accent/10 border border-accent/30 text-base font-semibold text-zinc-100">' + esc(s.name) + '</span>';
                        html += '</a>';
                    });
                    html += '</div></div>';
                }

                // Platinum: logos
                if (byTier.platinum) {
                    html += '<div class="mb-5"><h3 class="text-[10px] font-semibold uppercase tracking-widest mb-3" style="color:#e5e4e2">Platinum</h3><div class="flex flex-wrap items-center gap-3">';
                    byTier.platinum.forEach(function(s) {
                        html += linkOpen(s);
                        if (s.logo_url) html += '<img src="' + esc(s.logo_url) + '" alt="' + esc(s.name) + '" title="' + esc(s.name) + '" class="h-10 max-w-[160px] object-contain rounded-lg bg-white/[0.03] border border-zinc-700 p-1.5">';
                        else html += '<span class="px-4 py-2 rounded-lg bg-zinc-800/50 border border-zinc-700 text-sm font-medium text-zinc-200">' + esc(s.name) + '</span>';
                        html += '</a>';
                    });
                    html += '</div></div>';
                }

                // Gold: highlighted names + links
                if (byTier.gold) {
                    html += '<div class="mb-4"><h3 class="text-[10px] font-semibold uppercase tracking-widest mb-2" style="color:#ffd700">Gold</h3><div class="flex flex-wrap gap-2">';
                    byTier.gold.forEach(function(s) {
                        html += linkOpen(s) + '<span class="px-3 py-1.5 rounded-lg bg-yellow-500/10 border border-yellow-500/20 text-sm text-zinc-200">' + esc(s.name) + '</span></a>';
                    });
                    html += '</div></div>';
                }

                // Silver: names + links
                if (byTier.silver) {
                    html += '<div class="mb-4"><h3 class="text-[10px] font-semibold uppercase tracking-widest mb-2" style="color:#c0c0c0">Silver</h3><div class="flex flex-wrap gap-2">';
                    byTier.silver.forEach(function(s) {
                        html += linkOpen(s) + '<span class="px-3 py-1 rounded-lg bg-zinc-800/40 text-[13px] text-zinc-300">' + esc(s.name) + '</span></a>';
                    });
                    html += '</div></div>';
                }

                // Bronze: names only
                if (byTier.bronze) {
                    html += '<div><h3 class="text-[10px] font-semibold uppercase tracking-widest mb-2" style="color:#cd7f32">Bronze</h3><p class="text-[13px] text-zinc-400 leading-relaxed">';
                    html += byTier.bronze.map(function(s) { return esc(s.name); }).join('<span class="text-zinc-700"> · </span>');
                    html += '</p></div>';
                }

                el.innerHTML = html;
            }).catch(() => {
                document.getElementById('sponsor-wall').innerHTML = '<p class="text-sm text-zinc-500 text-center py-4">Failed to load sponsors.</p>';
            });

            // Auth check
            if (token) {
                document.getElementById('donate-form').classList.remove('hidden');
                loadSponsorEditor();
            } else {
                document.getElementById('donate-login').classList.remove('hidden');
            }

            async function loadSponsorEditor() {
                try {
                    var res = await fetch('/api/credits/donate/sponsor-profile', { headers: { 'Authorization': 'Bearer ' + token } });
                    if (!res.ok) return;
                    var data = await res.json();
                    if (!data.tier) return; // not a sponsor yet — keep the editor hidden

                    document.getElementById('sponsor-editor').classList.remove('hidden');
                    var tierName = data.tier.charAt(0).toUpperCase() + data.tier.slice(1);
                    document.getElementById('sponsor-status').textContent =
                        'You are a ' + tierName + ' sponsor (' + data.total_donated.toLocaleString() + ' credits donated). Customize how you appear on the wall.';

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

            // Preset buttons
            document.querySelectorAll('.donate-preset').forEach(function(btn) {
                btn.addEventListener('click', function() {
                    document.getElementById('donate-amount').value = btn.dataset.amount;
                    document.querySelectorAll('.donate-preset').forEach(b => b.classList.remove('border-accent', 'text-accent'));
                    btn.classList.add('border-accent', 'text-accent');
                });
            });

            // Donate
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
                    successEl.textContent = 'Thank you! You donated ' + amount + ' credits. Total: ' + (data.total_donated || 0).toLocaleString();
                    successEl.classList.remove('hidden');
                    document.getElementById('donate-amount').value = '';
                    document.getElementById('donate-message').value = '';
                    // Reload leaderboard and total
                    setTimeout(function() { window.location.reload(); }, 1500);
                } else {
                    errorEl.textContent = data.error || 'Failed to donate';
                    errorEl.classList.remove('hidden');
                }
            });
        })();
        "##
        </script>
    }
}
