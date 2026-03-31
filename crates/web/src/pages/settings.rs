use leptos::prelude::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <section class="py-20 px-6 min-h-[80vh] bg-gradient-to-b from-[#0a0a0e] via-[#060608] to-[#060608]">
            <div class="max-w-2xl mx-auto">
                <h1 class="text-2xl font-bold mb-8">"Account Settings"</h1>

                <div id="settings-error" class="hidden mb-4 p-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm"></div>
                <div id="settings-success" class="hidden mb-4 p-3 rounded-lg bg-green-500/10 border border-green-500/20 text-green-400 text-sm"></div>

                // Profile section
                <div class="mb-8">
                    <h2 class="text-base font-semibold mb-4 flex items-center gap-2">
                        <i class="ph ph-user text-lg text-accent"></i>"Profile"
                    </h2>
                    <div class="p-6 bg-surface-card border border-zinc-800 rounded-lg space-y-4">
                        <div>
                            <label class="block text-xs text-zinc-500 mb-1.5">"Username"</label>
                            <input type="text" id="settings-username" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors" />
                        </div>
                        <div>
                            <label class="block text-xs text-zinc-500 mb-1.5">"Email"</label>
                            <input type="email" id="settings-email" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors" />
                        </div>
                        <button onclick="saveProfile()" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                            <i class="ph ph-check text-base"></i>"Save Changes"
                        </button>
                    </div>
                </div>

                // Password section
                <div class="mb-8">
                    <h2 class="text-base font-semibold mb-4 flex items-center gap-2">
                        <i class="ph ph-lock text-lg text-accent"></i>"Change Password"
                    </h2>
                    <div class="p-6 bg-surface-card border border-zinc-800 rounded-lg space-y-4">
                        <div>
                            <label class="block text-xs text-zinc-500 mb-1.5">"Current Password"</label>
                            <input type="password" id="current-password" placeholder="Current password" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors" />
                        </div>
                        <div>
                            <label class="block text-xs text-zinc-500 mb-1.5">"New Password"</label>
                            <input type="password" id="new-password" placeholder="New password (min 8 characters)" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors" />
                        </div>
                        <div>
                            <label class="block text-xs text-zinc-500 mb-1.5">"Confirm New Password"</label>
                            <input type="password" id="confirm-new-password" placeholder="Confirm new password" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors" />
                        </div>
                        <button onclick="changePassword()" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                            <i class="ph ph-key text-base"></i>"Update Password"
                        </button>
                    </div>
                </div>

                // Payouts section
                <div class="mb-8">
                    <h2 class="text-base font-semibold mb-4 flex items-center gap-2">
                        <i class="ph ph-bank text-lg text-accent"></i>"Payouts"
                    </h2>
                    <div class="p-6 bg-surface-card border border-zinc-800 rounded-lg space-y-4">
                        <div id="connect-status">"Loading payout status..."</div>

                        // Withdrawal form (hidden until connected)
                        <div id="withdraw-section" class="hidden">
                            <div class="border-t border-zinc-800 pt-4 mt-4">
                                <h3 class="text-sm font-semibold mb-3">"Withdraw Credits"</h3>
                                <p class="text-xs text-zinc-500 mb-3">"Minimum 500 credits ($50). Credits are converted at $0.10 each."</p>
                                <div class="flex gap-2">
                                    <input type="number" id="withdraw-amount" min="500" step="100" placeholder="Amount in credits (min 500)"
                                        class="flex-1 px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent" />
                                    <button onclick="requestWithdrawal()" id="withdraw-btn"
                                        class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-green-600 text-white hover:bg-green-500 transition-colors">
                                        <i class="ph ph-arrow-up-right text-base"></i>"Withdraw"
                                    </button>
                                </div>
                                <p id="withdraw-usd" class="text-xs text-zinc-500 mt-2"></p>
                            </div>

                            // Withdrawal history
                            <div class="border-t border-zinc-800 pt-4 mt-4">
                                <h3 class="text-sm font-semibold mb-3">"Withdrawal History"</h3>
                                <div id="withdrawal-list"></div>
                            </div>
                        </div>
                    </div>
                </div>

                // Communication preferences
                <div class="mb-8">
                    <h2 class="text-base font-semibold mb-4 flex items-center gap-2">
                        <i class="ph ph-bell text-lg text-accent"></i>"Communication"
                    </h2>
                    <div class="p-6 bg-surface-card border border-zinc-800 rounded-lg space-y-4">
                        <ToggleRow id="pref-marketing" label="Product updates" desc="News about new features and releases." />
                        <ToggleRow id="pref-marketplace" label="Marketplace notifications" desc="When someone purchases or reviews your assets." />
                        <ToggleRow id="pref-comments" label="Comment notifications" desc="When someone replies to your articles or comments." />
                        <ToggleRow id="pref-security" label="Security alerts" desc="Sign-in from new devices and password changes." />
                    </div>
                </div>

                // Privacy
                <div class="mb-8">
                    <h2 class="text-base font-semibold mb-4 flex items-center gap-2">
                        <i class="ph ph-eye-slash text-lg text-accent"></i>"Privacy"
                    </h2>
                    <div class="p-6 bg-surface-card border border-zinc-800 rounded-lg space-y-4">
                        <div>
                            <label class="block text-xs text-zinc-500 mb-1.5">"Who can message me"</label>
                            <select id="privacy-messages" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors">
                                <option value="everyone">"Everyone"</option>
                                <option value="friends">"Friends only"</option>
                                <option value="nobody">"Nobody"</option>
                            </select>
                        </div>
                        <div>
                            <label class="block text-xs text-zinc-500 mb-1.5">"Profile visibility"</label>
                            <select id="privacy-visibility" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors">
                                <option value="public">"Public"</option>
                                <option value="friends_only">"Friends only"</option>
                            </select>
                        </div>
                        <div class="flex items-center justify-between">
                            <div>
                                <p class="text-sm text-zinc-50">"Show online status"</p>
                                <p class="text-xs text-zinc-500">"Let others see when you're online"</p>
                            </div>
                            <label class="relative inline-flex items-center cursor-pointer">
                                <input type="checkbox" id="privacy-online" checked class="sr-only peer" />
                                <div class="w-9 h-5 bg-zinc-700 rounded-full peer peer-checked:bg-accent transition-colors after:content-[''] after:absolute after:top-0.5 after:left-0.5 after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:after:translate-x-4"></div>
                            </label>
                        </div>
                        <button id="save-privacy" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                            <i class="ph ph-check text-base"></i>"Save Privacy Settings"
                        </button>
                        <div id="privacy-success" class="hidden text-xs text-green-400 mt-1">"Privacy settings saved"</div>
                    </div>
                </div>

                // Blocked Users
                <div class="mb-8">
                    <h2 class="text-base font-semibold mb-4 flex items-center gap-2">
                        <i class="ph ph-prohibit text-lg text-accent"></i>"Blocked Users"
                    </h2>
                    <div class="p-6 bg-surface-card border border-zinc-800 rounded-lg">
                        <div id="block-list">
                            <p class="text-sm text-zinc-500">"Loading..."</p>
                        </div>
                    </div>
                </div>

                // Connected Apps
                <div class="mb-8">
                    <h2 class="text-base font-semibold mb-4 flex items-center gap-2">
                        <i class="ph ph-plugs-connected text-lg text-accent"></i>"Connected Apps"
                    </h2>
                    <div class="p-6 bg-surface-card border border-zinc-800 rounded-lg">
                        <p class="text-xs text-zinc-500 mb-4">"Apps and games you have granted access to your account data. You can revoke access at any time."</p>
                        <div id="connected-apps">
                            <div class="text-center py-4">
                                <div class="inline-block animate-spin w-4 h-4 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                            </div>
                        </div>
                    </div>
                </div>

                // Danger zone
                <div>
                    <h2 class="text-base font-semibold mb-4 flex items-center gap-2 text-red-400">
                        <i class="ph ph-warning text-lg"></i>"Danger Zone"
                    </h2>
                    <div class="p-6 bg-surface-card border border-red-500/20 rounded-lg">
                        <p class="text-sm text-zinc-400 mb-4">"Permanently delete your account and all associated data. This action cannot be undone."</p>
                        <button onclick="if(confirm('Are you sure? This cannot be undone.')) deleteAccount()" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-red-500/10 text-red-400 border border-red-500/20 hover:bg-red-500/20 transition-colors">
                            <i class="ph ph-trash text-base"></i>"Delete Account"
                        </button>
                    </div>
                </div>
            </div>
        </section>

        <script>
            r#"
            function getToken() {
                return document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
            }
            function showMsg(type, msg) {
                const el = document.getElementById('settings-' + type);
                if (el) { el.textContent = msg; el.classList.remove('hidden'); setTimeout(() => el.classList.add('hidden'), 5000); }
            }

            // Load current user data
            (async function() {
                const token = getToken();
                if (!token) { window.location.href = '/login'; return; }
                try {
                    const res = await fetch('/api/auth/me', { headers: { 'Authorization': 'Bearer ' + token } });
                    if (!res.ok) { window.location.href = '/login'; return; }
                    const user = await res.json();
                    document.getElementById('settings-username').value = user.username;
                    document.getElementById('settings-email').value = user.email;
                } catch(e) {}
            })();

            async function saveProfile() {
                const token = getToken();
                const username = document.getElementById('settings-username').value;
                const email = document.getElementById('settings-email').value;
                try {
                    const res = await fetch('/api/auth/me', {
                        method: 'PUT',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({ username, email })
                    });
                    if (!res.ok) { const d = await res.json(); throw new Error(d.error || 'Failed'); }
                    // Update cookie
                    const userCookie = document.cookie.match('(^|;)\\s*user\\s*=\\s*([^;]+)')?.pop();
                    if (userCookie) {
                        const u = JSON.parse(decodeURIComponent(userCookie));
                        u.username = username;
                        u.email = email;
                        document.cookie = `user=${encodeURIComponent(JSON.stringify(u))};path=/;max-age=2592000;SameSite=Strict`;
                    }
                    showMsg('success', 'Profile updated!');
                } catch(e) { showMsg('error', e.message); }
            }

            async function changePassword() {
                const newPw = document.getElementById('new-password').value;
                const confirmPw = document.getElementById('confirm-new-password').value;
                if (newPw !== confirmPw) { showMsg('error', 'Passwords do not match'); return; }
                if (newPw.length < 8) { showMsg('error', 'Password must be at least 8 characters'); return; }
                showMsg('success', 'Password updated! (API endpoint coming soon)');
                document.getElementById('current-password').value = '';
                document.getElementById('new-password').value = '';
                document.getElementById('confirm-new-password').value = '';
            }

            async function deleteAccount() {
                showMsg('error', 'Account deletion coming soon. Contact support.');
            }

            // Payouts
            (async function loadConnectStatus() {
                const token = getToken();
                if (!token) return;
                try {
                    const res = await fetch('/api/credits/connect/status', { headers: { 'Authorization': 'Bearer ' + token } });
                    if (!res.ok) { document.getElementById('connect-status').innerHTML = '<p class="text-xs text-zinc-500">Payouts unavailable.</p>'; return; }
                    const data = await res.json();
                    const el = document.getElementById('connect-status');

                    if (data.onboarded) {
                        el.innerHTML = `
                            <div class="flex items-center gap-2">
                                <span class="w-2 h-2 rounded-full bg-green-400"></span>
                                <span class="text-sm text-green-400 font-medium">Bank account connected</span>
                            </div>
                            <p class="text-xs text-zinc-500 mt-1">Payouts are enabled. You can withdraw credits to your bank.</p>
                        `;
                        document.getElementById('withdraw-section').classList.remove('hidden');
                        loadWithdrawals();
                    } else if (data.connected) {
                        el.innerHTML = `
                            <div class="flex items-center gap-2">
                                <span class="w-2 h-2 rounded-full bg-amber-400"></span>
                                <span class="text-sm text-amber-400 font-medium">Onboarding incomplete</span>
                            </div>
                            <p class="text-xs text-zinc-500 mt-1">Please complete Stripe setup to enable withdrawals.</p>
                            <button onclick="startOnboarding()" class="mt-3 inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                                <i class="ph ph-arrow-square-out text-base"></i>Complete Setup
                            </button>
                        `;
                    } else {
                        el.innerHTML = `
                            <p class="text-sm text-zinc-300 mb-1">Connect your bank account to withdraw earnings.</p>
                            <p class="text-xs text-zinc-500 mb-3">We use Stripe for secure payouts. You'll be redirected to set up your account.</p>
                            <button onclick="startOnboarding()" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                                <i class="ph ph-bank text-base"></i>Connect Bank Account
                            </button>
                        `;
                    }
                } catch(e) { document.getElementById('connect-status').innerHTML = '<p class="text-xs text-zinc-500">Could not load payout status.</p>'; }
            })();

            async function startOnboarding() {
                const token = getToken();
                if (!token) return;
                try {
                    const res = await fetch('/api/credits/connect/onboard', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' }
                    });
                    const data = await res.json();
                    if (res.ok && data.url) window.location.href = data.url;
                    else showMsg('error', data.error || 'Failed to start onboarding');
                } catch(e) { showMsg('error', e.message); }
            }

            // Withdrawal amount preview
            document.getElementById('withdraw-amount')?.addEventListener('input', function() {
                const amount = parseInt(this.value) || 0;
                const usd = (amount * 0.10).toFixed(2);
                document.getElementById('withdraw-usd').textContent = amount >= 500 ? `= $${usd} USD` : 'Minimum 500 credits ($50)';
            });

            async function requestWithdrawal() {
                const token = getToken();
                if (!token) return;
                const amount = parseInt(document.getElementById('withdraw-amount').value);
                if (!amount || amount < 500) { showMsg('error', 'Minimum withdrawal is 500 credits ($50)'); return; }

                const btn = document.getElementById('withdraw-btn');
                btn.disabled = true;
                btn.innerHTML = '<i class="ph ph-spinner animate-spin"></i> Processing...';

                try {
                    const res = await fetch('/api/credits/withdraw', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({ amount })
                    });
                    const data = await res.json();
                    if (res.ok) {
                        showMsg('success', data.message);
                        document.getElementById('withdraw-amount').value = '';
                        document.getElementById('withdraw-usd').textContent = '';
                        loadWithdrawals();
                    } else {
                        showMsg('error', data.error || 'Withdrawal failed');
                    }
                } catch(e) { showMsg('error', e.message); }

                btn.disabled = false;
                btn.innerHTML = '<i class="ph ph-arrow-up-right text-base"></i> Withdraw';
            }

            async function loadWithdrawals() {
                const token = getToken();
                if (!token) return;
                try {
                    const res = await fetch('/api/credits/withdrawals', { headers: { 'Authorization': 'Bearer ' + token } });
                    if (!res.ok) return;
                    const data = await res.json();
                    const el = document.getElementById('withdrawal-list');

                    if (!data.length) { el.innerHTML = '<p class="text-xs text-zinc-500">No withdrawals yet.</p>'; return; }

                    el.innerHTML = data.map(w => {
                        const statusColors = { completed: 'text-green-400', pending: 'text-amber-400', processing: 'text-blue-400', failed: 'text-red-400', rejected: 'text-red-400' };
                        const color = statusColors[w.status] || 'text-zinc-400';
                        return `
                            <div class="flex items-center justify-between py-2.5 border-b border-zinc-800/50 last:border-0">
                                <div>
                                    <span class="text-sm font-medium">${w.amount_credits.toLocaleString()} credits</span>
                                    <span class="text-xs text-zinc-500 ml-2">($${(w.amount_usd_cents / 100).toFixed(2)})</span>
                                </div>
                                <div class="flex items-center gap-3">
                                    <span class="text-xs ${color}">${w.status}</span>
                                    <span class="text-[11px] text-zinc-600">${new Date(w.created_at).toLocaleDateString()}</span>
                                </div>
                            </div>`;
                    }).join('');
                } catch(e) {}
            }

            // ── Connected Apps ──
            (async function loadConnectedApps() {
                const token = getToken();
                const el = document.getElementById('connected-apps');
                if (!token) { el.innerHTML = '<p class="text-xs text-zinc-500">Sign in to see connected apps.</p>'; return; }
                try {
                    const res = await fetch('/api/gameservices/grants', { headers: { 'Authorization': 'Bearer ' + token } });
                    if (!res.ok) { el.innerHTML = '<p class="text-xs text-zinc-500">Could not load connected apps.</p>'; return; }
                    const grants = await res.json();
                    if (!grants.length) { el.innerHTML = '<p class="text-xs text-zinc-500">No apps connected to your account.</p>'; return; }
                    el.innerHTML = grants.map(g => `
                        <div class="flex items-center justify-between py-3 border-b border-zinc-800/50 last:border-0">
                            <div class="flex items-center gap-3">
                                <div class="w-9 h-9 rounded-lg bg-accent/10 flex items-center justify-center">
                                    ${g.app_icon_url ? '<img src="' + g.app_icon_url + '" class="w-6 h-6 rounded" />' : '<i class="ph ph-game-controller text-accent"></i>'}
                                </div>
                                <div>
                                    <div class="text-sm font-medium">${g.app_name}</div>
                                    <div class="text-[11px] text-zinc-500">${g.scopes_granted.join(', ')}</div>
                                    <div class="text-[10px] text-zinc-600">Connected ${new Date(g.granted_at).toLocaleDateString()}</div>
                                </div>
                            </div>
                            <button onclick="revokeApp('${g.app_id}', this)" class="px-3 py-1.5 rounded-lg text-xs text-red-400 hover:bg-red-950/30 border border-transparent hover:border-red-900/50 transition-all">Revoke</button>
                        </div>
                    `).join('');
                } catch(e) { el.innerHTML = '<p class="text-xs text-zinc-500">Error loading apps.</p>'; }
            })();

            async function revokeApp(appId, btn) {
                if (!confirm('Revoke access? The app will no longer be able to access your data.')) return;
                const token = getToken();
                if (!token) return;
                const res = await fetch('/api/gameservices/grants/' + appId, { method: 'DELETE', headers: { 'Authorization': 'Bearer ' + token } });
                if (res.ok) { btn.closest('[class*="flex items-center justify-between"]').remove(); showMsg('success', 'App access revoked.'); }
                else { showMsg('error', 'Failed to revoke access.'); }
            }

            // Check for connect success redirect
            if (new URLSearchParams(window.location.search).get('connect') === 'success') {
                showMsg('success', 'Bank account connected! Payouts are now enabled.');
                history.replaceState({}, '', '/settings');
            }

            // ── Privacy settings ──
            (async function() {
                var token = getToken();
                if (!token) return;

                // Load current settings
                try {
                    var res = await fetch('/api/auth/me', { headers: { 'Authorization': 'Bearer ' + token } });
                    var user = await res.json();
                    if (user.message_privacy) document.getElementById('privacy-messages').value = user.message_privacy;
                    if (user.profile_visibility) document.getElementById('privacy-visibility').value = user.profile_visibility;
                    var onlineEl = document.getElementById('privacy-online');
                    if (onlineEl && user.online_status_visible !== undefined) onlineEl.checked = user.online_status_visible;
                } catch(e) {}

                // Save privacy
                document.getElementById('save-privacy')?.addEventListener('click', async function() {
                    var res = await fetch('/api/user/privacy', {
                        method: 'PUT',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({
                            message_privacy: document.getElementById('privacy-messages').value,
                            profile_visibility: document.getElementById('privacy-visibility').value,
                            online_status_visible: document.getElementById('privacy-online').checked,
                        })
                    });
                    if (res.ok) {
                        var el = document.getElementById('privacy-success');
                        if (el) { el.classList.remove('hidden'); setTimeout(function() { el.classList.add('hidden'); }, 3000); }
                    }
                });

                // Load block list
                try {
                    var bres = await fetch('/api/user/blocked', { headers: { 'Authorization': 'Bearer ' + token } });
                    var blocked = await bres.json();
                    var listEl = document.getElementById('block-list');
                    if (!Array.isArray(blocked) || blocked.length === 0) {
                        listEl.innerHTML = '<p class="text-sm text-zinc-500">No blocked users</p>';
                    } else {
                        listEl.innerHTML = blocked.map(function(b) {
                            return '<div class="flex items-center justify-between py-2.5 border-b border-zinc-800/50 last:border-0">' +
                                '<div class="flex items-center gap-3">' +
                                    '<div class="w-8 h-8 rounded-full bg-zinc-800 flex items-center justify-center text-xs font-bold text-zinc-400">' + (b.username || '?')[0].toUpperCase() + '</div>' +
                                    '<span class="text-sm text-zinc-300">' + b.username + '</span>' +
                                '</div>' +
                                '<button onclick="unblockUser(\'' + b.user_id + '\')" class="text-xs text-accent hover:text-accent-hover">Unblock</button>' +
                            '</div>';
                        }).join('');
                    }
                } catch(e) {
                    var listEl = document.getElementById('block-list');
                    if (listEl) listEl.innerHTML = '<p class="text-sm text-zinc-500">No blocked users</p>';
                }

                window.unblockUser = async function(userId) {
                    await fetch('/api/user/blocked/' + userId, {
                        method: 'DELETE', headers: { 'Authorization': 'Bearer ' + token }
                    });
                    window.location.reload();
                };
            })();
            "#
        </script>
    }
}

#[component]
fn ToggleRow(id: &'static str, label: &'static str, desc: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between">
            <div>
                <p class="text-sm text-zinc-50">{label}</p>
                <p class="text-xs text-zinc-500">{desc}</p>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" id=id checked class="sr-only peer" />
                <div class="w-9 h-5 bg-zinc-700 rounded-full peer peer-checked:bg-accent transition-colors after:content-[''] after:absolute after:top-0.5 after:left-0.5 after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all peer-checked:after:translate-x-4"></div>
            </label>
        </div>
    }
}
