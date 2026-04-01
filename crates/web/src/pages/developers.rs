use leptos::prelude::*;

#[component]
pub fn DevelopersPage() -> impl IntoView {
    view! {
        <section class="py-12 px-6 min-h-screen">
            <div class="max-w-4xl mx-auto">
                <div class="mb-10">
                    <h1 class="text-3xl font-bold">"Developers"</h1>
                    <p class="text-zinc-400 mt-2">"Build integrations, automate uploads, and extend the Renzora ecosystem with our API."</p>
                </div>

                <div id="dev-content">
                    <div class="text-center py-12">
                        <div class="inline-block animate-spin w-6 h-6 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                    </div>
                </div>
            </div>
        </section>
        <script>
            r##"
            (async function() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                const el = document.getElementById('dev-content');

                // Fetch existing tokens if logged in
                let tokens = [];
                if (token) {
                    try {
                        const res = await fetch('/api/api-tokens', { headers: { 'Authorization': 'Bearer ' + token } });
                        if (res.ok) tokens = await res.json();
                    } catch(e) {}
                }

                el.innerHTML = `
                    <!-- API Overview -->
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-10">
                        <div class="p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl">
                            <div class="w-10 h-10 rounded-lg bg-accent/10 flex items-center justify-center mb-3">
                                <i class="ph ph-game-controller text-accent text-lg"></i>
                            </div>
                            <h3 class="font-semibold mb-1">Game Services</h3>
                            <p class="text-sm text-zinc-500">Achievements, leaderboards, player stats, and friends for your game.</p>
                        </div>
                        <div class="p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl">
                            <div class="w-10 h-10 rounded-lg bg-accent/10 flex items-center justify-center mb-3">
                                <i class="ph ph-shield-check text-accent text-lg"></i>
                            </div>
                            <h3 class="font-semibold mb-1">Scoped Access</h3>
                            <p class="text-sm text-zinc-500">Users grant your app specific permissions. No scope, no access.</p>
                        </div>
                        <div class="p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl">
                            <div class="w-10 h-10 rounded-lg bg-accent/10 flex items-center justify-center mb-3">
                                <i class="ph ph-brackets-curly text-accent text-lg"></i>
                            </div>
                            <h3 class="font-semibold mb-1">REST API</h3>
                            <p class="text-sm text-zinc-500">Marketplace API, game services, and multipart uploads.</p>
                        </div>
                    </div>

                    <!-- Developer Apps Section -->
                    <div class="mb-10">
                        <div class="flex items-center justify-between mb-4">
                            <h2 class="text-xl font-semibold">My Apps</h2>
                            ${token ? '<button onclick="document.getElementById(\\\'register-app-form\\\').classList.toggle(\\\'hidden\\\')" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all">Register New App</button>' : ''}
                        </div>
                        ${!token ? '<p class="text-sm text-zinc-500"><a href="/login" class="text-accent hover:text-accent-hover">Sign in</a> to register developer apps.</p>' : `
                            <div id="app-list" class="space-y-2"></div>
                            <div id="register-app-form" class="hidden mt-4 p-5 bg-zinc-900/50 border border-zinc-800 rounded-xl">
                                <h3 class="text-sm font-semibold text-zinc-200 mb-3">Register New App</h3>
                                <div class="space-y-3">
                                    <div>
                                        <label class="text-xs text-zinc-400 block mb-1">App Name</label>
                                        <input id="app-name" type="text" class="w-full px-4 py-2 bg-zinc-900 border border-zinc-700 rounded-lg text-sm text-zinc-200 placeholder:text-zinc-600" placeholder="My Game" />
                                    </div>
                                    <div>
                                        <label class="text-xs text-zinc-400 block mb-1">Description</label>
                                        <textarea id="app-desc" class="w-full px-4 py-2 bg-zinc-900 border border-zinc-700 rounded-lg text-sm text-zinc-200 placeholder:text-zinc-600 resize-none" rows="2" placeholder="What does your app do?"></textarea>
                                    </div>
                                    <div>
                                        <label class="text-xs text-zinc-400 block mb-1">Website URL (optional)</label>
                                        <input id="app-url" type="url" class="w-full px-4 py-2 bg-zinc-900 border border-zinc-700 rounded-lg text-sm text-zinc-200 placeholder:text-zinc-600" placeholder="https://mygame.com" />
                                    </div>
                                    <div>
                                        <label class="text-xs text-zinc-400 block mb-1">Redirect URI (optional, for OAuth)</label>
                                        <input id="app-redirect" type="url" class="w-full px-4 py-2 bg-zinc-900 border border-zinc-700 rounded-lg text-sm text-zinc-200 placeholder:text-zinc-600" placeholder="https://mygame.com/callback" />
                                    </div>
                                    <div class="flex gap-2">
                                        <button id="submit-app-btn" onclick="registerApp()" class="px-4 py-2 bg-accent hover:bg-accent-hover text-white text-sm font-medium rounded-lg transition-colors">Register App</button>
                                        <button onclick="document.getElementById('register-app-form').classList.add('hidden')" class="px-4 py-2 bg-zinc-800 hover:bg-zinc-700 text-zinc-300 text-sm font-medium rounded-lg transition-colors">Cancel</button>
                                    </div>
                                    <div id="app-register-error" class="hidden text-xs text-red-400"></div>
                                </div>
                            </div>
                        `}
                    </div>

                    <!-- Scopes Reference -->
                    <div class="mb-10">
                        <h2 class="text-xl font-semibold mb-4">Permission Scopes</h2>
                        <p class="text-sm text-zinc-500 mb-3">When creating an app token, specify exactly which scopes it needs. Users must grant each scope before your app can use it.</p>
                        <div class="bg-white/[0.02] border border-zinc-800/50 rounded-xl overflow-hidden">
                            <table class="w-full text-sm">
                                <thead><tr class="border-b border-zinc-800/50">
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Scope</th>
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Description</th>
                                </tr></thead>
                                <tbody>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">profile:read</td><td class="px-4 py-2.5 text-zinc-400">Read username and avatar</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">friends:read</td><td class="px-4 py-2.5 text-zinc-400">Read player's friend list</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">friends:write</td><td class="px-4 py-2.5 text-zinc-400">Send and accept friend requests</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">achievements:read</td><td class="px-4 py-2.5 text-zinc-400">Read player achievements</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">achievements:write</td><td class="px-4 py-2.5 text-zinc-400">Unlock achievements for players</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">stats:read</td><td class="px-4 py-2.5 text-zinc-400">Read player stats (play time, kills, etc.)</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">stats:write</td><td class="px-4 py-2.5 text-zinc-400">Update player stats</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">leaderboards:read</td><td class="px-4 py-2.5 text-zinc-400">Read leaderboard scores</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">leaderboards:write</td><td class="px-4 py-2.5 text-zinc-400">Submit leaderboard scores</td></tr>
                                    <tr><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">inventory:read</td><td class="px-4 py-2.5 text-zinc-400">Read purchased assets and games</td></tr>
                                </tbody>
                            </table>
                        </div>
                    </div>

                    <!-- API Reference Link -->
                    <div class="mb-10">
                        <div class="flex items-center justify-between mb-4">
                            <h2 class="text-xl font-semibold">API Reference</h2>
                        </div>
                        <p class="text-sm text-zinc-400 mb-4">Full API documentation with examples, authentication guides, and SDK references.</p>
                        <a href="/docs/developer/api-authentication" class="inline-flex items-center gap-2 px-6 py-3 bg-accent hover:bg-accent-hover text-white text-sm font-medium rounded-xl transition-colors">
                            <i class="ph ph-book-open"></i> View API Documentation
                        </a>
                    </div>

                    <!-- API Tokens Section -->
                    <div class="mb-10">
                        <div class="flex items-center justify-between mb-4">
                            <h2 class="text-xl font-semibold">API Tokens</h2>
                            ${token ? '<button onclick="createToken()" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all">Create Token</button>' : ''}
                        </div>

                        ${!token ? '<p class="text-sm text-zinc-500"><a href="/login" class="text-accent hover:text-accent-hover">Sign in</a> to manage API tokens.</p>' : `
                            <div id="token-list" class="space-y-2">
                                ${tokens.length === 0 ? '<p class="text-sm text-zinc-500">No API tokens yet. Create one to get started.</p>' :
                                    tokens.map(t => tokenRow(t)).join('')}
                            </div>
                            <div id="new-token-banner" class="hidden mt-4 p-4 bg-green-950/30 border border-green-800/50 rounded-xl">
                                <div class="flex items-center gap-2 mb-2">
                                    <i class="ph ph-check-circle text-green-400"></i>
                                    <span class="text-sm font-medium text-green-400">Token created</span>
                                </div>
                                <p class="text-xs text-zinc-400 mb-2">Copy this token now. It will not be shown again.</p>
                                <div class="flex items-center gap-2">
                                    <code id="new-token-value" class="flex-1 px-3 py-2 bg-black/40 rounded-lg text-xs text-zinc-300 font-mono select-all overflow-x-auto"></code>
                                    <button onclick="copyToken()" class="px-3 py-2 rounded-lg text-xs bg-white/5 hover:bg-white/10 transition-colors">Copy</button>
                                </div>
                            </div>
                        `}
                    </div>

                `;
            })();

            // ── App management ──
            async function loadApps() {
                if (!token) return;
                try {
                    const res = await fetch('/api/gameservices/apps', { headers: { 'Authorization': 'Bearer ' + token } });
                    if (!res.ok) return;
                    const apps = await res.json();
                    const el = document.getElementById('app-list');
                    if (!el) return;
                    if (apps.length === 0) { el.innerHTML = '<p class="text-sm text-zinc-500">No apps registered. Create one to get started with Game Services.</p>'; return; }
                    el.innerHTML = apps.map(a => `
                        <div class="p-4 bg-white/[0.02] border border-zinc-800/50 rounded-xl">
                            <div class="flex items-center justify-between mb-2">
                                <div class="flex items-center gap-3">
                                    <div class="w-10 h-10 rounded-lg bg-accent/10 flex items-center justify-center"><i class="ph ph-game-controller text-accent"></i></div>
                                    <div>
                                        <div class="font-medium">${a.name}</div>
                                        <div class="text-xs text-zinc-500">Client ID: <code class="text-zinc-400">${a.client_id}</code></div>
                                    </div>
                                </div>
                                <div class="flex items-center gap-2">
                                    <button onclick="manageAppTokens('${a.id}', '${a.name}')" class="px-3 py-1.5 rounded-lg text-xs text-accent hover:bg-accent/10 border border-transparent hover:border-accent/20 transition-all">Tokens</button>
                                    <button onclick="deleteApp('${a.id}', this)" class="px-3 py-1.5 rounded-lg text-xs text-red-400 hover:bg-red-950/30 border border-transparent hover:border-red-900/50 transition-all">Delete</button>
                                </div>
                            </div>
                            ${a.description ? '<p class="text-xs text-zinc-500 ml-13">' + a.description + '</p>' : ''}
                        </div>
                    `).join('');
                } catch(e) {}
            }
            loadApps();

            async function registerApp() {
                var name = document.getElementById('app-name').value.trim();
                var desc = document.getElementById('app-desc').value.trim();
                var url = document.getElementById('app-url').value.trim();
                var redirect = document.getElementById('app-redirect').value.trim();
                var errorEl = document.getElementById('app-register-error');
                errorEl.classList.add('hidden');

                if (!name) { errorEl.textContent = 'App name is required'; errorEl.classList.remove('hidden'); return; }

                var res = await fetch('/api/gameservices/apps', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ name: name, description: desc, website_url: url, redirect_uri: redirect })
                });
                var data = await res.json();
                if (data.client_id) {
                    document.getElementById('register-app-form').classList.add('hidden');
                    alert('App registered!\\n\\nClient ID: ' + data.client_id + '\\nClient Secret: ' + data.client_secret + '\\n\\nSave the secret — it will not be shown again.');
                    window.location.reload();
                } else {
                    errorEl.textContent = data.error || data.message || 'Failed to register app';
                    errorEl.classList.remove('hidden');
                }
            }

            async function deleteApp(id, btn) {
                if (!confirm('Delete this app? All tokens, achievements, and leaderboards will be removed.')) return;
                const res = await fetch('/api/gameservices/apps/' + id, { method: 'DELETE', headers: { 'Authorization': 'Bearer ' + token } });
                if (res.ok) { loadApps(); } else { alert('Failed to delete app'); }
            }

            async function manageAppTokens(appId, appName) {
                try {
                    const res = await fetch('/api/gameservices/apps/' + appId + '/tokens', { headers: { 'Authorization': 'Bearer ' + token } });
                    const tokens = await res.json();
                    const scopeList = ['profile:read','friends:read','friends:write','achievements:read','achievements:write','stats:read','stats:write','leaderboards:read','leaderboards:write','inventory:read'];
                    let msg = 'Tokens for ' + appName + ':\\n\\n';
                    if (tokens.length === 0) msg += '(no tokens)\\n';
                    else tokens.forEach(t => { msg += t.name + ' (' + t.prefix + '...) — ' + t.scopes.join(', ') + '\\n'; });
                    msg += '\\nCreate a new token? Enter a name (or Cancel):';
                    const tokenName = prompt(msg);
                    if (!tokenName) return;
                    const scopeInput = prompt('Enter scopes (comma separated):\\n' + scopeList.join(', '));
                    if (!scopeInput) return;
                    const scopes = scopeInput.split(',').map(s => s.trim()).filter(Boolean);
                    const createRes = await fetch('/api/gameservices/apps/' + appId + '/tokens', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({ name: tokenName, scopes })
                    });
                    const data = await createRes.json();
                    if (!createRes.ok) { alert(data.error || 'Failed to create token'); return; }
                    alert('Token created!\\n\\n' + data.token + '\\n\\nScopes: ' + data.scopes.join(', ') + '\\n\\nSave this — it will not be shown again.');
                } catch(e) { alert('Error: ' + e.message); }
            }

            function tokenRow(t) {
                return '<div class="flex items-center justify-between p-3 bg-white/[0.02] border border-zinc-800/50 rounded-lg">' +
                    '<div class="flex items-center gap-3">' +
                        '<div class="w-8 h-8 rounded-lg bg-accent/10 flex items-center justify-center"><i class="ph ph-key text-accent text-sm"></i></div>' +
                        '<div>' +
                            '<div class="text-sm font-medium">' + t.name + '</div>' +
                            '<div class="text-xs text-zinc-600">' + t.prefix + '... · Created ' + new Date(t.created_at).toLocaleDateString() +
                            (t.last_used_at ? ' · Last used ' + new Date(t.last_used_at).toLocaleDateString() : ' · Never used') +
                            (t.expires_at ? ' · Expires ' + new Date(t.expires_at).toLocaleDateString() : '') + '</div>' +
                        '</div>' +
                    '</div>' +
                    '<button onclick="revokeToken(\'' + t.id + '\', this)" class="px-3 py-1.5 rounded-lg text-xs text-red-400 hover:bg-red-950/30 hover:text-red-300 border border-transparent hover:border-red-900/50 transition-all">Revoke</button>' +
                '</div>';
            }

            async function createToken() {
                var name = prompt('Token name (e.g. "upload-bot"):');
                if (!name) return;
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                try {
                    var res = await fetch('/api/api-tokens', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({ name: name })
                    });
                    var data = await res.json();
                    if (!res.ok) { alert(data.message || 'Failed to create token'); return; }
                    document.getElementById('new-token-value').textContent = data.token;
                    document.getElementById('new-token-banner').classList.remove('hidden');
                    // Add new token row to the list without reloading the page
                    var list = document.getElementById('token-list');
                    var empty = list.querySelector('p');
                    if (empty) empty.remove();
                    list.insertAdjacentHTML('afterbegin', tokenRow({
                        id: data.id,
                        name: data.name,
                        prefix: data.prefix,
                        created_at: data.created_at,
                        last_used_at: null,
                        expires_at: data.expires_at || null
                    }));
                } catch(e) { alert('Error: ' + e.message); }
            }

            async function revokeToken(id, btn) {
                if (!confirm('Revoke this API token? Any integrations using it will stop working.')) return;
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                var res = await fetch('/api/api-tokens/' + id, {
                    method: 'DELETE',
                    headers: { 'Authorization': 'Bearer ' + token }
                });
                if (res.ok) { btn.closest('[class*="flex items-center justify-between"]').remove(); }
                else { alert('Failed to revoke token'); }
            }

            function copyToken() {
                var val = document.getElementById('new-token-value').textContent;
                navigator.clipboard.writeText(val);
            }
            "##
        </script>
    }
}
