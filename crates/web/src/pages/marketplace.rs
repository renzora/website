use leptos::prelude::*;

#[component]
pub fn MarketplacePage() -> impl IntoView {
    view! {
        <section class="min-h-[calc(100vh-3rem)]">

        // ── Particles (full screen fixed) ──
        <canvas id="mp-particles" class="fixed inset-0 w-full h-full pointer-events-none z-0"></canvas>

        // ── Hero ──
        <div class="relative overflow-hidden">
            <div class="absolute inset-0 bg-gradient-to-b from-teal-500/[0.05] via-cyan-500/[0.02] to-transparent"></div>
            <div class="absolute top-0 left-1/3 w-[600px] h-[300px] bg-teal-500/[0.04] rounded-full blur-3xl"></div>
            <div class="absolute bottom-0 right-1/4 w-[400px] h-[200px] bg-cyan-500/[0.03] rounded-full blur-3xl"></div>
            <div class="relative max-w-[1400px] mx-auto px-6 pt-10 pb-8">
                <div class="flex items-end justify-between">
                    <div>
                        <div class="flex items-center gap-3 mb-2">
                            <div class="w-10 h-10 rounded-xl bg-teal-500/10 border border-teal-500/20 flex items-center justify-center">
                                <i class="ph ph-storefront text-xl text-teal-400"></i>
                            </div>
                            <div>
                                <h1 class="text-2xl font-bold tracking-tight">"Marketplace"</h1>
                                <p class="text-zinc-500 text-sm">"Assets, plugins, models, audio, and more."</p>
                            </div>
                        </div>
                    </div>
                    <a id="publish-btn-hero" href="/marketplace/upload" class="hidden group inline-flex items-center gap-2 px-5 py-2.5 rounded-xl text-sm font-medium bg-teal-500 text-white hover:bg-teal-400 transition-all hover:shadow-[0_0_24px_rgba(20,184,166,0.25)] hover:-translate-y-0.5">
                        <i class="ph ph-plus-circle text-base group-hover:rotate-90 transition-transform duration-300"></i>"Upload"
                    </a>
                </div>
                // Quick stats
                <div class="flex items-center gap-6 mt-5">
                    <div class="flex items-center gap-2 text-sm">
                        <span id="mp-stat-total" class="font-semibold text-zinc-200">"—"</span>
                        <span class="text-zinc-600">"assets"</span>
                    </div>
                    <div class="w-px h-4 bg-zinc-800"></div>
                    <div class="flex items-center gap-2 text-sm">
                        <span id="mp-stat-free" class="font-semibold text-emerald-400">"—"</span>
                        <span class="text-zinc-600">"free"</span>
                    </div>
                </div>
            </div>
        </div>

        <div class="max-w-[1400px] mx-auto flex">
            // ── Left Sidebar: Categories ──
            <aside class="w-56 shrink-0 border-r border-white/[0.04] sticky top-14 h-[calc(100vh-3.5rem)] overflow-y-auto hidden lg:block bg-black/20 backdrop-blur-xl">
                <div class="py-3">
                    <div class="px-3 py-1.5 text-[10px] font-semibold text-zinc-600 uppercase tracking-wider">"Categories"</div>
                    <div id="mp-sidebar-cats">
                        <button class="w-full flex items-center gap-2.5 px-4 py-2 text-sm bg-white/5 text-zinc-50">
                            <i class="ph ph-squares-four text-base"></i>"All"
                        </button>
                    </div>
                </div>
                <div class="py-2 border-t border-zinc-800">
                    <div class="px-3 py-1.5 text-[10px] font-semibold text-zinc-600 uppercase tracking-wider">"Price"</div>
                    <button onclick="setPrice('all')" id="price-all" class="mp-price-btn w-full flex items-center gap-2.5 px-4 py-2 text-sm bg-white/5 text-zinc-50">
                        <i class="ph ph-coins text-base"></i>"All Prices"
                    </button>
                    <button onclick="setPrice('free')" id="price-free" class="mp-price-btn w-full flex items-center gap-2.5 px-4 py-2 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                        <i class="ph ph-gift text-base"></i>"Free Only"
                    </button>
                    <button onclick="setPrice('paid')" id="price-paid" class="mp-price-btn w-full flex items-center gap-2.5 px-4 py-2 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                        <i class="ph ph-credit-card text-base"></i>"Paid Only"
                    </button>
                </div>
                <div class="py-2 border-t border-zinc-800">
                    <div class="px-3 py-1.5 text-[10px] font-semibold text-zinc-600 uppercase tracking-wider">"Min Rating"</div>
                    <button onclick="setMinRating(0)" id="rating-0" class="mp-rating-btn w-full flex items-center gap-2.5 px-4 py-2 text-sm bg-white/5 text-zinc-50">
                        <i class="ph ph-star text-base"></i>"Any"
                    </button>
                    <button onclick="setMinRating(3)" id="rating-3" class="mp-rating-btn w-full flex items-center gap-2.5 px-4 py-2 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                        <span class="text-amber-400">"★★★"</span><span class="text-zinc-600">"☆☆"</span>"& up"
                    </button>
                    <button onclick="setMinRating(4)" id="rating-4" class="mp-rating-btn w-full flex items-center gap-2.5 px-4 py-2 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                        <span class="text-amber-400">"★★★★"</span><span class="text-zinc-600">"☆"</span>"& up"
                    </button>
                    <button onclick="setMinRating(5)" id="rating-5" class="mp-rating-btn w-full flex items-center gap-2.5 px-4 py-2 text-sm text-zinc-400 hover:text-zinc-50 hover:bg-white/5 transition-all">
                        <span class="text-amber-400">"★★★★★"</span>"only"
                    </button>
                </div>
                <div class="flex-1"></div>
            </aside>

            // ── Main Content ──
            <div class="flex-1 min-w-0">
                // Top bar: search + sort + filters
                <div class="sticky top-14 z-20 bg-black/30 backdrop-blur-2xl px-6 py-3">
                    <div class="flex items-center gap-3">
                        // Mobile category toggle
                        <button onclick="toggleMobileCats()" class="lg:hidden inline-flex items-center gap-1.5 px-3 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-400 text-sm hover:border-zinc-600 transition-all shrink-0">
                            <i class="ph ph-list text-base"></i>
                        </button>
                        <div class="relative flex-1">
                            <i class="ph ph-magnifying-glass absolute left-3 top-1/2 -translate-y-1/2 text-zinc-500"></i>
                            <input type="text" id="mp-search" placeholder="Search assets..." oninput="loadAssets()" class="w-full pl-9 pr-4 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-teal-500/50 focus:bg-white/[0.05] transition-all" />
                        </div>
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
                    </div>
                    // Advanced filter panel
                    <div id="adv-filter-panel" class="hidden mt-3 p-4 bg-white/[0.02] border border-zinc-800/50 rounded-xl">
                        <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
                            <div>
                                <label class="text-[10px] text-zinc-500 uppercase tracking-wider font-medium mb-1.5 block">"Price Range"</label>
                                <div class="flex items-center gap-2">
                                    <input type="number" id="adv-min-price" placeholder="Min" min="0" class="w-full px-2.5 py-1.5 bg-white/[0.03] border border-zinc-800/50 rounded-lg text-zinc-50 text-xs outline-none focus:border-teal-500/50 transition-all" />
                                    <span class="text-zinc-600 text-xs">"—"</span>
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

        <script>
            r##"
            // ── Particle canvas ──
            (function() {
                const canvas = document.getElementById('mp-particles');
                if (!canvas) return;
                const ctx = canvas.getContext('2d');
                let w, h, particles = [];
                function resize() {
                    w = canvas.width = window.innerWidth;
                    h = canvas.height = window.innerHeight;
                }
                resize();
                window.addEventListener('resize', resize);

                for (let i = 0; i < 40; i++) {
                    particles.push({
                        x: Math.random() * w,
                        y: Math.random() * h,
                        r: Math.random() * 1.5 + 0.5,
                        dx: (Math.random() - 0.5) * 0.3,
                        dy: (Math.random() - 0.5) * 0.2,
                        o: Math.random() * 0.3 + 0.1
                    });
                }

                function draw() {
                    ctx.clearRect(0, 0, w, h);
                    for (const p of particles) {
                        p.x += p.dx;
                        p.y += p.dy;
                        if (p.x < 0) p.x = w;
                        if (p.x > w) p.x = 0;
                        if (p.y < 0) p.y = h;
                        if (p.y > h) p.y = 0;
                        ctx.beginPath();
                        ctx.arc(p.x, p.y, p.r, 0, Math.PI * 2);
                        ctx.fillStyle = `rgba(94, 234, 212, ${p.o})`;
                        ctx.fill();
                    }
                    // Draw subtle connections
                    for (let i = 0; i < particles.length; i++) {
                        for (let j = i + 1; j < particles.length; j++) {
                            const dx = particles[i].x - particles[j].x;
                            const dy = particles[i].y - particles[j].y;
                            const dist = Math.sqrt(dx * dx + dy * dy);
                            if (dist < 120) {
                                ctx.beginPath();
                                ctx.moveTo(particles[i].x, particles[i].y);
                                ctx.lineTo(particles[j].x, particles[j].y);
                                ctx.strokeStyle = `rgba(94, 234, 212, ${0.04 * (1 - dist / 120)})`;
                                ctx.lineWidth = 0.5;
                                ctx.stroke();
                            }
                        }
                    }
                    requestAnimationFrame(draw);
                }
                draw();
            })();

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

                const catRes = await fetch('/api/marketplace/categories');
                const dbCats = catRes.ok ? await catRes.json() : [];
                const categories = [{slug: 'all', name: 'All', icon: 'ph-squares-four'}, ...dbCats.map(c => ({slug: c.slug, name: c.name, icon: c.icon}))];

                // Sidebar categories
                const sideEl = document.getElementById('mp-sidebar-cats');
                sideEl.innerHTML = categories.map(c => `
                    <button onclick="setCategory('${c.slug}')" id="scat-${c.slug}" class="mp-cat-btn w-full flex items-center gap-2.5 px-4 py-2 text-sm transition-all ${c.slug === currentCategory ? 'bg-white/5 text-zinc-50' : 'text-zinc-400 hover:text-zinc-50 hover:bg-white/5'}">
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
                document.querySelectorAll('.mp-price-btn').forEach(el => {
                    el.classList.remove('bg-white/5', 'text-zinc-50');
                    el.classList.add('text-zinc-400');
                });
                const active = document.getElementById('price-' + val);
                if (active) { active.classList.add('bg-white/5', 'text-zinc-50'); active.classList.remove('text-zinc-400'); }
                loadAssets();
            }

            function setMinRating(val) {
                currentMinRating = val;
                currentPage = 1;
                document.querySelectorAll('.mp-rating-btn').forEach(el => {
                    el.classList.remove('bg-white/5', 'text-zinc-50');
                    el.classList.add('text-zinc-400');
                });
                const active = document.getElementById('rating-' + val);
                if (active) { active.classList.add('bg-white/5', 'text-zinc-50'); active.classList.remove('text-zinc-400'); }
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
                document.querySelectorAll('.mp-cat-btn').forEach(el => {
                    el.classList.remove('bg-white/5', 'text-zinc-50');
                    el.classList.add('text-zinc-400');
                });
                const sActive = document.getElementById('scat-' + slug);
                if (sActive) { sActive.classList.add('bg-white/5', 'text-zinc-50'); sActive.classList.remove('text-zinc-400'); }
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
                document.querySelectorAll('.mp-rating-btn').forEach(el => {
                    el.classList.remove('bg-white/5', 'text-zinc-50');
                    el.classList.add('text-zinc-400');
                });
                const rBtn = document.getElementById('rating-' + currentMinRating);
                if (rBtn) { rBtn.classList.add('bg-white/5', 'text-zinc-50'); rBtn.classList.remove('text-zinc-400'); }

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
                document.querySelectorAll('.mp-rating-btn').forEach(el => {
                    el.classList.remove('bg-white/5', 'text-zinc-50');
                    el.classList.add('text-zinc-400');
                });
                document.getElementById('rating-0')?.classList.add('bg-white/5', 'text-zinc-50');
                document.querySelectorAll('.mp-price-btn').forEach(el => {
                    el.classList.remove('bg-white/5', 'text-zinc-50');
                    el.classList.add('text-zinc-400');
                });
                document.getElementById('price-all')?.classList.add('bg-white/5', 'text-zinc-50');

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

                // Animate cards in
                if (typeof anime !== 'undefined') {
                    const cards = document.querySelectorAll('#mp-grid > a, #mp-grid > div');
                    anime({ targets: cards, opacity: [0,1], translateY: [30,0], scale: [0.92,1], delay: anime.stagger(30, {from: 'first'}), duration: 500, easing: 'easeOutCubic' });
                }

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
            "##
        </script>

        <style>
            r#"
            /* anime.js handles card animations */
            .mp-scroll::-webkit-scrollbar { width: 6px; }
            .mp-scroll::-webkit-scrollbar-track { background: transparent; }
            .mp-scroll::-webkit-scrollbar-thumb { background: #27272a; border-radius: 3px; }
            .mp-scroll::-webkit-scrollbar-thumb:hover { background: #3f3f46; }
            .mp-scroll { scrollbar-width: thin; scrollbar-color: #27272a transparent; }
            .no-scrollbar::-webkit-scrollbar { display: none; }
            .no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
            "#
        </style>
    }
}
