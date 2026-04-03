use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav id="main-nav" class="sticky top-0 z-50 transition-all duration-300" style="text-shadow: 0 1px 4px rgba(0,0,0,0.5)">
            <div class="max-w-[1400px] mx-auto w-full px-6 h-14 flex items-center gap-6">
                // Logo
                <a href="/" class="text-lg font-bold tracking-tight text-white hover:text-accent transition-colors">"Renzora"</a>

                // Nav links
                <div class="flex gap-1 flex-1" id="nav-links">
                    <a href="/download" class="nav-link text-base text-zinc-400 hover:text-white hover:bg-white/[0.06] px-3.5 py-1.5 rounded-lg transition-all flex items-center gap-1.5" data-path="/download">
                        <i class="ph ph-download-simple text-lg"></i>"Engine"
                    </a>
                    <a href="/marketplace" class="nav-link text-base text-zinc-400 hover:text-white hover:bg-white/[0.06] px-3.5 py-1.5 rounded-lg transition-all flex items-center gap-1.5" data-path="/marketplace">
                        <i class="ph ph-storefront text-lg"></i>"Marketplace"
                    </a>
                    <a href="/games" class="nav-link text-base text-zinc-400 hover:text-white hover:bg-white/[0.06] px-3.5 py-1.5 rounded-lg transition-all flex items-center gap-1.5" data-path="/games">
                        <i class="ph ph-game-controller text-lg"></i>"Games"
                    </a>
                    <a href="/feed" class="nav-link text-base text-zinc-400 hover:text-white hover:bg-white/[0.06] px-3.5 py-1.5 rounded-lg transition-all flex items-center gap-1.5" data-path="/feed">
                        <i class="ph ph-users text-lg"></i>"Social"
                    </a>
                    <a href="/docs" class="nav-link text-base text-zinc-400 hover:text-white hover:bg-white/[0.06] px-3.5 py-1.5 rounded-lg transition-all flex items-center gap-1.5" data-path="/docs">
                        <i class="ph ph-book-open text-lg"></i>"Docs"
                    </a>
                </div>

                // XP bar (logged in only)
                <div id="nav-xp-wrap" class="hidden items-center gap-2">
                    <div class="flex items-center gap-2">
                        <div class="relative flex items-center">
                            // Level badge
                            <div class="w-8 h-8 rounded-lg flex items-center justify-center z-10">
                                <span id="nav-level" class="text-[11px] font-black text-accent">"1"</span>
                            </div>
                            // XP bar
                            <div class="w-28 h-4 -ml-1 bg-black/40 border border-white/[0.08] rounded-r-lg overflow-hidden shadow-inner">
                                <div id="nav-xp-bar" class="h-full bg-gradient-to-r from-accent to-purple-500 rounded-r-lg transition-all relative" style="width:0%">
                                    <div class="absolute inset-0 bg-[linear-gradient(90deg,transparent_25%,rgba(255,255,255,0.15)_50%,transparent_75%)] bg-[length:200%_100%] animate-[xpShimmer_2s_linear_infinite]"></div>
                                </div>
                            </div>
                        </div>
                        <span id="nav-xp-text" class="text-[10px] text-zinc-500 font-medium">"0 XP"</span>
                    </div>
                </div>

                // Search
                <div class="relative" id="global-search-wrap">
                    <button onclick="toggleGlobalSearch()" class="text-zinc-400 hover:text-white p-2 rounded-lg hover:bg-white/[0.06] transition-all" title="Search (Ctrl+K)">
                        <i class="ph ph-magnifying-glass text-lg"></i>
                    </button>
                    <div id="global-search-panel" class="hidden absolute right-0 top-full mt-2 w-[420px] bg-[rgba(10,10,16,0.95)] backdrop-blur-2xl border border-white/[0.08] rounded-xl shadow-2xl shadow-black/60 overflow-hidden z-50">
                        <div class="flex items-center gap-2 px-4 py-3 border-b border-white/[0.06]">
                            <i class="ph ph-magnifying-glass text-zinc-500"></i>
                            <input type="text" id="global-search-input" placeholder="Search assets, users, docs..." oninput="globalSearch(this.value)" class="flex-1 bg-transparent text-sm text-zinc-50 outline-none placeholder:text-zinc-600" />
                            <kbd class="text-[10px] text-zinc-600 border border-white/[0.08] rounded px-1.5 py-0.5">"Esc"</kbd>
                        </div>
                        <div id="global-search-results" class="max-h-[400px] overflow-y-auto">
                            <div class="px-4 py-8 text-center text-xs text-zinc-600">"Type to search across marketplace, users, and docs."</div>
                        </div>
                    </div>
                </div>

                // Logged-out
                <div id="nav-guest" class="flex gap-2">
                    <a id="nav-signin-link" href="/login" class="text-sm text-zinc-300 hover:text-white bg-accent/80 hover:bg-accent px-4 py-1.5 rounded-lg transition-all flex items-center gap-1.5">
                        <i class="ph ph-sign-in text-base"></i>"Sign In"
                    </a>
                </div>

                // Logged-in
                <div id="nav-user" class="hidden items-center gap-2">
                    // Messages
                    <a href="/messages" class="relative p-2 rounded-lg hover:bg-white/[0.06] transition-colors" title="Messages">
                        <i class="ph ph-chat-circle-dots text-lg text-zinc-400 hover:text-zinc-200"></i>
                        <span id="msg-badge" class="hidden absolute -top-0.5 -right-0.5 w-4 h-4 bg-accent rounded-full text-[9px] font-bold text-white flex items-center justify-center"></span>
                    </a>
                    // Notifications
                    <div class="relative" id="notif-wrap">
                        <button onclick="toggleNotifs()" class="text-zinc-400 hover:text-white p-2 rounded-lg hover:bg-white/[0.06] transition-all relative">
                            <i class="ph ph-bell text-lg"></i>
                            <span id="notif-badge" class="hidden absolute -top-0.5 -right-0.5 w-4 h-4 bg-red-500 rounded-full text-[9px] text-white flex items-center justify-center font-bold"></span>
                        </button>
                        <div id="notif-dropdown" class="hidden absolute right-0 top-full mt-2 w-80 bg-[rgba(10,10,16,0.95)] backdrop-blur-2xl border border-white/[0.08] rounded-xl shadow-2xl overflow-hidden z-50">
                            <div class="flex justify-between items-center px-3 py-2.5 border-b border-white/[0.06]">
                                <span class="text-xs font-semibold text-zinc-300">"Notifications"</span>
                                <button onclick="markAllRead()" class="text-xs text-accent hover:text-accent-hover">"Mark all read"</button>
                            </div>
                            <div id="notif-list" class="max-h-80 overflow-y-auto">
                                <p class="text-xs text-zinc-500 p-4 text-center">"No notifications"</p>
                            </div>
                        </div>
                    </div>
                    // Credits
                    <a href="/wallet" class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-white/[0.04] border border-white/[0.05] hover:bg-white/[0.08] transition-all">
                        <i class="ph ph-coins text-sm text-amber-400"></i>
                        <span id="nav-credits" class="text-sm text-white font-semibold">"0"</span>
                    </a>
                    // User
                    <div class="relative" id="user-dropdown-wrap">
                        <button onclick="toggleDropdown()" id="user-dropdown-btn" class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-white/[0.04] border border-white/[0.05] hover:bg-white/[0.08] transition-all cursor-pointer">
                            <i class="ph ph-user-circle text-base text-zinc-300"></i>
                            <span id="nav-username" class="text-sm text-zinc-200"></span>
                            <i class="ph ph-caret-down text-xs text-zinc-500"></i>
                        </button>
                        <div id="user-dropdown" class="hidden absolute right-0 top-full mt-2 w-52 bg-[rgba(10,10,16,0.95)] backdrop-blur-2xl border border-white/[0.08] rounded-xl shadow-2xl overflow-hidden z-50 py-1">
                            <a id="nav-profile-link" href="/profile" class="flex items-center gap-2.5 px-4 py-2.5 text-sm text-zinc-400 hover:text-white hover:bg-white/[0.06] transition-all">
                                <i class="ph ph-user text-base"></i>"Profile"
                            </a>
                            <a href="/library" class="flex items-center gap-2.5 px-4 py-2.5 text-sm text-zinc-400 hover:text-white hover:bg-white/[0.06] transition-all">
                                <i class="ph ph-books text-base"></i>"My Library"
                            </a>
                            <a id="nav-sell-link" href="/marketplace/sell" class="flex items-center gap-2.5 px-4 py-2.5 text-sm text-zinc-400 hover:text-white hover:bg-white/[0.06] transition-all">
                                <i class="ph ph-storefront text-base" id="nav-sell-icon"></i><span id="nav-sell-text">"Sell on Marketplace"</span>
                            </a>
                            <a href="/teams" class="flex items-center gap-2.5 px-4 py-2.5 text-sm text-zinc-400 hover:text-white hover:bg-white/[0.06] transition-all">
                                <i class="ph ph-users-three text-base"></i>"Teams"
                            </a>
                            <a href="/subscription" class="flex items-center gap-2.5 px-4 py-2.5 text-sm text-zinc-400 hover:text-white hover:bg-white/[0.06] transition-all">
                                <i class="ph ph-crown text-base"></i>"Subscription"
                            </a>
                            <a href="/developers" class="flex items-center gap-2.5 px-4 py-2.5 text-sm text-zinc-400 hover:text-white hover:bg-white/[0.06] transition-all">
                                <i class="ph ph-code text-base"></i>"Developers"
                            </a>
                            <a href="/settings" class="flex items-center gap-2.5 px-4 py-2.5 text-sm text-zinc-400 hover:text-white hover:bg-white/[0.06] transition-all">
                                <i class="ph ph-gear text-base"></i>"Settings"
                            </a>
                            <div class="border-t border-white/[0.06] my-1"></div>
                            <button onclick="handleLogout()" class="w-full flex items-center gap-2.5 px-4 py-2.5 text-sm text-red-400 hover:text-red-300 hover:bg-white/[0.06] transition-all cursor-pointer">
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
                if (userCookie && guest && user) {
                    try {
                        const u = JSON.parse(decodeURIComponent(userCookie));
                        guest.classList.add('hidden');
                        user.classList.remove('hidden');
                        user.classList.add('flex');
                        if (username) username.textContent = u.username;
                        const profileLink = document.getElementById('nav-profile-link');
                        if (profileLink) profileLink.href = '/profile/' + u.username;
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
                        if (msg.event === 'new_message') {
                            var msgBadge = document.getElementById('msg-badge');
                            if (msgBadge) {
                                var current = parseInt(msgBadge.textContent) || 0;
                                msgBadge.textContent = current + 1;
                                msgBadge.classList.remove('hidden');
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
                                    <div class="text-[11px] text-zinc-600">${a.category} · ${a.price_credits === 0 ? 'Free' : a.price_credits + ' credits'}</div>
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

            // Highlight active nav link
            (function() {
                const path = window.location.pathname;
                document.querySelectorAll('.nav-link').forEach(link => {
                    const linkPath = link.getAttribute('data-path');
                    if (path === linkPath || path.startsWith(linkPath + '/')) {
                        link.classList.remove('text-zinc-400');
                        link.classList.add('text-white', 'bg-white/[0.06]');
                    }
                });
                // Highlight parent for child pages
                const parents = { '/games': '/games', '/courses': '/marketplace', '/forum': '/feed', '/community': '/feed', '/messages': '/feed', '/developers': '/docs' };
                for (const [sub, parent] of Object.entries(parents)) {
                    if (path === sub || path.startsWith(sub + '/')) {
                        const parentLink = document.querySelector(`.nav-link[data-path="${parent}"]`);
                        if (parentLink) { parentLink.classList.remove('text-zinc-400'); parentLink.classList.add('text-white', 'bg-white/[0.06]'); }
                    }
                }
            })();

            // Set redirect param on sign in link
            const signinLink = document.getElementById('nav-signin-link');
            if (signinLink && window.location.pathname !== '/login' && window.location.pathname !== '/register') {
                signinLink.href = '/login?redirect=' + encodeURIComponent(window.location.pathname + window.location.search);
            }

            updateNav();
            loadNotifs();

            // Load XP bar
            (async function() {
                const t = getCookie('token');
                if (!t) return;
                try {
                    const res = await fetch('/api/levels/me', { headers: { 'Authorization': 'Bearer ' + t } });
                    if (!res.ok) return;
                    const d = await res.json();
                    document.getElementById('nav-xp-wrap')?.classList.remove('hidden');
                    document.getElementById('nav-xp-wrap')?.classList.add('flex');
                    document.getElementById('nav-level').textContent = d.level;
                    document.getElementById('nav-xp-bar').style.width = d.progress_percent.toFixed(0) + '%';
                    document.getElementById('nav-xp-text').textContent = d.total_xp.toLocaleString() + ' XP';
                } catch(e) {}
            })();

            // Message unread count
            fetch('/api/messages/unread-count', { headers: { 'Authorization': 'Bearer ' + getCookie('token') } })
                .then(function(r) { return r.json(); })
                .then(function(data) {
                    var badge = document.getElementById('msg-badge');
                    if (badge && data.count > 0) {
                        badge.textContent = data.count > 9 ? '9+' : data.count;
                        badge.classList.remove('hidden');
                    }
                }).catch(function() {});
            connectWs();  // Live updates from here on
            checkCreatorStatus();

            // ── Nav scroll effect — transparent at top, glass on scroll ──
            let navScrolled = false;
            const nav = document.getElementById('main-nav');
            window.addEventListener('scroll', function() {
                if (window.scrollY > 20 && !navScrolled) {
                    navScrolled = true;
                    nav.style.background = 'rgba(6,6,8,0.75)';
                    nav.style.backdropFilter = 'blur(24px)';
                    nav.style.webkitBackdropFilter = 'blur(24px)';
                } else if (window.scrollY <= 20 && navScrolled) {
                    navScrolled = false;
                    nav.style.background = 'transparent';
                    nav.style.backdropFilter = 'none';
                    nav.style.webkitBackdropFilter = 'none';
                }
            });
            "#
        </script>

        <style>
            r#"
            @keyframes xpShimmer {
                0% { background-position: -200% 0; }
                100% { background-position: 200% 0; }
            }
            "#
        </style>
    }
}
