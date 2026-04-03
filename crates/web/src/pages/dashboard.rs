use leptos::prelude::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <section class="min-h-screen bg-[#060608]">

            // ── Hero header ──
            <div class="relative overflow-hidden border-b border-zinc-800/40">
                <div class="absolute inset-0 bg-gradient-to-br from-accent/[0.06] via-transparent to-purple-500/[0.04]"></div>
                <div class="absolute top-0 left-1/2 -translate-x-1/2 w-[800px] h-[400px] bg-accent/[0.03] rounded-full blur-3xl"></div>
                <div class="relative max-w-[1300px] mx-auto px-6 pt-10 pb-8">
                    <div class="flex items-end justify-between">
                        <div class="flex items-center gap-3">
                            <div class="w-10 h-10 rounded-xl bg-accent/10 border border-accent/20 flex items-center justify-center">
                                <i class="ph ph-chart-pie-slice text-xl text-accent"></i>
                            </div>
                            <div>
                                <h1 class="text-2xl font-bold tracking-tight">"Creator Dashboard"</h1>
                                <p class="text-zinc-500 text-sm">"Track performance and manage your content."</p>
                            </div>
                        </div>
                        <a href="/marketplace/upload" class="group inline-flex items-center gap-2 px-5 py-2.5 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_24px_rgba(99,102,241,0.25)] hover:-translate-y-0.5">
                            <i class="ph ph-plus-circle text-base group-hover:rotate-90 transition-transform duration-300"></i>"Create New"
                        </a>
                    </div>

                    // ── Stats inline ──
                    <div class="grid grid-cols-2 lg:grid-cols-4 gap-4 mt-6">
                        <div class="flex items-center gap-3 p-3 rounded-xl bg-white/[0.03] border border-zinc-800/40">
                            <div class="w-9 h-9 rounded-lg bg-accent/10 flex items-center justify-center shrink-0">
                                <i class="ph ph-package text-accent text-base"></i>
                            </div>
                            <div>
                                <div class="text-xl font-bold" id="stat-assets">"0"</div>
                                <div class="text-[10px] text-zinc-500 uppercase tracking-wider">"Assets"</div>
                            </div>
                        </div>
                        <div class="flex items-center gap-3 p-3 rounded-xl bg-white/[0.03] border border-zinc-800/40">
                            <div class="w-9 h-9 rounded-lg bg-cyan-500/10 flex items-center justify-center shrink-0">
                                <i class="ph ph-download-simple text-cyan-400 text-base"></i>
                            </div>
                            <div>
                                <div class="text-xl font-bold" id="stat-downloads">"0"</div>
                                <div class="text-[10px] text-zinc-500 uppercase tracking-wider">"Downloads"</div>
                            </div>
                        </div>
                        <div class="flex items-center gap-3 p-3 rounded-xl bg-white/[0.03] border border-zinc-800/40">
                            <div class="w-9 h-9 rounded-lg bg-green-500/10 flex items-center justify-center shrink-0">
                                <i class="ph ph-coins text-green-400 text-base"></i>
                            </div>
                            <div>
                                <div class="text-xl font-bold"><span id="stat-earnings">"0"</span><span class="text-xs font-normal text-zinc-500 ml-1">"cr"</span></div>
                                <div class="text-[10px] text-zinc-500 uppercase tracking-wider">"Earnings"</div>
                            </div>
                        </div>
                        <div class="flex items-center gap-3 p-3 rounded-xl bg-white/[0.03] border border-zinc-800/40">
                            <div class="w-9 h-9 rounded-lg bg-purple-500/10 flex items-center justify-center shrink-0">
                                <i class="ph ph-wallet text-purple-400 text-base"></i>
                            </div>
                            <div>
                                <div class="text-xl font-bold"><span id="stat-balance">"0"</span><span class="text-xs font-normal text-zinc-500 ml-1">"cr"</span></div>
                                <div class="text-[10px] text-zinc-500 uppercase tracking-wider">"Balance"</div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="max-w-[1300px] mx-auto px-6 py-6">

                // ── Loading ──
                <div id="dashboard-loading" class="flex flex-col items-center justify-center py-32">
                    <div class="w-8 h-8 border-2 border-zinc-800 border-t-accent rounded-full animate-spin"></div>
                    <p class="text-zinc-600 text-sm mt-4">"Loading..."</p>
                </div>

                // ── Content ──
                <div id="dashboard-content" class="hidden">

                    // Toolbar: tabs + view toggle
                    <div class="flex items-center justify-between mb-4">
                        <div class="flex items-center gap-1 p-1 bg-white/[0.02] rounded-lg border border-zinc-800/40">
                            <button id="tab-assets" onclick="switchTab('assets')" class="px-3.5 py-1.5 rounded-md text-xs font-medium bg-white/[0.06] text-white transition-all">"Assets"</button>
                            <button id="tab-games" onclick="switchTab('games')" class="px-3.5 py-1.5 rounded-md text-xs font-medium text-zinc-500 hover:text-zinc-300 transition-all">"Games"</button>
                            <button id="tab-earnings" onclick="switchTab('earnings')" class="px-3.5 py-1.5 rounded-md text-xs font-medium text-zinc-500 hover:text-zinc-300 transition-all">"Earnings"</button>
                            <button id="tab-progress" onclick="switchTab('progress')" class="px-3.5 py-1.5 rounded-md text-xs font-medium text-zinc-500 hover:text-zinc-300 transition-all">"Seller Level"</button>
                        </div>
                        <div class="flex items-center gap-2">
                            <span id="db-item-count" class="text-xs text-zinc-600 mr-2"></span>
                            <div class="flex items-center gap-0.5 p-0.5 bg-white/[0.02] rounded-lg border border-zinc-800/40" id="view-toggle-wrap">
                                <button onclick="setDbView('list')" id="view-list" class="p-1.5 rounded-md text-zinc-300 bg-white/[0.06] transition-all" title="List view">
                                    <i class="ph ph-list text-sm"></i>
                                </button>
                                <button onclick="setDbView('grid')" id="view-grid" class="p-1.5 rounded-md text-zinc-500 hover:text-zinc-300 transition-all" title="Grid view">
                                    <i class="ph ph-grid-four text-sm"></i>
                                </button>
                            </div>
                        </div>
                    </div>

                    // ── Assets tab ──
                    <div id="panel-assets">
                        <div id="assets-container"></div>
                        <div id="assets-pagination" class="hidden flex items-center justify-center gap-3 py-4">
                            <button onclick="dbGoPage('assets', dbAssetPage - 1)" id="assets-prev" class="w-8 h-8 rounded-lg bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:text-zinc-200 hover:border-zinc-600 transition-all disabled:opacity-30 disabled:pointer-events-none flex items-center justify-center">
                                <i class="ph ph-caret-left text-sm"></i>
                            </button>
                            <div class="flex items-center gap-2 text-xs text-zinc-500">
                                "Page "
                                <input type="number" id="assets-page-input" min="1" value="1" onchange="dbGoPage('assets', parseInt(this.value)||1)" class="w-12 px-2 py-1 bg-white/[0.03] border border-zinc-800/50 rounded-lg text-zinc-50 text-xs text-center outline-none focus:border-accent/50 transition-all" />
                                " of "<span id="assets-total-pages">"1"</span>
                            </div>
                            <button onclick="dbGoPage('assets', dbAssetPage + 1)" id="assets-next" class="w-8 h-8 rounded-lg bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:text-zinc-200 hover:border-zinc-600 transition-all disabled:opacity-30 disabled:pointer-events-none flex items-center justify-center">
                                <i class="ph ph-caret-right text-sm"></i>
                            </button>
                        </div>
                    </div>

                    // ── Games tab ──
                    <div id="panel-games" class="hidden">
                        <div id="games-container"></div>
                        <div id="games-pagination" class="hidden flex items-center justify-center gap-3 py-4">
                            <button onclick="dbGoPage('games', dbGamePage - 1)" id="games-prev" class="w-8 h-8 rounded-lg bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:text-zinc-200 hover:border-zinc-600 transition-all disabled:opacity-30 disabled:pointer-events-none flex items-center justify-center">
                                <i class="ph ph-caret-left text-sm"></i>
                            </button>
                            <div class="flex items-center gap-2 text-xs text-zinc-500">
                                "Page "
                                <input type="number" id="games-page-input" min="1" value="1" onchange="dbGoPage('games', parseInt(this.value)||1)" class="w-12 px-2 py-1 bg-white/[0.03] border border-zinc-800/50 rounded-lg text-zinc-50 text-xs text-center outline-none focus:border-accent/50 transition-all" />
                                " of "<span id="games-total-pages">"1"</span>
                            </div>
                            <button onclick="dbGoPage('games', dbGamePage + 1)" id="games-next" class="w-8 h-8 rounded-lg bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:text-zinc-200 hover:border-zinc-600 transition-all disabled:opacity-30 disabled:pointer-events-none flex items-center justify-center">
                                <i class="ph ph-caret-right text-sm"></i>
                            </button>
                        </div>
                    </div>

                    // ── Earnings tab ──
                    <div id="panel-earnings" class="hidden">
                        <div id="earnings-container"></div>
                    </div>

                    // ── Seller Level panel ──
                    <div id="panel-progress" class="hidden">
                        <div id="progress-container">
                            <div class="text-center py-12"><div class="w-6 h-6 border-2 border-zinc-800 border-t-accent rounded-full animate-spin mx-auto"></div></div>
                        </div>
                    </div>

                </div>
            </div>
        </section>

        <script>
            r#"
            const DB_PER_PAGE = 20;
            let dbView = 'list';
            let dbAssetPage = 1;
            let dbGamePage = 1;
            let allAssets = [];
            let allGames = [];
            let allEarnings = [];
            let progressLoaded = false;

            function parseDate(s) {
                if (!s) return null;
                const iso = s.replace(/^(\d{4}-\d{2}-\d{2})\s+(\d{2}:\d{2}:\d{2}).*?\s+([+-]\d{2}):(\d{2}).*$/, '$1T$2$3:$4');
                const d = new Date(iso);
                return isNaN(d.getTime()) ? null : d;
            }
            function fmtShortDate(s) {
                const d = parseDate(s);
                return d ? d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' }) : '';
            }

            // ── Tab switching ──
            function switchTab(tab) {
                ['assets', 'games', 'earnings', 'progress'].forEach(t => {
                    document.getElementById('panel-' + t)?.classList.toggle('hidden', t !== tab);
                    const btn = document.getElementById('tab-' + t);
                    if (!btn) return;
                    if (t === tab) { btn.classList.add('bg-white/[0.06]', 'text-white'); btn.classList.remove('text-zinc-500'); }
                    else { btn.classList.remove('bg-white/[0.06]', 'text-white'); btn.classList.add('text-zinc-500'); }
                });
                document.getElementById('view-toggle-wrap').classList.toggle('hidden', tab === 'earnings' || tab === 'progress');
                updateItemCount(tab);
                if (tab === 'progress' && !progressLoaded) loadSellerProgress();
            }

            function updateItemCount(tab) {
                const el = document.getElementById('db-item-count');
                if (tab === 'assets') el.textContent = allAssets.length + ' assets';
                else if (tab === 'games') el.textContent = allGames.length + ' games';
                else el.textContent = allEarnings.length + ' transactions';
            }

            // ── View toggle ──
            function setDbView(v) {
                dbView = v;
                document.getElementById('view-list').className = 'p-1.5 rounded-md transition-all ' + (v === 'list' ? 'text-zinc-300 bg-white/[0.06]' : 'text-zinc-500 hover:text-zinc-300');
                document.getElementById('view-grid').className = 'p-1.5 rounded-md transition-all ' + (v === 'grid' ? 'text-zinc-300 bg-white/[0.06]' : 'text-zinc-500 hover:text-zinc-300');
                renderAssets();
                renderGames();
            }

            // ── Counter animation ──
            function animateCounter(id, target) {
                const el = document.getElementById(id);
                if (!el || target === 0) { if (el) el.textContent = '0'; return; }
                let current = 0;
                const steps = 25;
                const inc = target / steps;
                const timer = setInterval(() => {
                    current += inc;
                    if (current >= target) { current = target; clearInterval(timer); }
                    el.textContent = Math.round(current).toLocaleString();
                }, 20);
            }

            // ── Asset renderers ──
            function assetListRow(a, i) {
                const status = a.published
                    ? '<span class="w-1.5 h-1.5 rounded-full bg-green-400 inline-block"></span><span class="text-green-400">Live</span>'
                    : '<span class="w-1.5 h-1.5 rounded-full bg-amber-400 inline-block"></span><span class="text-amber-400">Draft</span>';
                const thumb = a.thumbnail_url
                    ? `<img src="${a.thumbnail_url}" class="w-full h-full object-cover" />`
                    : `<i class="ph ph-package text-lg text-zinc-600"></i>`;
                return `<a href="/marketplace/asset/${a.slug}" class="flex items-center gap-4 px-5 py-3.5 hover:bg-white/[0.02] transition-all group db-row" style="animation-delay:${i * 20}ms">
                    <div class="w-11 h-11 rounded-lg bg-zinc-900 border border-zinc-800/50 flex items-center justify-center shrink-0 overflow-hidden">${thumb}</div>
                    <div class="flex-1 min-w-0">
                        <div class="text-sm font-medium group-hover:text-accent transition-colors truncate">${a.name}</div>
                        <div class="flex items-center gap-2 mt-0.5 text-[11px] text-zinc-600">
                            <span>${a.category}</span><span class="text-zinc-800">·</span>
                            <span>${a.downloads.toLocaleString()} dl</span><span class="text-zinc-800">·</span>
                            <span>${a.views.toLocaleString()} views</span>
                        </div>
                    </div>
                    <div class="flex items-center gap-3 shrink-0">
                        <div class="flex items-center gap-1.5 text-[11px]">${status}</div>
                        <span class="text-xs text-zinc-400 min-w-[50px] text-right">${a.price_credits === 0 ? 'Free' : a.price_credits.toLocaleString() + ' cr'}</span>
                        <a href="/marketplace/asset/${a.slug}/edit" onclick="event.stopPropagation()" class="p-1.5 rounded-md text-zinc-700 hover:text-zinc-300 hover:bg-white/[0.05] transition-all opacity-0 group-hover:opacity-100"><i class="ph ph-pencil-simple text-sm"></i></a>
                    </div>
                </a>`;
            }

            function assetGridCard(a, i) {
                const thumb = a.thumbnail_url
                    ? `<img src="${a.thumbnail_url}" class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300" />`
                    : `<div class="w-full h-full flex items-center justify-center"><i class="ph ph-package text-2xl text-zinc-700"></i></div>`;
                const statusColor = a.published ? 'bg-green-400' : 'bg-amber-400';
                return `<a href="/marketplace/asset/${a.slug}" class="block group db-row" style="animation-delay:${i * 20}ms">
                    <div class="bg-white/[0.02] border border-zinc-800/40 rounded-xl overflow-hidden hover:border-accent/20 transition-all">
                        <div class="aspect-[4/3] bg-zinc-900 relative overflow-hidden">
                            ${thumb}
                            <div class="absolute top-2 right-2 w-2 h-2 rounded-full ${statusColor}"></div>
                        </div>
                        <div class="p-3">
                            <div class="text-sm font-medium truncate group-hover:text-accent transition-colors">${a.name}</div>
                            <div class="flex items-center justify-between mt-1.5 text-[11px] text-zinc-500">
                                <span>${a.downloads.toLocaleString()} dl</span>
                                <span class="font-medium ${a.price_credits === 0 ? 'text-emerald-400' : 'text-zinc-400'}">${a.price_credits === 0 ? 'Free' : a.price_credits.toLocaleString() + ' cr'}</span>
                            </div>
                        </div>
                    </div>
                </a>`;
            }

            function renderAssets() {
                const container = document.getElementById('assets-container');
                const start = (dbAssetPage - 1) * DB_PER_PAGE;
                const page = allAssets.slice(start, start + DB_PER_PAGE);
                const totalPages = Math.ceil(allAssets.length / DB_PER_PAGE);

                if (!page.length) {
                    container.innerHTML = `<div class="text-center py-16 rounded-xl border border-zinc-800/40 bg-white/[0.01]">
                        <div class="w-12 h-12 rounded-xl bg-white/[0.03] border border-zinc-800/40 flex items-center justify-center mx-auto mb-3"><i class="ph ph-package text-xl text-zinc-700"></i></div>
                        <p class="text-zinc-600 text-sm">No assets yet</p>
                        <a href="/marketplace/upload" class="text-accent text-sm mt-1 inline-block hover:underline">Upload your first asset <i class="ph ph-arrow-right text-xs"></i></a>
                    </div>`;
                    document.getElementById('assets-pagination').classList.add('hidden');
                    return;
                }

                if (dbView === 'grid') {
                    container.innerHTML = `<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3">${page.map((a, i) => assetGridCard(a, i)).join('')}</div>`;
                } else {
                    container.innerHTML = `<div class="rounded-xl border border-zinc-800/40 bg-white/[0.01] overflow-hidden divide-y divide-zinc-800/30">${page.map((a, i) => assetListRow(a, i)).join('')}</div>`;
                }

                // Pagination
                const pagEl = document.getElementById('assets-pagination');
                if (totalPages <= 1) { pagEl.classList.add('hidden'); return; }
                pagEl.classList.remove('hidden');
                document.getElementById('assets-page-input').value = dbAssetPage;
                document.getElementById('assets-total-pages').textContent = totalPages;
                document.getElementById('assets-prev').disabled = dbAssetPage <= 1;
                document.getElementById('assets-next').disabled = dbAssetPage >= totalPages;

                // Animate rows
                if (typeof anime !== 'undefined') {
                    anime({ targets: '#assets-container .db-row', opacity: [0,1], translateX: [-15,0], delay: anime.stagger(30), duration: 400, easing: 'easeOutCubic' });
                }
            }

            // ── Game renderers ──
            function gameListRow(g, i) {
                const status = g.published
                    ? '<span class="w-1.5 h-1.5 rounded-full bg-green-400 inline-block"></span><span class="text-green-400">Live</span>'
                    : '<span class="w-1.5 h-1.5 rounded-full bg-amber-400 inline-block"></span><span class="text-amber-400">Draft</span>';
                const thumb = g.thumbnail_url
                    ? `<img src="${g.thumbnail_url}" class="w-full h-full object-cover" />`
                    : `<i class="ph ph-game-controller text-lg text-zinc-600"></i>`;
                return `<a href="/games/${g.slug}" class="flex items-center gap-4 px-5 py-3.5 hover:bg-white/[0.02] transition-all group db-row" style="animation-delay:${i * 20}ms">
                    <div class="w-11 h-11 rounded-lg bg-zinc-900 border border-zinc-800/50 flex items-center justify-center shrink-0 overflow-hidden">${thumb}</div>
                    <div class="flex-1 min-w-0">
                        <div class="text-sm font-medium group-hover:text-accent transition-colors truncate">${g.name}</div>
                        <div class="flex items-center gap-2 mt-0.5 text-[11px] text-zinc-600">
                            <span>${g.category}</span><span class="text-zinc-800">·</span>
                            <span>${g.downloads.toLocaleString()} downloads</span>
                        </div>
                    </div>
                    <div class="flex items-center gap-3 shrink-0">
                        <div class="flex items-center gap-1.5 text-[11px]">${status}</div>
                        <span class="text-xs text-zinc-400 min-w-[50px] text-right">${g.price_credits === 0 ? 'Free' : g.price_credits.toLocaleString() + ' cr'}</span>
                    </div>
                </a>`;
            }

            function gameGridCard(g, i) {
                const thumb = g.thumbnail_url
                    ? `<img src="${g.thumbnail_url}" class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300" />`
                    : `<div class="w-full h-full flex items-center justify-center"><i class="ph ph-game-controller text-2xl text-zinc-700"></i></div>`;
                const statusColor = g.published ? 'bg-green-400' : 'bg-amber-400';
                return `<a href="/games/${g.slug}" class="block group db-row" style="animation-delay:${i * 20}ms">
                    <div class="bg-white/[0.02] border border-zinc-800/40 rounded-xl overflow-hidden hover:border-accent/20 transition-all">
                        <div class="aspect-[4/3] bg-zinc-900 relative overflow-hidden">
                            ${thumb}
                            <div class="absolute top-2 right-2 w-2 h-2 rounded-full ${statusColor}"></div>
                        </div>
                        <div class="p-3">
                            <div class="text-sm font-medium truncate group-hover:text-accent transition-colors">${g.name}</div>
                            <div class="flex items-center justify-between mt-1.5 text-[11px] text-zinc-500">
                                <span>${g.downloads.toLocaleString()} dl</span>
                                <span class="font-medium ${g.price_credits === 0 ? 'text-emerald-400' : 'text-zinc-400'}">${g.price_credits === 0 ? 'Free' : g.price_credits.toLocaleString() + ' cr'}</span>
                            </div>
                        </div>
                    </div>
                </a>`;
            }

            function renderGames() {
                const container = document.getElementById('games-container');
                const start = (dbGamePage - 1) * DB_PER_PAGE;
                const page = allGames.slice(start, start + DB_PER_PAGE);
                const totalPages = Math.ceil(allGames.length / DB_PER_PAGE);

                if (!page.length) {
                    container.innerHTML = `<div class="text-center py-16 rounded-xl border border-zinc-800/40 bg-white/[0.01]">
                        <div class="w-12 h-12 rounded-xl bg-white/[0.03] border border-zinc-800/40 flex items-center justify-center mx-auto mb-3"><i class="ph ph-game-controller text-xl text-zinc-700"></i></div>
                        <p class="text-zinc-600 text-sm">No games yet</p>
                        <a href="/marketplace/upload" class="text-accent text-sm mt-1 inline-block hover:underline">Publish your first game <i class="ph ph-arrow-right text-xs"></i></a>
                    </div>`;
                    document.getElementById('games-pagination').classList.add('hidden');
                    return;
                }

                if (dbView === 'grid') {
                    container.innerHTML = `<div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3">${page.map((g, i) => gameGridCard(g, i)).join('')}</div>`;
                } else {
                    container.innerHTML = `<div class="rounded-xl border border-zinc-800/40 bg-white/[0.01] overflow-hidden divide-y divide-zinc-800/30">${page.map((g, i) => gameListRow(g, i)).join('')}</div>`;
                }

                const pagEl = document.getElementById('games-pagination');
                if (totalPages <= 1) { pagEl.classList.add('hidden'); return; }
                pagEl.classList.remove('hidden');
                document.getElementById('games-page-input').value = dbGamePage;
                document.getElementById('games-total-pages').textContent = totalPages;
                document.getElementById('games-prev').disabled = dbGamePage <= 1;
                document.getElementById('games-next').disabled = dbGamePage >= totalPages;

                if (typeof anime !== 'undefined') {
                    anime({ targets: '#games-container .db-row', opacity: [0,1], translateX: [-15,0], delay: anime.stagger(30), duration: 400, easing: 'easeOutCubic' });
                }
            }

            // ── Earnings renderer ──
            function renderEarnings() {
                const container = document.getElementById('earnings-container');
                if (!allEarnings.length) {
                    container.innerHTML = `<div class="text-center py-16 rounded-xl border border-zinc-800/40 bg-white/[0.01]">
                        <div class="w-12 h-12 rounded-xl bg-white/[0.03] border border-zinc-800/40 flex items-center justify-center mx-auto mb-3"><i class="ph ph-chart-line-up text-xl text-zinc-700"></i></div>
                        <p class="text-zinc-600 text-sm">No earnings yet</p>
                        <p class="text-zinc-700 text-xs mt-1">Earnings appear when users purchase your content.</p>
                    </div>`;
                    return;
                }
                container.innerHTML = `<div class="rounded-xl border border-zinc-800/40 bg-white/[0.01] overflow-hidden divide-y divide-zinc-800/30">${allEarnings.map((e, i) => `
                    <div class="flex items-center justify-between px-5 py-3 hover:bg-white/[0.015] transition-all db-row" style="animation-delay:${i * 20}ms">
                        <div class="flex items-center gap-3 min-w-0">
                            <div class="w-8 h-8 rounded-lg bg-green-500/10 flex items-center justify-center shrink-0">
                                <i class="ph ph-arrow-down-left text-green-400 text-sm"></i>
                            </div>
                            <span class="text-sm text-zinc-300 truncate">${e.asset_name}</span>
                        </div>
                        <div class="flex items-center gap-4 shrink-0">
                            <span class="text-sm font-semibold text-green-400">+${e.amount.toLocaleString()}</span>
                            <span class="text-[11px] text-zinc-600 min-w-[70px] text-right">${fmtShortDate(e.created_at)}</span>
                        </div>
                    </div>
                `).join('')}</div>`;
            }

            async function loadSellerProgress() {
                progressLoaded = true;
                const container = document.getElementById('progress-container');
                try {
                    const [levelRes, progressRes] = await Promise.all([
                        fetch('/api/levels/me', { headers }),
                        fetch('/api/levels/seller-progress', { headers }),
                    ]);
                    const level = levelRes.ok ? await levelRes.json() : null;
                    const progress = progressRes.ok ? await progressRes.json() : null;

                    if (!level || !progress) {
                        container.innerHTML = '<p class="text-zinc-600 text-sm text-center py-12">Could not load seller progress.</p>';
                        return;
                    }

                    const cur = progress.current_level;
                    const next = progress.next_level;

                    container.innerHTML = `
                        <div class="space-y-4">
                            <!-- Current level card -->
                            <div class="rounded-xl border border-zinc-800/40 bg-white/[0.01] p-5">
                                <div class="flex items-center justify-between mb-3">
                                    <div class="flex items-center gap-3">
                                        <div class="w-10 h-10 rounded-xl flex items-center justify-center" style="background:${cur.color}15;border:1px solid ${cur.color}30">
                                            <i class="ph ph-storefront text-xl" style="color:${cur.color}"></i>
                                        </div>
                                        <div>
                                            <div class="text-sm font-semibold" style="color:${cur.color}">${cur.name}</div>
                                            <div class="text-[10px] text-zinc-500">Level ${cur.level}</div>
                                        </div>
                                    </div>
                                    ${next ? `<div class="text-right">
                                        <div class="text-[10px] text-zinc-500">Next: <span style="color:${next.color}">${next.name}</span></div>
                                        <div class="text-[10px] text-zinc-600">${progress.xp_to_next} XP to go</div>
                                    </div>` : '<div class="text-[10px] text-green-400">Max level!</div>'}
                                </div>
                                ${next ? `
                                <div class="h-2 bg-zinc-800 rounded-full overflow-hidden">
                                    <div class="h-full rounded-full transition-all" style="width:${progress.progress_percent.toFixed(0)}%;background:${cur.color}"></div>
                                </div>
                                <div class="text-[10px] text-zinc-600 mt-1">${level.seller_xp} / ${next.min_xp} seller XP (${progress.progress_percent.toFixed(0)}%)</div>
                                ` : ''}
                                ${cur.perks ? `<div class="text-[10px] text-zinc-500 mt-2"><i class="ph ph-gift"></i> Perks: ${cur.perks}</div>` : ''}
                            </div>

                            <!-- User level card -->
                            <div class="rounded-xl border border-zinc-800/40 bg-white/[0.01] p-5">
                                <div class="flex items-center gap-3 mb-2">
                                    <div class="w-8 h-8 rounded-lg bg-accent/10 flex items-center justify-center">
                                        <i class="ph ph-lightning text-accent"></i>
                                    </div>
                                    <div>
                                        <div class="text-sm font-semibold text-accent">Level ${level.level}</div>
                                        <div class="text-[10px] text-zinc-500">${level.total_xp.toLocaleString()} total XP</div>
                                    </div>
                                </div>
                                <div class="h-1.5 bg-zinc-800 rounded-full overflow-hidden">
                                    <div class="h-full bg-accent rounded-full" style="width:${level.progress_percent.toFixed(0)}%"></div>
                                </div>
                                <div class="text-[10px] text-zinc-600 mt-1">${level.xp_for_current_level} / ${level.xp_for_next_level} XP to level ${level.level + 1}</div>
                            </div>

                            <!-- Tasks -->
                            ${progress.tasks.length ? `
                            <div class="rounded-xl border border-zinc-800/40 bg-white/[0.01] p-5">
                                <h3 class="text-xs font-semibold text-zinc-300 mb-3 flex items-center gap-1.5"><i class="ph ph-list-checks text-accent"></i>Tasks for next level</h3>
                                <div class="space-y-2">
                                    ${progress.tasks.map(t => `
                                        <div class="flex items-center gap-3">
                                            <div class="w-5 h-5 rounded-md flex items-center justify-center ${t.completed ? 'bg-green-500/20' : 'bg-zinc-800'}">
                                                <i class="ph ${t.completed ? 'ph-check text-green-400' : 'ph-circle text-zinc-600'} text-xs"></i>
                                            </div>
                                            <div class="flex-1 min-w-0">
                                                <div class="text-xs ${t.completed ? 'text-zinc-400 line-through' : 'text-zinc-300'}">${t.description}</div>
                                                <div class="h-1 bg-zinc-800 rounded-full mt-1 overflow-hidden">
                                                    <div class="h-full bg-accent/60 rounded-full" style="width:${Math.min(100, t.current_value/t.target_value*100).toFixed(0)}%"></div>
                                                </div>
                                            </div>
                                            <div class="text-[10px] text-zinc-500 shrink-0">${t.current_value}/${t.target_value}</div>
                                            <div class="text-[10px] text-accent shrink-0">+${t.xp_reward} XP</div>
                                        </div>
                                    `).join('')}
                                </div>
                            </div>` : ''}
                        </div>`;

                    if (typeof anime !== 'undefined') {
                        anime({ targets: '#progress-container > div > div', opacity: [0,1], translateY: [20,0], delay: anime.stagger(80), duration: 500, easing: 'easeOutCubic' });
                    }
                } catch(e) { console.error(e); container.innerHTML = '<p class="text-zinc-600 text-sm text-center py-12">Failed to load.</p>'; }
            }

            function dbGoPage(type, p) {
                if (type === 'assets') {
                    const total = Math.ceil(allAssets.length / DB_PER_PAGE);
                    dbAssetPage = Math.max(1, Math.min(p, total));
                    renderAssets();
                } else {
                    const total = Math.ceil(allGames.length / DB_PER_PAGE);
                    dbGamePage = Math.max(1, Math.min(p, total));
                    renderGames();
                }
            }

            // ── Init ──
            (async function() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }

                const headers = { 'Authorization': 'Bearer ' + token };

                try {
                    const [statsRes, earningsRes, assetsRes, gamesRes] = await Promise.all([
                        fetch('/api/creator/stats', { headers }),
                        fetch('/api/creator/earnings', { headers }),
                        fetch('/api/marketplace/my-assets', { headers }),
                        fetch('/api/games/my-games', { headers }),
                    ]);

                    if (statsRes.ok) {
                        const stats = await statsRes.json();
                        animateCounter('stat-assets', stats.total_assets);
                        animateCounter('stat-downloads', stats.total_downloads);
                        animateCounter('stat-earnings', stats.total_earnings);
                        animateCounter('stat-balance', stats.credit_balance);
                    }

                    if (assetsRes.ok) {
                        const data = await assetsRes.json();
                        allAssets = data.assets || [];
                    }
                    if (gamesRes.ok) {
                        const data = await gamesRes.json();
                        allGames = data.games || [];
                    }
                    if (earningsRes.ok) {
                        const data = await earningsRes.json();
                        allEarnings = data.earnings || [];
                    }

                    renderAssets();
                    renderGames();
                    renderEarnings();
                    updateItemCount('assets');
                } catch(e) { console.error('Dashboard error:', e); }

                document.getElementById('dashboard-loading').classList.add('hidden');
                document.getElementById('dashboard-content').classList.remove('hidden');

                // anime.js entrance animations
                if (typeof anime !== 'undefined') {
                    // Stat cards bounce in
                    anime({ targets: '.grid.grid-cols-2 > div', opacity: [0,1], translateY: [25,0], scale: [0.9,1], delay: anime.stagger(80), duration: 600, easing: 'easeOutBack' });
                    // Tab bar slide in
                    anime({ targets: '.flex.items-center.justify-between', opacity: [0,1], translateX: [-20,0], duration: 500, easing: 'easeOutCubic', delay: 300 });
                }
            })();
            "#
        </script>

        <style>
            r#"
            .db-row { opacity: 0; }
            "#
        </style>
    }
}
