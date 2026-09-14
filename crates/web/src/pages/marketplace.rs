use leptos::prelude::*;
use leptos_meta::{Title, Meta};

#[component]
pub fn MarketplacePage() -> impl IntoView {
    view! {
        <Title text="Renzora Marketplace, Models, Shaders & Scripts for Bevy" />
        <Meta name="description" content="Browse the Renzora marketplace: ready-made 3D models, shaders, scripts and assets for the Bevy editor. Free and paid packs you can import straight into your Bevy project." />

        <section class="min-h-[calc(100vh-3rem)]">

        <div class="flex">
            // ── Left Sidebar: Categories ──
            <aside class="w-64 shrink-0 border-r border-white/[0.04] sticky top-[60px] h-[calc(100vh-60px)] overflow-y-auto hidden lg:block bg-surface">
                <div class="p-3">
                    <div class="menu-label">"Categories"</div>
                    <div id="mp-sidebar-cats">
                        <button class="menu-item active">
                            <i class="ph ph-squares-four text-base"></i>"All"
                        </button>
                    </div>
                </div>
                <div class="p-3 border-t border-zinc-800">
                    <div class="menu-label">"Price"</div>
                    <button onclick="setPrice('all')" id="price-all" class="mp-price-btn menu-item active">
                        <i class="ph ph-coins text-base"></i>"All Prices"
                    </button>
                    <button onclick="setPrice('free')" id="price-free" class="mp-price-btn menu-item">
                        <i class="ph ph-gift text-base"></i>"Free Only"
                    </button>
                    <button onclick="setPrice('paid')" id="price-paid" class="mp-price-btn menu-item">
                        <i class="ph ph-credit-card text-base"></i>"Paid Only"
                    </button>
                </div>
                <div class="p-3 border-t border-zinc-800">
                    <div class="menu-label">"Min Rating"</div>
                    <button onclick="setMinRating(0)" id="rating-0" class="mp-rating-btn menu-item active">
                        <i class="ph ph-star text-base"></i>"Any"
                    </button>
                    <button onclick="setMinRating(3)" id="rating-3" class="mp-rating-btn menu-item">
                        <span class="text-amber-400">"★★★"</span><span class="text-zinc-600">"☆☆"</span>"& up"
                    </button>
                    <button onclick="setMinRating(4)" id="rating-4" class="mp-rating-btn menu-item">
                        <span class="text-amber-400">"★★★★"</span><span class="text-zinc-600">"☆"</span>"& up"
                    </button>
                    <button onclick="setMinRating(5)" id="rating-5" class="mp-rating-btn menu-item">
                        <span class="text-amber-400">"★★★★★"</span>"only"
                    </button>
                </div>
                <div class="flex-1"></div>
            </aside>

            // ── Main Content ──
            <div class="flex-1 min-w-0">
                // Top bar: search + sort + filters
                <div class="sticky top-[60px] z-20 bg-black/30 backdrop-blur-2xl px-6 py-3">
                    <div class="flex items-center gap-3">
                        // Mobile category toggle
                        <button onclick="toggleMobileCats()" class="lg:hidden inline-flex items-center gap-1.5 px-3 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-400 text-sm hover:border-zinc-600 transition-all shrink-0">
                            <i class="ph ph-list text-base"></i>
                        </button>
                        <div class="relative flex-1">
                            <i class="ph ph-magnifying-glass absolute left-3 top-1/2 -translate-y-1/2 text-zinc-500"></i>
                            <input type="text" id="mp-search" placeholder="Search assets..." oninput="loadAssets()" class="w-full pl-9 pr-4 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-teal-500/50 focus:bg-white/[0.05] transition-all" />
                        </div>
                        // In the top bar rather than inside the Filters panel.
                        // Which engine you are on decides what is installable at
                        // all, so it is not the same kind of question as a price
                        // range, and a collapsed panel meant nobody found it.
                        <select id="mp-engine" onchange="setEngineFilter(this.value)" title="Show only what this engine version can run"
                                class="px-3 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm focus:border-teal-500/50 transition-all shrink-0">
                            <option value="">"All engine versions"</option>
                        </select>
                        <select id="mp-sort" onchange="loadAssets()" class="px-3 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm focus:border-teal-500/50 transition-all shrink-0">
                            <option value="newest">"Newest"</option>
                            <option value="popular">"Most Popular"</option>
                            <option value="top_rated">"Top Rated"</option>
                            <option value="price_asc">"Price: Low → High"</option>
                            <option value="price_desc">"Price: High → Low"</option>
                        </select>
                        <button onclick="toggleAdvancedFilter()" id="adv-filter-btn" class="inline-flex items-center gap-1.5 px-3 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-400 text-sm hover:border-zinc-600 hover:text-zinc-200 transition-all shrink-0">
                            <i class="ph ph-faders-horizontal text-base"></i><span class="hidden sm:inline">"Filters"</span>
                        </button>
                        // View toggle
                        <div class="flex items-center gap-0.5 p-0.5 bg-white/[0.02] rounded-lg border border-zinc-800/40 shrink-0">
                            <button onclick="setMpView('grid')" id="mp-view-grid" class="p-1.5 rounded-md text-zinc-300 bg-white/[0.06] transition-all" title="Grid">
                                <i class="ph ph-grid-four text-sm"></i>
                            </button>
                            <button onclick="setMpView('list')" id="mp-view-list" class="p-1.5 rounded-md text-zinc-500 hover:text-zinc-300 transition-all" title="List">
                                <i class="ph ph-list text-sm"></i>
                            </button>
                        </div>
                        // Result count
                        <span id="mp-result-count" class="text-xs text-zinc-600 shrink-0 hidden sm:block"></span>
                        // Upload (creators only, revealed via JS)
                        <a id="publish-btn-hero" href="/marketplace/upload" class="hidden group inline-flex items-center gap-1.5 px-3 py-2.5 rounded-xl text-sm font-medium bg-teal-500 text-white hover:bg-teal-400 transition-all shrink-0">
                            <i class="ph ph-plus-circle text-base group-hover:rotate-90 transition-transform duration-300"></i><span class="hidden sm:inline">"Upload"</span>
                        </a>
                    </div>
                    // Advanced filter panel
                    <div id="adv-filter-panel" class="hidden mt-3 p-4 bg-white/[0.02] border border-zinc-800/50 rounded-xl">
                        <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
                            <div>
                                <label class="text-[10px] text-zinc-500 uppercase tracking-wider font-medium mb-1.5 block">"Price Range"</label>
                                <div class="flex items-center gap-2">
                                    <input type="number" id="adv-min-price" placeholder="Min" min="0" class="w-full px-2.5 py-1.5 bg-white/[0.03] border border-zinc-800/50 rounded-lg text-zinc-50 text-xs outline-none focus:border-teal-500/50 transition-all" />
                                    <span class="text-zinc-600 text-xs">"-"</span>
                                    <input type="number" id="adv-max-price" placeholder="Max" min="0" class="w-full px-2.5 py-1.5 bg-white/[0.03] border border-zinc-800/50 rounded-lg text-zinc-50 text-xs outline-none focus:border-teal-500/50 transition-all" />
                                </div>
                            </div>
                            <div>
                                <label class="text-[10px] text-zinc-500 uppercase tracking-wider font-medium mb-1.5 block">"Min Rating"</label>
                                <select id="adv-min-rating" class="w-full px-2.5 py-1.5 bg-white/[0.03] border border-zinc-800/50 rounded-lg text-zinc-50 text-xs focus:border-teal-500/50 transition-all">
                                    <option value="0">"Any"</option>
                                    <option value="3">"★★★ & up"</option>
                                    <option value="4">"★★★★ & up"</option>
                                    <option value="5">"★★★★★ only"</option>
                                </select>
                            </div>
                            <div>
                                <label class="text-[10px] text-zinc-500 uppercase tracking-wider font-medium mb-1.5 block">"Licence"</label>
                                <select id="adv-licence" class="w-full px-2.5 py-1.5 bg-white/[0.03] border border-zinc-800/50 rounded-lg text-zinc-50 text-xs focus:border-teal-500/50 transition-all">
                                    <option value="">"Any"</option>
                                    <option value="standard">"Standard"</option>
                                    <option value="extended">"Extended"</option>
                                    <option value="cc0">"CC0 (Public Domain)"</option>
                                    <option value="mit">"MIT"</option>
                                    <option value="apache2">"Apache 2.0"</option>
                                    <option value="gpl3">"GPL 3.0"</option>
                                </select>
                            </div>
                            <div>
                                <label class="text-[10px] text-zinc-500 uppercase tracking-wider font-medium mb-1.5 block">"Tag"</label>
                                <input type="text" id="adv-tag" placeholder="e.g. low-poly" class="w-full px-2.5 py-1.5 bg-white/[0.03] border border-zinc-800/50 rounded-lg text-zinc-50 text-xs outline-none focus:border-teal-500/50 transition-all" />
                            </div>
                            // Populated from /api/marketplace/engine-versions, so
                            // this is empty until that resolves. Left as "Any"
                            // rather than defaulting to the newest: browsing the
                            // website is not browsing from an engine, and
                            // guessing one would silently hide listings.
                        </div>
                        <div class="flex items-center gap-3 mt-3">
                            <button onclick="applyAdvancedFilter()" class="inline-flex items-center gap-1.5 px-4 py-1.5 rounded-lg text-xs font-medium bg-teal-500 text-white hover:bg-teal-400 transition-all">
                                <i class="ph ph-check text-sm"></i>"Apply"
                            </button>
                            <button onclick="clearAdvancedFilter()" class="inline-flex items-center gap-1.5 px-4 py-1.5 rounded-lg text-xs font-medium text-zinc-400 hover:text-zinc-200 bg-white/[0.03] border border-zinc-800/50 hover:border-zinc-600 transition-all">
                                "Clear"
                            </button>
                        </div>
                    </div>
                    // Mobile categories (hidden by default)
                    <div id="mp-mobile-cats" class="hidden lg:hidden mt-3">
                        <div id="mp-categories" class="flex gap-2 flex-wrap">"Loading..."</div>
                    </div>
                </div>

                // Asset grid
                <div class="flex-1 overflow-y-auto mp-scroll" id="mp-scroll-area">
                    <div class="p-3">
                        <div id="mp-grid" class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-3">
                            <div class="col-span-full text-center py-16">
                                <div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-teal-400 rounded-full"></div>
                            </div>
                        </div>
                    </div>

                    // Pagination
                    <div id="mp-pagination" class="hidden flex items-center justify-center gap-3 py-4">
                        <button onclick="goPage(currentPage - 1)" id="mp-prev" class="inline-flex items-center justify-center w-8 h-8 rounded-lg bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:text-zinc-200 hover:border-zinc-600 transition-all disabled:opacity-30 disabled:pointer-events-none">
                            <i class="ph ph-caret-left text-sm"></i>
                        </button>
                        <div class="flex items-center gap-2 text-xs text-zinc-500">
                            "Page"
                            <input type="number" id="mp-page-input" min="1" value="1" onchange="goPage(parseInt(this.value)||1)" class="w-12 px-2 py-1 bg-white/[0.03] border border-zinc-800/50 rounded-lg text-zinc-50 text-xs text-center outline-none focus:border-teal-500/50 transition-all" />
                            "of "<span id="mp-total-pages">"1"</span>
                        </div>
                        <button onclick="goPage(currentPage + 1)" id="mp-next" class="inline-flex items-center justify-center w-8 h-8 rounded-lg bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:text-zinc-200 hover:border-zinc-600 transition-all disabled:opacity-30 disabled:pointer-events-none">
                            <i class="ph ph-caret-right text-sm"></i>
                        </button>
                    </div>
                </div>
            </div>
        </div>
        </section>

        // ── Quick look ──────────────────────────────────────────────────────
        // A listing opens over the grid instead of replacing it, so closing it
        // returns to the same scroll position, filters and page rather than a
        // reload of all three. Hidden until something opens it, and empty until
        // then too: the renderer fills `#mp-overlay-root`.
        <div id="mp-overlay" class="hidden fixed inset-0 z-[70]" role="dialog" aria-modal="true" aria-label="Asset details">
            <div class="absolute inset-0 bg-black/70 backdrop-blur-sm"></div>
            // The scroll container covers the whole viewport, scrim included, so
            // a click on "the outside" lands here and never on the scrim. That
            // is why closing is handled on this element and decided by what the
            // click was NOT inside, rather than by a listener on the backdrop.
            <div class="absolute inset-0 overflow-y-auto mp-scroll" id="mp-overlay-scroll">
                <div class="min-h-full px-4 py-6 sm:px-6 sm:py-10">
                    <div id="mp-overlay-panel" class="relative max-w-5xl mx-auto rounded-2xl border border-zinc-800/60 bg-surface shadow-2xl shadow-black/60">
                        <div class="px-4 sm:px-6 py-6" id="mp-overlay-root"></div>
                    </div>
                </div>
            </div>
        </div>

        // The same renderer the standalone listing page uses, so the overlay
        // cannot drift from it. stats-chart first, because the renderer mounts
        // the activity chart as soon as it has drawn.
        <script src=crate::assets::STATS_CHART_JS.as_str()></script>
        <script src=crate::assets::ASSET_DETAIL_JS.as_str()></script>

        <script>
            r##"
            // ── Marketplace logic ──
            let mpView = 'grid';
            let lastAssets = [];
            let currentCategory = new URLSearchParams(window.location.search).get('category') || 'all';
            let currentPage = 1;
            let currentPrice = 'all';
            let currentMinRating = 0;

            function setMpView(v) {
                mpView = v;
                document.getElementById('mp-view-grid').className = 'p-1.5 rounded-md transition-all ' + (v === 'grid' ? 'text-zinc-300 bg-white/[0.06]' : 'text-zinc-500 hover:text-zinc-300');
                document.getElementById('mp-view-list').className = 'p-1.5 rounded-md transition-all ' + (v === 'list' ? 'text-zinc-300 bg-white/[0.06]' : 'text-zinc-500 hover:text-zinc-300');
                if (lastAssets.length) renderMpAssets(lastAssets);
            }

            function toggleMobileCats() {
                document.getElementById('mp-mobile-cats').classList.toggle('hidden');
            }

            (async function() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                const pubBtn = document.getElementById('publish-btn-hero');
                if (pubBtn && token) { pubBtn.classList.remove('hidden'); }

                // The engine filter is additive, so a failure here leaves the
                // select on "Any" and the grid unfiltered rather than breaking
                // browsing.
                const engRes = await fetch('/api/marketplace/engine-versions');
                if (engRes.ok) {
                    const engines = await engRes.json();
                    const sel = document.getElementById('mp-engine');
                    if (sel) {
                        sel.innerHTML = '<option value="">All engine versions</option>' + engines.map(e =>
                            `<option value="${e.version}"${e.version === advEngine ? ' selected' : ''}>${e.version}</option>`).join('');
                    }
                }

                const catRes = await fetch('/api/marketplace/categories');
                const dbCats = catRes.ok ? await catRes.json() : [];
                const categories = [{slug: 'all', name: 'All', icon: 'ph-squares-four'}, ...dbCats.map(c => ({slug: c.slug, name: c.name, icon: c.icon}))];

                // Sidebar categories
                const sideEl = document.getElementById('mp-sidebar-cats');
                sideEl.innerHTML = categories.map(c => `
                    <button onclick="setCategory('${c.slug}')" id="scat-${c.slug}" class="menu-item mp-cat-btn ${c.slug === currentCategory ? 'active' : ''}">
                        <i class="ph ${c.icon} text-base"></i>${c.name}
                    </button>
                `).join('');

                // Mobile category chips
                const catEl = document.getElementById('mp-categories');
                catEl.innerHTML = categories.map(c => `
                    <button onclick="setCategory('${c.slug}')" id="cat-${c.slug}" class="inline-flex items-center gap-1.5 px-3.5 py-1.5 rounded-full text-xs font-medium transition-all ${c.slug === currentCategory ? 'bg-teal-500 text-white' : 'bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200'}">
                        <i class="ph ${c.icon}"></i>${c.name}
                    </button>
                `).join('');

                loadAssets();
            })();

            function setPrice(val) {
                currentPrice = val;
                currentPage = 1;
                document.querySelectorAll('.mp-price-btn').forEach(el => el.classList.remove('active'));
                const active = document.getElementById('price-' + val);
                if (active) { active.classList.add('active'); active.classList.remove('text-zinc-400'); }
                loadAssets();
            }

            function setMinRating(val) {
                currentMinRating = val;
                currentPage = 1;
                document.querySelectorAll('.mp-rating-btn').forEach(el => el.classList.remove('active'));
                const active = document.getElementById('rating-' + val);
                if (active) { active.classList.add('active'); active.classList.remove('text-zinc-400'); }
                loadAssets();
            }

            function setCategory(slug) {
                currentCategory = slug;
                currentPage = 1;
                const url = new URL(window.location);
                if (slug === 'all') url.searchParams.delete('category');
                else url.searchParams.set('category', slug);
                history.pushState({}, '', url);
                // Update sidebar
                document.querySelectorAll('.mp-cat-btn').forEach(el => el.classList.remove('active'));
                const sActive = document.getElementById('scat-' + slug);
                if (sActive) { sActive.classList.add('active'); sActive.classList.remove('text-zinc-400'); }
                // Update mobile chips
                document.querySelectorAll('[id^="cat-"]').forEach(el => {
                    const isActive = el.id === 'cat-' + slug;
                    el.className = `inline-flex items-center gap-1.5 px-3.5 py-1.5 rounded-full text-xs font-medium transition-all ${isActive ? 'bg-teal-500 text-white' : 'bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200'}`;
                });
                loadAssets();
            }

            // ── Advanced filter ──
            let advMaxPrice = null;
            let advTag = '';
            let advEngine = '';

            // Its own setter rather than part of Apply: this one lives in the top
            // bar and takes effect on change, like sort does.
            function setEngineFilter(v) {
                advEngine = v || '';
                currentPage = 1;
                loadAssets();
            }

            function toggleAdvancedFilter() {
                const panel = document.getElementById('adv-filter-panel');
                const btn = document.getElementById('adv-filter-btn');
                panel.classList.toggle('hidden');
                if (!panel.classList.contains('hidden')) {
                    btn.classList.add('border-teal-500/50', 'text-teal-400');
                } else {
                    btn.classList.remove('border-teal-500/50', 'text-teal-400');
                }
            }

            function applyAdvancedFilter() {
                const maxPrice = document.getElementById('adv-max-price')?.value;
                const minRating = document.getElementById('adv-min-rating')?.value;
                const tag = document.getElementById('adv-tag')?.value?.trim();

                advMaxPrice = maxPrice ? parseInt(maxPrice) : null;
                advTag = tag || '';
                currentMinRating = parseInt(minRating) || 0;

                // Sync sidebar rating buttons
                document.querySelectorAll('.mp-rating-btn').forEach(el => el.classList.remove('active'));
                const rBtn = document.getElementById('rating-' + currentMinRating);
                if (rBtn) { rBtn.classList.add('active'); rBtn.classList.remove('text-zinc-400'); }

                // Handle free from min price = 0
                const minPrice = document.getElementById('adv-min-price')?.value;
                if (minPrice === '0' && !maxPrice) {
                    currentPrice = 'free';
                }

                currentPage = 1;
                loadAssets();
            }

            function clearAdvancedFilter() {
                document.getElementById('adv-min-price').value = '';
                document.getElementById('adv-max-price').value = '';
                document.getElementById('adv-min-rating').value = '0';
                document.getElementById('adv-licence').value = '';
                document.getElementById('adv-tag').value = '';
                advMaxPrice = null;
                advTag = '';
                currentMinRating = 0;
                currentPrice = 'all';
                currentPage = 1;

                // Reset sidebar buttons
                document.querySelectorAll('.mp-rating-btn').forEach(el => el.classList.remove('active'));
                document.getElementById('rating-0')?.classList.add('active');
                document.querySelectorAll('.mp-price-btn').forEach(el => el.classList.remove('active'));
                document.getElementById('price-all')?.classList.add('active');

                loadAssets();
            }

            async function loadAssets() {
                const q = document.getElementById('mp-search')?.value || '';
                const sort = document.getElementById('mp-sort')?.value || 'newest';
                const cat = currentCategory === 'all' ? '' : currentCategory;
                let url = `/api/marketplace?page=${currentPage}&sort=${sort}`;
                if (q) url += '&q=' + encodeURIComponent(q);
                if (cat) url += '&category=' + cat;
                if (currentPrice === 'free') url += '&free=true';
                if (currentMinRating > 0) url += '&min_rating=' + currentMinRating;
                if (advMaxPrice !== null) url += '&max_price=' + advMaxPrice;
                if (advTag) url += '&tag=' + encodeURIComponent(advTag);
                // Filters the grid AND relabels each card with the version that
                // engine would actually get, which is not always the newest one
                // published.
                if (advEngine) url += '&engine=' + encodeURIComponent(advEngine);

                const res = await fetch(url);
                const data = await res.json();
                const el = document.getElementById('mp-grid');

                // Update result count
                const countEl = document.getElementById('mp-result-count');
                if (countEl) countEl.textContent = (data.total || 0) + ' asset' + ((data.total||0) !== 1 ? 's' : '');

                // Update hero stats
                const statTotal = document.getElementById('mp-stat-total');
                const statFree = document.getElementById('mp-stat-free');
                if (statTotal) statTotal.textContent = (data.total || 0).toLocaleString();

                if (!data.assets?.length) {
                    if (statFree) statFree.textContent = '0';
                    el.innerHTML = `
                        <div class="col-span-full text-center py-20">
                            <div class="w-16 h-16 bg-zinc-800/50 rounded-2xl flex items-center justify-center mx-auto mb-4">
                                <i class="ph ph-storefront text-3xl text-zinc-600"></i>
                            </div>
                            <p class="text-zinc-500 text-sm">${q ? 'No results. Try a different search.' : 'No assets found with these filters.'}</p>
                        </div>`;
                    document.getElementById('mp-pagination').classList.add('hidden');
                    return;
                }

                // Count free assets in results
                if (statFree) {
                    const freeCount = data.assets.filter(a => a.price_credits === 0).length;
                    statFree.textContent = freeCount.toString();
                }

                lastAssets = data.assets;
                renderMpAssets(data.assets);

                // Pagination
                const totalPages = Math.ceil(data.total / data.per_page);
                const pagEl = document.getElementById('mp-pagination');
                if (totalPages <= 1) {
                    pagEl.classList.add('hidden');
                } else {
                    pagEl.classList.remove('hidden');
                    document.getElementById('mp-page-input').value = currentPage;
                    document.getElementById('mp-page-input').max = totalPages;
                    document.getElementById('mp-total-pages').textContent = totalPages;
                    document.getElementById('mp-prev').disabled = currentPage <= 1;
                    document.getElementById('mp-next').disabled = currentPage >= totalPages;
                }
            }

            function renderMpAssets(assets) {
                const el = document.getElementById('mp-grid');
                if (mpView === 'list') {
                    el.className = 'divide-y divide-zinc-800/30 rounded-xl border border-zinc-800/40 bg-white/[0.01] overflow-hidden';
                    el.innerHTML = assets.map((a, i) => {
                        const priceLabel = a.price_credits === 0 ? 'Free' : a.price_credits.toLocaleString() + ' cr';
                        const fullStars = a.rating_count > 0 ? Math.round(a.rating_avg) : 0;
                        const starsHtml = a.rating_count > 0 ? `<span class="text-amber-400">${'★'.repeat(fullStars)}</span><span class="text-zinc-700">${'☆'.repeat(5-fullStars)}</span><span class="text-zinc-500 ml-1">(${a.rating_count})</span>` : '';
                        const thumb = a.thumbnail_url
                            ? `<img src="${a.thumbnail_url}" class="w-full h-full object-cover" loading="lazy" />`
                            : `<i class="ph ph-package text-lg text-zinc-600"></i>`;
                        const tagsHtml = (a.tags || []).slice(0, 3).map(t => `<span class="inline-block px-1.5 py-0.5 rounded text-[10px] bg-white/[0.05] text-zinc-500">${t}</span>`).join('');
                        return `<a href="/marketplace/asset/${a.slug}" class="flex items-center gap-4 px-5 py-3 hover:bg-white/[0.02] transition-all group" style="animation: mpFadeIn 0.2s ease both; animation-delay:${i * 10}ms">
                            <div class="w-12 h-12 rounded-lg bg-zinc-900 border border-zinc-800/50 flex items-center justify-center shrink-0 overflow-hidden">${thumb}</div>
                            <div class="flex-1 min-w-0">
                                <div class="text-sm font-medium group-hover:text-teal-400 transition-colors truncate">${a.name}</div>
                                <div class="flex items-center gap-2 mt-0.5 text-[11px] text-zinc-500">
                                    <span>${a.creator_name}</span><span class="text-zinc-800">·</span>
                                    <span>${a.category}</span><span class="text-zinc-800">·</span>
                                    <span>${a.downloads.toLocaleString()} dl</span>
                                    ${starsHtml ? `<span class="text-zinc-800">·</span>${starsHtml}` : ''}
                                </div>
                            </div>
                            <div class="flex items-center gap-3 shrink-0">
                                ${tagsHtml ? `<div class="hidden md:flex items-center gap-1">${tagsHtml}</div>` : ''}
                                <span class="text-xs font-semibold ${a.price_credits === 0 ? 'text-emerald-400' : 'text-zinc-300'} min-w-[50px] text-right">${priceLabel}</span>
                            </div>
                        </a>`;
                    }).join('');
                } else {
                    el.className = 'grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-3';
                    el.innerHTML = assets.map((a, i) => {
                        const fullStars = a.rating_count > 0 ? Math.round(a.rating_avg) : 0;
                        const starsHtml = `<span class="text-amber-400 text-sm">${'★'.repeat(fullStars)}</span><span class="text-zinc-700 text-sm">${'☆'.repeat(5 - fullStars)}</span><span class="text-[11px] text-zinc-500 ml-1">(${a.rating_count})</span>`;
                        const priceLabel = a.price_credits === 0 ? 'Free' : a.price_credits.toLocaleString() + ' credits';
                        const avatarHtml = a.creator_avatar_url
                            ? `<img src="${a.creator_avatar_url}" class="w-5 h-5 rounded-full object-cover" />`
                            : `<div class="w-5 h-5 rounded-full bg-zinc-800 flex items-center justify-center"><i class="ph ph-user text-[9px] text-zinc-500"></i></div>`;
                        const tagsHtml = (a.tags || []).slice(0, 2).map(t => `<span class="inline-block px-1.5 py-0.5 rounded text-[10px] bg-white/[0.05] text-zinc-500 shrink-0">${t}</span>`).join('');
                        return `
                        <a href="/marketplace/asset/${a.slug}" class="block group" style="animation: mpFadeIn 0.25s ease both; animation-delay: ${i * 15}ms">
                            <div class="bg-white/[0.02] border border-zinc-800/40 rounded-xl overflow-hidden hover:border-teal-500/20 transition-all duration-200 hover:shadow-[0_0_20px_rgba(20,184,166,0.05)]">
                                <div class="aspect-[4/3] bg-zinc-900 relative overflow-hidden">
                                    ${a.thumbnail_url
                                        ? `<img src="${a.thumbnail_url}" class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500 ease-out" loading="lazy" />`
                                        : `<div class="w-full h-full flex items-center justify-center"><i class="ph ph-package text-3xl text-zinc-800"></i></div>`}
                                </div>
                                <div class="p-3">
                                    <h3 class="text-sm font-medium text-zinc-200 group-hover:text-white truncate">${a.name}</h3>
                                    <div class="flex items-center gap-1.5 mt-2">
                                        ${avatarHtml}
                                        <span class="text-xs text-zinc-300 truncate">${a.creator_name}</span>
                                    </div>
                                    <div class="flex items-center justify-between mt-2">
                                        <div class="flex items-center">${starsHtml}</div>
                                        <span class="text-xs font-semibold ${a.price_credits === 0 ? 'text-emerald-400' : 'text-zinc-300'} shrink-0">${priceLabel}</span>
                                    </div>
                                    <div class="flex items-center gap-1.5 mt-2 flex-wrap">
                                        ${tagsHtml}
                                        <span class="inline-block px-1.5 py-0.5 rounded text-[10px] bg-white/[0.05] text-zinc-500 shrink-0">${a.category}</span>
                                    </div>
                                </div>
                            </div>
                        </a>`;
                    }).join('');
                }
            }

            function goPage(p) {
                const totalPages = parseInt(document.getElementById('mp-total-pages')?.textContent) || 1;
                if (p < 1) p = 1;
                if (p > totalPages) p = totalPages;
                currentPage = p;
                loadAssets();
                window.scrollTo({top: 0, behavior: 'smooth'});
            }

            // ── Quick look ──────────────────────────────────────────────────
            //
            // Clicking a card opens the listing over the grid and pushes its
            // URL, so the address bar names what is on screen, the link is
            // shareable, and Back closes the overlay onto the grid exactly as it
            // was rather than reloading it.
            //
            // A middle click, a modifier click or a right click is left alone:
            // those mean "open a copy elsewhere", and an overlay cannot honour
            // that. Hence the guard below rather than a blanket preventDefault.

            // Responses warmed on hover, keyed by slug.
            //
            // A Response body can only be read once, so `take` hands the entry
            // over and drops it. A miss returns undefined and the renderer
            // fetches normally, which is also what happens when the pointer
            // moved too fast for the delay to fire.
            window.assetDetailPrefetch = (function () {
                const warm = new Map();
                let timer = null;
                const LIMIT = 30;

                // The slug, but only for a link to the LISTING itself.
                //
                // /marketplace/asset/<slug> is the listing; /files, /edit and
                // /releases/new underneath it are their own pages. Matching on
                // the prefix alone swallowed those too and asked the API for a
                // listing called "vibrance/files", which answered 404 and left
                // the overlay reading "Asset not found". Anything with a further
                // path segment navigates normally.
                function slugFrom(a) {
                    const href = (a && a.getAttribute('href')) || '';
                    const parts = href.split('/marketplace/asset/');
                    if (parts.length !== 2) return null;
                    const rest = parts[1].split(/[?#]/)[0];
                    return (rest && !rest.includes('/')) ? rest : null;
                }

                function warmUp(slug) {
                    if (!slug || warm.has(slug)) return;
                    // Cheap insurance against a long browse warming hundreds of
                    // listings: drop the oldest rather than grow without bound.
                    if (warm.size >= LIMIT) warm.delete(warm.keys().next().value);
                    const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                    const headers = token ? { 'Authorization': 'Bearer ' + token } : {};
                    warm.set(slug, fetch('/api/marketplace/detail/' + slug, { headers })
                        .catch(() => null));
                }

                return {
                    slugFrom,
                    // 120ms: long enough that sweeping the pointer across a row
                    // of cards on the way somewhere else costs nothing, short
                    // enough to be well ahead of a click.
                    hover(a) {
                        clearTimeout(timer);
                        const slug = slugFrom(a);
                        if (slug) timer = setTimeout(() => warmUp(slug), 120);
                    },
                    cancel() { clearTimeout(timer); },
                    take(slug) {
                        const p = warm.get(slug);
                        if (!p) return null;
                        warm.delete(slug);
                        // A failed prefetch must not become a failed render.
                        return p.then(r => (r && r.ok) ? r : fetch('/api/marketplace/detail/' + slug));
                    },
                };
            })();

            function assetLinkFrom(target) {
                const a = target.closest && target.closest('a[href^="/marketplace/asset/"]');
                return a || null;
            }

            document.addEventListener('mouseover', e => {
                const a = assetLinkFrom(e.target);
                if (a) window.assetDetailPrefetch.hover(a);
            }, { passive: true });
            document.addEventListener('mouseout', e => {
                if (assetLinkFrom(e.target)) window.assetDetailPrefetch.cancel();
            }, { passive: true });

            document.addEventListener('click', e => {
                // Anything that means "open elsewhere" stays a normal link.
                if (e.defaultPrevented || e.button !== 0 || e.metaKey || e.ctrlKey ||
                    e.shiftKey || e.altKey) return;
                const a = assetLinkFrom(e.target);
                if (!a || a.target === '_blank') return;
                const slug = window.assetDetailPrefetch.slugFrom(a);
                if (!slug) return;
                e.preventDefault();
                openQuickLook(slug, true);
            });

            let quickLookOpen = false;

            function openQuickLook(slug, push) {
                const overlay = document.getElementById('mp-overlay');
                const root = document.getElementById('mp-overlay-root');
                if (!overlay || !root || !window.renderAssetDetail) {
                    // No renderer means no overlay. Fall back to the page rather
                    // than swallowing the click.
                    window.location.href = '/marketplace/asset/' + slug;
                    return;
                }
                if (push) history.pushState({ quickLook: slug }, '', '/marketplace/asset/' + slug);

                root.innerHTML = '<div class="text-center py-20"><div class="inline-block animate-spin w-6 h-6 border-2 border-zinc-700 border-t-accent rounded-full"></div></div>';
                overlay.classList.remove('hidden');
                // The grid must not scroll behind the overlay, and must be
                // exactly where it was when this closes.
                document.body.style.overflow = 'hidden';
                document.getElementById('mp-overlay-scroll').scrollTop = 0;
                quickLookOpen = true;
                window.renderAssetDetail(slug, root);
            }

            function closeQuickLook(pop) {
                const overlay = document.getElementById('mp-overlay');
                if (!overlay || !quickLookOpen) return;
                overlay.classList.add('hidden');
                document.getElementById('mp-overlay-root').innerHTML = '';
                document.body.style.overflow = '';
                quickLookOpen = false;
                // `pop` means history already moved; anything else has to move it.
                if (!pop) history.back();
            }

            // Click anywhere off the panel to close.
            //
            // Both the press and the release have to land outside it. Selecting
            // text in the listing and releasing past its edge is a drag, not a
            // dismissal, and closing on the release alone would throw the page
            // away mid-selection.
            (function () {
                const scroll = document.getElementById('mp-overlay-scroll');
                if (!scroll) return;
                let pressedOutside = false;
                const outside = t => !(t.closest && t.closest('#mp-overlay-panel'));
                scroll.addEventListener('mousedown', e => { pressedOutside = outside(e.target); });
                scroll.addEventListener('click', e => {
                    if (pressedOutside && outside(e.target)) closeQuickLook();
                    pressedOutside = false;
                });
            })();

            document.addEventListener('keydown', e => {
                if (e.key === 'Escape' && quickLookOpen) closeQuickLook();
            });

            // Back and forward. The URL is the state: an asset path means open,
            // anything else means closed.
            window.addEventListener('popstate', () => {
                const m = window.location.pathname.match(/^\/marketplace\/asset\/([^/?#]+)/);
                if (m) openQuickLook(m[1], false);
                else closeQuickLook(true);
            });
            "##
        </script>

        <style>
            r#"
            .mp-scroll::-webkit-scrollbar { width: 6px; }
            .mp-scroll::-webkit-scrollbar-track { background: transparent; }
            .mp-scroll::-webkit-scrollbar-thumb { background: #27272a; border-radius: 3px; }
            .mp-scroll::-webkit-scrollbar-thumb:hover { background: #3f3f46; }
            .mp-scroll { scrollbar-width: thin; scrollbar-color: #27272a transparent; }
            .no-scrollbar::-webkit-scrollbar { display: none; }
            .no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }

            /* The renderer draws a "Back to Marketplace" link for the standalone
               page. In the overlay the marketplace is literally behind it and
               the close button is three pixels away, so it is noise here. */
            #mp-overlay .detail-back { display: none; }

            /* The listing page runs its own fixed background layer. Inside the
               overlay it would sit over the scrim and under nothing useful. */
            #mp-overlay #asset-bg-layer { display: none; }
            "#
        </style>
    }
}
