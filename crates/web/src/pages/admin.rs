use leptos::prelude::*;

/// Staff console. Everything here is gated server-side by `verify_admin` on each
/// `/api/admin/*` call — the client-side check only decides whether to render the
/// UI, so a tampered cookie reveals an empty shell, not data.
#[component]
pub fn AdminPage() -> impl IntoView {
    view! {
        <section class="py-8 px-6 min-h-[80vh]">
            <div class="max-w-[1500px] mx-auto">

                <div id="admin-denied" class="hidden text-center py-24">
                    <div class="w-16 h-16 bg-zinc-800/50 rounded-full flex items-center justify-center mx-auto mb-4">
                        <i class="ph ph-shield-warning text-2xl text-zinc-500"></i>
                    </div>
                    <p class="text-zinc-300 font-medium mb-1">"Admins only"</p>
                    <p class="text-zinc-500 text-sm mb-5">"This area is restricted to Renzora staff."</p>
                    <a href="/" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all">"Back to downloads"</a>
                </div>

                <div id="admin-loading" class="text-center py-24">
                    <div class="inline-block animate-spin w-6 h-6 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                </div>

                <div id="admin-root" class="hidden flex gap-6 items-start">

                    // ── Vertical menu ──
                    <aside class="w-56 shrink-0 sticky top-20">
                        <div class="mb-5">
                            <h1 class="text-xl font-bold flex items-center gap-2">
                                <i class="ph ph-shield-check text-accent"></i>"Admin"
                            </h1>
                            <p class="text-xs text-zinc-600 mt-1 truncate" id="admin-whoami"></p>
                        </div>

                        <div class="relative mb-4">
                            <i class="ph ph-magnifying-glass absolute left-3 top-1/2 -translate-y-1/2 text-zinc-600 text-sm"></i>
                            <input type="text" id="admin-search" placeholder="Search..." oninput="adminGlobalSearch(this.value)"
                                class="w-full pl-8 pr-3 py-2 bg-white/[0.02] border border-zinc-800/50 rounded-lg text-zinc-50 text-xs outline-none focus:border-accent/50 transition-all" />
                            <div id="admin-search-results" class="hidden absolute left-0 right-0 top-full mt-2 bg-[rgba(12,7,21,0.97)] border border-white/[0.08] rounded-xl shadow-2xl z-50 max-h-80 overflow-y-auto"></div>
                        </div>

                        <nav id="admin-menu" class="space-y-0.5"></nav>
                    </aside>

                    // ── Content ──
                    <div class="flex-1 min-w-0">
                        <div id="admin-error" class="hidden mb-4 p-3 rounded-xl bg-red-500/10 border border-red-500/20 text-red-400 text-sm"></div>
                        <div id="admin-ok" class="hidden mb-4 p-3 rounded-xl bg-green-500/10 border border-green-500/20 text-green-400 text-sm"></div>

                        // ── Overview ──
                        <div id="apanel-overview">
                            <div id="admin-stats" class="grid grid-cols-2 lg:grid-cols-5 gap-3 mb-4"></div>
                            <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                                <div class="p-5 bg-white/[0.02] border border-zinc-800/50 rounded-2xl">
                                    <h2 class="text-sm font-semibold mb-4 flex items-center gap-2"><i class="ph ph-trend-up text-emerald-400"></i>"This month vs last"</h2>
                                    <div id="admin-growth" class="space-y-3"></div>
                                </div>
                                <div class="p-5 bg-white/[0.02] border border-zinc-800/50 rounded-2xl">
                                    <h2 class="text-sm font-semibold mb-4 flex items-center gap-2"><i class="ph ph-desktop text-sky-400"></i>"Engine downloads by platform"</h2>
                                    <div id="admin-platforms" class="space-y-2"></div>
                                </div>
                                <div class="p-5 bg-white/[0.02] border border-zinc-800/50 rounded-2xl">
                                    <h2 class="text-sm font-semibold mb-4 flex items-center gap-2"><i class="ph ph-crown text-amber-400"></i>"Top creators by earnings"</h2>
                                    <div id="admin-creators" class="space-y-1"></div>
                                </div>
                                <div class="p-5 bg-white/[0.02] border border-zinc-800/50 rounded-2xl">
                                    <h2 class="text-sm font-semibold mb-4 flex items-center gap-2"><i class="ph ph-shopping-cart text-accent"></i>"Top buyers by spend"</h2>
                                    <div id="admin-buyers" class="space-y-1"></div>
                                </div>
                            </div>
                        </div>

                        // ── Analytics ──
                        <div id="apanel-analytics" class="hidden">
                            // Filters sit in one row above the chart.
                            <div class="flex flex-wrap items-center gap-2 mb-4">
                                <select id="chart-metric" onchange="loadChart()"
                                    class="px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent">
                                    <option value="downloads">"Engine downloads"</option>
                                    <option value="registrations">"User registrations"</option>
                                    <option value="assets_added">"Marketplace assets"</option>
                                    <option value="revenue">"Revenue (top-ups)"</option>
                                    <option value="purchases">"Purchase volume"</option>
                                    <option value="commission">"Platform commission"</option>
                                    <option value="withdrawals">"Withdrawals paid"</option>
                                    <option value="waitlist">"Game waiting list"</option>
                                </select>
                                <div class="flex items-center gap-1 p-1 bg-white/[0.02] rounded-lg border border-zinc-800/40">
                                    <button id="cb-day" onclick="setBucket('day')" class="px-3 py-1.5 rounded-md text-xs font-medium text-zinc-400 hover:text-zinc-200 transition-all">"Daily"</button>
                                    <button id="cb-week" onclick="setBucket('week')" class="px-3 py-1.5 rounded-md text-xs font-medium text-zinc-400 hover:text-zinc-200 transition-all">"Weekly"</button>
                                    <button id="cb-month" onclick="setBucket('month')" class="px-3 py-1.5 rounded-md text-xs font-medium bg-blue-600 text-white transition-all">"Monthly"</button>
                                </div>
                                <select id="chart-range" onchange="loadChart()"
                                    class="px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent">
                                    <option value="30">"Last 30 days"</option>
                                    <option value="90">"Last 90 days"</option>
                                    <option value="365" selected>"Last 12 months"</option>
                                    <option value="1095">"Last 3 years"</option>
                                </select>
                                <div class="flex-1"></div>
                                <button onclick="toggleChartTable()" id="chart-table-btn"
                                    class="inline-flex items-center gap-1.5 px-3 py-2 rounded-lg text-xs font-medium bg-white/[0.04] border border-zinc-800 text-zinc-300 hover:border-accent hover:text-white transition-all">
                                    <i class="ph ph-table"></i>"Table"
                                </button>
                            </div>

                            <div class="p-5 bg-white/[0.02] border border-zinc-800/50 rounded-2xl">
                                <div class="flex flex-wrap items-start justify-between gap-4 mb-5">
                                    <div>
                                        <h2 id="chart-title" class="text-base font-semibold">"Engine downloads"</h2>
                                        <p id="chart-sub" class="text-xs text-zinc-600 mt-0.5"></p>
                                    </div>
                                    <div id="chart-summary" class="flex gap-6"></div>
                                </div>
                                <div id="chart-wrap" class="relative">
                                    <div id="chart-svg"></div>
                                    <div id="chart-tip" class="hidden pointer-events-none absolute z-20 px-2.5 py-1.5 rounded-lg bg-[rgba(12,7,21,0.97)] border border-white/[0.1] shadow-xl text-xs whitespace-nowrap"></div>
                                </div>
                                <div id="chart-table" class="hidden mt-5 max-h-72 overflow-y-auto"></div>
                            </div>
                        </div>

                        // ── Assets ──
                        <div id="apanel-assets" class="hidden">
                            <div class="flex flex-wrap gap-2 mb-4">
                                <input type="text" id="asset-q" placeholder="Search asset name..." oninput="debouncedAssets()"
                                    class="flex-1 min-w-[200px] px-4 py-2.5 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50" />
                                <select id="asset-published" onchange="loadAssets(1)"
                                    class="px-4 py-2.5 bg-surface border border-zinc-800 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent">
                                    <option value="">"All"</option>
                                    <option value="true">"Published"</option>
                                    <option value="false">"Unpublished"</option>
                                </select>
                            </div>
                            <div id="assets-table" class="rounded-2xl border border-zinc-800/40 bg-white/[0.01] overflow-hidden"></div>
                            <div id="assets-pager" class="flex items-center justify-center gap-3 py-4"></div>
                        </div>

                        // ── Review queue ──
                        <div id="apanel-review" class="hidden">
                            <p class="text-sm text-zinc-500 mb-4">"Assets awaiting moderation, oldest first."</p>
                            <div id="review-table" class="rounded-2xl border border-zinc-800/40 bg-white/[0.01] overflow-hidden"></div>
                        </div>

                        // ── Tags ──
                        <div id="apanel-tags" class="hidden">
                            <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                                <div>
                                    <h2 class="text-sm font-semibold mb-3 flex items-center gap-2"><i class="ph ph-tag text-amber-400"></i>"Pending tags"</h2>
                                    <div id="tags-pending" class="rounded-2xl border border-zinc-800/40 bg-white/[0.01] overflow-hidden"></div>
                                </div>
                                <div>
                                    <h2 class="text-sm font-semibold mb-3 flex items-center gap-2"><i class="ph ph-folder text-sky-400"></i>"Pending subcategories"</h2>
                                    <div id="subcats-pending" class="rounded-2xl border border-zinc-800/40 bg-white/[0.01] overflow-hidden"></div>
                                </div>
                            </div>
                        </div>

                        // ── Users ──
                        <div id="apanel-users" class="hidden">
                            <input type="text" id="user-q" placeholder="Search username or email..." oninput="debouncedUsers()"
                                class="w-full mb-4 px-4 py-2.5 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50" />
                            <div id="users-table" class="rounded-2xl border border-zinc-800/40 bg-white/[0.01] overflow-hidden"></div>
                            <div id="users-pager" class="flex items-center justify-center gap-3 py-4"></div>
                        </div>

                        // ── Gift cards ──
                        <div id="apanel-gifts" class="hidden">
                            <div class="flex flex-wrap gap-2 mb-4">
                                <input type="text" id="gift-q" placeholder="Search code or username..." oninput="debouncedGifts()"
                                    class="flex-1 min-w-[200px] px-4 py-2.5 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50" />
                                <select id="gift-status" onchange="loadGifts(1)"
                                    class="px-4 py-2.5 bg-surface border border-zinc-800 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent">
                                    <option value="">"All"</option>
                                    <option value="pending">"Pending"</option>
                                    <option value="redeemed">"Redeemed"</option>
                                    <option value="void">"Void"</option>
                                </select>
                            </div>
                            <div id="gifts-table" class="rounded-2xl border border-zinc-800/40 bg-white/[0.01] overflow-hidden"></div>
                            <div id="gifts-pager" class="flex items-center justify-center gap-3 py-4"></div>
                        </div>

                        // ── Game waiting list ──
                        <div id="apanel-waitlist" class="hidden">
                            <div id="waitlist-stats" class="grid grid-cols-2 lg:grid-cols-4 gap-3 mb-4"></div>
                            <div class="flex flex-wrap gap-2 mb-4">
                                <input type="text" id="wl-q" placeholder="Search email or source..." oninput="debouncedWaitlist()"
                                    class="flex-1 min-w-[200px] px-4 py-2.5 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50" />
                                <button onclick="exportWaitlist()" class="inline-flex items-center gap-1.5 px-4 py-2.5 rounded-xl text-sm font-medium bg-white/[0.04] border border-zinc-800 text-zinc-300 hover:border-accent hover:text-white transition-all">
                                    <i class="ph ph-download-simple"></i>"Export emails"
                                </button>
                            </div>
                            <div id="waitlist-table" class="rounded-2xl border border-zinc-800/40 bg-white/[0.01] overflow-hidden"></div>
                            <div id="waitlist-pager" class="flex items-center justify-center gap-3 py-4"></div>
                        </div>

                        // ── Statements ──
                        <div id="apanel-ledger" class="hidden">
                            <div class="flex flex-wrap items-center gap-2 mb-4">
                                <select id="ledger-type" onchange="loadLedger(1)"
                                    class="px-4 py-2.5 bg-surface border border-zinc-800 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent">
                                    <option value="">"All types"</option>
                                    <option value="purchase">"Purchase"</option>
                                    <option value="earning">"Earning"</option>
                                    <option value="topup">"Top-up"</option>
                                    <option value="withdrawal">"Withdrawal"</option>
                                    <option value="refund">"Refund"</option>
                                    <option value="admin_credit">"Admin credit"</option>
                                </select>
                                <div class="flex-1"></div>
                                <select id="ledger-month" class="px-4 py-2.5 bg-surface border border-zinc-800 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent"></select>
                                <button onclick="downloadStatement()" class="inline-flex items-center gap-1.5 px-4 py-2.5 rounded-xl text-sm font-medium bg-white/[0.04] border border-zinc-800 text-zinc-300 hover:border-accent hover:text-white transition-all">
                                    <i class="ph ph-download-simple"></i>"Export CSV"
                                </button>
                            </div>
                            <div id="ledger-table" class="rounded-2xl border border-zinc-800/40 bg-white/[0.01] overflow-hidden"></div>
                            <div id="ledger-pager" class="flex items-center justify-center gap-3 py-4"></div>
                        </div>
                    </div>
                </div>

                // ── User editor ──
                <div id="user-modal" class="hidden fixed inset-0 z-[60] items-center justify-center bg-black/70 p-4" onclick="if(event.target===this)closeUserModal()">
                    <div class="w-full max-w-lg max-h-[90vh] overflow-y-auto bg-[rgba(12,7,21,0.98)] border border-white/[0.08] rounded-2xl p-6">
                        <div class="flex items-start justify-between mb-5">
                            <div>
                                <h2 class="text-lg font-semibold" id="um-title">"Edit user"</h2>
                                <p class="text-xs text-zinc-500 mt-0.5" id="um-sub"></p>
                            </div>
                            <button onclick="closeUserModal()" class="text-zinc-500 hover:text-white p-1"><i class="ph ph-x text-lg"></i></button>
                        </div>
                        <div class="space-y-4">
                            <div class="grid grid-cols-2 gap-3">
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1.5">"Username"</label>
                                    <input type="text" id="um-username" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent" />
                                </div>
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1.5">"Role"</label>
                                    <select id="um-role" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent">
                                        <option value="user">"user"</option>
                                        <option value="moderator">"moderator"</option>
                                        <option value="admin">"admin"</option>
                                    </select>
                                </div>
                            </div>
                            <div>
                                <label class="block text-xs text-zinc-500 mb-1.5">"Email"</label>
                                <input type="email" id="um-email" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent" />
                            </div>
                            <div>
                                <label class="block text-xs text-zinc-500 mb-1.5">"Credit balance"</label>
                                <input type="number" id="um-balance" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent" />
                                <p class="text-xs text-zinc-600 mt-1">"Sets the balance outright. To record a movement instead, use the adjustment below."</p>
                            </div>
                            <button onclick="saveUser()" class="w-full px-4 py-2.5 rounded-lg text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all">"Save changes"</button>

                            <div class="pt-4 border-t border-zinc-800/60 space-y-3">
                                <p class="text-xs text-zinc-500 uppercase tracking-wider font-medium">"Credit adjustment"</p>
                                <div class="grid grid-cols-2 gap-3">
                                    <input type="number" id="um-adjust" placeholder="Amount (+/-)" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent" />
                                    <input type="text" id="um-adjust-reason" placeholder="Reason" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent" />
                                </div>
                                <button onclick="adjustCredits()" class="w-full px-4 py-2.5 rounded-lg text-sm font-medium bg-white/[0.04] border border-zinc-800 text-zinc-300 hover:border-accent hover:text-white transition-all">"Apply adjustment"</button>
                                <p class="text-xs text-zinc-600">"Writes a transaction, so it shows up in statements and the audit log."</p>
                            </div>

                            <div class="pt-4 border-t border-zinc-800/60">
                                <button onclick="toggleBan()" class="w-full px-4 py-2.5 rounded-lg text-sm font-medium bg-red-500/10 border border-red-500/20 text-red-400 hover:bg-red-500/20 transition-all">"Ban user"</button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <script>
        r##"
        const A = { tab: 'overview', assetPage: 1, userPage: 1, giftPage: 1, ledgerPage: 1, wlPage: 1, editing: null };

        function tok() { return document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop(); }
        function H() { return { 'Authorization': 'Bearer ' + tok() }; }
        function HJ() { return { 'Authorization': 'Bearer ' + tok(), 'Content-Type': 'application/json' }; }

        function esc(v) {
            const d = document.createElement('div');
            d.textContent = v == null ? '' : String(v);
            return d.innerHTML;
        }
        function num(n) { return (n ?? 0).toLocaleString(); }
        function shortDate(s) {
            if (!s) return '—';
            const d = new Date(s);
            return isNaN(d) ? '—' : d.toLocaleDateString('en-GB', { day: 'numeric', month: 'short', year: '2-digit' });
        }
        function flash(kind, msg) {
            const el = document.getElementById(kind === 'ok' ? 'admin-ok' : 'admin-error');
            el.textContent = msg;
            el.classList.remove('hidden');
            setTimeout(() => el.classList.add('hidden'), 4000);
        }
        // Every admin call funnels through here so one 401/403 handler covers them all.
        async function api(path, opts) {
            const res = await fetch('/api/admin' + path, opts || { headers: H() });
            if (res.status === 401 || res.status === 403) { showDenied(); throw new Error('Not authorised'); }
            const text = await res.text();
            let data = {};
            try { data = text ? JSON.parse(text) : {}; } catch (e) { data = { error: text.slice(0, 200) }; }
            if (!res.ok) throw new Error(data.error || 'Request failed');
            return data;
        }
        function debounce(fn, ms) { let t; return function() { clearTimeout(t); t = setTimeout(fn, ms); }; }

        // ──────────────────────────────────────
        // Vertical menu
        // ──────────────────────────────────────
        const MENU = [
            { id: 'overview',  label: 'Overview',   icon: 'ph-gauge' },
            { id: 'analytics', label: 'Analytics',  icon: 'ph-chart-line' },
            { id: 'assets',    label: 'Assets',     icon: 'ph-package' },
            { id: 'review',    label: 'Review queue', icon: 'ph-clipboard-text', badge: true },
            { id: 'tags',      label: 'Tags',       icon: 'ph-tag' },
            { id: 'users',     label: 'Users',      icon: 'ph-users' },
            { id: 'gifts',     label: 'Gift cards', icon: 'ph-gift' },
            { id: 'waitlist',  label: 'Game waitlist', icon: 'ph-envelope-simple' },
            { id: 'ledger',    label: 'Statements', icon: 'ph-receipt' }
        ];

        function renderMenu() {
            document.getElementById('admin-menu').innerHTML = MENU.map(m =>
                '<button id="atab-' + m.id + '" onclick="adminTab(\'' + m.id + '\')" '
                + 'class="w-full flex items-center gap-2.5 px-3 py-2 rounded-lg text-sm font-medium text-zinc-400 hover:text-zinc-200 hover:bg-white/[0.03] transition-all">'
                + '<i class="ph ' + m.icon + ' text-base"></i><span class="flex-1 text-left">' + m.label + '</span>'
                + (m.badge ? '<span id="review-badge" class="hidden text-[10px] font-bold px-1.5 py-0.5 rounded-full bg-amber-500 text-black"></span>' : '')
                + '</button>').join('');
        }

        function adminTab(name) {
            A.tab = name;
            MENU.forEach(m => {
                document.getElementById('apanel-' + m.id).classList.toggle('hidden', m.id !== name);
                const btn = document.getElementById('atab-' + m.id);
                const on = m.id === name;
                btn.classList.toggle('bg-blue-600', on);
                btn.classList.toggle('text-white', on);
                btn.classList.toggle('text-zinc-400', !on);
                btn.classList.toggle('hover:text-zinc-200', !on);
                btn.classList.toggle('hover:bg-white/[0.03]', !on);
            });
            if (name === 'analytics') loadChart();
            if (name === 'assets') loadAssets(1);
            if (name === 'review') loadReviewQueue();
            if (name === 'tags') loadTags();
            if (name === 'users') loadUsers(1);
            if (name === 'gifts') loadGifts(1);
            if (name === 'waitlist') loadWaitlist(1);
            if (name === 'ledger') loadLedger(1);
        }

        function table(headers, rows, empty) {
            if (!rows.length) return '<p class="text-center text-sm text-zinc-600 py-12">' + empty + '</p>';
            return '<div class="overflow-x-auto"><table class="w-full text-sm"><thead><tr class="border-b border-zinc-800/60">'
                + headers.map(h => '<th class="text-left font-medium text-zinc-500 text-xs uppercase tracking-wider px-4 py-3">' + h + '</th>').join('')
                + '</tr></thead><tbody class="divide-y divide-zinc-800/30">' + rows.join('') + '</tbody></table></div>';
        }
        const PER_PAGE = 50;
        function pagerButtons(elId, page, prevOk, nextOk, label, fn) {
            const el = document.getElementById(elId);
            if (!prevOk && !nextOk) { el.innerHTML = ''; return; }
            el.innerHTML = '<button ' + (prevOk ? '' : 'disabled') + ' onclick="' + fn + '(' + (page - 1) + ')" class="px-3 py-1.5 rounded-lg text-xs bg-white/[0.04] border border-zinc-800 text-zinc-300 disabled:opacity-30">Prev</button>'
                + '<span class="text-xs text-zinc-500">' + label + '</span>'
                + '<button ' + (nextOk ? '' : 'disabled') + ' onclick="' + fn + '(' + (page + 1) + ')" class="px-3 py-1.5 rounded-lg text-xs bg-white/[0.04] border border-zinc-800 text-zinc-300 disabled:opacity-30">Next</button>';
        }
        function pager(elId, page, total, fn) {
            const pages = Math.max(1, Math.ceil(total / PER_PAGE));
            pagerButtons(elId, page, page > 1, page < pages, 'Page ' + page + ' of ' + pages, fn);
        }
        function pagerByCount(elId, page, count, fn) {
            pagerButtons(elId, page, page > 1, count === PER_PAGE, 'Page ' + page, fn);
        }
        function statCard(label, value, icon, colour) {
            return '<div class="p-4 bg-white/[0.02] border border-zinc-800/50 rounded-2xl">'
                + '<div class="flex items-center gap-2 mb-2"><i class="ph ' + icon + ' ' + colour + '"></i>'
                + '<span class="text-[10px] uppercase tracking-wider text-zinc-500 font-medium">' + label + '</span></div>'
                + '<div class="text-2xl font-bold">' + value + '</div></div>';
        }

        // ──────────────────────────────────────
        // Overview
        // ──────────────────────────────────────
        async function loadOverview() {
            try {
                const s = await api('/stats');
                document.getElementById('admin-stats').innerHTML =
                      statCard('Users', num(s.total_users), 'ph-users', 'text-sky-400')
                    + statCard('Assets', num(s.total_assets), 'ph-package', 'text-accent')
                    + statCard('Transactions', num(s.total_transactions), 'ph-receipt', 'text-emerald-400')
                    + statCard('Credits in circulation', num(s.total_credits_circulating), 'ph-coin', 'text-amber-400')
                    + statCard('Open disputes', num(s.open_disputes), 'ph-warning-circle', s.open_disputes > 0 ? 'text-red-400' : 'text-zinc-500');
            } catch (e) { flash('err', e.message); }

            try {
                const b = await api('/analytics/business');
                const rows = [
                    ['New users', b.current_month_users, b.last_month_users],
                    ['New assets', b.current_month_assets, b.last_month_assets]
                ];
                document.getElementById('admin-growth').innerHTML = rows.map(function(r) {
                    const cur = r[1] ?? 0, prev = r[2] ?? 0, delta = cur - prev;
                    const cls = delta > 0 ? 'text-emerald-400' : delta < 0 ? 'text-red-400' : 'text-zinc-500';
                    return '<div class="flex items-center justify-between"><span class="text-sm text-zinc-400">' + r[0] + '</span>'
                        + '<span class="text-sm"><span class="font-semibold">' + num(cur) + '</span> '
                        + '<span class="' + cls + ' text-xs">' + (delta > 0 ? '+' : '') + num(delta) + ' vs ' + num(prev) + '</span></span></div>';
                }).join('');

                document.getElementById('admin-platforms').innerHTML = (b.platforms || []).length
                    ? b.platforms.map(p => '<div class="flex items-center justify-between text-sm"><span class="text-zinc-400">' + esc(p.platform) + '</span><span class="text-zinc-200 font-medium">' + num(p.count) + '</span></div>').join('')
                    : '<p class="text-sm text-zinc-600">No downloads recorded yet.</p>';

                const nameList = (arr, key, suffix) => (arr || []).length
                    ? arr.map((r, i) => '<div class="flex items-center justify-between text-sm py-1">'
                        + '<span class="text-zinc-400"><span class="text-zinc-600 mr-2">' + (i + 1) + '</span>' + esc(r.username) + '</span>'
                        + '<span class="text-zinc-200 font-medium">' + num(r[key]) + suffix + '</span></div>').join('')
                    : '<p class="text-sm text-zinc-600">Nothing yet.</p>';
                document.getElementById('admin-creators').innerHTML = nameList(b.top_creators, 'total_earnings', ' cr');
                document.getElementById('admin-buyers').innerHTML = nameList(b.top_buyers, 'total_spent', ' cr');
            } catch (e) { flash('err', e.message); }
        }

        // ──────────────────────────────────────
        // Analytics chart
        //
        // One metric at a time, so a single series: no legend (the title names
        // it), one hue, recessive grid, crosshair on hover, and a table view for
        // the numbers. The site renders dark-only, so this is stepped for the
        // dark surface rather than flipped from a light palette.
        // ──────────────────────────────────────
        const CHART = { bucket: 'month', data: [], showTable: false };
        const SERIES = '#818cf8';     // indigo-400, the site accent on a dark surface
        const GRID   = 'rgba(255,255,255,0.06)';
        const AXIS   = '#71717a';     // zinc-500

        const METRIC_LABELS = {
            downloads: ['Engine downloads', ''], registrations: ['User registrations', ''],
            assets_added: ['Marketplace assets', ''], revenue: ['Revenue (top-ups)', ' cr'],
            purchases: ['Purchase volume', ' cr'], commission: ['Platform commission', ' cr'],
            withdrawals: ['Withdrawals paid', ' cr'], waitlist: ['Game waiting list', '']
        };

        function setBucket(b) {
            CHART.bucket = b;
            ['day', 'week', 'month'].forEach(x => {
                const btn = document.getElementById('cb-' + x);
                btn.classList.toggle('bg-blue-600', x === b);
                btn.classList.toggle('text-white', x === b);
                btn.classList.toggle('text-zinc-400', x !== b);
                btn.classList.toggle('hover:text-zinc-200', x !== b);
            });
            loadChart();
        }

        function bucketLabel(iso) {
            const d = new Date(iso);
            if (CHART.bucket === 'month') return d.toLocaleDateString('en-GB', { month: 'short', year: '2-digit' });
            return d.toLocaleDateString('en-GB', { day: 'numeric', month: 'short' });
        }

        async function loadChart() {
            const metric = document.getElementById('chart-metric').value;
            const days = document.getElementById('chart-range').value;
            const [label, suffix] = METRIC_LABELS[metric] || [metric, ''];
            document.getElementById('chart-title').textContent = label;
            try {
                const d = await api('/analytics/timeseries?metric=' + metric + '&bucket=' + CHART.bucket + '&days=' + days);
                CHART.data = d.data || [];
                document.getElementById('chart-sub').textContent =
                    CHART.bucket === 'day' ? 'Daily' : CHART.bucket === 'week' ? 'Weekly' : 'Monthly';
                drawChart(suffix);
                drawChartSummary(suffix);
                drawChartTable(label, suffix);
            } catch (e) {
                flash('err', e.message);
                document.getElementById('chart-svg').innerHTML =
                    '<p class="text-center text-sm text-zinc-600 py-16">Could not load this metric.</p>';
            }
        }

        function drawChartSummary(suffix) {
            const vals = CHART.data.map(d => d.value);
            const total = vals.reduce((a, b) => a + b, 0);
            const peak = vals.length ? Math.max(...vals) : 0;
            const avg = vals.length ? Math.round(total / vals.length) : 0;
            const cell = (l, v) => '<div><div class="text-[10px] uppercase tracking-wider text-zinc-600">' + l + '</div>'
                + '<div class="text-lg font-semibold text-zinc-100">' + num(v) + suffix + '</div></div>';
            document.getElementById('chart-summary').innerHTML = cell('Total', total) + cell('Average', avg) + cell('Peak', peak);
        }

        function drawChart(suffix) {
            const host = document.getElementById('chart-svg');
            const pts = CHART.data;
            if (!pts.length) {
                host.innerHTML = '<p class="text-center text-sm text-zinc-600 py-16">No data in this period.</p>';
                return;
            }

            const W = Math.max(host.clientWidth || 720, 320), Hgt = 260;
            const ml = 52, mr = 16, mt = 12, mb = 30;
            const iw = W - ml - mr, ih = Hgt - mt - mb;
            const vals = pts.map(p => p.value);
            const max = Math.max(...vals, 1);
            // Round the top of the scale up so gridlines land on tidy numbers.
            const step = Math.pow(10, Math.floor(Math.log10(max))) / 2 || 1;
            const top = Math.ceil(max / step) * step;

            const x = i => ml + (pts.length === 1 ? iw / 2 : (i / (pts.length - 1)) * iw);
            const y = v => mt + ih - (v / top) * ih;

            let g = '';
            const TICKS = 4;
            for (let i = 0; i <= TICKS; i++) {
                const v = (top / TICKS) * i, yy = y(v);
                g += '<line x1="' + ml + '" y1="' + yy + '" x2="' + (W - mr) + '" y2="' + yy + '" stroke="' + GRID + '" stroke-width="1"/>';
                g += '<text x="' + (ml - 8) + '" y="' + (yy + 4) + '" text-anchor="end" font-size="10" fill="' + AXIS + '">' + num(Math.round(v)) + '</text>';
            }

            // Sparse x labels — at most 8, so they never collide.
            const stride = Math.max(1, Math.ceil(pts.length / 8));
            pts.forEach((p, i) => {
                if (i % stride !== 0 && i !== pts.length - 1) return;
                g += '<text x="' + x(i) + '" y="' + (Hgt - 8) + '" text-anchor="middle" font-size="10" fill="' + AXIS + '">' + esc(bucketLabel(p.bucket)) + '</text>';
            });

            const line = pts.map((p, i) => (i ? 'L' : 'M') + x(i).toFixed(1) + ' ' + y(p.value).toFixed(1)).join(' ');
            const area = line + ' L' + x(pts.length - 1).toFixed(1) + ' ' + (mt + ih) + ' L' + x(0).toFixed(1) + ' ' + (mt + ih) + ' Z';

            // Hover targets are full-height columns, so the pointer never has to
            // find a 3px dot.
            let hit = '';
            const colW = iw / Math.max(pts.length, 1);
            pts.forEach((p, i) => {
                hit += '<rect x="' + (x(i) - colW / 2) + '" y="' + mt + '" width="' + colW + '" height="' + ih + '" fill="transparent" '
                    + 'onmousemove="chartHover(event,' + i + ')" onmouseleave="chartOut()"></rect>';
            });

            host.innerHTML =
                '<svg viewBox="0 0 ' + W + ' ' + Hgt + '" width="100%" height="' + Hgt + '" role="img" aria-label="' + esc(document.getElementById('chart-title').textContent) + ' over time">'
                + '<defs><linearGradient id="cg" x1="0" y1="0" x2="0" y2="1">'
                + '<stop offset="0%" stop-color="' + SERIES + '" stop-opacity="0.28"/>'
                + '<stop offset="100%" stop-color="' + SERIES + '" stop-opacity="0"/>'
                + '</linearGradient></defs>'
                + g
                + '<path d="' + area + '" fill="url(#cg)"/>'
                + '<path d="' + line + '" fill="none" stroke="' + SERIES + '" stroke-width="2" stroke-linejoin="round" stroke-linecap="round"/>'
                + '<line id="chart-cross" class="hidden" y1="' + mt + '" y2="' + (mt + ih) + '" stroke="' + SERIES + '" stroke-width="1" stroke-dasharray="3 3"/>'
                + '<circle id="chart-dot" class="hidden" r="4.5" fill="' + SERIES + '" stroke="#0b0617" stroke-width="2"/>'
                + hit
                + '</svg>';

            CHART.geom = { x, y, suffix };
        }

        function chartHover(ev, i) {
            const p = CHART.data[i];
            if (!p || !CHART.geom) return;
            const cx = CHART.geom.x(i), cy = CHART.geom.y(p.value);
            const cross = document.getElementById('chart-cross');
            const dot = document.getElementById('chart-dot');
            cross.setAttribute('x1', cx); cross.setAttribute('x2', cx); cross.classList.remove('hidden');
            dot.setAttribute('cx', cx); dot.setAttribute('cy', cy); dot.classList.remove('hidden');

            const tip = document.getElementById('chart-tip');
            tip.innerHTML = '<div class="text-zinc-500">' + esc(bucketLabel(p.bucket)) + '</div>'
                + '<div class="text-zinc-100 font-semibold">' + num(p.value) + (CHART.geom.suffix || '') + '</div>';
            tip.classList.remove('hidden');
            const wrap = document.getElementById('chart-wrap').getBoundingClientRect();
            let left = ev.clientX - wrap.left + 12;
            if (left + 140 > wrap.width) left = ev.clientX - wrap.left - 140;
            tip.style.left = left + 'px';
            tip.style.top = Math.max(0, ev.clientY - wrap.top - 40) + 'px';
        }
        function chartOut() {
            document.getElementById('chart-cross').classList.add('hidden');
            document.getElementById('chart-dot').classList.add('hidden');
            document.getElementById('chart-tip').classList.add('hidden');
        }

        function drawChartTable(label, suffix) {
            const rows = CHART.data.map(p =>
                '<tr class="border-b border-zinc-800/30"><td class="px-3 py-1.5 text-zinc-500">' + esc(bucketLabel(p.bucket)) + '</td>'
                + '<td class="px-3 py-1.5 text-right text-zinc-200">' + num(p.value) + suffix + '</td></tr>');
            document.getElementById('chart-table').innerHTML =
                '<table class="w-full text-xs"><caption class="sr-only">' + esc(label) + '</caption>'
                + '<thead><tr><th class="px-3 py-1.5 text-left text-zinc-600 font-medium">Period</th>'
                + '<th class="px-3 py-1.5 text-right text-zinc-600 font-medium">Value</th></tr></thead><tbody>'
                + rows.join('') + '</tbody></table>';
        }
        function toggleChartTable() {
            CHART.showTable = !CHART.showTable;
            document.getElementById('chart-table').classList.toggle('hidden', !CHART.showTable);
        }
        window.addEventListener('resize', debounce(() => { if (A.tab === 'analytics' && CHART.data.length) drawChart(CHART.geom?.suffix || ''); }, 200));

        // ──────────────────────────────────────
        // Assets
        // ──────────────────────────────────────
        const debouncedAssets = debounce(() => loadAssets(1), 300);
        async function loadAssets(page) {
            A.assetPage = page || 1;
            const q = document.getElementById('asset-q').value.trim();
            const pub = document.getElementById('asset-published').value;
            let url = '/assets?page=' + A.assetPage;
            if (q) url += '&q=' + encodeURIComponent(q);
            if (pub) url += '&published=' + pub;
            try {
                const d = await api(url);
                const rows = (d.assets || []).map(a =>
                    '<tr class="hover:bg-white/[0.02]">'
                    + '<td class="px-4 py-3"><a href="/marketplace/asset/' + esc(a.slug) + '" class="text-zinc-200 hover:text-accent">' + esc(a.name) + '</a></td>'
                    + '<td class="px-4 py-3 text-zinc-500">' + esc(a.creator_name) + '</td>'
                    + '<td class="px-4 py-3 text-zinc-500">' + esc(a.category) + '</td>'
                    + '<td class="px-4 py-3 text-zinc-400">' + (a.price_credits === 0 ? 'Free' : num(a.price_credits) + ' cr') + '</td>'
                    + '<td class="px-4 py-3 text-zinc-500">' + num(a.downloads) + '</td>'
                    + '<td class="px-4 py-3">' + (a.published ? '<span class="text-xs text-emerald-400">Published</span>' : '<span class="text-xs text-amber-400">Draft</span>') + '</td>'
                    + '<td class="px-4 py-3 text-right whitespace-nowrap">'
                    + '<button onclick="togglePublish(\'' + a.id + '\')" class="text-xs text-zinc-400 hover:text-white mr-3">' + (a.published ? 'Unpublish' : 'Publish') + '</button>'
                    + '<button onclick="deleteAsset(\'' + a.id + '\')" class="text-xs text-red-400/70 hover:text-red-400">Delete</button>'
                    + '</td></tr>');
                document.getElementById('assets-table').innerHTML =
                    table(['Name', 'Creator', 'Category', 'Price', 'Downloads', 'Status', ''], rows, 'No assets found.');
                pagerByCount('assets-pager', A.assetPage, rows.length, 'loadAssets');
            } catch (e) { flash('err', e.message); }
        }
        async function togglePublish(id) {
            try { await api('/assets/' + id + '/publish', { method: 'PUT', headers: H() }); flash('ok', 'Asset updated.'); loadAssets(A.assetPage); }
            catch (e) { flash('err', e.message); }
        }
        async function deleteAsset(id) {
            if (!confirm('Delete this asset? This cannot be undone.')) return;
            try { await api('/assets/' + id, { method: 'DELETE', headers: H() }); flash('ok', 'Asset deleted.'); loadAssets(A.assetPage); }
            catch (e) { flash('err', e.message); }
        }

        // ──────────────────────────────────────
        // Review queue
        // ──────────────────────────────────────
        async function refreshReviewBadge() {
            try {
                const items = await api('/review-queue');
                const badge = document.getElementById('review-badge');
                const n = (items || []).length;
                if (badge) {
                    badge.textContent = n > 99 ? '99+' : n;
                    badge.classList.toggle('hidden', n === 0);
                }
                return items;
            } catch (e) { return []; }
        }
        async function loadReviewQueue() {
            const items = await refreshReviewBadge();
            const rows = (items || []).map(a =>
                '<tr class="hover:bg-white/[0.02]">'
                + '<td class="px-4 py-3"><a href="/marketplace/asset/' + esc(a.slug) + '" class="text-zinc-200 hover:text-accent">' + esc(a.name) + '</a></td>'
                + '<td class="px-4 py-3 text-zinc-500">' + esc(a.username) + '</td>'
                + '<td class="px-4 py-3 text-zinc-500">' + esc(a.category) + '</td>'
                + '<td class="px-4 py-3 text-zinc-600">' + shortDate(a.created_at) + '</td>'
                + '<td class="px-4 py-3 text-right whitespace-nowrap">'
                + '<button onclick="reviewAsset(\'' + a.id + '\',\'approved\')" class="text-xs text-emerald-400 hover:text-emerald-300 mr-3">Approve</button>'
                + '<button onclick="reviewAsset(\'' + a.id + '\',\'rejected\')" class="text-xs text-red-400/70 hover:text-red-400">Reject</button>'
                + '</td></tr>');
            document.getElementById('review-table').innerHTML =
                table(['Asset', 'Creator', 'Category', 'Submitted', ''], rows, 'Nothing waiting for review.');
        }
        async function reviewAsset(id, status) {
            try {
                await api('/assets/' + id + '/review', { method: 'PUT', headers: HJ(), body: JSON.stringify({ status }) });
                flash('ok', 'Asset ' + status + '.');
                loadReviewQueue();
            } catch (e) { flash('err', e.message); }
        }

        // ──────────────────────────────────────
        // Tags & subcategories
        // ──────────────────────────────────────
        async function loadTags() {
            try {
                const tags = await api('/tags/pending');
                const rows = (tags || []).map(t =>
                    '<tr class="hover:bg-white/[0.02]"><td class="px-4 py-3 text-zinc-200">' + esc(t.name) + '</td>'
                    + '<td class="px-4 py-3 text-zinc-600">' + shortDate(t.created_at) + '</td>'
                    + '<td class="px-4 py-3 text-right whitespace-nowrap">'
                    + '<button onclick="approveTag(\'' + t.id + '\')" class="text-xs text-emerald-400 hover:text-emerald-300 mr-3">Approve</button>'
                    + '<button onclick="deleteTag(\'' + t.id + '\')" class="text-xs text-red-400/70 hover:text-red-400">Reject</button>'
                    + '</td></tr>');
                document.getElementById('tags-pending').innerHTML = table(['Tag', 'Submitted', ''], rows, 'No tags waiting.');
            } catch (e) { flash('err', e.message); }

            try {
                const subs = await api('/subcategories/pending');
                const rows = (subs || []).map(sc =>
                    '<tr class="hover:bg-white/[0.02]"><td class="px-4 py-3 text-zinc-200">' + esc(sc.name) + '</td>'
                    + '<td class="px-4 py-3 font-mono text-xs text-zinc-500">' + esc(sc.slug) + '</td>'
                    + '<td class="px-4 py-3 text-right whitespace-nowrap">'
                    + '<button onclick="approveSubcat(\'' + sc.id + '\')" class="text-xs text-emerald-400 hover:text-emerald-300 mr-3">Approve</button>'
                    + '<button onclick="deleteSubcat(\'' + sc.id + '\')" class="text-xs text-red-400/70 hover:text-red-400">Reject</button>'
                    + '</td></tr>');
                document.getElementById('subcats-pending').innerHTML = table(['Subcategory', 'Slug', ''], rows, 'No subcategories waiting.');
            } catch (e) { flash('err', e.message); }
        }
        async function approveTag(id) {
            try { await api('/tags/' + id + '/approve', { method: 'PUT', headers: H() }); flash('ok', 'Tag approved.'); loadTags(); }
            catch (e) { flash('err', e.message); }
        }
        async function deleteTag(id) {
            try { await api('/tags/' + id, { method: 'DELETE', headers: H() }); flash('ok', 'Tag removed.'); loadTags(); }
            catch (e) { flash('err', e.message); }
        }
        async function approveSubcat(id) {
            try { await api('/subcategories/' + id + '/approve', { method: 'PUT', headers: H() }); flash('ok', 'Subcategory approved.'); loadTags(); }
            catch (e) { flash('err', e.message); }
        }
        async function deleteSubcat(id) {
            try { await api('/subcategories/' + id, { method: 'DELETE', headers: H() }); flash('ok', 'Subcategory removed.'); loadTags(); }
            catch (e) { flash('err', e.message); }
        }

        // ──────────────────────────────────────
        // Users
        // ──────────────────────────────────────
        const debouncedUsers = debounce(() => loadUsers(1), 300);
        async function loadUsers(page) {
            A.userPage = page || 1;
            const q = document.getElementById('user-q').value.trim();
            let url = '/users?page=' + A.userPage;
            if (q) url += '&q=' + encodeURIComponent(q);
            try {
                const users = await api(url);
                const rows = (users || []).map(u =>
                    '<tr class="hover:bg-white/[0.02]">'
                    + '<td class="px-4 py-3 text-zinc-200">' + esc(u.username) + '</td>'
                    + '<td class="px-4 py-3 text-zinc-500">' + esc(u.email) + '</td>'
                    + '<td class="px-4 py-3"><span class="text-xs px-2 py-0.5 rounded-full ' + (u.role === 'admin' ? 'bg-accent/15 text-accent' : 'bg-white/[0.04] text-zinc-400') + '">' + esc(u.role) + '</span></td>'
                    + '<td class="px-4 py-3 text-amber-400">' + num(u.credit_balance) + '</td>'
                    + '<td class="px-4 py-3 text-zinc-600">' + shortDate(u.created_at) + '</td>'
                    + '<td class="px-4 py-3 text-right"><button onclick="openUser(\'' + u.id + '\')" class="text-xs text-accent hover:text-accent-hover">Manage</button></td>'
                    + '</tr>');
                document.getElementById('users-table').innerHTML =
                    table(['User', 'Email', 'Role', 'Credits', 'Joined', ''], rows, 'No users found.');
                pagerByCount('users-pager', A.userPage, rows.length, 'loadUsers');
            } catch (e) { flash('err', e.message); }
        }
        async function openUser(id) {
            try {
                const d = await api('/users/' + id + '/detail');
                A.editing = d;
                document.getElementById('um-title').textContent = d.username;
                document.getElementById('um-sub').textContent = d.id;
                document.getElementById('um-username').value = d.username || '';
                document.getElementById('um-email').value = d.email || '';
                document.getElementById('um-role').value = d.role || 'user';
                document.getElementById('um-balance').value = d.credit_balance ?? 0;
                document.getElementById('um-adjust').value = '';
                document.getElementById('um-adjust-reason').value = '';
                const m = document.getElementById('user-modal');
                m.classList.remove('hidden'); m.classList.add('flex');
            } catch (e) { flash('err', e.message); }
        }
        function closeUserModal() {
            const m = document.getElementById('user-modal');
            m.classList.add('hidden'); m.classList.remove('flex');
            A.editing = null;
        }
        async function saveUser() {
            if (!A.editing) return;
            const body = {
                username: document.getElementById('um-username').value.trim(),
                email: document.getElementById('um-email').value.trim(),
                role: document.getElementById('um-role').value,
                credit_balance: parseInt(document.getElementById('um-balance').value) || 0
            };
            try {
                await api('/users/' + A.editing.id + '/edit', { method: 'PUT', headers: HJ(), body: JSON.stringify(body) });
                flash('ok', 'User saved.'); closeUserModal(); loadUsers(A.userPage);
            } catch (e) { flash('err', e.message); }
        }
        async function adjustCredits() {
            if (!A.editing) return;
            const amount = parseInt(document.getElementById('um-adjust').value);
            const reason = document.getElementById('um-adjust-reason').value.trim();
            if (!amount) return flash('err', 'Enter a non-zero amount.');
            if (!reason) return flash('err', 'A reason is required — it goes in the audit log.');
            try {
                await api('/users/' + A.editing.id + '/credit', { method: 'POST', headers: HJ(), body: JSON.stringify({ amount, reason }) });
                flash('ok', 'Adjustment applied.'); closeUserModal(); loadUsers(A.userPage);
            } catch (e) { flash('err', e.message); }
        }
        async function toggleBan() {
            if (!A.editing) return;
            const reason = prompt('Reason for ban (leave blank to cancel):');
            if (!reason) return;
            try {
                await api('/users/' + A.editing.id + '/ban', { method: 'POST', headers: HJ(), body: JSON.stringify({ reason }) });
                flash('ok', 'User banned.'); closeUserModal();
            } catch (e) { flash('err', e.message); }
        }

        // ──────────────────────────────────────
        // Gift cards
        // ──────────────────────────────────────
        const debouncedGifts = debounce(() => loadGifts(1), 300);
        async function loadGifts(page) {
            A.giftPage = page || 1;
            const q = document.getElementById('gift-q').value.trim();
            const st = document.getElementById('gift-status').value;
            let url = '/gift-cards?page=' + A.giftPage;
            if (q) url += '&q=' + encodeURIComponent(q);
            if (st) url += '&status=' + st;
            try {
                const d = await api(url);
                const badge = s => s === 'redeemed' ? 'text-emerald-400' : s === 'void' ? 'text-zinc-500' : 'text-amber-400';
                const rows = (d.items || []).map(g =>
                    '<tr class="hover:bg-white/[0.02]">'
                    + '<td class="px-4 py-3 font-mono text-xs text-zinc-200">' + esc(g.code) + '</td>'
                    + '<td class="px-4 py-3 text-amber-400">' + num(g.amount) + ' cr</td>'
                    + '<td class="px-4 py-3 text-zinc-500">' + esc(g.sender_name) + '</td>'
                    + '<td class="px-4 py-3 text-zinc-500">' + (g.redeemer_name ? esc(g.redeemer_name) : '—') + '</td>'
                    + '<td class="px-4 py-3"><span class="text-xs ' + badge(g.status) + '">' + esc(g.status) + '</span></td>'
                    + '<td class="px-4 py-3 text-zinc-600">' + shortDate(g.created_at) + '</td>'
                    + '<td class="px-4 py-3 text-right">'
                    + (g.status === 'pending'
                        ? '<button onclick="voidGift(\'' + g.id + '\',\'' + esc(g.code) + '\')" class="text-xs text-red-400/70 hover:text-red-400">Void &amp; refund</button>'
                        : '<span class="text-xs text-zinc-700">—</span>')
                    + '</td></tr>');
                document.getElementById('gifts-table').innerHTML =
                    table(['Code', 'Amount', 'Sender', 'Redeemed by', 'Status', 'Created', ''], rows, 'No gift cards found.');
                pager('gifts-pager', A.giftPage, d.total ?? rows.length, 'loadGifts');
            } catch (e) { flash('err', e.message); }
        }
        async function voidGift(id, code) {
            if (!confirm('Void gift card ' + code + '? The credits go back to the sender.')) return;
            try {
                const d = await api('/gift-cards/' + id + '/void', { method: 'PUT', headers: H() });
                flash('ok', 'Voided — ' + num(d.refunded) + ' credits refunded to the sender.');
                loadGifts(A.giftPage);
            } catch (e) { flash('err', e.message); }
        }

        // ──────────────────────────────────────
        // Game waiting list
        // ──────────────────────────────────────
        const debouncedWaitlist = debounce(() => loadWaitlist(1), 300);
        async function loadWaitlist(page) {
            A.wlPage = page || 1;
            const q = document.getElementById('wl-q').value.trim();
            let url = '/waitlist?page=' + A.wlPage;
            if (q) url += '&q=' + encodeURIComponent(q);
            try {
                const d = await api(url);
                document.getElementById('waitlist-stats').innerHTML =
                      statCard('Total signups', num(d.total), 'ph-envelope-simple', 'text-fuchsia-400')
                    + statCard('Today', num(d.today), 'ph-clock', 'text-sky-400')
                    + statCard('Last 7 days', num(d.week), 'ph-calendar', 'text-emerald-400')
                    + statCard('Last 30 days', num(d.month), 'ph-calendar-blank', 'text-amber-400');
                const rows = (d.items || []).map(w =>
                    '<tr class="hover:bg-white/[0.02]">'
                    + '<td class="px-4 py-3 text-zinc-200">' + esc(w.email) + '</td>'
                    + '<td class="px-4 py-3 text-zinc-500">' + esc(w.source) + '</td>'
                    + '<td class="px-4 py-3 text-zinc-600">' + shortDate(w.created_at) + '</td>'
                    + '</tr>');
                document.getElementById('waitlist-table').innerHTML =
                    table(['Email', 'Source', 'Signed up', ''], rows, 'No signups yet.');
                pager('waitlist-pager', A.wlPage, d.total ?? rows.length, 'loadWaitlist');
            } catch (e) { flash('err', e.message); }
        }
        // Exports the page currently in view — paging through gives the rest.
        function exportWaitlist() {
            const rows = Array.from(document.querySelectorAll('#waitlist-table tbody tr'))
                .map(tr => Array.from(tr.children).slice(0, 3).map(td => '"' + td.textContent.replace(/"/g, '""') + '"').join(','));
            if (!rows.length) return flash('err', 'Nothing to export.');
            const csv = 'email,source,signed_up\n' + rows.join('\n');
            const url = URL.createObjectURL(new Blob([csv], { type: 'text/csv' }));
            const a = document.createElement('a');
            a.href = url; a.download = 'renzora-waitlist-page' + A.wlPage + '.csv';
            document.body.appendChild(a); a.click(); a.remove();
            URL.revokeObjectURL(url);
        }

        // ──────────────────────────────────────
        // Statements
        // ──────────────────────────────────────
        const LEDGER_LABELS = {
            purchase: 'text-red-400', earning: 'text-emerald-400', topup: 'text-sky-400',
            withdrawal: 'text-amber-400', refund: 'text-zinc-400', admin_credit: 'text-accent'
        };
        async function loadLedger(page) {
            A.ledgerPage = page || 1;
            const t = document.getElementById('ledger-type').value;
            let url = '/transactions?page=' + A.ledgerPage;
            if (t) url += '&type=' + encodeURIComponent(t);
            try {
                const d = await api(url);
                const rows = (d.items || []).map(x =>
                    '<tr class="hover:bg-white/[0.02]">'
                    + '<td class="px-4 py-3 text-zinc-600">' + shortDate(x.created_at) + '</td>'
                    + '<td class="px-4 py-3 text-zinc-300">' + esc(x.username) + '</td>'
                    + '<td class="px-4 py-3"><span class="text-xs ' + (LEDGER_LABELS[x.type] || 'text-zinc-400') + '">' + esc(x.type) + '</span></td>'
                    + '<td class="px-4 py-3 font-medium ' + (x.amount < 0 ? 'text-red-400' : 'text-emerald-400') + '">' + (x.amount > 0 ? '+' : '') + num(x.amount) + '</td>'
                    + '<td class="px-4 py-3 text-zinc-500">' + esc(x.asset_name || x.reason || '—') + '</td></tr>');
                document.getElementById('ledger-table').innerHTML =
                    table(['Date', 'User', 'Type', 'Amount', 'Detail'], rows, 'No transactions found.');
                pager('ledger-pager', A.ledgerPage, d.total ?? rows.length, 'loadLedger');
            } catch (e) { flash('err', e.message); }
        }
        function fillMonths() {
            const sel = document.getElementById('ledger-month');
            const now = new Date();
            let html = '';
            for (let i = 0; i < 12; i++) {
                const d = new Date(now.getFullYear(), now.getMonth() - i, 1);
                html += '<option value="' + d.getFullYear() + '-' + (d.getMonth() + 1) + '">'
                    + d.toLocaleDateString('en-GB', { month: 'long', year: 'numeric' }) + '</option>';
            }
            sel.innerHTML = html;
        }
        // The CSV endpoint needs the bearer token, so fetch and save the blob —
        // a plain link would send no Authorization header.
        async function downloadStatement() {
            const [year, month] = document.getElementById('ledger-month').value.split('-');
            try {
                const res = await fetch('/api/admin/reports/monthly/' + year + '/' + month + '/csv', { headers: H() });
                if (!res.ok) throw new Error('Export failed');
                const url = URL.createObjectURL(await res.blob());
                const a = document.createElement('a');
                a.href = url;
                a.download = 'renzora-statement-' + year + '-' + String(month).padStart(2, '0') + '.csv';
                document.body.appendChild(a); a.click(); a.remove();
                URL.revokeObjectURL(url);
            } catch (e) { flash('err', e.message); }
        }

        // ──────────────────────────────────────
        // Global search
        // ──────────────────────────────────────
        let searchTimer;
        function adminGlobalSearch(q) {
            clearTimeout(searchTimer);
            const box = document.getElementById('admin-search-results');
            if (!q || q.trim().length < 2) { box.classList.add('hidden'); return; }
            searchTimer = setTimeout(async () => {
                try {
                    const d = await api('/global-search?q=' + encodeURIComponent(q.trim()));
                    let html = '';
                    (d.users || []).forEach(u => {
                        html += '<button onclick="openUser(\'' + u.id + '\');document.getElementById(\'admin-search-results\').classList.add(\'hidden\')" class="w-full text-left px-4 py-2.5 hover:bg-white/[0.04] transition-colors">'
                            + '<span class="text-sm text-zinc-200">' + esc(u.username) + '</span> <span class="text-xs text-zinc-600">' + esc(u.email) + '</span></button>';
                    });
                    (d.assets || []).forEach(a => {
                        html += '<a href="/marketplace/asset/' + esc(a.slug) + '" class="block px-4 py-2.5 hover:bg-white/[0.04] transition-colors">'
                            + '<span class="text-sm text-zinc-200">' + esc(a.name) + '</span> <span class="text-xs text-zinc-600">asset</span></a>';
                    });
                    box.innerHTML = html || '<p class="px-4 py-3 text-xs text-zinc-600">No matches.</p>';
                    box.classList.remove('hidden');
                } catch (e) { box.classList.add('hidden'); }
            }, 250);
        }
        document.addEventListener('click', function(e) {
            const input = document.getElementById('admin-search');
            const box = document.getElementById('admin-search-results');
            if (box && e.target !== input && !box.contains(e.target)) box.classList.add('hidden');
        });

        // ──────────────────────────────────────
        // Boot
        // ──────────────────────────────────────
        function showDenied() {
            document.getElementById('admin-loading').classList.add('hidden');
            document.getElementById('admin-root').classList.add('hidden');
            document.getElementById('admin-denied').classList.remove('hidden');
        }

        (async function init() {
            if (!tok()) { showDenied(); return; }
            // The real gate is verify_admin on every endpoint; probing /stats first
            // means a non-admin sees the denial rather than a half-drawn console.
            try { await api('/stats'); } catch (e) { showDenied(); return; }

            try {
                const u = JSON.parse(decodeURIComponent(document.cookie.match('(^|;)\\s*user\\s*=\\s*([^;]+)')?.pop() || ''));
                document.getElementById('admin-whoami').textContent = u.username || '';
            } catch (e) {}

            document.getElementById('admin-loading').classList.add('hidden');
            document.getElementById('admin-root').classList.remove('hidden');
            document.getElementById('admin-root').classList.add('flex');
            renderMenu();
            adminTab('overview');
            fillMonths();
            loadOverview();
            refreshReviewBadge();
        })();
        "##
        </script>
    }
}
