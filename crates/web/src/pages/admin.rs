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
                        <span class="text-xs text-zinc-500 uppercase">"Disputes"</span>
                        <div id="stat-disputes" class="text-xl font-bold mt-1">"—"</div>
                    </div>
                </div>

                <div class="flex gap-1 mb-6 border-b border-zinc-800 pb-px overflow-x-auto">
                    <button onclick="showTab('users')" class="admin-tab px-4 py-2 text-sm rounded-t-lg bg-surface-card text-zinc-50" id="tab-users">"Users"</button>
                    <button onclick="showTab('assets')" class="admin-tab px-4 py-2 text-sm rounded-t-lg text-zinc-400" id="tab-assets">"Assets"</button>
                    <button onclick="showTab('categories')" class="admin-tab px-4 py-2 text-sm rounded-t-lg text-zinc-400" id="tab-categories">"Categories"</button>
                    <button onclick="showTab('disputes')" class="admin-tab px-4 py-2 text-sm rounded-t-lg text-zinc-400" id="tab-disputes">"Disputes"</button>
                    <button onclick="showTab('roles')" class="admin-tab px-4 py-2 text-sm rounded-t-lg text-zinc-400" id="tab-roles">"Roles"</button>
                    <button onclick="showTab('forum')" class="admin-tab px-4 py-2 text-sm rounded-t-lg text-zinc-400" id="tab-forum">"Forum"</button>
                    <button onclick="showTab('badges')" class="admin-tab px-4 py-2 text-sm rounded-t-lg text-zinc-400" id="tab-badges">"Badges"</button>
                    <button onclick="showTab('settings')" class="admin-tab px-4 py-2 text-sm rounded-t-lg text-zinc-400" id="tab-settings">"Settings"</button>
                    <button onclick="showTab('docs')" class="admin-tab px-4 py-2 text-sm rounded-t-lg text-zinc-400" id="tab-docs">"Docs"</button>
                </div>

                <div id="admin-content"></div>

                // Modal overlay
                <div id="admin-modal" class="hidden fixed inset-0 bg-black/60 z-50 flex items-center justify-center p-4">
                    <div class="bg-surface-card border border-zinc-800 rounded-xl w-full max-w-lg max-h-[80vh] overflow-y-auto p-6">
                        <div id="modal-content"></div>
                    </div>
                </div>
            </div>
        </section>

        <script>
            r##"
            function getToken() { return document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop(); }
            async function api(path, opts = {}) {
                const token = getToken();
                if (!token) { window.location.href = '/login'; return; }
                const res = await fetch('/api/admin' + path, {
                    ...opts,
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json', ...opts.headers }
                });
                if (res.status === 401) { window.location.href = '/login'; return; }
                return res.json();
            }
            function showModal(html) { document.getElementById('modal-content').innerHTML = html; document.getElementById('admin-modal').classList.remove('hidden'); }
            function hideModal() { document.getElementById('admin-modal').classList.add('hidden'); }
            document.getElementById('admin-modal')?.addEventListener('click', e => { if (e.target.id === 'admin-modal') hideModal(); });

            let currentTab = 'users';
            function showTab(name) {
                currentTab = name;
                document.querySelectorAll('.admin-tab').forEach(t => { t.classList.remove('bg-surface-card', 'text-zinc-50'); t.classList.add('text-zinc-400'); });
                document.getElementById('tab-' + name)?.classList.add('bg-surface-card', 'text-zinc-50');
                document.getElementById('tab-' + name)?.classList.remove('text-zinc-400');
                const loaders = { users: loadUsers, assets: loadAssets, categories: loadCategories, disputes: loadDisputes, roles: loadRoles, forum: loadForumCats, badges: loadBadges, settings: loadSettings, docs: loadDocs };
                if (loaders[name]) loaders[name]();
            }

            async function loadStats() {
                const d = await api('/stats');
                if (!d) return;
                document.getElementById('stat-users').textContent = d.total_users;
                document.getElementById('stat-assets').textContent = d.total_assets;
                document.getElementById('stat-txns').textContent = d.total_transactions;
                document.getElementById('stat-credits').textContent = d.total_credits_circulating;
                document.getElementById('stat-disputes').textContent = d.open_disputes;
            }

            // ── Users ──
            async function loadUsers() {
                const el = document.getElementById('admin-content');
                el.innerHTML = '<div class="flex justify-between items-center mb-4"><h2 class="text-lg font-semibold">Users</h2><input type="text" id="user-q" placeholder="Search..." oninput="loadUsers()" class="px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent w-64" /></div><div id="user-list" class="space-y-2">Loading...</div>';
                const q = document.getElementById('user-q')?.value || '';
                const data = await api('/users?q=' + encodeURIComponent(q));
                if (!data) return;
                document.getElementById('user-list').innerHTML = data.map(u => `
                    <div class="flex items-center justify-between p-3 bg-surface border border-zinc-800 rounded-lg">
                        <div class="flex items-center gap-3">
                            <a href="/profile/${u.username}" class="text-sm font-medium text-accent hover:text-accent-hover">${u.username}</a>
                            <span class="text-xs text-zinc-500">${u.email}</span>
                            <span class="text-xs px-2 py-0.5 rounded bg-zinc-800">${u.role}</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="text-xs text-zinc-500">${u.credit_balance} credits</span>
                            <button onclick="editUser('${u.id}','${u.username}','${u.email}','${u.role}')" class="text-xs px-2 py-1 rounded bg-zinc-800 text-zinc-300 hover:bg-zinc-700"><i class="ph ph-pencil-simple"></i></button>
                            <button onclick="showUserNotes('${u.id}','${u.username}')" class="text-xs px-2 py-1 rounded bg-zinc-800 text-zinc-300 hover:bg-zinc-700" title="Mod notes"><i class="ph ph-note"></i></button>
                            <button onclick="showBanModal('${u.id}','${u.username}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400 hover:bg-red-500/20"><i class="ph ph-gavel"></i></button>
                        </div>
                    </div>
                `).join('') || '<p class="text-zinc-500 text-sm">No users</p>';
            }

            function editUser(id, username, email, role) {
                showModal(`
                    <h3 class="text-base font-semibold mb-4">Edit User: ${username}</h3>
                    <div class="space-y-3">
                        <div><label class="block text-xs text-zinc-500 mb-1">Role</label>
                        <select id="eu-role" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50">
                            ${['user','creator','moderator','editor','admin'].map(r => `<option value="${r}" ${r===role?'selected':''}>${r}</option>`).join('')}
                        </select></div>
                        <button onclick="saveUserRole('${id}')" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover">Save</button>
                    </div>
                `);
            }
            async function saveUserRole(id) { await api('/users/'+id+'/role', { method: 'PUT', body: JSON.stringify({ role: document.getElementById('eu-role').value }) }); hideModal(); loadUsers(); loadStats(); }

            function showBanModal(id, username) {
                showModal(`
                    <h3 class="text-base font-semibold mb-4">Ban: ${username}</h3>
                    <div class="space-y-3">
                        <div><label class="block text-xs text-zinc-500 mb-1">Reason</label>
                        <textarea id="ban-reason" rows="2" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50"></textarea></div>
                        <div><label class="block text-xs text-zinc-500 mb-1">Duration</label>
                        <select id="ban-dur" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50">
                            <option value="">Permanent</option>
                            <option value="1">1 hour</option>
                            <option value="24">24 hours</option>
                            <option value="168">7 days</option>
                            <option value="720">30 days</option>
                        </select></div>
                        <div class="flex gap-2">
                            <button onclick="doBan('${id}')" class="px-4 py-2 rounded-lg text-sm font-medium bg-red-500 text-white hover:bg-red-600">Ban</button>
                            <button onclick="doUnban('${id}')" class="px-4 py-2 rounded-lg text-sm text-zinc-400 hover:text-zinc-50">Unban</button>
                        </div>
                    </div>
                `);
            }
            async function doBan(id) { const dur = document.getElementById('ban-dur').value; await api('/users/'+id+'/ban', { method: 'POST', body: JSON.stringify({ reason: document.getElementById('ban-reason').value, duration_hours: dur ? parseInt(dur) : null }) }); hideModal(); loadUsers(); }
            async function doUnban(id) { await api('/users/'+id+'/unban', { method: 'POST' }); hideModal(); loadUsers(); }

            function showUserNotes(id, username) {
                showModal(`<h3 class="text-base font-semibold mb-4">Mod Notes: ${username}</h3><div id="notes-list" class="space-y-2 mb-4">Loading...</div>
                    <textarea id="new-note" rows="2" placeholder="Add a note..." class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 mb-2"></textarea>
                    <button onclick="addNote('${id}')" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover">Add Note</button>`);
                loadNotes(id);
            }
            async function loadNotes(id) {
                const data = await api('/users/'+id+'/notes');
                document.getElementById('notes-list').innerHTML = (data||[]).map(n => `
                    <div class="p-3 bg-surface-card border border-zinc-800 rounded-lg">
                        <div class="flex justify-between"><span class="text-xs text-accent">${n.author_name}</span><span class="text-[11px] text-zinc-500">${new Date(n.created_at).toLocaleString()}</span></div>
                        <p class="text-sm text-zinc-300 mt-1">${n.content}</p>
                        <button onclick="deleteNote('${n.id}','${id}')" class="text-[11px] text-red-400 hover:text-red-300 mt-1">Delete</button>
                    </div>
                `).join('') || '<p class="text-zinc-500 text-sm">No notes</p>';
            }
            async function addNote(uid) { const c = document.getElementById('new-note').value; if(!c) return; await api('/users/'+uid+'/notes', { method: 'POST', body: JSON.stringify({ content: c }) }); document.getElementById('new-note').value = ''; loadNotes(uid); }
            async function deleteNote(nid, uid) { await api('/notes/'+nid, { method: 'DELETE' }); loadNotes(uid); }

            // ── Assets ──
            async function loadAssets() {
                const el = document.getElementById('admin-content');
                el.innerHTML = '<div class="flex justify-between items-center mb-4"><h2 class="text-lg font-semibold">Assets</h2><input type="text" id="asset-q" placeholder="Search..." oninput="loadAssets()" class="px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent w-64" /></div><div id="asset-list" class="space-y-2">Loading...</div>';
                const q = document.getElementById('asset-q')?.value || '';
                const data = await api('/assets?q=' + encodeURIComponent(q));
                if (!data?.assets) return;
                document.getElementById('asset-list').innerHTML = data.assets.map(a => `
                    <div class="flex items-center justify-between p-3 bg-surface border border-zinc-800 rounded-lg">
                        <div><span class="text-sm font-medium">${a.name}</span><span class="text-xs text-zinc-500 ml-2">by ${a.creator_name} · ${a.category} · ${a.downloads} downloads · ${a.price_credits} credits</span></div>
                        <div class="flex items-center gap-2">
                            <span class="text-xs ${a.published ? 'text-green-400' : 'text-zinc-500'}">${a.published ? 'Live' : 'Draft'}</span>
                            <button onclick="togglePub('${a.id}')" class="text-xs px-2 py-1 rounded bg-zinc-800 text-zinc-300 hover:bg-zinc-700">${a.published ? 'Unpublish' : 'Publish'}</button>
                            <button onclick="if(confirm('Delete this asset?')) delAsset('${a.id}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400 hover:bg-red-500/20">Delete</button>
                        </div>
                    </div>
                `).join('') || '<p class="text-zinc-500 text-sm">No assets</p>';
            }
            async function togglePub(id) { await api('/assets/'+id+'/publish', { method: 'PUT' }); loadAssets(); loadStats(); }
            async function delAsset(id) { await api('/assets/'+id, { method: 'DELETE' }); loadAssets(); loadStats(); }

            // ── Categories ──
            async function loadCategories() {
                const el = document.getElementById('admin-content');
                const data = await api('/categories');
                el.innerHTML = '<div class="flex justify-between items-center mb-4"><h2 class="text-lg font-semibold">Marketplace Categories</h2><button onclick="newCategory()" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-accent text-white hover:bg-accent-hover"><i class="ph ph-plus"></i> New</button></div>' +
                    ((data||[]).map(c => `
                    <div class="flex items-center justify-between p-3 bg-surface border border-zinc-800 rounded-lg mb-2">
                        <div class="flex items-center gap-2"><i class="ph ${c.icon} text-lg text-accent"></i><span class="text-sm font-medium">${c.name}</span><span class="text-xs text-zinc-500">${c.description}</span></div>
                        <div class="flex items-center gap-2"><span class="text-xs text-zinc-500">${c.max_file_size_mb}MB max</span>
                        <button onclick="editCategory('${c.id}','${c.name}','${c.description}','${c.icon}',${c.max_file_size_mb})" class="text-xs px-2 py-1 rounded bg-zinc-800 text-zinc-300 hover:bg-zinc-700"><i class="ph ph-pencil-simple"></i></button>
                        <button onclick="if(confirm('Delete?')) delCategory('${c.id}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400 hover:bg-red-500/20">Delete</button></div>
                    </div>`).join('') || '<p class="text-zinc-500 text-sm">No categories</p>');
            }
            function newCategory() { showModal(`<h3 class="text-base font-semibold mb-4">New Category</h3><div class="space-y-3">
                <div><label class="block text-xs text-zinc-500 mb-1">Name</label><input id="nc-name" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Slug</label><input id="nc-slug" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Description</label><input id="nc-desc" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Icon</label><input id="nc-icon" value="ph-folder" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Max file size (MB)</label><input id="nc-size" type="number" value="50" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <button onclick="saveNewCategory()" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover">Create</button></div>`); }
            async function saveNewCategory() { await api('/categories', { method: 'POST', body: JSON.stringify({ name: document.getElementById('nc-name').value, slug: document.getElementById('nc-slug').value, description: document.getElementById('nc-desc').value, icon: document.getElementById('nc-icon').value, max_file_size_mb: parseInt(document.getElementById('nc-size').value) }) }); hideModal(); loadCategories(); }
            function editCategory(id,name,desc,icon,size) { showModal(`<h3 class="text-base font-semibold mb-4">Edit: ${name}</h3><div class="space-y-3">
                <div><label class="block text-xs text-zinc-500 mb-1">Name</label><input id="ec-name" value="${name}" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Description</label><input id="ec-desc" value="${desc}" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Icon</label><input id="ec-icon" value="${icon}" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Max size (MB)</label><input id="ec-size" type="number" value="${size}" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <button onclick="saveEditCategory('${id}')" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover">Save</button></div>`); }
            async function saveEditCategory(id) { await api('/categories/'+id, { method: 'PUT', body: JSON.stringify({ name: document.getElementById('ec-name').value, description: document.getElementById('ec-desc').value, icon: document.getElementById('ec-icon').value, max_file_size_mb: parseInt(document.getElementById('ec-size').value) }) }); hideModal(); loadCategories(); }
            async function delCategory(id) { await api('/categories/'+id, { method: 'DELETE' }); loadCategories(); }

            // ── Disputes ──
            async function loadDisputes() {
                const el = document.getElementById('admin-content');
                const data = await api('/disputes');
                el.innerHTML = '<h2 class="text-lg font-semibold mb-4">Disputes</h2>' +
                    ((data?.disputes||[]).map(d => `
                    <div class="p-3 bg-surface border border-zinc-800 rounded-lg mb-2">
                        <div class="flex justify-between items-start">
                            <div><span class="text-xs px-2 py-0.5 rounded ${d.status==='open'?'bg-yellow-500/10 text-yellow-400':'bg-green-500/10 text-green-400'}">${d.status}</span>
                            <p class="text-sm text-zinc-300 mt-2">${d.reason}</p>
                            ${d.admin_notes ? `<p class="text-xs text-zinc-500 mt-1">Notes: ${d.admin_notes}</p>` : ''}</div>
                            ${d.status === 'open' ? `<button onclick="resolveDispute('${d.id}')" class="text-xs px-3 py-1 rounded bg-accent text-white hover:bg-accent-hover">Resolve</button>` : ''}
                        </div>
                    </div>`).join('') || '<p class="text-zinc-500 text-sm py-4">No disputes</p>');
            }
            async function resolveDispute(id) { showModal(`<h3 class="text-base font-semibold mb-4">Resolve Dispute</h3><div class="space-y-3">
                <div><label class="block text-xs text-zinc-500 mb-1">Status</label><select id="rd-status" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50"><option value="resolved">Resolved</option><option value="rejected">Rejected</option><option value="refunded">Refunded</option></select></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Admin notes</label><textarea id="rd-notes" rows="3" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50"></textarea></div>
                <button onclick="doResolve('${id}')" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover">Submit</button></div>`); }
            async function doResolve(id) { await api('/disputes/'+id+'/resolve', { method: 'PUT', body: JSON.stringify({ status: document.getElementById('rd-status').value, admin_notes: document.getElementById('rd-notes').value }) }); hideModal(); loadDisputes(); loadStats(); }

            // ── Roles ──
            async function loadRoles() {
                const el = document.getElementById('admin-content');
                const data = await api('/roles');
                const perms = ['manage_users','manage_roles','manage_bans','manage_assets','manage_categories','manage_docs','manage_forum','manage_disputes','manage_settings','manage_badges','mod_notes','view_admin'];
                el.innerHTML = '<div class="flex justify-between items-center mb-4"><h2 class="text-lg font-semibold">Roles</h2><button onclick="newRole()" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-accent text-white hover:bg-accent-hover"><i class="ph ph-plus"></i> New Role</button></div>' +
                    ((data||[]).map(r => `
                    <div class="p-4 bg-surface border border-zinc-800 rounded-lg mb-2">
                        <div class="flex justify-between items-center mb-2">
                            <div class="flex items-center gap-2"><span class="w-3 h-3 rounded-full" style="background:${r.color}"></span><span class="text-sm font-semibold">${r.name}</span>${r.is_staff ? '<span class="text-[10px] px-1.5 py-0.5 rounded bg-amber-500/10 text-amber-400">STAFF</span>' : ''}</div>
                            <div class="flex gap-2"><button onclick='editRolePerms("${r.id}","${r.name}",${JSON.stringify(JSON.stringify(r.permissions))})' class="text-xs px-2 py-1 rounded bg-zinc-800 text-zinc-300 hover:bg-zinc-700"><i class="ph ph-pencil-simple"></i> Permissions</button>
                            <button onclick="if(confirm('Delete role?')) delRole('${r.id}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400 hover:bg-red-500/20">Delete</button></div>
                        </div>
                        <div class="flex flex-wrap gap-1">${perms.map(p => `<span class="text-[10px] px-1.5 py-0.5 rounded ${r.permissions[p] ? 'bg-green-500/10 text-green-400' : 'bg-zinc-800 text-zinc-600'}">${p.replace('manage_','')}</span>`).join('')}</div>
                    </div>`).join('') || '<p class="text-zinc-500 text-sm">No roles</p>');
            }
            function newRole() { showModal(`<h3 class="text-base font-semibold mb-4">New Role</h3><div class="space-y-3">
                <div><label class="block text-xs text-zinc-500 mb-1">Name</label><input id="nr-name" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Color</label><input id="nr-color" type="color" value="#6366f1" class="w-10 h-10 rounded cursor-pointer" /></div>
                <div class="flex items-center gap-2"><input id="nr-staff" type="checkbox" class="rounded" /><label class="text-sm text-zinc-300">Staff role</label></div>
                <button onclick="saveNewRole()" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover">Create</button></div>`); }
            async function saveNewRole() { await api('/roles', { method: 'POST', body: JSON.stringify({ name: document.getElementById('nr-name').value, color: document.getElementById('nr-color').value, is_staff: document.getElementById('nr-staff').checked, permissions: {} }) }); hideModal(); loadRoles(); }
            function editRolePerms(id, name, permsJson) {
                const perms = JSON.parse(permsJson);
                const allPerms = ['manage_users','manage_roles','manage_bans','manage_assets','manage_categories','manage_docs','manage_forum','manage_disputes','manage_settings','manage_badges','mod_notes','view_admin'];
                showModal(`<h3 class="text-base font-semibold mb-4">Permissions: ${name}</h3><div class="space-y-2 mb-4">${allPerms.map(p => `
                    <label class="flex items-center gap-2"><input type="checkbox" id="perm-${p}" ${perms[p]?'checked':''} class="rounded" /><span class="text-sm text-zinc-300">${p}</span></label>
                `).join('')}</div><button onclick="savePerms('${id}')" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover">Save</button>`);
            }
            async function savePerms(id) {
                const allPerms = ['manage_users','manage_roles','manage_bans','manage_assets','manage_categories','manage_docs','manage_forum','manage_disputes','manage_settings','manage_badges','mod_notes','view_admin'];
                const perms = {}; allPerms.forEach(p => { perms[p] = document.getElementById('perm-'+p)?.checked || false; });
                await api('/roles/'+id+'/permissions', { method: 'PUT', body: JSON.stringify({ permissions: perms }) }); hideModal(); loadRoles();
            }
            async function delRole(id) { await api('/roles/'+id, { method: 'DELETE' }); loadRoles(); }

            // ── Forum ──
            async function loadForumCats() {
                const el = document.getElementById('admin-content');
                const data = await api('/forum-categories');
                el.innerHTML = '<div class="flex justify-between items-center mb-4"><h2 class="text-lg font-semibold">Forum Categories</h2><button onclick="newForumCat()" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-accent text-white hover:bg-accent-hover"><i class="ph ph-plus"></i> New</button></div>' +
                    ((data||[]).map(c => `
                    <div class="flex items-center justify-between p-3 bg-surface border border-zinc-800 rounded-lg mb-2">
                        <div class="flex items-center gap-2"><i class="ph ${c.icon} text-lg text-accent"></i><span class="text-sm font-medium">${c.name}</span><span class="text-xs text-zinc-500">${c.description} · ${c.thread_count} threads · ${c.post_count} posts</span></div>
                        <button onclick="if(confirm('Delete?')) delForumCat('${c.id}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400 hover:bg-red-500/20">Delete</button>
                    </div>`).join('') || '<p class="text-zinc-500 text-sm">No categories</p>');
            }
            function newForumCat() { showModal(`<h3 class="text-base font-semibold mb-4">New Forum Category</h3><div class="space-y-3">
                <div><label class="block text-xs text-zinc-500 mb-1">Name</label><input id="nfc-name" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Slug</label><input id="nfc-slug" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Description</label><input id="nfc-desc" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Icon</label><input id="nfc-icon" value="ph-chat-circle" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <button onclick="saveNewForumCat()" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover">Create</button></div>`); }
            async function saveNewForumCat() { await api('/forum-categories', { method: 'POST', body: JSON.stringify({ name: document.getElementById('nfc-name').value, slug: document.getElementById('nfc-slug').value, description: document.getElementById('nfc-desc').value, icon: document.getElementById('nfc-icon').value }) }); hideModal(); loadForumCats(); }
            async function delForumCat(id) { await api('/forum-categories/'+id, { method: 'DELETE' }); loadForumCats(); }

            // ── Badges ──
            async function loadBadges() {
                const el = document.getElementById('admin-content');
                const data = await api('/badges');
                el.innerHTML = '<h2 class="text-lg font-semibold mb-4">Badges</h2>' +
                    ((data?.badges||[]).map(b => `
                    <div class="flex items-center justify-between p-3 bg-surface border border-zinc-800 rounded-lg mb-2">
                        <div class="flex items-center gap-2"><i class="ph ${b.icon} text-lg" style="color:${b.color}"></i><span class="text-sm font-medium">${b.name}</span><span class="text-xs text-zinc-500">${b.description}</span></div>
                        <span class="text-xs text-zinc-500 font-mono">${b.slug}</span>
                    </div>`).join('') || '<p class="text-zinc-500 text-sm">No badges</p>');
            }

            // ── Settings ──
            async function loadSettings() {
                const el = document.getElementById('admin-content');
                const data = await api('/settings');
                if (!data) return;
                el.innerHTML = '<h2 class="text-lg font-semibold mb-4">Site Settings</h2><div class="space-y-3">' +
                    Object.entries(data).map(([k, v]) => `
                    <div class="flex items-center gap-3">
                        <label class="text-sm text-zinc-400 w-56 font-mono">${k}</label>
                        <input type="text" value="${v}" onchange="saveSetting('${k}', this.value)" class="flex-1 px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent" />
                    </div>`).join('') + '</div>';
            }
            async function saveSetting(k, v) { await api('/settings', { method: 'PUT', body: JSON.stringify({ key: k, value: v }) }); }

            // ── Docs ──
            async function loadDocs() {
                const el = document.getElementById('admin-content');
                const data = await api('/docs');
                el.innerHTML = '<div class="flex justify-between items-center mb-4"><h2 class="text-lg font-semibold">Docs</h2><button onclick="newDoc()" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-accent text-white hover:bg-accent-hover"><i class="ph ph-plus"></i> New Page</button></div>' +
                    ((data?.docs||[]).map(d => `
                    <div class="flex items-center justify-between p-3 bg-surface border border-zinc-800 rounded-lg mb-2">
                        <div><span class="text-sm font-medium">${d.title}</span><span class="text-xs text-zinc-500 ml-2">/${d.slug} · ${d.category}</span></div>
                        <div class="flex items-center gap-2"><span class="text-xs ${d.published ? 'text-green-400' : 'text-zinc-500'}">${d.published ? 'Published' : 'Draft'}</span>
                        <button onclick="if(confirm('Delete?')) delDoc('${d.id}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400 hover:bg-red-500/20">Delete</button></div>
                    </div>`).join('') || '<p class="text-zinc-500 text-sm">No docs</p>');
            }
            function newDoc() { showModal(`<h3 class="text-base font-semibold mb-4">New Doc Page</h3><div class="space-y-3">
                <div><label class="block text-xs text-zinc-500 mb-1">Slug</label><input id="nd-slug" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Title</label><input id="nd-title" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Category</label><input id="nd-cat" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Content (markdown)</label><textarea id="nd-content" rows="6" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50"></textarea></div>
                <button onclick="saveNewDoc()" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover">Create</button></div>`); }
            async function saveNewDoc() { await api('/docs', { method: 'POST', body: JSON.stringify({ slug: document.getElementById('nd-slug').value, title: document.getElementById('nd-title').value, category: document.getElementById('nd-cat').value, content: document.getElementById('nd-content').value }) }); hideModal(); loadDocs(); }
            async function delDoc(id) { await api('/docs/'+id, { method: 'DELETE' }); loadDocs(); }

            // Init
            loadStats();
            showTab('users');
            "##
        </script>
    }
}
