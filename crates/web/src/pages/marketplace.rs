use leptos::prelude::*;

#[component]
pub fn MarketplacePage() -> impl IntoView {
    view! {
        <section class="min-h-[calc(100vh-3.5rem)] flex">
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
                <div class="p-4 border-t border-zinc-800">
                    <a id="publish-btn" href="/login" class="hidden w-full inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all">
                        <i class="ph ph-upload-simple text-base"></i>"Publish"
                    </a>
                </div>
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
                    </div>
                    // Mobile categories (hidden by default)
                    <div id="mp-mobile-cats" class="hidden lg:hidden mt-3">
                        <div id="mp-categories" class="flex gap-2 flex-wrap">"Loading..."</div>
                    </div>
                </div>

                // Asset grid
                <div class="p-6">
                    <div id="mp-grid" class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-4">
                        <div class="col-span-full text-center py-16">
                            <div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                        </div>
                    </div>

                    // Pagination
                    <div id="mp-pagination" class="flex justify-center gap-2 mt-8"></div>
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
                const pubBtn = document.getElementById('publish-btn');
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

                el.innerHTML = data.assets.map((a, i) => `
                    <a href="/marketplace/asset/${a.slug}" class="block group asset-card" style="animation: fadeSlideUp 0.4s ease both; animation-delay: ${i * 50}ms">
                        <div class="bg-white/[0.02] border border-zinc-800/50 rounded-xl overflow-hidden hover:border-zinc-700 hover:bg-white/[0.04] transition-all hover:shadow-lg hover:shadow-black/20 hover:-translate-y-0.5">
                            <div class="h-40 bg-surface-panel flex items-center justify-center relative overflow-hidden">
                                ${a.thumbnail_url ? `<div class="absolute inset-0 bg-cover bg-center blur-2xl scale-110 opacity-50" style="background-image:url('${a.thumbnail_url}')"></div><img src="${a.thumbnail_url}" class="relative z-10 w-full h-full object-cover group-hover:scale-105 transition-transform duration-500" />` : `<i class="ph ph-package text-3xl text-zinc-700"></i>`}
                                <span class="absolute top-2 right-2 z-20 text-[10px] px-2 py-0.5 rounded-full bg-black/60 text-zinc-300 backdrop-blur-md border border-white/5">${a.category}</span>
                                ${a.price_credits === 0 ? `<span class="absolute top-2 left-2 z-20 text-[10px] px-2 py-0.5 rounded-full bg-green-500/20 text-green-400 backdrop-blur-md border border-green-500/10">Free</span>` : ''}
                            </div>
                            <div class="p-4">
                                <h3 class="text-sm font-semibold group-hover:text-accent transition-colors truncate">${a.name}</h3>
                                <p class="text-xs text-zinc-500 mt-1 line-clamp-2 leading-relaxed">${a.description}</p>
                                <div class="flex items-center justify-between mt-3 pt-3 border-t border-zinc-800/30">
                                    <span class="text-xs text-zinc-400">${a.creator_name}</span>
                                    <span class="text-xs font-semibold ${a.price_credits === 0 ? 'text-green-400' : 'text-zinc-50'}">${a.price_credits === 0 ? 'Free' : a.price_credits.toLocaleString() + ' cr'}</span>
                                </div>
                                <div class="flex items-center gap-3 mt-2 text-[11px] text-zinc-600">
                                    <span><i class="ph ph-download-simple"></i> ${a.downloads.toLocaleString()}</span>
                                    <span>v${a.version}</span>
                                    ${a.rating_count > 0 ? `<span class="text-amber-400/80">${'★'.repeat(Math.round(a.rating_avg))}${'☆'.repeat(5 - Math.round(a.rating_avg))}</span><span>(${a.rating_count})</span>` : ''}
                                </div>
                            </div>
                        </div>
                    </a>
                `).join('');

                const totalPages = Math.ceil(data.total / data.per_page);
                const pagEl = document.getElementById('mp-pagination');
                if (totalPages <= 1) { pagEl.innerHTML = ''; return; }
                let pag = '';
                for (let i = 1; i <= totalPages; i++) {
                    pag += `<button onclick="goPage(${i})" class="w-8 h-8 rounded-lg text-xs font-medium transition-all ${i === currentPage ? 'bg-accent text-white shadow-[0_0_12px_rgba(99,102,241,0.3)]' : 'bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600'}">${i}</button>`;
                }
                pagEl.innerHTML = pag;
            }

            function goPage(p) { currentPage = p; loadAssets(); window.scrollTo({top: 0, behavior: 'smooth'}); }
            "##
        </script>

        <style>
            r#"
            @keyframes fadeSlideUp {
                from { opacity: 0; transform: translateY(16px); }
                to { opacity: 1; transform: translateY(0); }
            }
            "#
        </style>
    }
}
