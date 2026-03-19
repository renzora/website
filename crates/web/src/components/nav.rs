use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="sticky top-0 z-50 bg-[rgba(10,10,11,0.8)] backdrop-blur-xl border-b border-zinc-800">
            <div class="max-w-[1200px] mx-auto px-6 h-14 flex items-center gap-8">
                <a href="/" class="text-lg font-bold tracking-tight">"Renzora"</a>
                <div class="flex gap-6 flex-1">
                    <a href="/marketplace" class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors flex items-center gap-1.5">
                        <i class="ph ph-storefront text-base"></i>"Marketplace"
                    </a>
                    <a href="/forum" class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors flex items-center gap-1.5">
                        <i class="ph ph-chat-circle text-base"></i>"Forum"
                    </a>
                    <a href="/community" class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors flex items-center gap-1.5">
                        <i class="ph ph-pencil-line text-base"></i>"Articles"
                    </a>
                    <a href="/engine" class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors flex items-center gap-1.5">
                        <i class="ph ph-rocket-launch text-base"></i>"Engine"
                    </a>
                    <a href="/docs" class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors flex items-center gap-1.5">
                        <i class="ph ph-book-open text-base"></i>"Docs"
                    </a>
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
                            <a href="/dashboard" class="flex items-center gap-2 px-3 py-2.5 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                                <i class="ph ph-chart-bar text-base"></i>"Dashboard"
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
            function updateNav() {
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
                        const credits = document.getElementById('nav-credits');
                        if (credits) credits.textContent = u.credit_balance || '0';
                        if (adminLink && u.role === 'admin') {
                            adminLink.classList.remove('hidden');
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

            updateNav();
            loadNotifs(); // Initial load only
            connectWs();  // Live updates from here on
            "#
        </script>
    }
}
