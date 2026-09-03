use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        // ── Fixed left sidebar ──
        <aside id="app-sidebar">
            // Main menu (mobile only, the desktop menu is in the top header)
            <nav class="lg:hidden mx-3 mt-4 rounded-2xl border border-white/[0.06] bg-gradient-to-br from-white/[0.07] to-white/[0.02] p-2 space-y-0.5 overflow-hidden">
                <a href="/" class="side-link nav-link" data-path="/">
                    <i class="ph ph-download-simple text-lg"></i>"Download Engine"
                </a>
                <a href="/marketplace" class="side-link nav-link" data-path="/marketplace">
                    <i class="ph ph-storefront text-lg"></i>"Marketplace"
                </a>
                <a href="/docs" class="side-link nav-link" data-path="/docs">
                    <i class="ph ph-book-open text-lg"></i>"Docs"
                </a>
                <a href="/donate" class="side-link nav-link donate" data-path="/donate">
                    <i class="ph ph-heart text-lg"></i>"Donate"
                </a>
            </nav>

            // Cards, centered in the remaining space
            <div class="flex-1 flex flex-col justify-center px-3 space-y-2">
                // XP card (logged in)
                <div id="nav-xp-wrap" class="hidden flex-col rounded-xl p-3 bg-white/[0.03] border border-white/[0.07]">
                    <div class="flex items-center justify-between mb-2">
                        <span class="text-[10px] font-semibold uppercase tracking-[0.14em] text-zinc-400">"Level "<span id="nav-level" class="text-accent font-bold">"1"</span></span>
                        <span id="nav-xp-text" class="text-[10px] text-zinc-500 font-medium">"0 XP"</span>
                    </div>
                    <div class="h-3 bg-black/40 border border-white/[0.08] rounded-full overflow-hidden shadow-inner">
                        <div id="nav-xp-bar" class="h-full bg-gradient-to-r from-accent to-secondary rounded-full transition-all relative" style="width:0%">
                            <div class="absolute inset-0 bg-[linear-gradient(90deg,transparent_25%,rgba(255,255,255,0.15)_50%,transparent_75%)] bg-[length:200%_100%] animate-[xpShimmer_2s_linear_infinite]"></div>
                        </div>
                    </div>
                </div>

                // Community goal (everyone)
                <a href="/donate" id="nav-goal-card" class="hidden rounded-xl p-3 bg-gradient-to-br from-accent/[0.12] to-purple-600/[0.06] border border-white/[0.07] hover:border-white/[0.14] transition-colors">
                    <div class="flex items-center justify-between mb-1.5">
                        <span id="nav-goal-title" class="text-[9px] font-semibold uppercase tracking-[0.16em] text-zinc-400 truncate">"Community Goal"</span>
                        <span id="nav-goal-percent" class="text-[10px] font-semibold text-accent shrink-0 ml-1">"0%"</span>
                    </div>
                    <div class="h-2 bg-black/40 border border-white/[0.08] rounded-full overflow-hidden mb-1.5">
                        <div id="nav-goal-bar" class="h-full bg-gradient-to-r from-accent to-purple-500 rounded-full transition-all duration-700" style="width:0%"></div>
                    </div>
                    <div class="text-[10px] text-zinc-500"><span id="nav-goal-current" class="text-zinc-300 font-medium">"0"</span>" / "<span id="nav-goal-target">"0"</span>" this month"</div>
                </a>

                // Sign in / register box (logged out)
                <div id="nav-side-guest" class="rounded-xl p-3 bg-white/[0.03] border border-white/[0.07]">
                    <p class="text-[9px] font-semibold uppercase tracking-[0.16em] text-zinc-400">"Sign in to Renzora"</p>
                    <form class="mt-2 space-y-1.5" onsubmit="return sideLogin(event)">
                        <input type="email" name="email" required placeholder="Email" autocomplete="email" class="w-full px-2.5 py-1.5 bg-black/30 border border-white/[0.08] rounded-lg text-zinc-50 text-xs outline-none focus:border-accent/50 transition-all placeholder:text-zinc-600" />
                        <input type="password" name="password" required placeholder="Password" autocomplete="current-password" class="w-full px-2.5 py-1.5 bg-black/30 border border-white/[0.08] rounded-lg text-zinc-50 text-xs outline-none focus:border-accent/50 transition-all placeholder:text-zinc-600" />
                        <p id="nav-side-login-err" class="hidden text-[10px] text-red-400 leading-snug"></p>
                        <button type="submit" id="nav-side-login-btn" class="w-full text-xs font-semibold text-white bg-purple-600 hover:bg-purple-500 transition-colors rounded-lg py-1.5">"Sign In"</button>
                    </form>
                    <a href="/register" class="mt-2 block text-center text-[11px] text-zinc-500 hover:text-zinc-300 transition-colors">"New here? "<span class="text-accent font-semibold">"Create an account"</span></a>
                </div>

                // Renzora Game card (coming soon)
                <div class="rounded-xl p-3 bg-gradient-to-br from-fuchsia-500/[0.15] to-purple-600/[0.08] border border-white/[0.07]">
                    <div class="flex items-center justify-between gap-2">
                        <p class="text-[9px] font-semibold uppercase tracking-[0.16em] text-zinc-400">"Renzora Game"</p>
                        <span class="text-[9px] font-semibold px-1.5 py-0.5 rounded bg-amber-500/15 text-amber-400 shrink-0">"Coming Soon"</span>
                    </div>
                    <div class="flex items-center gap-1.5 mt-1.5">
                        <i class="ph ph-game-controller text-fuchsia-400 text-sm"></i>
                        <span class="text-sm font-semibold text-white">"Open-world adventure"</span>
                    </div>
                    <a href="/game" class="mt-2.5 block text-center text-xs font-semibold text-white bg-fuchsia-600 hover:bg-fuchsia-500 transition-colors rounded-lg py-1.5">"Join Waiting List"</a>
                </div>

                // Renzora Game card (coming soon)
                <div class="rounded-xl p-3 bg-gradient-to-br from-fuchsia-500/[0.15] to-purple-600/[0.08] border border-white/[0.07]">
                    <div class="flex items-center justify-between gap-2">
                        <p class="text-[9px] font-semibold uppercase tracking-[0.16em] text-zinc-400">"Renzora Game"</p>
                        <span class="text-[9px] font-semibold px-1.5 py-0.5 rounded bg-amber-500/15 text-amber-400 shrink-0">"Coming Soon"</span>
                    </div>
                    <div class="flex items-center gap-1.5 mt-1.5">
                        <i class="ph ph-game-controller text-fuchsia-400 text-sm"></i>
                        <span class="text-sm font-semibold text-white">"Open-world adventure"</span>
                    </div>
                    <a href="/game" class="mt-2.5 block text-center text-xs font-semibold text-white bg-fuchsia-600 hover:bg-fuchsia-500 transition-colors rounded-lg py-1.5">"Join Waiting List"</a>
                </div>

                // Engine download card
                <div class="rounded-xl p-3 bg-gradient-to-br from-accent/[0.15] to-secondary/[0.08] border border-white/[0.07]">
                    <p class="text-[9px] font-semibold uppercase tracking-[0.16em] text-zinc-400">"Renzora Engine"</p>
                    <div class="flex items-center gap-1.5 mt-1.5">
                        <i class="ph ph-download-simple text-secondary text-sm"></i>
                        <span class="text-sm font-semibold text-white">"r1-alpha6"</span>
                        <span class="text-[9px] font-semibold px-1.5 py-0.5 rounded bg-emerald-500/15 text-emerald-400">"Free"</span>
                    </div>
                    <a href="/" class="mt-2.5 block text-center text-xs font-semibold text-white bg-purple-600 hover:bg-purple-500 transition-colors rounded-lg py-1.5">"Download"</a>
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
            // Brand
            <a href="/" class="flex items-center gap-2.5 min-w-0 shrink-0 mr-1">
                <img src="/assets/previews/hazel.webp" alt="Hazel" width="36" height="36" class="w-9 h-9 rounded-lg object-cover shrink-0" />
                <div class="leading-none hidden sm:block">
                    <div class="text-[15px] font-bold text-white tracking-tight">"renzora"</div>
                    <div class="text-[9px] font-semibold uppercase tracking-[0.18em] text-zinc-400 mt-1">"Game Engine"</div>
                </div>
            </a>

            // Mobile hamburger (opens the sidebar on small screens)
            <button id="sidebar-burger" onclick="toggleSidebar()" aria-label="Open navigation menu" class="lg:hidden text-zinc-400 hover:text-white p-1.5 rounded-lg hover:bg-white/[0.06] transition-all">
                <i class="ph ph-list text-xl"></i>
            </button>

            // Search (left of the header)
            <div class="relative" id="global-search-wrap">
                <button onclick="toggleGlobalSearch()" class="text-zinc-400 hover:text-white p-2 rounded-lg hover:bg-white/[0.06] transition-all" title="Search (Ctrl+K)">
                    <i class="ph ph-magnifying-glass text-lg"></i>
                </button>
                <div id="global-search-panel" class="hidden absolute left-0 top-full mt-2 w-[420px] max-w-[90vw] bg-[rgba(12,7,21,0.95)] backdrop-blur-2xl border border-white/[0.08] rounded-xl shadow-2xl shadow-black/60 overflow-hidden z-50">
                    <div class="flex items-center gap-2 px-4 py-3 border-b border-white/[0.06]">
                        <i class="ph ph-magnifying-glass text-zinc-500"></i>
                        <input type="text" id="global-search-input" placeholder="Search assets and docs..." oninput="globalSearch(this.value)" class="flex-1 bg-transparent text-sm text-zinc-50 outline-none placeholder:text-zinc-600" />
                        <kbd class="text-[10px] text-zinc-600 border border-white/[0.08] rounded px-1.5 py-0.5">"Esc"</kbd>
                    </div>
                    <div id="global-search-results" class="max-h-[400px] overflow-y-auto">
                        <div class="px-4 py-8 text-center text-xs text-zinc-600">"Type to search the marketplace and docs."</div>
                    </div>
                </div>
            </div>

            // Main nav (desktop top bar)
            <nav class="hidden lg:flex items-center gap-1 ml-2">
                <a href="/" class="top-link nav-link" data-path="/"><i class="ph ph-download-simple text-base"></i>"Download Engine"</a>
                <a href="/marketplace" class="top-link nav-link" data-path="/marketplace"><i class="ph ph-storefront text-base"></i>"Marketplace"</a>
                <a href="/docs" class="top-link nav-link" data-path="/docs"><i class="ph ph-book-open text-base"></i>"Docs"</a>
                <a href="/donate" class="top-link nav-link donate" data-path="/donate"><i class="ph ph-heart text-base"></i>"Donate"</a>
            </nav>

            <div class="flex-1"></div>

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
                    <span class="text-xs text-amber-400/80 font-medium hidden sm:inline">"credits"</span>
                </a>
                // User
                <div class="relative" id="user-dropdown-wrap">
                    <button onclick="toggleDropdown()" id="user-dropdown-btn" class="flex items-center gap-1.5 pl-1 pr-2 py-1 rounded-full bg-white/[0.04] border border-white/[0.06] hover:bg-white/[0.08] transition-all cursor-pointer">
                        <span class="w-7 h-7 rounded-full bg-gradient-to-br from-accent to-secondary flex items-center justify-center text-white text-xs font-bold" id="nav-avatar-initial"></span>
                        <span id="nav-username" class="text-sm text-zinc-200 hidden sm:block"></span>
                        <i class="ph ph-caret-down text-xs text-zinc-500"></i>
                    </button>
                    <div id="user-dropdown" class="hidden absolute right-0 top-full mt-2 w-52 bg-[rgba(12,7,21,0.95)] backdrop-blur-2xl border border-white/[0.08] rounded-xl shadow-2xl overflow-hidden z-50 py-1">
                        <a href="/library" class="flex items-center gap-2.5 px-4 py-2.5 text-sm text-zinc-400 hover:text-white hover:bg-white/[0.06] transition-all">
                            <i class="ph ph-books text-base"></i>"My Library"
                        </a>
                        <a id="nav-sell-link" href="/marketplace/sell" class="flex items-center gap-2.5 px-4 py-2.5 text-sm text-zinc-400 hover:text-white hover:bg-white/[0.06] transition-all">
                            <i class="ph ph-storefront text-base" id="nav-sell-icon"></i><span id="nav-sell-text">"Sell on Marketplace"</span>
                        </a>
                        <a href="/donate" class="flex items-center gap-2.5 px-4 py-2.5 text-sm text-zinc-400 hover:text-white hover:bg-white/[0.06] transition-all">
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
                    el.innerHTML = '<div class="px-4 py-8 text-center text-xs text-zinc-600">Type to search the marketplace and docs.</div>';
                    return;
                }
                gsTimeout = setTimeout(async () => {
                    el.innerHTML = '<div class="px-4 py-6 text-center"><div class="inline-block animate-spin w-4 h-4 border-2 border-zinc-700 border-t-accent rounded-full"></div></div>';
                    const q = encodeURIComponent(query.trim());
                    const [assetsRes, docsRes] = await Promise.all([
                        fetch('/api/marketplace?q=' + q + '&page=1').then(r => r.ok ? r.json() : { assets: [] }).catch(() => ({ assets: [] })),
                        fetch('/api/docs/search?q=' + q).then(r => r.ok ? r.json() : []).catch(() => []),
                    ]);

                    const assets = (assetsRes.assets || []).slice(0, 5);
                    const docs = (docsRes || []).slice(0, 5);

                    if (!assets.length && !docs.length) {
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

            // ── Single nav bootstrap: credits, XP, creator status ──
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
                const parents = { '/developers': '/docs' };
                for (const [sub, parent] of Object.entries(parents)) {
                    if (path === sub || path.startsWith(sub + '/')) {
                        const parentLink = document.querySelector(`.nav-link[data-path="${parent}"]`);
                        if (parentLink) parentLink.classList.add('active');
                    }
                }
            })();

            // Set redirect param on sign in link
            const signinLink = document.getElementById('nav-signin-link');
            const _sp = window.location.pathname;
            // Deep pages carry a redirect back to themselves; the download root does not.
            if (signinLink && _sp !== '/login' && _sp !== '/register' && _sp !== '/') {
                signinLink.href = '/login?redirect=' + encodeURIComponent(_sp + window.location.search);
            }

            // ── Sidebar card: community goal ──
            async function loadNavGoal() {
                const card = document.getElementById('nav-goal-card');
                if (!card) return;
                try {
                    const data = await fetch('/api/credits/donate/sponsors').then(r => r.ok ? r.json() : null).catch(() => null);
                    const goal = data && data.goal;
                    if (!goal || !goal.enabled) return;
                    document.getElementById('nav-goal-title').textContent = goal.title || 'Community Goal';
                    document.getElementById('nav-goal-current').textContent = (goal.current || 0).toLocaleString();
                    document.getElementById('nav-goal-target').textContent = (goal.target || 0).toLocaleString();
                    const pct = goal.percent || 0;
                    document.getElementById('nav-goal-percent').textContent = pct + '%';
                    card.classList.remove('hidden');
                    setTimeout(() => { const bar = document.getElementById('nav-goal-bar'); if (bar) bar.style.width = Math.min(100, pct) + '%'; }, 100);
                } catch (e) {}
            }

            updateNav();
            loadUserSummary();  // ONE request: credits, XP, creator status
            loadNavGoal();      // community goal card (everyone)
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
