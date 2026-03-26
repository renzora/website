use leptos::prelude::*;

#[component]
pub fn AdminPage() -> impl IntoView {
    view! {
        <link rel="stylesheet" href="https://cdn.quilljs.com/1.3.7/quill.snow.css" />
        <script src="https://cdn.quilljs.com/1.3.7/quill.min.js"></script>
        <section class="min-h-screen flex">
            // ── Vertical Sidebar ──
            <aside class="w-56 bg-surface-card border-r border-zinc-800 flex flex-col shrink-0 sticky top-14 h-[calc(100vh-3.5rem)] overflow-y-auto">
                <div class="p-4 border-b border-zinc-800">
                    <div class="flex items-center gap-2">
                        <i class="ph ph-shield-check text-xl text-accent"></i>
                        <h1 class="text-base font-bold">"Admin Panel"</h1>
                    </div>
                </div>
                <nav class="flex-1 py-2">
                    <div class="px-3 py-1.5 text-[10px] font-semibold text-zinc-600 uppercase tracking-wider">"Overview"</div>
                    <button onclick="showTab('analytics')" id="tab-analytics" class="admin-tab w-full flex items-center gap-2.5 px-4 py-2 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                        <i class="ph ph-chart-line text-base"></i>"Analytics"
                    </button>
                    <div class="px-3 py-1.5 mt-2 text-[10px] font-semibold text-zinc-600 uppercase tracking-wider">"Management"</div>
                    <button onclick="showTab('users')" id="tab-users" class="admin-tab w-full flex items-center gap-2.5 px-4 py-2 text-sm bg-white/5 text-zinc-50">
                        <i class="ph ph-users text-base"></i>"Users"
                    </button>
                    <button onclick="showTab('assets')" id="tab-assets" class="admin-tab w-full flex items-center gap-2.5 px-4 py-2 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                        <i class="ph ph-package text-base"></i>"Assets"
                    </button>
                    <button onclick="showTab('categories')" id="tab-categories" class="admin-tab w-full flex items-center gap-2.5 px-4 py-2 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                        <i class="ph ph-folders text-base"></i>"Categories"
                    </button>
                    <button onclick="showTab('roles')" id="tab-roles" class="admin-tab w-full flex items-center gap-2.5 px-4 py-2 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                        <i class="ph ph-crown text-base"></i>"Roles"
                    </button>
                    <button onclick="showTab('badges')" id="tab-badges" class="admin-tab w-full flex items-center gap-2.5 px-4 py-2 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                        <i class="ph ph-medal text-base"></i>"Badges"
                    </button>
                    <div class="px-3 py-1.5 mt-2 text-[10px] font-semibold text-zinc-600 uppercase tracking-wider">"Content"</div>
                    <button onclick="showTab('forum')" id="tab-forum" class="admin-tab w-full flex items-center gap-2.5 px-4 py-2 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                        <i class="ph ph-chat-circle-dots text-base"></i>"Forum"
                    </button>
                    <button onclick="showTab('docs')" id="tab-docs" class="admin-tab w-full flex items-center gap-2.5 px-4 py-2 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                        <i class="ph ph-book-open text-base"></i>"Docs"
                    </button>
                    <button onclick="showTab('reviews')" id="tab-reviews" class="admin-tab w-full flex items-center gap-2.5 px-4 py-2 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                        <i class="ph ph-star text-base"></i>"Reviews"
                    </button>
                    <div class="px-3 py-1.5 mt-2 text-[10px] font-semibold text-zinc-600 uppercase tracking-wider">"Finance"</div>
                    <button onclick="showTab('withdrawals')" id="tab-withdrawals" class="admin-tab w-full flex items-center gap-2.5 px-4 py-2 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                        <i class="ph ph-bank text-base"></i>"Withdrawals"
                    </button>
                    <button onclick="showTab('disputes')" id="tab-disputes" class="admin-tab w-full flex items-center gap-2.5 px-4 py-2 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                        <i class="ph ph-warning text-base"></i>"Disputes"
                    </button>
                    <button onclick="showTab('promos')" id="tab-promos" class="admin-tab w-full flex items-center gap-2.5 px-4 py-2 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                        <i class="ph ph-ticket text-base"></i>"Promos"
                    </button>
                    <div class="px-3 py-1.5 mt-2 text-[10px] font-semibold text-zinc-600 uppercase tracking-wider">"System"</div>
                    <button onclick="showTab('settings')" id="tab-settings" class="admin-tab w-full flex items-center gap-2.5 px-4 py-2 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                        <i class="ph ph-gear text-base"></i>"Settings"
                    </button>
                </nav>
            </aside>

            // ── Main Content ──
            <div class="flex-1 p-6 min-w-0">
                // Stats row
                <div id="admin-stats" class="grid grid-cols-2 md:grid-cols-5 gap-3 mb-6">
                    <div class="p-4 bg-surface-card border border-zinc-800 rounded-lg">
                        <span class="text-[10px] text-zinc-500 uppercase tracking-wider">"Users"</span>
                        <div id="stat-users" class="text-xl font-bold mt-1">"—"</div>
                    </div>
                    <div class="p-4 bg-surface-card border border-zinc-800 rounded-lg">
                        <span class="text-[10px] text-zinc-500 uppercase tracking-wider">"Assets"</span>
                        <div id="stat-assets" class="text-xl font-bold mt-1">"—"</div>
                    </div>
                    <div class="p-4 bg-surface-card border border-zinc-800 rounded-lg">
                        <span class="text-[10px] text-zinc-500 uppercase tracking-wider">"Transactions"</span>
                        <div id="stat-txns" class="text-xl font-bold mt-1">"—"</div>
                    </div>
                    <div class="p-4 bg-surface-card border border-zinc-800 rounded-lg">
                        <span class="text-[10px] text-zinc-500 uppercase tracking-wider">"Credits"</span>
                        <div id="stat-credits" class="text-xl font-bold mt-1">"—"</div>
                    </div>
                    <div class="p-4 bg-surface-card border border-zinc-800 rounded-lg">
                        <span class="text-[10px] text-zinc-500 uppercase tracking-wider">"Disputes"</span>
                        <div id="stat-disputes" class="text-xl font-bold mt-1">"—"</div>
                    </div>
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
                document.querySelectorAll('.admin-tab').forEach(t => {
                    t.classList.remove('bg-white/5', 'text-zinc-50');
                    t.classList.add('text-zinc-400');
                });
                const tab = document.getElementById('tab-' + name);
                if (tab) {
                    tab.classList.add('bg-white/5', 'text-zinc-50');
                    tab.classList.remove('text-zinc-400');
                }
                const loaders = { users: loadUsers, assets: loadAssets, categories: loadCategories, disputes: loadDisputes, roles: loadRoles, forum: loadForumCats, badges: loadBadges, reviews: loadFlaggedReviews, withdrawals: loadWithdrawals, promos: loadPromos, settings: loadSettings, docs: loadDocs, analytics: loadAnalytics };
                if (loaders[name]) loaders[name]();
            }

            async function loadStats() {
                const d = await api('/stats');
                if (!d) return;
                document.getElementById('stat-users').textContent = d.total_users?.toLocaleString() || '0';
                document.getElementById('stat-assets').textContent = d.total_assets?.toLocaleString() || '0';
                document.getElementById('stat-txns').textContent = d.total_transactions?.toLocaleString() || '0';
                document.getElementById('stat-credits').textContent = d.total_credits_circulating?.toLocaleString() || '0';
                document.getElementById('stat-disputes').textContent = d.open_disputes?.toLocaleString() || '0';
            }

            // ── Analytics ──
            async function loadAnalytics() {
                const el = document.getElementById('admin-content');
                el.innerHTML = '<div class="text-center py-12"><div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-accent rounded-full"></div></div>';
                const d = await api('/analytics');
                if (!d) return;
                const usd = (credits) => '$' + (credits * 0.10).toFixed(2);
                el.innerHTML = `
                    <div class="flex justify-between items-center mb-6">
                        <h2 class="text-lg font-semibold">Platform Analytics</h2>
                        <button onclick="exportAnalyticsPDF()" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover"><i class="ph ph-file-pdf text-base"></i>Export PDF</button>
                    </div>
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-3 mb-6">
                        <div class="p-4 bg-surface border border-zinc-800 rounded-lg">
                            <div class="text-[10px] text-zinc-500 uppercase tracking-wider">Total Revenue</div>
                            <div class="text-2xl font-bold text-green-400 mt-1">${usd(d.total_revenue)}</div>
                            <div class="text-xs text-zinc-500">${d.total_revenue?.toLocaleString()} credits</div>
                        </div>
                        <div class="p-4 bg-surface border border-zinc-800 rounded-lg">
                            <div class="text-[10px] text-zinc-500 uppercase tracking-wider">Platform Commission</div>
                            <div class="text-2xl font-bold text-accent mt-1">${usd(d.platform_commission)}</div>
                            <div class="text-xs text-zinc-500">${d.platform_commission?.toLocaleString()} credits</div>
                        </div>
                        <div class="p-4 bg-surface border border-zinc-800 rounded-lg">
                            <div class="text-[10px] text-zinc-500 uppercase tracking-wider">Creator Earnings</div>
                            <div class="text-2xl font-bold text-blue-400 mt-1">${usd(d.creator_earnings)}</div>
                            <div class="text-xs text-zinc-500">${d.creator_earnings?.toLocaleString()} credits</div>
                        </div>
                        <div class="p-4 bg-surface border border-zinc-800 rounded-lg">
                            <div class="text-[10px] text-zinc-500 uppercase tracking-wider">Total Sales</div>
                            <div class="text-2xl font-bold text-amber-400 mt-1">${d.sales_count?.toLocaleString()}</div>
                            <div class="text-xs text-zinc-500">${usd(d.total_purchases)} volume</div>
                        </div>
                    </div>
                    <div class="grid grid-cols-2 md:grid-cols-3 gap-3 mb-6">
                        <div class="p-4 bg-surface border border-zinc-800 rounded-lg">
                            <div class="text-[10px] text-zinc-500 uppercase tracking-wider">Withdrawn</div>
                            <div class="text-xl font-bold mt-1">${usd(d.withdrawn)}</div>
                        </div>
                        <div class="p-4 bg-surface border border-zinc-800 rounded-lg">
                            <div class="text-[10px] text-zinc-500 uppercase tracking-wider">Pending Withdrawals</div>
                            <div class="text-xl font-bold text-amber-400 mt-1">${usd(d.pending_withdrawals)}</div>
                        </div>
                        <div class="p-4 bg-surface border border-zinc-800 rounded-lg">
                            <div class="text-[10px] text-zinc-500 uppercase tracking-wider">Referral Payouts</div>
                            <div class="text-xl font-bold text-purple-400 mt-1">${usd(d.referral_total)}</div>
                        </div>
                    </div>
                    ${d.top_assets?.length ? `
                    <div class="p-4 bg-surface border border-zinc-800 rounded-lg">
                        <h3 class="text-sm font-semibold mb-3">Top Selling Assets</h3>
                        <div class="space-y-2">
                            ${d.top_assets.map((a, i) => `
                                <div class="flex items-center justify-between py-1.5 ${i > 0 ? 'border-t border-zinc-800/50' : ''}">
                                    <div class="flex items-center gap-2">
                                        <span class="text-xs text-zinc-500 w-5">#${i+1}</span>
                                        <a href="/marketplace/asset/${a.slug}" class="text-sm text-accent hover:text-accent-hover">${a.name}</a>
                                    </div>
                                    <div class="flex items-center gap-4">
                                        <span class="text-xs text-zinc-400">${a.sales} sales</span>
                                        <span class="text-xs font-semibold text-green-400">${a.revenue?.toLocaleString()} credits</span>
                                    </div>
                                </div>
                            `).join('')}
                        </div>
                    </div>` : ''}
                `;
            }

            function exportAnalyticsPDF() {
                // Build a printable version
                const content = document.getElementById('admin-content').cloneNode(true);
                // Remove the export button
                const btn = content.querySelector('button');
                if (btn) btn.remove();
                const w = window.open('', '_blank');
                w.document.write(`<!DOCTYPE html><html><head><title>Renzora Analytics Report</title>
                    <style>body{font-family:system-ui;padding:2rem;color:#333;max-width:900px;margin:0 auto}
                    h2{font-size:1.5rem;margin-bottom:1rem}h3{font-size:1rem}
                    .grid{display:grid;gap:1rem;margin-bottom:1.5rem}
                    .grid-cols-2{grid-template-columns:1fr 1fr}.grid-cols-3{grid-template-columns:1fr 1fr 1fr}.grid-cols-4{grid-template-columns:1fr 1fr 1fr 1fr}
                    @media(max-width:600px){.grid-cols-3,.grid-cols-4{grid-template-columns:1fr 1fr}}
                    div[class*="p-4"]{padding:1rem;border:1px solid #ddd;border-radius:8px}
                    div[class*="text-2xl"]{font-size:1.5rem;font-weight:bold}div[class*="text-xl"]{font-size:1.25rem;font-weight:bold}
                    div[class*="text-xs"]{font-size:0.75rem;color:#666}div[class*="text-\\[10px\\]"]{font-size:0.625rem;color:#999;text-transform:uppercase}
                    a{color:#6366f1}@media print{body{padding:0}}</style>
                    </head><body><h2>Renzora Analytics Report</h2><p style="color:#666;font-size:0.875rem">Generated ${new Date().toLocaleDateString('en-US', {year:'numeric',month:'long',day:'numeric'})}</p><hr style="margin:1rem 0;border-color:#eee">${content.innerHTML}</body></html>`);
                w.document.close();
                setTimeout(() => { w.print(); }, 500);
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
                            <button onclick="editUserFull('${u.id}')" class="text-xs px-2 py-1 rounded bg-zinc-800 text-zinc-300 hover:bg-zinc-700" title="Edit user"><i class="ph ph-pencil-simple"></i> Edit</button>
                            <button onclick="showUserNotes('${u.id}','${u.username}')" class="text-xs px-2 py-1 rounded bg-zinc-800 text-zinc-300 hover:bg-zinc-700" title="Mod notes"><i class="ph ph-note"></i></button>
                            <button onclick="showBanModal('${u.id}','${u.username}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400 hover:bg-red-500/20"><i class="ph ph-gavel"></i></button>
                        </div>
                    </div>
                `).join('') || '<p class="text-zinc-500 text-sm">No users</p>';
            }

            async function editUserFull(id) {
                // Fetch all users and find the one
                const users = await api('/users?q=');
                const u = users?.find(x => x.id === id);
                if (!u) return;
                // Fetch full user profile for extra fields
                const token = getToken();
                let profile = {};
                try {
                    const pres = await fetch('/api/profiles/view/' + u.username, { headers: { 'Authorization': 'Bearer ' + token } });
                    if (pres.ok) profile = await pres.json();
                } catch(e) {}

                showModal(`
                    <h3 class="text-base font-semibold mb-4">Edit User: ${u.username}</h3>
                    <div class="space-y-3 max-h-[60vh] overflow-y-auto">
                        <div class="grid grid-cols-2 gap-3">
                            <div><label class="block text-xs text-zinc-500 mb-1">Username</label>
                            <input id="eu-username" value="${u.username}" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                            <div><label class="block text-xs text-zinc-500 mb-1">Email</label>
                            <input id="eu-email" value="${u.email}" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                        </div>
                        <div class="grid grid-cols-2 gap-3">
                            <div><label class="block text-xs text-zinc-500 mb-1">Role</label>
                            <select id="eu-role" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50">
                                ${['user','creator','moderator','editor','admin'].map(r => '<option value="'+r+'" '+(r===u.role?'selected':'')+'>'+r+'</option>').join('')}
                            </select></div>
                            <div><label class="block text-xs text-zinc-500 mb-1">Credits</label>
                            <input id="eu-credits" type="number" value="${u.credit_balance}" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                        </div>
                        <div><label class="block text-xs text-zinc-500 mb-1">Bio</label>
                        <textarea id="eu-bio" rows="2" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50">${profile.bio || ''}</textarea></div>
                        <div class="grid grid-cols-2 gap-3">
                            <div><label class="block text-xs text-zinc-500 mb-1">Location</label>
                            <input id="eu-location" value="${profile.location || ''}" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                            <div><label class="block text-xs text-zinc-500 mb-1">Website</label>
                            <input id="eu-website" value="${profile.website || ''}" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                        </div>
                        <div class="grid grid-cols-3 gap-3">
                            <div><label class="block text-xs text-zinc-500 mb-1">Gender</label>
                            <select id="eu-gender" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50">
                                <option value="" ${!profile.gender?'selected':''}>—</option>
                                <option value="Male" ${profile.gender==='Male'?'selected':''}>Male</option>
                                <option value="Female" ${profile.gender==='Female'?'selected':''}>Female</option>
                                <option value="Non-binary" ${profile.gender==='Non-binary'?'selected':''}>Non-binary</option>
                            </select></div>
                            <div><label class="block text-xs text-zinc-500 mb-1">Profile Color</label>
                            <input id="eu-profile-color" type="color" value="${profile.profile_color || '#6366f1'}" class="w-full h-9 rounded cursor-pointer bg-transparent border border-zinc-800" /></div>
                            <div><label class="block text-xs text-zinc-500 mb-1">Banner Color</label>
                            <input id="eu-banner-color" type="color" value="${profile.banner_color || '#1e1b4b'}" class="w-full h-9 rounded cursor-pointer bg-transparent border border-zinc-800" /></div>
                        </div>
                        <div class="flex gap-2 pt-2">
                            <button onclick="saveUserFull('${id}')" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover">Save All</button>
                            <button onclick="hideModal()" class="px-4 py-2 rounded-lg text-sm text-zinc-400 hover:text-zinc-50">Cancel</button>
                        </div>
                    </div>
                `);
            }

            async function saveUserFull(id) {
                await api('/users/'+id+'/edit', { method: 'PUT', body: JSON.stringify({
                    username: document.getElementById('eu-username').value,
                    email: document.getElementById('eu-email').value,
                    role: document.getElementById('eu-role').value,
                    credit_balance: parseInt(document.getElementById('eu-credits').value) || 0,
                    bio: document.getElementById('eu-bio').value,
                    location: document.getElementById('eu-location').value,
                    website: document.getElementById('eu-website').value,
                    gender: document.getElementById('eu-gender').value,
                    profile_color: document.getElementById('eu-profile-color').value,
                    banner_color: document.getElementById('eu-banner-color').value,
                })});
                hideModal(); loadUsers(); loadStats();
            }

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
                const perms = ['manage_users','manage_roles','manage_bans','manage_assets','manage_categories','manage_docs','manage_forum','manage_disputes','manage_settings','manage_badges','mod_notes','view_admin','send_message','post_forum','upload_asset','submit_review','create_course'];
                el.innerHTML = '<div class="flex justify-between items-center mb-4"><h2 class="text-lg font-semibold">Roles</h2><button onclick="newRole()" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-accent text-white hover:bg-accent-hover"><i class="ph ph-plus"></i> New Role</button></div>' +
                    ((data||[]).map(r => `
                    <div class="p-4 bg-surface border border-zinc-800 rounded-lg mb-2">
                        <div class="flex justify-between items-center mb-2">
                            <div class="flex items-center gap-2"><span class="w-3 h-3 rounded-full" style="background:${r.color}"></span><span class="text-sm font-semibold">${r.name}</span>${r.is_staff ? '<span class="text-[10px] px-1.5 py-0.5 rounded bg-amber-500/10 text-amber-400">STAFF</span>' : ''}</div>
                            <div class="flex gap-2"><button onclick='editRolePerms("${r.id}","${r.name}",${JSON.stringify(JSON.stringify(r.permissions))})' class="text-xs px-2 py-1 rounded bg-zinc-800 text-zinc-300 hover:bg-zinc-700"><i class="ph ph-pencil-simple"></i> Permissions</button>
                            <button onclick="if(confirm('Delete role?')) delRole('${r.id}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400 hover:bg-red-500/20">Delete</button></div>
                        </div>
                        <div class="flex flex-wrap gap-1">${perms.map(p => `<span class="text-[10px] px-1.5 py-0.5 rounded ${r.permissions[p] ? 'bg-green-500/10 text-green-400' : 'bg-zinc-800 text-zinc-600'}">${p}</span>`).join('')}</div>
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
                const allPerms = ['manage_users','manage_roles','manage_bans','manage_assets','manage_categories','manage_docs','manage_forum','manage_disputes','manage_settings','manage_badges','mod_notes','view_admin','send_message','post_forum','upload_asset','submit_review','create_course'];
                showModal(`<h3 class="text-base font-semibold mb-4">Permissions: ${name}</h3>
                    <div class="mb-3"><p class="text-[10px] text-zinc-500 uppercase tracking-wider mb-2">Admin Permissions</p>
                    <div class="space-y-1">${allPerms.slice(0,12).map(p => `<label class="flex items-center gap-2"><input type="checkbox" id="perm-${p}" ${perms[p]?'checked':''} class="rounded accent-accent" /><span class="text-sm text-zinc-300">${p}</span></label>`).join('')}</div></div>
                    <div class="mb-4"><p class="text-[10px] text-zinc-500 uppercase tracking-wider mb-2">User Permissions</p>
                    <div class="space-y-1">${allPerms.slice(12).map(p => `<label class="flex items-center gap-2"><input type="checkbox" id="perm-${p}" ${perms[p]?'checked':''} class="rounded accent-accent" /><span class="text-sm text-zinc-300">${p}</span></label>`).join('')}</div></div>
                    <button onclick="savePerms('${id}')" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover">Save</button>`);
            }
            async function savePerms(id) {
                const allPerms = ['manage_users','manage_roles','manage_bans','manage_assets','manage_categories','manage_docs','manage_forum','manage_disputes','manage_settings','manage_badges','mod_notes','view_admin','send_message','post_forum','upload_asset','submit_review','create_course'];
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

            // ── Badges with creation + rules ──
            async function loadBadges() {
                const el = document.getElementById('admin-content');
                const data = await api('/badges');
                el.innerHTML = `<div class="flex justify-between items-center mb-4">
                    <h2 class="text-lg font-semibold">Badges</h2>
                    <button onclick="newBadge()" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-accent text-white hover:bg-accent-hover"><i class="ph ph-plus"></i> Create Badge</button>
                </div>` +
                    ((data?.badges||[]).map(b => `
                    <div class="flex items-center justify-between p-3 bg-surface border border-zinc-800 rounded-lg mb-2">
                        <div class="flex items-center gap-2">
                            <i class="ph ${b.icon} text-lg" style="color:${b.color}"></i>
                            <span class="text-sm font-medium">${b.name}</span>
                            <span class="text-xs text-zinc-500">${b.description}</span>
                            ${b.auto_rule ? `<span class="text-[10px] px-1.5 py-0.5 rounded bg-blue-500/10 text-blue-400">Auto: ${b.auto_rule} ≥ ${b.auto_threshold}</span>` : ''}
                        </div>
                        <div class="flex gap-2">
                            <span class="text-xs text-zinc-500 font-mono">${b.slug}</span>
                            <button onclick="if(confirm('Delete badge?')) deleteBadge('${b.id}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400 hover:bg-red-500/20">Delete</button>
                        </div>
                    </div>`).join('') || '<p class="text-zinc-500 text-sm">No badges</p>');
            }

            function newBadge() {
                showModal(`<h3 class="text-base font-semibold mb-4">Create Badge</h3>
                    <div class="space-y-3">
                        <div class="grid grid-cols-2 gap-3">
                            <div><label class="block text-xs text-zinc-500 mb-1">Name</label><input id="nb-name" placeholder="Top Seller" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                            <div><label class="block text-xs text-zinc-500 mb-1">Slug</label><input id="nb-slug" placeholder="top-seller" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                        </div>
                        <div><label class="block text-xs text-zinc-500 mb-1">Description</label><input id="nb-desc" placeholder="Awarded to top sellers" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                        <div class="grid grid-cols-2 gap-3">
                            <div><label class="block text-xs text-zinc-500 mb-1">Icon (Phosphor class)</label><input id="nb-icon" value="ph-trophy" placeholder="ph-trophy" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                            <div><label class="block text-xs text-zinc-500 mb-1">Color</label><input id="nb-color" type="color" value="#f59e0b" class="w-full h-9 rounded cursor-pointer bg-transparent border border-zinc-800" /></div>
                        </div>
                        <div class="border-t border-zinc-800 pt-3">
                            <p class="text-xs text-zinc-400 mb-2"><i class="ph ph-lightning text-amber-400"></i> Auto-award Rule (optional)</p>
                            <div class="grid grid-cols-2 gap-3">
                                <div><label class="block text-xs text-zinc-500 mb-1">Rule Type</label>
                                <select id="nb-rule" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50">
                                    <option value="">None (manual only)</option>
                                    <option value="sales_count">Sales count</option>
                                    <option value="post_count">Forum post count</option>
                                    <option value="follower_count">Follower count</option>
                                    <option value="download_count">Total downloads</option>
                                    <option value="earning_total">Total earnings (credits)</option>
                                    <option value="account_age_days">Account age (days)</option>
                                </select></div>
                                <div><label class="block text-xs text-zinc-500 mb-1">Threshold</label>
                                <input id="nb-threshold" type="number" placeholder="e.g. 100" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                            </div>
                        </div>
                        <button onclick="saveNewBadge()" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover">Create Badge</button>
                    </div>`);
            }

            async function saveNewBadge() {
                const rule = document.getElementById('nb-rule').value;
                const threshold = document.getElementById('nb-threshold').value;
                await api('/badges/create', { method: 'POST', body: JSON.stringify({
                    name: document.getElementById('nb-name').value,
                    slug: document.getElementById('nb-slug').value,
                    description: document.getElementById('nb-desc').value,
                    icon: document.getElementById('nb-icon').value,
                    color: document.getElementById('nb-color').value,
                    auto_rule: rule || null,
                    auto_threshold: threshold ? parseInt(threshold) : null,
                })});
                hideModal(); loadBadges();
            }
            async function deleteBadge(id) { await api('/badges/'+id, { method: 'DELETE' }); loadBadges(); }

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
            let docQuill = null;
            async function loadDocs() {
                const el = document.getElementById('admin-content');
                const data = await api('/docs');
                el.innerHTML = `
                    <div class="flex justify-between items-center mb-4">
                        <h2 class="text-lg font-semibold">Documentation Pages</h2>
                        <button onclick="newDoc()" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-accent text-white hover:bg-accent-hover"><i class="ph ph-plus"></i> New Page</button>
                    </div>
                    ${(data?.docs||[]).map(d => `
                    <div class="flex items-center justify-between p-3 bg-surface border border-zinc-800 rounded-lg mb-2">
                        <div class="flex-1"><span class="text-sm font-medium">${d.title}</span><span class="text-xs text-zinc-500 ml-2">/${d.slug} · ${d.category} · #${d.sort_order}</span></div>
                        <div class="flex items-center gap-2">
                            <span class="text-xs ${d.published ? 'text-green-400' : 'text-zinc-500'}">${d.published ? 'Published' : 'Draft'}</span>
                            <button onclick="toggleDocPublish('${d.id}', ${!d.published})" class="text-xs px-2 py-1 rounded bg-zinc-800 text-zinc-300 hover:bg-zinc-700">${d.published ? 'Unpublish' : 'Publish'}</button>
                            <button onclick="editDoc('${d.id}')" class="text-xs px-2 py-1 rounded bg-zinc-800 text-zinc-300 hover:bg-zinc-700"><i class="ph ph-pencil-simple"></i></button>
                            <button onclick="if(confirm('Delete?')) delDoc('${d.id}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400 hover:bg-red-500/20"><i class="ph ph-trash"></i></button>
                        </div>
                    </div>`).join('') || '<p class="text-zinc-500 text-sm">No docs yet.</p>'}`;
            }
            function initDocEditor(content) { setTimeout(() => { docQuill = new Quill('#doc-editor', { theme: 'snow', placeholder: 'Write content...', modules: { toolbar: [[{header:[1,2,3,false]}],['bold','italic','underline','strike'],[{list:'ordered'},{list:'bullet'}],['blockquote','code-block'],['link','image'],['clean']] } }); if (content) docQuill.root.innerHTML = content; }, 100); }
            function newDoc() { showModal(`<h3 class="text-base font-semibold mb-4">New Doc Page</h3><div class="space-y-3"><div class="grid grid-cols-2 gap-3"><div><label class="block text-xs text-zinc-500 mb-1">Slug</label><input id="nd-slug" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div><div><label class="block text-xs text-zinc-500 mb-1">Category</label><input id="nd-cat" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div></div><div><label class="block text-xs text-zinc-500 mb-1">Title</label><input id="nd-title" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div><div><label class="block text-xs text-zinc-500 mb-1">Sort Order</label><input id="nd-sort" type="number" value="0" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div><div><label class="block text-xs text-zinc-500 mb-1">Content</label><div id="doc-editor" class="bg-surface border border-zinc-800 rounded-lg" style="min-height:300px"></div></div><div class="flex gap-2"><button onclick="saveNewDoc(false)" class="px-4 py-2 rounded-lg text-sm font-medium bg-zinc-800 text-zinc-300">Save Draft</button><button onclick="saveNewDoc(true)" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover">Save & Publish</button></div></div>`); initDocEditor(''); }
            async function saveNewDoc(publish) { const content = docQuill ? docQuill.root.innerHTML : ''; await api('/docs', { method: 'POST', body: JSON.stringify({ slug: document.getElementById('nd-slug').value, title: document.getElementById('nd-title').value, category: document.getElementById('nd-cat').value, content })}); if (publish) { const data = await api('/docs'); const newest = data?.docs?.[data.docs.length-1]; if(newest) await api('/docs/'+newest.id, { method: 'PUT', body: JSON.stringify({ published: true, sort_order: parseInt(document.getElementById('nd-sort').value)||0 }) }); } hideModal(); loadDocs(); }
            async function editDoc(id) { const data = await api('/docs'); const doc = data?.docs?.find(d => d.id === id); if(!doc) return; const res = await fetch('/api/docs/'+doc.slug); const full = res.ok ? await res.json() : {content:''}; showModal(`<h3 class="text-base font-semibold mb-4">Edit: ${doc.title}</h3><div class="space-y-3"><div><label class="block text-xs text-zinc-500 mb-1">Title</label><input id="ed-title" value="${doc.title}" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div><div class="grid grid-cols-2 gap-3"><div><label class="block text-xs text-zinc-500 mb-1">Category</label><input id="ed-cat" value="${doc.category}" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div><div><label class="block text-xs text-zinc-500 mb-1">Sort Order</label><input id="ed-sort" type="number" value="${doc.sort_order}" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div></div><div><label class="block text-xs text-zinc-500 mb-1">Content</label><div id="doc-editor" class="bg-surface border border-zinc-800 rounded-lg" style="min-height:300px"></div></div><button onclick="saveEditDoc('${id}')" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover">Save</button></div>`); initDocEditor(full.content||''); }
            async function saveEditDoc(id) { const content = docQuill ? docQuill.root.innerHTML : ''; await api('/docs/'+id, { method: 'PUT', body: JSON.stringify({ title: document.getElementById('ed-title').value, category: document.getElementById('ed-cat').value, content, sort_order: parseInt(document.getElementById('ed-sort').value)||0 })}); hideModal(); loadDocs(); }
            async function toggleDocPublish(id, publish) { await api('/docs/'+id, { method: 'PUT', body: JSON.stringify({ published: publish }) }); loadDocs(); }
            async function delDoc(id) { await api('/docs/'+id, { method: 'DELETE' }); loadDocs(); }

            // ── Reviews ──
            async function loadFlaggedReviews() {
                const el = document.getElementById('admin-content');
                const data = await api('/reviews/flagged');
                if (!data) return;
                el.innerHTML = `<h2 class="text-lg font-semibold mb-4">Flagged Reviews <span class="text-zinc-500 text-sm font-normal">(${data.length})</span></h2>` +
                (data.length ? data.map(r => {
                    const stars = '★'.repeat(r.rating) + '☆'.repeat(5 - r.rating);
                    return `<div class="p-4 bg-surface border border-zinc-800 rounded-lg mb-2 ${r.hidden ? 'opacity-50' : ''}">
                        <div class="flex justify-between items-start mb-2"><div><span class="text-amber-400 text-sm">${stars}</span><span class="text-sm font-medium ml-2">${r.title || '(no title)'}</span></div>
                        <div class="flex items-center gap-2"><a href="/profile/${r.author_name}" class="text-xs text-accent">${r.author_name}</a><span class="text-xs text-zinc-500">on</span><span class="text-xs text-zinc-300">${r.asset_name}</span></div></div>
                        <p class="text-sm text-zinc-400 mb-2">${r.content || '(no content)'}</p>
                        ${r.flag_reason ? `<p class="text-xs text-red-400 mb-2"><i class="ph ph-flag"></i> ${r.flag_reason}</p>` : ''}
                        <div class="flex gap-2">
                            <button onclick="dismissFlag('${r.id}')" class="text-xs px-2 py-1 rounded bg-zinc-800 text-zinc-300 hover:bg-zinc-700">Dismiss</button>
                            <button onclick="hideReview('${r.id}')" class="text-xs px-2 py-1 rounded bg-amber-500/10 text-amber-400">${r.hidden?'Unhide':'Hide'}</button>
                            <button onclick="if(confirm('Delete?')) delReview('${r.id}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400">Delete</button>
                        </div></div>`;
                }).join('') : '<p class="text-zinc-500 text-sm py-4">No flagged reviews.</p>');
            }
            async function dismissFlag(id) { await api('/reviews/'+id+'/dismiss', { method: 'PUT' }); loadFlaggedReviews(); }
            async function hideReview(id) { const data = await api('/reviews/flagged'); const r = data?.find(x=>x.id===id); if(r?.hidden) await api('/reviews/'+id+'/unhide',{method:'PUT'}); else await api('/reviews/'+id+'/hide',{method:'PUT'}); loadFlaggedReviews(); }
            async function delReview(id) { await api('/reviews/'+id, { method: 'DELETE' }); loadFlaggedReviews(); }

            // ── Withdrawals with accept + investigate ──
            async function loadWithdrawals() {
                const el = document.getElementById('admin-content');
                const data = await api('/withdrawals');
                if (!data) return;
                el.innerHTML = `<h2 class="text-lg font-semibold mb-4">Withdrawals</h2>` +
                (data.length ? data.map(w => {
                    const statusColors = { completed: 'text-green-400 bg-green-500/10', pending: 'text-amber-400 bg-amber-500/10', processing: 'text-blue-400 bg-blue-500/10', failed: 'text-red-400 bg-red-500/10', rejected: 'text-red-400 bg-red-500/10' };
                    const sc = statusColors[w.status] || 'text-zinc-400 bg-zinc-800';
                    const usd = (w.amount_usd_cents / 100).toFixed(2);
                    return `
                    <div class="flex items-center justify-between p-3 bg-surface border border-zinc-800 rounded-lg mb-2">
                        <div class="flex items-center gap-3">
                            <a href="/profile/${w.username}" class="text-sm font-medium text-accent hover:text-accent-hover">${w.username}</a>
                            <span class="text-sm font-semibold">${w.amount_credits?.toLocaleString()} credits</span>
                            <span class="text-xs text-zinc-500">$${usd}</span>
                            <span class="text-[10px] px-2 py-0.5 rounded ${sc}">${w.status}</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="text-xs text-zinc-500">${new Date(w.created_at).toLocaleDateString()}</span>
                            ${w.status === 'pending' || w.status === 'processing' ? `
                                <button onclick="acceptWithdrawal('${w.id}')" class="text-xs px-2 py-1 rounded bg-green-500/10 text-green-400 hover:bg-green-500/20"><i class="ph ph-check"></i> Accept</button>
                                <button onclick="rejectWithdrawal('${w.id}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400 hover:bg-red-500/20"><i class="ph ph-x"></i> Reject</button>
                            ` : ''}
                            <button onclick="investigateWithdrawal('${w.id}','${w.username}')" class="text-xs px-2 py-1 rounded bg-zinc-800 text-zinc-300 hover:bg-zinc-700"><i class="ph ph-magnifying-glass"></i> Investigate</button>
                            ${w.failure_reason ? `<span class="text-[10px] text-red-400" title="${w.failure_reason}"><i class="ph ph-info"></i></span>` : ''}
                        </div>
                    </div>`;
                }).join('') : '<p class="text-zinc-500 text-sm py-4">No withdrawals yet.</p>');
            }

            async function acceptWithdrawal(id) {
                if (!confirm('Accept this withdrawal and mark as completed?')) return;
                await api('/withdrawals/'+id+'/accept', { method: 'PUT' });
                loadWithdrawals();
            }

            async function rejectWithdrawal(id) {
                const reason = prompt('Rejection reason:');
                if (!reason) return;
                await api('/withdrawals/'+id+'/reject', { method: 'PUT', body: JSON.stringify({ reason }) });
                loadWithdrawals();
            }

            async function investigateWithdrawal(id, username) {
                const data = await api('/withdrawals/'+id+'/transactions');
                if (!data) return;
                const txns = data.transactions || [];
                showModal(`
                    <h3 class="text-base font-semibold mb-2">Investigate: ${username}</h3>
                    <p class="text-xs text-zinc-500 mb-4">Withdrawal: ${data.withdrawal_amount?.toLocaleString()} credits</p>
                    <div class="space-y-1 max-h-[50vh] overflow-y-auto">
                        ${txns.length ? txns.map(t => {
                            const isPos = t.amount > 0;
                            const color = isPos ? 'text-green-400' : 'text-red-400';
                            return `<div class="flex justify-between items-center py-2 border-b border-zinc-800/50">
                                <div><span class="text-xs px-1.5 py-0.5 rounded bg-zinc-800">${t.type}</span><span class="text-xs text-zinc-500 ml-2">${new Date(t.created_at).toLocaleString()}</span></div>
                                <span class="text-sm font-semibold ${color}">${isPos?'+':''}${t.amount}</span>
                            </div>`;
                        }).join('') : '<p class="text-zinc-500 text-sm">No transactions found.</p>'}
                    </div>
                `);
            }

            // ── Promos ──
            async function loadPromos() {
                const el = document.getElementById('admin-content');
                const data = await api('/promo-codes');
                if (!data) return;
                el.innerHTML = `<div class="flex justify-between items-center mb-4"><h2 class="text-lg font-semibold">Promo Codes</h2><button onclick="newPromo()" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-accent text-white hover:bg-accent-hover"><i class="ph ph-plus"></i> New Code</button></div>
                <p class="text-xs text-zinc-500 mb-4">Promo codes reduce the platform's 20% fee. A code with 10% discount means the creator gets 90% instead of 80%.</p>` +
                (data.length ? data.map(p => {
                    const expired = p.expires_at && new Date(p.expires_at) < new Date();
                    const maxed = p.max_uses && p.times_used >= p.max_uses;
                    const statusColor = !p.active ? 'text-zinc-500' : expired || maxed ? 'text-amber-400' : 'text-green-400';
                    const statusText = !p.active ? 'Inactive' : expired ? 'Expired' : maxed ? 'Max used' : 'Active';
                    return `<div class="flex items-center justify-between p-3 bg-surface border border-zinc-800 rounded-lg mb-2">
                        <div class="flex items-center gap-3"><code class="text-sm font-mono font-bold text-accent bg-accent/10 px-2 py-0.5 rounded">${p.code}</code><span class="text-xs text-zinc-400">-${p.discount_percent}%</span><span class="text-xs ${statusColor}">${statusText}</span></div>
                        <div class="flex items-center gap-3"><span class="text-xs text-zinc-500">Uses: ${p.max_uses ? p.times_used+'/'+p.max_uses : p.times_used+'/∞'}</span>
                        <button onclick="togglePromo('${p.id}')" class="text-xs px-2 py-1 rounded bg-zinc-800 text-zinc-300 hover:bg-zinc-700">${p.active?'Disable':'Enable'}</button>
                        <button onclick="if(confirm('Delete?')) delPromo('${p.id}')" class="text-xs px-2 py-1 rounded bg-red-500/10 text-red-400">Delete</button></div>
                    </div>`;
                }).join('') : '<p class="text-zinc-500 text-sm py-4">No promo codes.</p>');
            }
            function newPromo() { showModal(`<h3 class="text-base font-semibold mb-4">New Promo Code</h3><div class="space-y-3">
                <div><label class="block text-xs text-zinc-500 mb-1">Code</label><input id="np-code" placeholder="LAUNCH2026" maxlength="32" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 uppercase" /></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Discount (% off fee, 1-20)</label><input id="np-discount" type="number" min="1" max="20" value="10" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50" /></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Max Uses</label><select id="np-uses" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50"><option value="">Unlimited</option><option value="1">1</option><option value="5">5</option><option value="10">10</option><option value="25">25</option><option value="50">50</option><option value="100">100</option></select></div>
                <div><label class="block text-xs text-zinc-500 mb-1">Expires</label><select id="np-expires" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50"><option value="">Never</option><option value="24">24h</option><option value="168">7d</option><option value="720">30d</option><option value="2160">90d</option></select></div>
                <button onclick="savePromo()" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover">Create</button></div>`); }
            async function savePromo() { const code = document.getElementById('np-code').value.trim(); if(!code){alert('Code required');return;} await api('/promo-codes', { method: 'POST', body: JSON.stringify({ code, discount_percent: parseInt(document.getElementById('np-discount').value), max_uses: document.getElementById('np-uses').value ? parseInt(document.getElementById('np-uses').value) : null, expires_hours: document.getElementById('np-expires').value ? parseInt(document.getElementById('np-expires').value) : null })}); hideModal(); loadPromos(); }
            async function togglePromo(id) { await api('/promo-codes/'+id+'/toggle', { method: 'PUT' }); loadPromos(); }
            async function delPromo(id) { await api('/promo-codes/'+id, { method: 'DELETE' }); loadPromos(); }

            // Init
            loadStats();
            showTab('users');
            "##
        </script>
    }
}
