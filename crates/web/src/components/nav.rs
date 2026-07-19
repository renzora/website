use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        // ── Fixed left sidebar ──
        <aside id="app-sidebar">
            // Nav links (top)
            <nav class="shrink-0 px-3 py-4">
                <div class="space-y-0.5">
                    <a href="/" class="side-link nav-link" data-path="/">
                        <i class="ph ph-house text-lg"></i>"Home"
                    </a>
                    <a href="/download" class="side-link nav-link" data-path="/download">
                        <i class="ph ph-download-simple text-lg"></i>"Download Engine"
                    </a>
                    <a href="/marketplace" class="side-link nav-link" data-path="/marketplace">
                        <i class="ph ph-storefront text-lg"></i>"Marketplace"
                    </a>
                    <a href="/community" class="side-link nav-link" data-path="/community">
                        <i class="ph ph-users-three text-lg"></i>"Community"
                    </a>
                    <a href="/docs" class="side-link nav-link" data-path="/docs">
                        <i class="ph ph-book-open text-lg"></i>"Docs"
                    </a>
                    <a href="/donate" class="side-link nav-link" data-path="/donate">
                        <i class="ph ph-heart text-lg"></i>"Donate"
                    </a>
                </div>
            </nav>

            // Cards, centered in the space between the nav and the footer
            <div class="flex-1 flex flex-col justify-center px-3 space-y-2">
                // Sign in / register box (logged out)
                <div id="nav-side-guest" class="rounded-xl p-3 bg-white/[0.03] border border-white/[0.07]">
                    <p class="text-[9px] font-semibold uppercase tracking-[0.16em] text-zinc-400">"Join the conversation"</p>
                    <form class="mt-2 space-y-1.5" onsubmit="return sideLogin(event)">
                        <input type="email" name="email" required placeholder="Email" autocomplete="email" class="w-full px-2.5 py-1.5 bg-black/30 border border-white/[0.08] rounded-lg text-zinc-50 text-xs outline-none focus:border-accent/50 transition-all placeholder:text-zinc-600" />
                        <input type="password" name="password" required placeholder="Password" autocomplete="current-password" class="w-full px-2.5 py-1.5 bg-black/30 border border-white/[0.08] rounded-lg text-zinc-50 text-xs outline-none focus:border-accent/50 transition-all placeholder:text-zinc-600" />
                        <p id="nav-side-login-err" class="hidden text-[10px] text-red-400 leading-snug"></p>
                        <button type="submit" id="nav-side-login-btn" class="w-full text-xs font-semibold text-white bg-purple-600 hover:bg-purple-500 transition-colors rounded-lg py-1.5">"Sign In"</button>
                    </form>
                    <a href="/register" class="mt-2 block text-center text-[11px] text-zinc-500 hover:text-zinc-300 transition-colors">"New here? "<span class="text-accent font-semibold">"Create an account"</span></a>
                </div>

                // Engine download card
                <div class="rounded-xl p-3 bg-gradient-to-br from-accent/[0.15] to-secondary/[0.08] border border-white/[0.07]">
                    <p class="text-[9px] font-semibold uppercase tracking-[0.16em] text-zinc-400">"Renzora Engine"</p>
                    <div class="flex items-center gap-1.5 mt-1.5">
                        <i class="ph ph-download-simple text-secondary text-sm"></i>
                        <span class="text-sm font-semibold text-white">"r1-alpha6"</span>
                        <span class="text-[9px] font-semibold px-1.5 py-0.5 rounded bg-emerald-500/15 text-emerald-400">"Free"</span>
                    </div>
                    <a href="/download" class="mt-2.5 block text-center text-xs font-semibold text-white bg-purple-600 hover:bg-purple-500 transition-colors rounded-lg py-1.5">"Download"</a>
                </div>

                // Credits card (logged in)
                <a href="/wallet" id="nav-side-credits" class="hidden rounded-xl p-3 bg-white/[0.03] border border-white/[0.07] hover:bg-white/[0.05] transition-colors">
                    <p class="text-[9px] font-semibold uppercase tracking-[0.16em] text-zinc-500">"Your Credits"</p>
                    <div class="flex items-center gap-1.5 mt-1.5">
                        <i class="ph ph-coin text-amber-400 text-base"></i>
                        <span id="nav-credits-side" class="text-lg font-bold text-white">"0"</span>
                    </div>
                    <span class="mt-2.5 block text-center text-xs font-semibold text-white bg-amber-500/90 hover:bg-amber-500 transition-colors rounded-lg py-1.5">"Get more Credits"</span>
                </a>
            </div>

            // Footer — social links + legal, pinned to the bottom
            <footer class="shrink-0 px-3 py-3 space-y-2.5">
                <div class="flex items-center justify-center gap-1">
                    <a href="https://youtube.com/@renzoragame" target="_blank" rel="noopener noreferrer" aria-label="YouTube" class="p-2 rounded-lg text-zinc-500 hover:text-white hover:bg-white/[0.06] transition-all"><i class="ph ph-youtube-logo text-lg"></i></a>
                    <a href="https://github.com/renzora" target="_blank" rel="noopener noreferrer" aria-label="GitHub" class="p-2 rounded-lg text-zinc-500 hover:text-white hover:bg-white/[0.06] transition-all"><i class="ph ph-github-logo text-lg"></i></a>
                    <a href="https://discord.gg/9UHUGUyDJv" target="_blank" rel="noopener noreferrer" aria-label="Discord" class="p-2 rounded-lg text-zinc-500 hover:text-white hover:bg-white/[0.06] transition-all"><i class="ph ph-discord-logo text-lg"></i></a>
                </div>
                <div class="flex flex-wrap items-center justify-center gap-x-1.5 gap-y-1 text-[11px] text-zinc-600">
                    <a href="/privacy" class="hover:text-zinc-300 transition-colors">"Privacy"</a>
                    <span class="text-zinc-700">"·"</span>
                    <a href="/terms" class="hover:text-zinc-300 transition-colors">"Terms"</a>
                    <span class="text-zinc-700">"·"</span>
                    <span class="text-zinc-600">"© 2026 Renzora"</span>
                </div>
            </footer>
        </aside>

        // ── Mobile scrim ──
        <div id="sidebar-scrim" onclick="toggleSidebar()"></div>

        // ── Fixed top header ──
        <header id="app-header">
            // Mobile hamburger
            <button id="sidebar-burger" onclick="toggleSidebar()" aria-label="Open navigation menu" class="text-zinc-400 hover:text-white p-1.5 -ml-1 rounded-lg hover:bg-white/[0.06] transition-all">
                <i class="ph ph-list text-xl"></i>
            </button>

            // Page brand — mirrors the sidebar logo (icon + renzora / Game Engine)
            <a href="/" class="flex items-center gap-2.5 min-w-0">
                <div class="w-9 h-9 rounded-xl bg-gradient-to-br from-accent to-secondary flex items-center justify-center shadow-lg shadow-accent/20 shrink-0">
                    <i class="ph ph-star text-white text-lg"></i>
                </div>
                <div class="leading-none">
                    <div class="text-[15px] font-bold text-white tracking-tight">"renzora"</div>
                    <div class="text-[9px] font-semibold uppercase tracking-[0.18em] text-zinc-400 mt-1">"Game Engine"</div>
                </div>
            </a>

            <div class="flex-1"></div>

            // XP bar (logged in only)
            <div id="nav-xp-wrap" class="hidden items-center gap-2 mr-1">
                <div class="relative flex items-center">
                    <div class="w-7 h-7 rounded-lg flex items-center justify-center z-10">
                        <span id="nav-level" class="text-[11px] font-black text-accent">"1"</span>
                    </div>
                    <div class="w-24 h-3.5 -ml-1 bg-black/40 border border-white/[0.08] rounded-r-lg overflow-hidden shadow-inner">
                        <div id="nav-xp-bar" class="h-full bg-gradient-to-r from-accent to-secondary rounded-r-lg transition-all relative" style="width:0%">
                            <div class="absolute inset-0 bg-[linear-gradient(90deg,transparent_25%,rgba(255,255,255,0.15)_50%,transparent_75%)] bg-[length:200%_100%] animate-[xpShimmer_2s_linear_infinite]"></div>
                        </div>
                    </div>
                </div>
                <span id="nav-xp-text" class="text-[10px] text-zinc-500 font-medium hidden md:block">"0 XP"</span>
            </div>

            // Search
            <div class="relative" id="global-search-wrap">
                <button onclick="toggleGlobalSearch()" class="text-zinc-400 hover:text-white p-2 rounded-lg hover:bg-white/[0.06] transition-all" title="Search (Ctrl+K)">
                    <i class="ph ph-magnifying-glass text-lg"></i>
                </button>
                <div id="global-search-panel" class="hidden absolute right-0 top-full mt-2 w-[420px] max-w-[90vw] bg-[rgba(12,7,21,0.95)] backdrop-blur-2xl border border-white/[0.08] rounded-xl shadow-2xl shadow-black/60 overflow-hidden z-50">
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
                <a id="nav-signin-link" href="/login" class="text-sm text-white bg-purple-600 hover:bg-purple-500 px-4 py-1.5 rounded-lg transition-all flex items-center gap-1.5">
                    <i class="ph ph-sign-in text-base"></i>"Sign In"
                </a>
            </div>

            // Logged-in
            <div id="nav-user" class="hidden items-center gap-2">
                // Credits
                <a href="/wallet" class="flex items-center gap-1.5 px-3 py-1.5 rounded-full bg-amber-500/10 border border-amber-500/20 hover:bg-amber-500/20 transition-all">
                    <i class="ph ph-coin text-sm text-amber-400"></i>
                    <span id="nav-credits" class="text-sm text-white font-semibold">"0"</span>
                </a>
                // Messages
                <a href="/messages" class="relative p-2 rounded-lg hover:bg-white/[0.06] transition-colors" title="Messages">
                    <i class="ph ph-chat-circle-dots text-lg text-zinc-400 hover:text-zinc-200"></i>
                    <span id="msg-badge" class="hidden absolute top-0 right-0 min-w-[16px] h-4 px-1 bg-accent rounded-full text-[9px] font-bold text-white flex items-center justify-center"></span>
                </a>
                // Notifications
                <div class="relative" id="notif-wrap">
                    <button onclick="toggleNotifs()" class="text-zinc-400 hover:text-white p-2 rounded-lg hover:bg-white/[0.06] transition-all relative">
                        <i class="ph ph-bell text-lg"></i>
                        <span id="notif-badge" class="hidden absolute top-0 right-0 min-w-[16px] h-4 px-1 bg-red-500 rounded-full text-[9px] text-white flex items-center justify-center font-bold"></span>
                    </button>
                    <div id="notif-dropdown" class="hidden absolute right-0 top-full mt-2 w-80 max-w-[90vw] bg-[rgba(12,7,21,0.95)] backdrop-blur-2xl border border-white/[0.08] rounded-xl shadow-2xl overflow-hidden z-50">
                        <div class="flex justify-between items-center px-3 py-2.5 border-b border-white/[0.06]">
                            <span class="text-xs font-semibold text-zinc-300">"Notifications"</span>
                            <button onclick="markAllRead()" class="text-xs text-accent hover:text-accent-hover">"Mark all read"</button>
                        </div>
                        <div id="notif-list" class="max-h-80 overflow-y-auto">
                            <p class="text-xs text-zinc-500 p-4 text-center">"No notifications"</p>
                        </div>
                        <a href="/notifications" class="block px-3 py-2.5 text-center text-xs text-accent hover:text-accent-hover border-t border-white/[0.06]">"See all notifications"</a>
                    </div>
                </div>
                // User
                <div class="relative" id="user-dropdown-wrap">
                    <button onclick="toggleDropdown()" id="user-dropdown-btn" class="flex items-center gap-1.5 pl-1 pr-2 py-1 rounded-full bg-white/[0.04] border border-white/[0.06] hover:bg-white/[0.08] transition-all cursor-pointer">
                        <span class="w-7 h-7 rounded-full bg-gradient-to-br from-accent to-secondary flex items-center justify-center text-white text-xs font-bold" id="nav-avatar-initial"></span>
                        <span id="nav-username" class="text-sm text-zinc-200 hidden sm:block"></span>
                        <i class="ph ph-caret-down text-xs text-zinc-500"></i>
                    </button>
                    <div id="user-dropdown" class="hidden absolute right-0 top-full mt-2 w-52 bg-[rgba(12,7,21,0.95)] backdrop-blur-2xl border border-white/[0.08] rounded-xl shadow-2xl overflow-hidden z-50 py-1">
                        <a id="nav-profile-link" href="/profile" class="flex items-center gap-2.5 px-4 py-2.5 text-sm text-zinc-400 hover:text-white hover:bg-white/[0.06] transition-all">
                            <i class="ph ph-user text-base"></i>"Profile"
                        </a>
                        <a href="/library" class="flex items-center gap-2.5 px-4 py-2.5 text-sm text-zinc-400 hover:text-white hover:bg-white/[0.06] transition-all">
                            <i class="ph ph-books text-base"></i>"My Library"
                        </a>
                        <a href="/friends" class="flex items-center gap-2.5 px-4 py-2.5 text-sm text-zinc-400 hover:text-white hover:bg-white/[0.06] transition-all">
                            <i class="ph ph-user-plus text-base"></i>"Friends"
                        </a>
                        <a id="nav-sell-link" href="/marketplace/sell" class="flex items-center gap-2.5 px-4 py-2.5 text-sm text-zinc-400 hover:text-white hover:bg-white/[0.06] transition-all">
                            <i class="ph ph-storefront text-base" id="nav-sell-icon"></i><span id="nav-sell-text">"Sell on Marketplace"</span>
                        </a>
                        <a href="/teams" class="flex items-center gap-2.5 px-4 py-2.5 text-sm text-zinc-400 hover:text-white hover:bg-white/[0.06] transition-all">
                            <i class="ph ph-users-three text-base"></i>"Teams"
                        </a>
                        <a href="/subscription" class="flex items-center gap-2.5 px-4 py-2.5 text-sm text-zinc-400 hover:text-white hover:bg-white/[0.06] transition-all">
                            <i class="ph ph-heart text-base"></i>"Donate to Renzora"
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
        </header>


        <script>
            r#"
            function toggleSidebar() {
                const sb = document.getElementById('app-sidebar');
                const scrim = document.getElementById('sidebar-scrim');
                const open = sb.classList.toggle('open');
                if (scrim) scrim.classList.toggle('open', open);
            }
            function getCookie(name) {
                const v = document.cookie.match('(^|;)\\s*' + name + '\\s*=\\s*([^;]+)');
                return v ? v.pop() : null;
            }
            async function updateNav() {
                const userCookie = getCookie('user');
                const guest = document.getElementById('nav-guest');
                const user = document.getElementById('nav-user');
                const username = document.getElementById('nav-username');
                const sideCredits = document.getElementById('nav-side-credits');
                const sideGuest = document.getElementById('nav-side-guest');
                if (userCookie && guest && user) {
                    try {
                        const u = JSON.parse(decodeURIComponent(userCookie));
                        guest.classList.add('hidden');
                        user.classList.remove('hidden');
                        user.classList.add('flex');
                        if (username) username.textContent = u.username;
                        const initial = document.getElementById('nav-avatar-initial');
                        if (initial && u.username) initial.textContent = u.username.charAt(0).toUpperCase();
                        const profileLink = document.getElementById('nav-profile-link');
                        if (profileLink) profileLink.href = '/profile/' + u.username;
                        if (sideCredits) { sideCredits.classList.remove('hidden'); sideCredits.classList.add('block'); }
                        if (sideGuest) { sideGuest.classList.add('hidden'); }
                    } catch(e) {}
                    // Credits, badges, XP and creator status are loaded together by loadUserSummary().
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
            // Inline sidebar sign-in — mirrors the /login flow, then reloads in place.
            async function sideLogin(e) {
                e.preventDefault();
                const form = e.target;
                const err = document.getElementById('nav-side-login-err');
                const btn = document.getElementById('nav-side-login-btn');
                err.classList.add('hidden');
                btn.disabled = true; btn.textContent = 'Signing in...';
                try {
                    const res = await fetch('/api/auth/login', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ email: form.email.value, password: form.password.value })
                    });
                    const data = await res.json();
                    if (!res.ok) throw new Error(data.error || 'Invalid email or password');
                    document.cookie = `token=${data.access_token};path=/;max-age=2592000;SameSite=Strict`;
                    document.cookie = `refresh_token=${data.refresh_token};path=/;max-age=2592000;SameSite=Strict`;
                    document.cookie = `user=${encodeURIComponent(JSON.stringify(data.user))};path=/;max-age=2592000;SameSite=Strict`;
                    window.location.reload();
                } catch (error) {
                    err.textContent = error.message; err.classList.remove('hidden');
                    btn.disabled = false; btn.textContent = 'Sign In';
                }
                return false;
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
            async function loadNotifList() {
                const token = getCookie('token');
                if (!token) return;
                try {
                    const res = await fetch('/api/notifications', { headers: { 'Authorization': 'Bearer ' + token } });
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
                            // Refetch the balance rather than adding to the displayed
                            // text, the display is locale-formatted ("1,600") and
                            // parseInt would mangle it.
                            (async function() {
                                try {
                                    const t = getCookie('token');
                                    if (!t) return;
                                    const res = await fetch('/api/credits/balance', { headers: { 'Authorization': 'Bearer ' + t } });
                                    if (!res.ok) return;
                                    const data = await res.json();
                                    const bal = (data.credit_balance ?? 0).toLocaleString();
                                    const credits = document.getElementById('nav-credits');
                                    if (credits) credits.textContent = bal;
                                    const cs = document.getElementById('nav-credits-side');
                                    if (cs) cs.textContent = bal;
                                } catch(e) {}
                            })();
                        }
                        if (msg.event === 'new_message') {
                            var msgBadge = document.getElementById('msg-badge');
                            if (msgBadge) {
                                var current = parseInt(msgBadge.textContent) || 0;
                                msgBadge.textContent = current + 1;
                                msgBadge.classList.remove('hidden');
                            }
                            window.dispatchEvent(new CustomEvent('renzora:new_message', { detail: msg.data }));
                        }
                        if (msg.event === 'message_edited' || msg.event === 'message_deleted' || msg.event === 'read_receipt') {
                            window.dispatchEvent(new CustomEvent('renzora:' + msg.event, { detail: msg.data }));
                        }
                        if (msg.event === 'new_post') {
                            // Let the community feed (if open) show its "new posts" pill.
                            window.dispatchEvent(new CustomEvent('renzora:new_post', { detail: msg.data }));
                        }
                        if (msg.event === 'new_comment' || msg.event === 'post_liked') {
                            window.dispatchEvent(new CustomEvent('renzora:' + msg.event, { detail: msg.data }));
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
                            <a href="/docs/${d.version}/${d.slug}" class="flex items-center gap-3 px-4 py-2.5 hover:bg-white/[0.03] transition-all">
                                <div class="w-8 h-8 rounded-lg bg-accent/10 flex items-center justify-center shrink-0">
                                    <i class="ph ph-book-open text-sm text-accent"></i>
                                </div>
                                <div class="flex-1 min-w-0">
                                    <div class="text-sm text-zinc-200">${d.title}</div>
                                    <div class="text-[11px] text-zinc-600">${d.group} · ${d.category}</div>
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

            // ── Single nav bootstrap: credits, notifications, messages, XP, creator status ──
            async function loadUserSummary() {
                const token = getCookie('token');
                if (!token) return;
                try {
                    const res = await fetch('/api/user/summary', { headers: { 'Authorization': 'Bearer ' + token } });
                    if (!res.ok) return;
                    const d = await res.json();

                    // Credits (header pill + sidebar card)
                    const bal = (d.credit_balance ?? 0).toLocaleString();
                    const credits = document.getElementById('nav-credits');
                    if (credits) credits.textContent = bal;
                    const cs = document.getElementById('nav-credits-side');
                    if (cs) cs.textContent = bal;

                    // Notification badge
                    const nb = document.getElementById('notif-badge');
                    if (nb && d.notification_count > 0) {
                        nb.textContent = d.notification_count > 9 ? '9+' : d.notification_count;
                        nb.classList.remove('hidden');
                    }

                    // Message badge
                    const mb = document.getElementById('msg-badge');
                    if (mb && d.unread_messages > 0) {
                        mb.textContent = d.unread_messages > 9 ? '9+' : d.unread_messages;
                        mb.classList.remove('hidden');
                    }

                    // XP bar
                    const xpWrap = document.getElementById('nav-xp-wrap');
                    if (xpWrap) { xpWrap.classList.remove('hidden'); xpWrap.classList.add('flex'); }
                    const lvl = document.getElementById('nav-level');
                    if (lvl) lvl.textContent = d.level;
                    const bar = document.getElementById('nav-xp-bar');
                    if (bar) bar.style.width = (d.level_progress_percent ?? 0).toFixed(0) + '%';
                    const xpText = document.getElementById('nav-xp-text');
                    if (xpText) xpText.textContent = (d.total_xp ?? 0).toLocaleString() + ' XP';

                    // Swap "Sell on Marketplace" -> "Creator Dashboard" once onboarded
                    if (d.creator_policy_accepted) {
                        const link = document.getElementById('nav-sell-link');
                        const icon = document.getElementById('nav-sell-icon');
                        const text = document.getElementById('nav-sell-text');
                        if (link) link.href = '/dashboard';
                        if (icon) icon.className = 'ph ph-chart-pie text-base';
                        if (text) text.textContent = 'Creator Dashboard';
                    }
                } catch(e) {}
            }

            // Highlight active nav link
            (function() {
                const path = window.location.pathname;
                document.querySelectorAll('.nav-link').forEach(link => {
                    const linkPath = link.getAttribute('data-path');
                    if (linkPath === '/' ? path === '/' : (path === linkPath || path.startsWith(linkPath + '/'))) {
                        link.classList.add('active');
                    }
                });
                // Highlight parent for child pages
                const parents = { '/courses': '/marketplace', '/developers': '/docs', '/articles': '/community' };
                for (const [sub, parent] of Object.entries(parents)) {
                    if (path === sub || path.startsWith(sub + '/')) {
                        const parentLink = document.querySelector(`.nav-link[data-path="${parent}"]`);
                        if (parentLink) parentLink.classList.add('active');
                    }
                }
            })();

            // Set redirect param on sign in link
            const signinLink = document.getElementById('nav-signin-link');
            if (signinLink && window.location.pathname !== '/login' && window.location.pathname !== '/register') {
                signinLink.href = '/login?redirect=' + encodeURIComponent(window.location.pathname + window.location.search);
            }

            updateNav();
            loadUserSummary();  // ONE request: credits, notifications, messages, XP, creator status
            connectWs();        // Live updates from here on
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

        // Global image lightbox, any <img data-zoom> opens full-size on click (site-wide)
        <script>
            r##"
            (function() {
                if (window.__imgZoomBound) return;
                window.__imgZoomBound = true;
                function openZoom(src, cap) {
                    let ov = document.getElementById('img-zoom');
                    if (!ov) {
                        ov = document.createElement('div');
                        ov.id = 'img-zoom';
                        ov.className = 'img-zoom';
                        ov.innerHTML = '<button class="img-zoom-close" aria-label="Close">&times;</button><img alt="" /><div class="img-zoom-cap"></div>';
                        ov.addEventListener('click', (e) => { if (e.target === ov || e.target.classList.contains('img-zoom-close')) closeZoom(); });
                        document.addEventListener('keydown', (e) => { if (e.key === 'Escape') closeZoom(); });
                        document.body.appendChild(ov);
                    }
                    ov.querySelector('img').src = src;
                    ov.querySelector('.img-zoom-cap').textContent = cap || '';
                    ov.classList.add('open');
                    document.body.style.overflow = 'hidden';
                }
                function closeZoom() {
                    const ov = document.getElementById('img-zoom');
                    if (ov) { ov.classList.remove('open'); document.body.style.overflow = ''; }
                }
                document.addEventListener('click', (e) => {
                    const t = e.target;
                    const img = (t && t.closest) ? t.closest('img[data-zoom]') : null;
                    if (img) { e.preventDefault(); openZoom(img.currentSrc || img.src, img.alt); }
                });
            })();
            "##
        </script>
        <style>
            r#"
            img[data-zoom] { cursor: zoom-in; transition: filter 0.15s; }
            img[data-zoom]:hover { filter: brightness(1.06); }
            .img-zoom { position: fixed; inset: 0; z-index: 200; display: none; align-items: center; justify-content: center; background: rgba(0,0,0,0.85); backdrop-filter: blur(4px); padding: 2rem; }
            .img-zoom.open { display: flex; }
            .img-zoom img { max-width: 95vw; max-height: 88vh; border-radius: 10px; border: 1px solid #3f3f46; box-shadow: 0 20px 60px rgba(0,0,0,0.6); cursor: zoom-out; }
            .img-zoom-cap { position: absolute; bottom: 1.25rem; left: 0; right: 0; text-align: center; color: #a1a1aa; font-size: 0.8125rem; padding: 0 2rem; }
            .img-zoom-close { position: absolute; top: 1rem; right: 1.25rem; width: 40px; height: 40px; border-radius: 9999px; background: rgba(255,255,255,0.08); border: 1px solid #3f3f46; color: #fafafa; font-size: 1.5rem; line-height: 1; cursor: pointer; }
            .img-zoom-close:hover { background: rgba(255,255,255,0.15); }
            "#
        </style>
    }
}
