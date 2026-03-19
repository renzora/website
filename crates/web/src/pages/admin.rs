use leptos::prelude::*;

#[component]
pub fn AdminPage() -> impl IntoView {
    view! {
        <section class="py-10 px-6">
            <div class="max-w-[1200px] mx-auto">
                <div class="flex items-center gap-3 mb-8">
                    <i class="ph ph-shield-check text-2xl text-accent"></i>
                    <h1 class="text-2xl font-bold">"Admin Panel"</h1>
                </div>

                // Stats
                <div id="admin-stats" class="grid grid-cols-2 md:grid-cols-5 gap-3 mb-8">
                    <div class="p-4 bg-surface-card border border-zinc-800 rounded-lg">
                        <span class="text-xs text-zinc-500 uppercase">"Users"</span>
                        <div id="stat-users" class="text-xl font-bold mt-1">"—"</div>
                    </div>
                    <div class="p-4 bg-surface-card border border-zinc-800 rounded-lg">
                        <span class="text-xs text-zinc-500 uppercase">"Assets"</span>
                        <div id="stat-assets" class="text-xl font-bold mt-1">"—"</div>
                    </div>
                    <div class="p-4 bg-surface-card border border-zinc-800 rounded-lg">
                        <span class="text-xs text-zinc-500 uppercase">"Transactions"</span>
                        <div id="stat-txns" class="text-xl font-bold mt-1">"—"</div>
                    </div>
                    <div class="p-4 bg-surface-card border border-zinc-800 rounded-lg">
                        <span class="text-xs text-zinc-500 uppercase">"Credits"</span>
                        <div id="stat-credits" class="text-xl font-bold mt-1">"—"</div>
                    </div>
                    <div class="p-4 bg-surface-card border border-zinc-800 rounded-lg">
                        <span class="text-xs text-zinc-500 uppercase">"Open Disputes"</span>
                        <div id="stat-disputes" class="text-xl font-bold mt-1">"—"</div>
                    </div>
                </div>

                // Tabs
                <div class="flex gap-1 mb-6 border-b border-zinc-800 pb-px">
                    <button onclick="showTab('users')" class="admin-tab active px-4 py-2 text-sm rounded-t-lg transition-all" id="tab-users">"Users"</button>
                    <button onclick="showTab('assets')" class="admin-tab px-4 py-2 text-sm rounded-t-lg transition-all" id="tab-assets">"Assets"</button>
                    <button onclick="showTab('categories')" class="admin-tab px-4 py-2 text-sm rounded-t-lg transition-all" id="tab-categories">"Categories"</button>
                    <button onclick="showTab('disputes')" class="admin-tab px-4 py-2 text-sm rounded-t-lg transition-all" id="tab-disputes">"Disputes"</button>
                    <button onclick="showTab('forum')" class="admin-tab px-4 py-2 text-sm rounded-t-lg transition-all" id="tab-forum">"Forum"</button>
                    <button onclick="showTab('badges')" class="admin-tab px-4 py-2 text-sm rounded-t-lg transition-all" id="tab-badges">"Badges"</button>
                    <button onclick="showTab('settings')" class="admin-tab px-4 py-2 text-sm rounded-t-lg transition-all" id="tab-settings">"Settings"</button>
                    <button onclick="showTab('docs')" class="admin-tab px-4 py-2 text-sm rounded-t-lg transition-all" id="tab-docs">"Docs"</button>
                </div>

                // Tab content
                <div id="panel-users" class="admin-panel">
                    <div class="flex justify-between items-center mb-4">
                        <h2 class="text-lg font-semibold">"Users"</h2>
                        <input type="text" id="user-search" placeholder="Search users..." oninput="loadUsers()" class="px-3 py-2 bg-surface-card border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent w-64" />
                    </div>
                    <div id="users-list" class="space-y-2"></div>
                </div>

                <div id="panel-assets" class="admin-panel hidden">
                    <div class="flex justify-between items-center mb-4">
                        <h2 class="text-lg font-semibold">"Assets"</h2>
                        <input type="text" id="asset-search" placeholder="Search assets..." oninput="loadAssets()" class="px-3 py-2 bg-surface-card border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent w-64" />
                    </div>
                    <div id="assets-list" class="space-y-2"></div>
                </div>

                <div id="panel-categories" class="admin-panel hidden">
                    <h2 class="text-lg font-semibold mb-4">"Categories"</h2>
                    <div id="categories-list" class="space-y-2"></div>
                </div>

                <div id="panel-disputes" class="admin-panel hidden">
                    <h2 class="text-lg font-semibold mb-4">"Disputes"</h2>
                    <div id="disputes-list" class="space-y-2"></div>
                </div>

                <div id="panel-settings" class="admin-panel hidden">
                    <h2 class="text-lg font-semibold mb-4">"Site Settings"</h2>
                    <div id="settings-list" class="space-y-3"></div>
                </div>

                <div id="panel-docs" class="admin-panel hidden">
                    <div class="flex justify-between items-center mb-4">
                        <h2 class="text-lg font-semibold">"Documentation Pages"</h2>
                        <button onclick="showCreateDoc()" class="inline-flex items-center gap-1.5 px-4 py-2 rounded-lg text-xs font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                            <i class="ph ph-plus"></i>"New Page"
                        </button>
                    </div>
                    <div id="docs-list" class="space-y-2"></div>
                </div>

                <div id="panel-forum" class="admin-panel hidden">
                    <div class="flex justify-between items-center mb-4">
                        <h2 class="text-lg font-semibold">"Forum Categories"</h2>
                        <button onclick="createForumCat()" class="inline-flex items-center gap-1.5 px-4 py-2 rounded-lg text-xs font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                            <i class="ph ph-plus"></i>"New Category"
                        </button>
                    </div>
                    <div id="forum-cats-list" class="space-y-2"></div>
                </div>

                <div id="panel-badges" class="admin-panel hidden">
                    <h2 class="text-lg font-semibold mb-4">"Badges"</h2>
                    <div id="badges-list" class="space-y-2"></div>
                </div>
            </div>
        </section>

        <script>
            r#"
            function getToken() {
                return document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
            }
            function api(path, opts = {}) {
                const token = getToken();
                if (!token) { window.location.href = '/login'; return; }
                return fetch('/api/admin' + path, {
                    ...opts,
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json', ...opts.headers }
                }).then(r => { if (r.status === 401) window.location.href = '/login'; return r.json(); });
            }

            function showTab(name) {
                document.querySelectorAll('.admin-panel').forEach(p => p.classList.add('hidden'));
                document.querySelectorAll('.admin-tab').forEach(t => { t.classList.remove('bg-surface-card', 'text-zinc-50'); t.classList.add('text-zinc-400'); });
                document.getElementById('panel-' + name).classList.remove('hidden');
                const tab = document.getElementById('tab-' + name);
                tab.classList.add('bg-surface-card', 'text-zinc-50');
                tab.classList.remove('text-zinc-400');
                if (name === 'users') loadUsers();
                if (name === 'assets') loadAssets();
                if (name === 'categories') loadCategories();
                if (name === 'disputes') loadDisputes();
                if (name === 'settings') loadSettings();
                if (name === 'docs') loadDocs();
                if (name === 'forum') loadForumCats();
                if (name === 'badges') loadBadges();
            }

            async function loadStats() {
                const data = await api('/stats');
                if (data) {
                    document.getElementById('stat-users').textContent = data.total_users;
                    document.getElementById('stat-assets').textContent = data.total_assets;
                    document.getElementById('stat-txns').textContent = data.total_transactions;
                    document.getElementById('stat-credits').textContent = data.total_credits_circulating;
                    document.getElementById('stat-disputes').textContent = data.open_disputes;
                }
            }

            async function loadUsers() {
                const q = document.getElementById('user-search')?.value || '';
                const data = await api('/users?q=' + encodeURIComponent(q));
                const el = document.getElementById('users-list');
                if (!data || !Array.isArray(data)) { el.innerHTML = '<p class="text-zinc-500 text-sm">No users found</p>'; return; }
                el.innerHTML = data.map(u => `
                    <div class="flex items-center justify-between p-3 bg-surface-card border border-zinc-800 rounded-lg">
                        <div>
                            <span class="text-sm font-medium">${u.username}</span>
                            <span class="text-xs text-zinc-500 ml-2">${u.email}</span>
                        </div>
                        <div class="flex items-center gap-3">
                            <span class="text-xs px-2 py-0.5 rounded bg-zinc-800 text-zinc-300">${u.credit_balance} credits</span>
                            <select onchange="setRole('${u.id}', this.value)" class="text-xs bg-surface border border-zinc-800 rounded px-2 py-1 text-zinc-300">
                                <option value="user" ${u.role==='user'?'selected':''}>User</option>
                                <option value="creator" ${u.role==='creator'?'selected':''}>Creator</option>
                                <option value="admin" ${u.role==='admin'?'selected':''}>Admin</option>
                            </select>
                        </div>
                    </div>
                `).join('');
            }

            async function setRole(id, role) {
                await api('/users/' + id + '/role', { method: 'PUT', body: JSON.stringify({ role }) });
            }

            async function loadAssets() {
                const q = document.getElementById('asset-search')?.value || '';
                const data = await api('/assets?q=' + encodeURIComponent(q));
                const el = document.getElementById('assets-list');
                if (!data?.assets) { el.innerHTML = '<p class="text-zinc-500 text-sm">No assets</p>'; return; }
                el.innerHTML = data.assets.map(a => `
                    <div class="flex items-center justify-between p-3 bg-surface-card border border-zinc-800 rounded-lg">
                        <div>
                            <span class="text-sm font-medium">${a.name}</span>
                            <span class="text-xs text-zinc-500 ml-2">by ${a.creator_name} · ${a.category} · ${a.downloads} downloads</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="text-xs ${a.published ? 'text-green-400' : 'text-zinc-500'}">${a.published ? 'Published' : 'Draft'}</span>
                            <button onclick="togglePublish('${a.id}')" class="text-xs px-2 py-1 rounded bg-zinc-800 text-zinc-300 hover:bg-zinc-700">${a.published ? 'Unpublish' : 'Publish'}</button>
                            <button onclick="if(confirm('Delete this asset?')) deleteAsset('${a.id}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400 hover:bg-red-500/20">Delete</button>
                        </div>
                    </div>
                `).join('');
            }

            async function togglePublish(id) { await api('/assets/' + id + '/publish', { method: 'PUT' }); loadAssets(); }
            async function deleteAsset(id) { await api('/assets/' + id, { method: 'DELETE' }); loadAssets(); loadStats(); }

            async function loadCategories() {
                const data = await api('/categories');
                const el = document.getElementById('categories-list');
                if (!data || !Array.isArray(data)) { el.innerHTML = '<p class="text-zinc-500 text-sm">No categories</p>'; return; }
                el.innerHTML = data.map(c => `
                    <div class="flex items-center justify-between p-3 bg-surface-card border border-zinc-800 rounded-lg">
                        <div class="flex items-center gap-2">
                            <i class="ph ${c.icon} text-lg text-accent"></i>
                            <span class="text-sm font-medium">${c.name}</span>
                            <span class="text-xs text-zinc-500">${c.description}</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="text-xs text-zinc-500">${c.max_file_size_mb}MB max</span>
                            <button onclick="if(confirm('Delete?')) deleteCategory('${c.id}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400 hover:bg-red-500/20">Delete</button>
                        </div>
                    </div>
                `).join('');
            }

            async function deleteCategory(id) { await api('/categories/' + id, { method: 'DELETE' }); loadCategories(); }

            async function loadDisputes() {
                const data = await api('/disputes');
                const el = document.getElementById('disputes-list');
                if (!data?.disputes?.length) { el.innerHTML = '<p class="text-zinc-500 text-sm py-4">No disputes</p>'; return; }
                el.innerHTML = data.disputes.map(d => `
                    <div class="p-3 bg-surface-card border border-zinc-800 rounded-lg">
                        <div class="flex justify-between items-start">
                            <div>
                                <span class="text-xs px-2 py-0.5 rounded ${d.status==='open'?'bg-yellow-500/10 text-yellow-400':'bg-green-500/10 text-green-400'}">${d.status}</span>
                                <p class="text-sm mt-2">${d.reason}</p>
                            </div>
                            ${d.status === 'open' ? `<button onclick="resolveDispute('${d.id}')" class="text-xs px-3 py-1 rounded bg-accent text-white hover:bg-accent-hover">Resolve</button>` : ''}
                        </div>
                    </div>
                `).join('');
            }

            async function resolveDispute(id) {
                const notes = prompt('Admin notes:');
                if (notes === null) return;
                await api('/disputes/' + id + '/resolve', { method: 'PUT', body: JSON.stringify({ status: 'resolved', admin_notes: notes }) });
                loadDisputes(); loadStats();
            }

            async function loadSettings() {
                const data = await api('/settings');
                const el = document.getElementById('settings-list');
                if (!data) return;
                el.innerHTML = Object.entries(data).map(([k, v]) => `
                    <div class="flex items-center gap-3">
                        <label class="text-sm text-zinc-400 w-48">${k}</label>
                        <input type="text" value="${v}" onchange="updateSetting('${k}', this.value)" class="flex-1 px-3 py-2 bg-surface-card border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent" />
                    </div>
                `).join('');
            }

            async function updateSetting(key, value) {
                await api('/settings', { method: 'PUT', body: JSON.stringify({ key, value }) });
            }

            async function loadDocs() {
                const data = await api('/docs');
                const el = document.getElementById('docs-list');
                if (!data?.docs?.length) { el.innerHTML = '<p class="text-zinc-500 text-sm">No docs</p>'; return; }
                el.innerHTML = data.docs.map(d => `
                    <div class="flex items-center justify-between p-3 bg-surface-card border border-zinc-800 rounded-lg">
                        <div>
                            <span class="text-sm font-medium">${d.title}</span>
                            <span class="text-xs text-zinc-500 ml-2">/${d.slug} · ${d.category}</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="text-xs ${d.published ? 'text-green-400' : 'text-zinc-500'}">${d.published ? 'Published' : 'Draft'}</span>
                            <button onclick="if(confirm('Delete?')) deleteDoc('${d.id}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400 hover:bg-red-500/20">Delete</button>
                        </div>
                    </div>
                `).join('');
            }

            async function deleteDoc(id) { await api('/docs/' + id, { method: 'DELETE' }); loadDocs(); }

            // Forum categories
            async function loadForumCats() {
                const data = await api('/forum-categories');
                const el = document.getElementById('forum-cats-list');
                if (!data || !Array.isArray(data)) { el.innerHTML = '<p class="text-zinc-500 text-sm">No categories</p>'; return; }
                el.innerHTML = data.map(c => `
                    <div class="flex items-center justify-between p-3 bg-surface-card border border-zinc-800 rounded-lg">
                        <div class="flex items-center gap-2">
                            <i class="ph ${c.icon} text-lg text-accent"></i>
                            <span class="text-sm font-medium">${c.name}</span>
                            <span class="text-xs text-zinc-500">${c.description}</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="text-xs text-zinc-500">${c.thread_count} threads · ${c.post_count} posts</span>
                            <button onclick="if(confirm('Delete?')) deleteForumCat('${c.id}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400 hover:bg-red-500/20">Delete</button>
                        </div>
                    </div>
                `).join('');
            }
            async function createForumCat() {
                const name = prompt('Category name:'); if (!name) return;
                const slug = name.toLowerCase().replace(/[^a-z0-9]+/g, '-');
                const desc = prompt('Description:') || '';
                const icon = prompt('Icon (e.g. ph-chat-circle):') || 'ph-chat-circle';
                await api('/forum-categories', { method: 'POST', body: JSON.stringify({ name, slug, description: desc, icon }) });
                loadForumCats();
            }
            async function deleteForumCat(id) { await api('/forum-categories/' + id, { method: 'DELETE' }); loadForumCats(); }

            // Badges
            async function loadBadges() {
                const data = await api('/badges');
                const el = document.getElementById('badges-list');
                if (!data?.badges) { el.innerHTML = '<p class="text-zinc-500 text-sm">No badges</p>'; return; }
                el.innerHTML = data.badges.map(b => `
                    <div class="flex items-center justify-between p-3 bg-surface-card border border-zinc-800 rounded-lg">
                        <div class="flex items-center gap-2">
                            <i class="ph ${b.icon} text-lg" style="color:${b.color}"></i>
                            <span class="text-sm font-medium">${b.name}</span>
                            <span class="text-xs text-zinc-500">${b.description}</span>
                        </div>
                        <span class="text-xs text-zinc-500">${b.slug}</span>
                    </div>
                `).join('');
            }

            // Init
            if (name === 'forum') loadForumCats();
            if (name === 'badges') loadBadges();
            loadStats();
            showTab('users');
            "#
        </script>
    }
}
