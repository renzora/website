use leptos::prelude::*;

#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <section class="py-20 px-6">
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
                        document.cookie = `user=${encodeURIComponent(JSON.stringify(u))};path=/;max-age=604800;SameSite=Strict`;
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
