use leptos::prelude::*;

#[component]
pub fn MarketplacePage() -> impl IntoView {
    view! {
        <section class="min-h-[calc(100vh-3.5rem)] flex bg-[#08080a]">
            // ── Left Sidebar: Categories ──
            <aside class="w-56 shrink-0 bg-surface-card border-r border-zinc-800 sticky top-14 h-[calc(100vh-3.5rem)] overflow-y-auto hidden lg:block">
                <div class="p-4 border-b border-zinc-800">
                    <div class="flex items-center gap-2">
                        <i class="ph ph-storefront text-xl text-accent"></i>
                        <h1 class="text-base font-bold">"Marketplace"</h1>
                    </div>
                    <p class="text-[11px] text-zinc-600 mt-1">"Browse assets, plugins & more"</p>
                </div>
                <div class="py-2">
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
                // Top bar: search + sort
                <div class="sticky top-14 z-20 bg-[rgba(10,10,11,0.85)] backdrop-blur-xl border-b border-zinc-800/50 px-6 py-3">
                    <div class="flex items-center gap-3">
                        // Mobile category toggle
                        <button onclick="toggleMobileCats()" class="lg:hidden inline-flex items-center gap-1.5 px-3 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-400 text-sm hover:border-zinc-600 transition-all shrink-0">
                            <i class="ph ph-list text-base"></i>
                        </button>
                        <div class="relative flex-1">
                            <i class="ph ph-magnifying-glass absolute left-3 top-1/2 -translate-y-1/2 text-zinc-500"></i>
                            <input type="text" id="mp-search" placeholder="Search assets..." oninput="loadAssets()" class="w-full pl-9 pr-4 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 focus:bg-white/[0.05] transition-all" />
                        </div>
                        <select id="mp-sort" onchange="loadAssets()" class="px-3 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm focus:border-accent/50 transition-all shrink-0">
                            <option value="newest">"Newest"</option>
                            <option value="popular">"Most Popular"</option>
                            <option value="top_rated">"Top Rated"</option>
                            <option value="price_asc">"Price: Low → High"</option>
                            <option value="price_desc">"Price: High → Low"</option>
                        </select>
                        // Result count
                        <span id="mp-result-count" class="text-xs text-zinc-600 shrink-0 hidden sm:block"></span>
                        <a id="publish-btn-top" href="/login" class="hidden inline-flex items-center gap-1.5 px-4 py-2 bg-accent text-white text-sm font-medium hover:bg-accent-hover transition-all shrink-0">
                            <i class="ph ph-upload-simple text-base"></i>"Publish"
                        </a>
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
                                <div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                            </div>
                        </div>
                    </div>

                    // Pagination
                    <div id="mp-pagination" class="flex justify-center gap-1 py-4 border-t border-zinc-800/30"></div>
                </div>
            </div>
        </section>

        <script>
            r##"
            let currentCategory = new URLSearchParams(window.location.search).get('category') || 'all';
            let currentPage = 1;
            let currentPrice = 'all';
            let currentMinRating = 0;

            function toggleMobileCats() {
                document.getElementById('mp-mobile-cats').classList.toggle('hidden');
            }

            (async function() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                const pubBtn = document.getElementById('publish-btn-top');
                if (pubBtn) { pubBtn.classList.remove('hidden'); pubBtn.href = token ? '/marketplace/sell' : '/login'; }

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
                    <button onclick="setCategory('${c.slug}')" id="cat-${c.slug}" class="inline-flex items-center gap-1.5 px-3.5 py-1.5 rounded-full text-xs font-medium transition-all ${c.slug === currentCategory ? 'bg-accent text-white' : 'bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200'}">
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
                    el.className = `inline-flex items-center gap-1.5 px-3.5 py-1.5 rounded-full text-xs font-medium transition-all ${isActive ? 'bg-accent text-white' : 'bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200'}`;
                });
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

                const res = await fetch(url);
                const data = await res.json();
                const el = document.getElementById('mp-grid');

                // Update result count
                const countEl = document.getElementById('mp-result-count');
                if (countEl) countEl.textContent = (data.total || 0) + ' asset' + ((data.total||0) !== 1 ? 's' : '');

                if (!data.assets?.length) {
                    el.innerHTML = `
                        <div class="col-span-full text-center py-20">
                            <div class="w-16 h-16 bg-zinc-800/50 rounded-2xl flex items-center justify-center mx-auto mb-4">
                                <i class="ph ph-storefront text-3xl text-zinc-600"></i>
                            </div>
                            <p class="text-zinc-500 text-sm">${q ? 'No results. Try a different search.' : 'No assets found with these filters.'}</p>
                        </div>`;
                    document.getElementById('mp-pagination').innerHTML = '';
                    return;
                }

                el.innerHTML = data.assets.map((a, i) => {
                    const ratingAvg = a.rating_count > 0 ? a.rating_avg.toFixed(1) : '';
                    const priceLabel = a.price_credits === 0 ? 'Free' : a.price_credits.toLocaleString() + ' cr';
                    return `
                    <a href="/marketplace/asset/${a.slug}" class="block group" style="animation: fadeSlideUp 0.25s ease both; animation-delay: ${i * 20}ms">
                        <div class="bg-white/[0.02] border border-zinc-800/40 rounded-lg overflow-hidden hover:border-zinc-700/60 transition-all duration-200">
                            <div class="aspect-[4/3] bg-zinc-900 relative overflow-hidden">
                                ${a.thumbnail_url
                                    ? `<img src="${a.thumbnail_url}" class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500 ease-out" loading="lazy" />`
                                    : `<div class="w-full h-full flex items-center justify-center"><i class="ph ph-package text-3xl text-zinc-800"></i></div>`}
                                ${a.price_credits === 0 ? `<span class="absolute top-1.5 left-1.5 text-[9px] px-1.5 py-0.5 rounded bg-emerald-600 text-white font-semibold">FREE</span>` : ''}
                            </div>
                            <div class="p-2.5">
                                <h3 class="text-[12px] font-medium text-zinc-200 group-hover:text-white truncate">${a.name}</h3>
                                <div class="flex items-center justify-between mt-1">
                                    <span class="text-[10px] text-zinc-500 truncate">${a.creator_name}</span>
                                    <span class="text-[10px] font-semibold ${a.price_credits === 0 ? 'text-emerald-400' : 'text-zinc-300'} shrink-0 ml-1">${priceLabel}</span>
                                </div>
                                <div class="flex items-center gap-2 mt-1 text-[9px] text-zinc-600">
                                    <span>${a.category}</span>
                                    ${ratingAvg ? `<span class="text-amber-400">${ratingAvg}★</span>` : ''}
                                </div>
                            </div>
                        </div>
                    </a>`;
                }).join('');

                const totalPages = Math.ceil(data.total / data.per_page);
                const pagEl = document.getElementById('mp-pagination');
                if (totalPages <= 1) { pagEl.innerHTML = ''; return; }
                let pag = '';
                for (let i = 1; i <= totalPages; i++) {
                    pag += `<button onclick="goPage(${i})" class="w-8 h-8 text-xs font-medium transition-all ${i === currentPage ? 'bg-accent text-white' : 'bg-white/[0.03] text-zinc-400 hover:bg-white/[0.06] hover:text-zinc-200'}">${i}</button>`;
                }
                pagEl.innerHTML = pag;
            }

            function goPage(p) { currentPage = p; loadAssets(); window.scrollTo({top: 0, behavior: 'smooth'}); }
            "##
        </script>

        <style>
            r#"
            @keyframes fadeSlideUp {
                from { opacity: 0; transform: translateY(8px); }
                to { opacity: 1; transform: translateY(0); }
            }
            /* Custom scrollbar */
            .mp-scroll::-webkit-scrollbar { width: 6px; }
            .mp-scroll::-webkit-scrollbar-track { background: transparent; }
            .mp-scroll::-webkit-scrollbar-thumb { background: #27272a; border-radius: 3px; }
            .mp-scroll::-webkit-scrollbar-thumb:hover { background: #3f3f46; }
            .mp-scroll { scrollbar-width: thin; scrollbar-color: #27272a transparent; }
            "#
        </style>
    }
}
