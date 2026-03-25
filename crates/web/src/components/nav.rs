use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="sticky top-0 z-50 bg-[rgba(10,10,11,0.8)] backdrop-blur-xl border-b border-zinc-800">
            <div class="max-w-[1200px] mx-auto px-6 h-14 flex items-center gap-8">
                <a href="/" class="text-lg font-bold tracking-tight">"Renzora"</a>
                <div class="flex gap-6 flex-1">
                    <a href="/download" class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors flex items-center gap-1.5">
                        <i class="ph ph-download-simple text-base"></i>"Download"
                    </a>
                    <a href="/games" class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors flex items-center gap-1.5">
                        <i class="ph ph-game-controller text-base"></i>"Game Store"
                    </a>
                    <a href="/marketplace" class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors flex items-center gap-1.5">
                        <i class="ph ph-storefront text-base"></i>"Marketplace"
                    </a>
                    <a href="/docs" class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors flex items-center gap-1.5">
                        <i class="ph ph-book-open text-base"></i>"Docs"
                    </a>
                </div>
                // Global search
                <div class="relative" id="global-search-wrap">
                    <button onclick="toggleGlobalSearch()" class="text-sm text-zinc-400 hover:text-zinc-50 hover:bg-surface-card p-1.5 rounded-lg transition-all" title="Search">
                        <i class="ph ph-magnifying-glass text-lg"></i>
                    </button>
                    <div id="global-search-panel" class="hidden absolute right-0 top-full mt-2 w-[420px] bg-surface-card border border-zinc-800 rounded-xl shadow-2xl shadow-black/50 overflow-hidden z-50">
                        <div class="flex items-center gap-2 px-4 py-3 border-b border-zinc-800">
                            <i class="ph ph-magnifying-glass text-zinc-500"></i>
                            <input type="text" id="global-search-input" placeholder="Search assets, users, docs..." oninput="globalSearch(this.value)" class="flex-1 bg-transparent text-sm text-zinc-50 outline-none placeholder:text-zinc-600" />
                            <kbd class="text-[10px] text-zinc-600 border border-zinc-800 rounded px-1.5 py-0.5">"Esc"</kbd>
                        </div>
                        <div id="global-search-results" class="max-h-[400px] overflow-y-auto">
                            <div class="px-4 py-8 text-center text-xs text-zinc-600">"Type to search across marketplace, users, and docs."</div>
                        </div>
                    </div>
                </div>
                // Logged-out
                <div id="nav-guest" class="flex gap-2">
                    <a href="/login" class="text-sm text-zinc-400 hover:text-zinc-50 hover:bg-surface-card px-3 py-1.5 rounded-lg transition-all flex items-center gap-1.5">
                        <i class="ph ph-sign-in text-base"></i>"Sign In"
                    </a>
                </div>
                // Logged-in
                <div id="nav-user" class="hidden items-center gap-2">
                    // Notification bell
                    <div class="relative" id="notif-wrap">
                        <button onclick="toggleNotifs()" class="text-sm text-zinc-400 hover:text-zinc-50 hover:bg-surface-card p-1.5 rounded-lg transition-all relative">
                            <i class="ph ph-bell text-lg"></i>
                            <span id="notif-badge" class="hidden absolute -top-0.5 -right-0.5 w-4 h-4 bg-red-500 rounded-full text-[10px] text-white flex items-center justify-center font-bold"></span>
                        </button>
                        <div id="notif-dropdown" class="hidden absolute right-0 top-full mt-1 w-80 bg-surface-card border border-zinc-800 rounded-lg shadow-xl overflow-hidden z-50">
                            <div class="flex justify-between items-center px-3 py-2 border-b border-zinc-800">
                                <span class="text-xs font-semibold text-zinc-300">"Notifications"</span>
                                <button onclick="markAllRead()" class="text-xs text-accent hover:text-accent-hover">"Mark all read"</button>
                            </div>
                            <div id="notif-list" class="max-h-80 overflow-y-auto">
                                <p class="text-xs text-zinc-500 p-4 text-center">"No notifications"</p>
                            </div>
                        </div>
                    </div>
                    // Wallet with credit amount
                    <a href="/wallet" class="text-sm text-zinc-400 hover:text-zinc-50 hover:bg-surface-card px-3 py-1.5 rounded-lg transition-all flex items-center gap-1.5">
                        <i class="ph ph-wallet text-base"></i>
                        <span id="nav-credits" class="text-zinc-50 font-semibold text-base">"0"</span>
                        <span class="text-zinc-400 text-xs">"credits"</span>
                    </a>
                    <div class="w-px h-5 bg-zinc-800 mx-1"></div>
                    // User dropdown
                    <div class="relative" id="user-dropdown-wrap">
                        <button onclick="toggleDropdown()" id="user-dropdown-btn" class="text-sm text-zinc-300 hover:text-zinc-50 hover:bg-surface-card px-3 py-1.5 rounded-lg transition-all flex items-center gap-1.5 cursor-pointer">
                            <i class="ph ph-user-circle text-base"></i>
                            <span id="nav-username"></span>
                            <i class="ph ph-caret-down text-xs"></i>
                        </button>
                        <div id="user-dropdown" class="hidden absolute right-0 top-full mt-1 w-48 bg-surface-card border border-zinc-800 rounded-lg shadow-xl overflow-hidden z-50">
                            <a id="nav-profile-link" href="/profile" class="flex items-center gap-2 px-3 py-2.5 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                                <i class="ph ph-user text-base"></i>"Profile"
                            </a>
                            <a href="/library" class="flex items-center gap-2 px-3 py-2.5 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                                <i class="ph ph-books text-base"></i>"My Library"
                            </a>
                            <a href="/dashboard" class="flex items-center gap-2 px-3 py-2.5 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                                <i class="ph ph-chart-bar text-base"></i>"Dashboard"
                            </a>
                            <a id="nav-sell-link" href="/marketplace/sell" class="flex items-center gap-2 px-3 py-2.5 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                                <i class="ph ph-storefront text-base" id="nav-sell-icon"></i><span id="nav-sell-text">"Sell on Marketplace"</span>
                            </a>
                            <a href="/settings" class="flex items-center gap-2 px-3 py-2.5 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                                <i class="ph ph-gear text-base"></i>"Settings"
                            </a>
                            <div id="nav-admin-link" class="hidden">
                                <a href="/admin" class="flex items-center gap-2 px-3 py-2.5 text-sm text-amber-400 hover:text-amber-300 hover:bg-white/5 transition-all">
                                    <i class="ph ph-shield-check text-base"></i>"Admin Panel"
                                </a>
                            </div>
                            <div class="border-t border-zinc-800"></div>
                            <button onclick="handleLogout()" class="w-full flex items-center gap-2 px-3 py-2.5 text-sm text-red-400 hover:text-red-300 hover:bg-white/5 transition-all cursor-pointer">
                                <i class="ph ph-sign-out text-base"></i>"Sign Out"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </nav>

        <script>
            r#"
            function getCookie(name) {
                const v = document.cookie.match('(^|;)\\s*' + name + '\\s*=\\s*([^;]+)');
                return v ? v.pop() : null;
            }
            async function updateNav() {
                const userCookie = getCookie('user');
                const guest = document.getElementById('nav-guest');
                const user = document.getElementById('nav-user');
                const username = document.getElementById('nav-username');
                const adminLink = document.getElementById('nav-admin-link');
                if (userCookie && guest && user) {
                    try {
                        const u = JSON.parse(decodeURIComponent(userCookie));
                        guest.classList.add('hidden');
                        user.classList.remove('hidden');
                        user.classList.add('flex');
                        if (username) username.textContent = u.username;
                        const profileLink = document.getElementById('nav-profile-link');
                        if (profileLink) profileLink.href = '/profile/' + u.username;
                        if (adminLink && u.role === 'admin') {
                            adminLink.classList.remove('hidden');
                        }
                    } catch(e) {}

                    // Fetch live credit balance from API
                    try {
                        const t = getCookie('token');
                        if (t) {
                            const res = await fetch('/api/credits/balance', { headers: { 'Authorization': 'Bearer ' + t } });
                            if (res.ok) {
                                const data = await res.json();
                                const credits = document.getElementById('nav-credits');
                                if (credits) credits.textContent = (data.credit_balance ?? 0).toLocaleString();
                            }
                        }
                    } catch(e) {}
                }
            }
            function toggleDropdown() {
                const dd = document.getElementById('user-dropdown');
                dd.classList.toggle('hidden');
            }
            document.addEventListener('click', function(e) {
                const wrap = document.getElementById('user-dropdown-wrap');
                const dd = document.getElementById('user-dropdown');
                if (wrap && dd && !wrap.contains(e.target)) {
                    dd.classList.add('hidden');
                }
            });
            function handleLogout() {
                document.cookie = 'token=;path=/;max-age=0';
                document.cookie = 'refresh_token=;path=/;max-age=0';
                document.cookie = 'user=;path=/;max-age=0';
                window.location.href = '/';
            }
            function toggleNotifs() {
                const dd = document.getElementById('notif-dropdown');
                dd.classList.toggle('hidden');
                if (!dd.classList.contains('hidden')) loadNotifList();
            }
            document.addEventListener('click', function(e) {
                const wrap = document.getElementById('notif-wrap');
                const dd = document.getElementById('notif-dropdown');
                if (wrap && dd && !wrap.contains(e.target)) dd.classList.add('hidden');
            });
            async function loadNotifs() {
                const token = getCookie('token');
                if (!token) return;
                try {
                    const res = await fetch('/api/notifications/count', { headers: { 'Authorization': 'Bearer ' + token } });
                    if (!res.ok) return;
                    const data = await res.json();
                    const badge = document.getElementById('notif-badge');
                    if (badge && data.count > 0) {
                        badge.textContent = data.count > 9 ? '9+' : data.count;
                        badge.classList.remove('hidden');
                    }
                } catch(e) {}
            }
            async function loadNotifList() {
                const token = getCookie('token');
                if (!token) return;
                try {
                    const res = await fetch('/api/notifications/', { headers: { 'Authorization': 'Bearer ' + token } });
                    if (!res.ok) return;
                    const data = await res.json();
                    const el = document.getElementById('notif-list');
                    if (!data.notifications?.length) { el.innerHTML = '<p class=\"text-xs text-zinc-500 p-4 text-center\">No notifications</p>'; return; }
                    el.innerHTML = data.notifications.slice(0, 10).map(n => `
                        <a href="${n.link || '#'}" class="block px-3 py-2.5 hover:bg-white/5 transition-all border-b border-zinc-800/50 ${n.read ? '' : 'bg-accent/5'}">
                            <p class="text-xs font-medium ${n.read ? 'text-zinc-400' : 'text-zinc-50'}">${n.title}</p>
                            <p class="text-[11px] text-zinc-500 mt-0.5">${n.body}</p>
                        </a>
                    `).join('');
                } catch(e) {}
            }
            async function markAllRead() {
                const token = getCookie('token');
                if (!token) return;
                await fetch('/api/notifications/read-all', { method: 'PUT', headers: { 'Authorization': 'Bearer ' + token } });
                document.getElementById('notif-badge')?.classList.add('hidden');
                loadNotifList();
            }
            // WebSocket live updates
            let ws = null;
            function connectWs() {
                const token = getCookie('token');
                if (!token) return;
                const proto = location.protocol === 'https:' ? 'wss:' : 'ws:';
                ws = new WebSocket(proto + '//' + location.host + '/api/ws/live?token=' + token);
                ws.onmessage = function(e) {
                    try {
                        const msg = JSON.parse(e.data);
                        if (msg.event === 'notification') {
                            // Update notification badge
                            const badge = document.getElementById('notif-badge');
                            if (badge) {
                                const cur = parseInt(badge.textContent) || 0;
                                badge.textContent = cur + 1;
                                badge.classList.remove('hidden');
                            }
                        }
                        if (msg.event === 'credit_update') {
                            // Update credit display
                            const credits = document.getElementById('nav-credits');
                            if (credits) {
                                const cur = parseInt(credits.textContent) || 0;
                                credits.textContent = cur + (msg.data.amount || 0);
                            }
                        }
                        if (msg.event === 'new_post') {
                            // If viewing this thread, show a "new reply" banner
                            if (window.location.pathname.includes('/forum/thread/' + msg.data.thread_slug)) {
                                const banner = document.createElement('div');
                                banner.className = 'fixed bottom-4 right-4 bg-accent text-white px-4 py-2 rounded-lg text-sm cursor-pointer shadow-lg z-50';
                                banner.textContent = 'New reply — click to refresh';
                                banner.onclick = function() { window.location.reload(); };
                                document.body.appendChild(banner);
                                setTimeout(() => banner.remove(), 10000);
                            }
                        }
                    } catch(e) {}
                };
                ws.onclose = function() { setTimeout(connectWs, 5000); };
                ws.onerror = function() { ws.close(); };
            }

            // ── Auto-refresh tokens ──
            async function refreshSession() {
                const refreshToken = getCookie('refresh_token');
                if (!refreshToken) return;
                try {
                    const res = await fetch('/api/auth/refresh', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ refresh_token: refreshToken })
                    });
                    if (!res.ok) return;
                    const data = await res.json();
                    document.cookie = `token=${data.access_token};path=/;max-age=2592000;SameSite=Strict`;
                    document.cookie = `refresh_token=${data.refresh_token};path=/;max-age=2592000;SameSite=Strict`;
                    document.cookie = `user=${encodeURIComponent(JSON.stringify(data.user))};path=/;max-age=2592000;SameSite=Strict`;
                } catch(e) {}
            }

            // Check if access token is expiring soon (decode JWT exp)
            function tokenExpiresSoon() {
                const token = getCookie('token');
                if (!token) return false;
                try {
                    const payload = JSON.parse(atob(token.split('.')[1]));
                    const expiresIn = payload.exp - Math.floor(Date.now() / 1000);
                    return expiresIn < 86400; // refresh if less than 1 day left
                } catch(e) { return false; }
            }

            // Refresh on load if token is expiring soon, then periodically
            if (getCookie('token')) {
                if (tokenExpiresSoon()) refreshSession();
                setInterval(() => { if (tokenExpiresSoon()) refreshSession(); }, 3600000); // check hourly
            }

            // ── Global search ──
            let gsTimeout;
            function toggleGlobalSearch() {
                const panel = document.getElementById('global-search-panel');
                panel.classList.toggle('hidden');
                if (!panel.classList.contains('hidden')) {
                    document.getElementById('global-search-input').focus();
                }
            }
            document.addEventListener('click', function(e) {
                const wrap = document.getElementById('global-search-wrap');
                const panel = document.getElementById('global-search-panel');
                if (wrap && panel && !wrap.contains(e.target)) panel.classList.add('hidden');
            });
            document.addEventListener('keydown', function(e) {
                if (e.key === 'Escape') {
                    document.getElementById('global-search-panel')?.classList.add('hidden');
                }
                // Ctrl+K or Cmd+K to open search
                if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
                    e.preventDefault();
                    toggleGlobalSearch();
                }
            });

            async function globalSearch(query) {
                clearTimeout(gsTimeout);
                const el = document.getElementById('global-search-results');
                if (!query || query.trim().length < 2) {
                    el.innerHTML = '<div class="px-4 py-8 text-center text-xs text-zinc-600">Type to search across marketplace, users, and docs.</div>';
                    return;
                }
                gsTimeout = setTimeout(async () => {
                    el.innerHTML = '<div class="px-4 py-6 text-center"><div class="inline-block animate-spin w-4 h-4 border-2 border-zinc-700 border-t-accent rounded-full"></div></div>';
                    const q = encodeURIComponent(query.trim());
                    const [assetsRes, usersRes, docsRes] = await Promise.all([
                        fetch('/api/marketplace?q=' + q + '&page=1').then(r => r.ok ? r.json() : { assets: [] }).catch(() => ({ assets: [] })),
                        fetch('/api/profiles/search?q=' + q).then(r => r.ok ? r.json() : []).catch(() => []),
                        fetch('/api/docs/search?q=' + q).then(r => r.ok ? r.json() : []).catch(() => []),
                    ]);

                    const assets = (assetsRes.assets || []).slice(0, 5);
                    const users = (usersRes || []).slice(0, 5);
                    const docs = (docsRes || []).slice(0, 5);

                    if (!assets.length && !users.length && !docs.length) {
                        el.innerHTML = '<div class="px-4 py-8 text-center text-xs text-zinc-500">No results found.</div>';
                        return;
                    }

                    let html = '';

                    if (assets.length) {
                        html += '<div class="px-4 pt-3 pb-1"><span class="text-[10px] font-semibold uppercase tracking-wider text-zinc-500">Marketplace</span></div>';
                        html += assets.map(a => `
                            <a href="/marketplace/asset/${a.slug}" class="flex items-center gap-3 px-4 py-2.5 hover:bg-white/[0.03] transition-all">
                                <div class="w-8 h-8 rounded-lg bg-surface-panel border border-zinc-800/50 flex items-center justify-center shrink-0 overflow-hidden">
                                    ${a.thumbnail_url ? `<img src="${a.thumbnail_url}" class="w-full h-full object-cover" />` : `<i class="ph ph-package text-sm text-zinc-600"></i>`}
                                </div>
                                <div class="flex-1 min-w-0">
                                    <div class="text-sm text-zinc-200 truncate">${a.name}</div>
                                    <div class="text-[11px] text-zinc-600">${a.category} · ${a.price_credits === 0 ? 'Free' : a.price_credits + ' cr'}</div>
                                </div>
                                ${a.rating_count > 0 ? `<span class="text-[11px] text-amber-400">${'★'.repeat(Math.round(a.rating_avg))}</span>` : ''}
                            </a>
                        `).join('');
                    }

                    if (users.length) {
                        html += '<div class="px-4 pt-3 pb-1 border-t border-zinc-800/50"><span class="text-[10px] font-semibold uppercase tracking-wider text-zinc-500">Users</span></div>';
                        html += users.map(u => `
                            <a href="/profile/${u.username}" class="flex items-center gap-3 px-4 py-2.5 hover:bg-white/[0.03] transition-all">
                                <div class="w-8 h-8 rounded-full bg-surface-panel border border-zinc-800/50 flex items-center justify-center shrink-0 overflow-hidden">
                                    ${u.avatar_url ? `<img src="${u.avatar_url}" class="w-full h-full object-cover" />` : `<i class="ph ph-user text-sm text-zinc-600"></i>`}
                                </div>
                                <div class="flex-1 min-w-0">
                                    <div class="text-sm text-zinc-200">${u.username}</div>
                                    <div class="text-[11px] text-zinc-600">${u.role}</div>
                                </div>
                            </a>
                        `).join('');
                    }

                    if (docs.length) {
                        html += '<div class="px-4 pt-3 pb-1 border-t border-zinc-800/50"><span class="text-[10px] font-semibold uppercase tracking-wider text-zinc-500">Documentation</span></div>';
                        html += docs.map(d => `
                            <a href="/docs/${d.slug}" class="flex items-center gap-3 px-4 py-2.5 hover:bg-white/[0.03] transition-all">
                                <div class="w-8 h-8 rounded-lg bg-accent/10 flex items-center justify-center shrink-0">
                                    <i class="ph ph-book-open text-sm text-accent"></i>
                                </div>
                                <div class="flex-1 min-w-0">
                                    <div class="text-sm text-zinc-200">${d.title}</div>
                                    <div class="text-[11px] text-zinc-600">${d.section} · ${d.category}</div>
                                </div>
                            </a>
                        `).join('');
                    }

                    // View all link
                    html += `<div class="px-4 py-3 border-t border-zinc-800/50">
                        <a href="/marketplace?q=${q}" class="text-xs text-accent hover:text-accent-hover transition-colors">View all marketplace results →</a>
                    </div>`;

                    el.innerHTML = html;
                }, 250);
            }

            // Check creator onboard status to swap sell/dashboard link
            async function checkCreatorStatus() {
                const t = getCookie('token');
                if (!t) return;
                try {
                    const res = await fetch('/api/creator/onboard-status', { headers: { 'Authorization': 'Bearer ' + t } });
                    if (!res.ok) return;
                    const data = await res.json();
                    if (data.policy_accepted) {
                        const link = document.getElementById('nav-sell-link');
                        const icon = document.getElementById('nav-sell-icon');
                        const text = document.getElementById('nav-sell-text');
                        if (link) { link.href = '/dashboard'; }
                        if (icon) { icon.className = 'ph ph-chart-pie text-base'; }
                        if (text) { text.textContent = 'Creator Dashboard'; }
                    }
                } catch(e) {}
            }

            updateNav();
            loadNotifs(); // Initial load only
            connectWs();  // Live updates from here on
            checkCreatorStatus();
            "#
        </script>
    }
}
