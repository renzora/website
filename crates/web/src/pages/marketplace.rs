use leptos::prelude::*;

#[component]
pub fn MarketplacePage() -> impl IntoView {
    view! {
        <section class="relative py-8 px-6 min-h-[80vh]">
            <div class="max-w-[1200px] mx-auto">
                // Hero header
                <div class="relative mb-10 py-10 text-center overflow-hidden">
                    <div class="absolute inset-0 bg-gradient-to-b from-accent/5 via-transparent to-transparent rounded-2xl pointer-events-none"></div>
                    <div class="absolute top-0 left-1/2 -translate-x-1/2 w-64 h-px bg-gradient-to-r from-transparent via-accent/30 to-transparent"></div>
                    <div class="relative z-10">
                        <h1 class="text-3xl md:text-4xl font-bold">"Marketplace"</h1>
                        <p class="text-zinc-500 text-sm mt-2 max-w-md mx-auto">"Plugins, assets, themes, and scripts for every workflow."</p>
                        <a id="publish-btn" href="/login" class="hidden inline-flex items-center gap-2 px-4 py-2 mt-4 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)]">
                            <i class="ph ph-upload-simple text-base"></i>"Publish an Asset"
                        </a>
                    </div>
                </div>

                // Search + sort + filter toggle
                <div class="flex flex-col sm:flex-row gap-3 mb-4">
                    <div class="relative flex-1">
                        <i class="ph ph-magnifying-glass absolute left-3 top-1/2 -translate-y-1/2 text-zinc-500"></i>
                        <input type="text" id="mp-search" placeholder="Search assets..." oninput="loadAssets()" class="w-full pl-9 pr-4 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 focus:bg-white/[0.05] transition-all" />
                    </div>
                    <select id="mp-sort" onchange="loadAssets()" class="px-3 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm focus:border-accent/50 transition-all">
                        <option value="newest">"Newest"</option>
                        <option value="popular">"Most Popular"</option>
                        <option value="top_rated">"Top Rated"</option>
                        <option value="price_asc">"Price: Low to High"</option>
                        <option value="price_desc">"Price: High to Low"</option>
                    </select>
                    <button onclick="toggleFilters()" id="filter-toggle" class="inline-flex items-center gap-1.5 px-3.5 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-400 text-sm hover:border-zinc-600 hover:text-zinc-200 transition-all">
                        <i class="ph ph-faders text-base"></i>"Filters"
                        <span id="active-filter-count" class="hidden ml-1 w-4 h-4 rounded-full bg-accent text-white text-[10px] font-bold flex items-center justify-center"></span>
                    </button>
                </div>

                // Advanced filters panel
                <div id="filter-panel" class="hidden mb-6 p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl">
                    <div class="flex flex-wrap gap-6">
                        // Price filter
                        <div>
                            <label class="block text-xs text-zinc-500 mb-2 font-medium uppercase tracking-wider">"Price"</label>
                            <div class="flex gap-2">
                                <button onclick="setPrice('all')" id="price-all" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-accent text-white transition-all">"All"</button>
                                <button onclick="setPrice('free')" id="price-free" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 transition-all">"Free"</button>
                                <button onclick="setPrice('paid')" id="price-paid" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 transition-all">"Paid"</button>
                            </div>
                        </div>

                        // Max price
                        <div id="max-price-wrap" class="hidden">
                            <label class="block text-xs text-zinc-500 mb-2 font-medium uppercase tracking-wider">"Max Price"</label>
                            <input type="number" id="mp-max-price" min="0" placeholder="Max credits" oninput="loadAssets()"
                                class="w-32 px-3 py-1.5 bg-white/[0.02] border border-zinc-800/50 rounded-lg text-zinc-50 text-xs outline-none focus:border-accent/50" />
                        </div>

                        // Min rating
                        <div>
                            <label class="block text-xs text-zinc-500 mb-2 font-medium uppercase tracking-wider">"Min Rating"</label>
                            <div class="flex gap-1">
                                <button onclick="setMinRating(0)" id="rating-0" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-accent text-white transition-all">"Any"</button>
                                <button onclick="setMinRating(3)" id="rating-3" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 transition-all">"3+"<span class="text-amber-400 ml-0.5">"★"</span></button>
                                <button onclick="setMinRating(4)" id="rating-4" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 transition-all">"4+"<span class="text-amber-400 ml-0.5">"★"</span></button>
                                <button onclick="setMinRating(5)" id="rating-5" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 transition-all">"5"<span class="text-amber-400 ml-0.5">"★"</span></button>
                            </div>
                        </div>

                        // Clear
                        <div class="flex items-end">
                            <button onclick="clearFilters()" class="px-3 py-1.5 rounded-lg text-xs font-medium text-zinc-500 hover:text-zinc-300 transition-colors">"Clear All"</button>
                        </div>
                    </div>
                </div>

                // Category chips
                <div id="mp-categories" class="flex gap-2 flex-wrap mb-8">"Loading categories..."</div>

                // Asset grid
                <div id="mp-grid" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
                    <div class="col-span-full text-center py-16">
                        <div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                    </div>
                </div>

                // Pagination
                <div id="mp-pagination" class="flex justify-center gap-2 mt-8"></div>
            </div>
        </section>
        <script>
            r##"
            let currentCategory = new URLSearchParams(window.location.search).get('category') || 'all';
            let currentPage = 1;
            let currentPrice = 'all'; // all, free, paid
            let currentMinRating = 0;

            (async function() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                const pubBtn = document.getElementById('publish-btn');
                if (pubBtn) { pubBtn.classList.remove('hidden'); pubBtn.href = token ? '/marketplace/sell' : '/login'; }

                const catRes = await fetch('/api/marketplace/categories');
                const dbCats = catRes.ok ? await catRes.json() : [];
                const categories = [{slug: 'all', name: 'All', icon: 'ph-squares-four'}, ...dbCats.map(c => ({slug: c.slug, name: c.name, icon: c.icon}))];

                const catEl = document.getElementById('mp-categories');
                catEl.innerHTML = categories.map(c => `
                    <button onclick="setCategory('${c.slug}')" id="cat-${c.slug}" class="inline-flex items-center gap-1.5 px-3.5 py-1.5 rounded-full text-xs font-medium transition-all ${c.slug === currentCategory ? 'bg-accent text-white shadow-[0_0_12px_rgba(99,102,241,0.3)]' : 'bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200'}">
                        <i class="ph ${c.icon}"></i>${c.name}
                    </button>
                `).join('');

                loadAssets();
            })();

            function toggleFilters() {
                document.getElementById('filter-panel').classList.toggle('hidden');
            }

            function setPrice(val) {
                currentPrice = val;
                currentPage = 1;
                ['all','free','paid'].forEach(v => {
                    const el = document.getElementById('price-' + v);
                    el.className = `px-3 py-1.5 rounded-lg text-xs font-medium transition-all ${v === val ? 'bg-accent text-white' : 'bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600'}`;
                });
                document.getElementById('max-price-wrap').classList.toggle('hidden', val !== 'paid');
                updateFilterCount();
                loadAssets();
            }

            function setMinRating(val) {
                currentMinRating = val;
                currentPage = 1;
                [0,3,4,5].forEach(v => {
                    const el = document.getElementById('rating-' + v);
                    if (!el) return;
                    const isActive = v === val;
                    // Keep the inner HTML (star spans) but update the wrapper class
                    if (isActive) {
                        el.classList.remove('bg-white/[0.03]', 'border', 'border-zinc-800/50', 'text-zinc-400', 'hover:border-zinc-600');
                        el.classList.add('bg-accent', 'text-white');
                    } else {
                        el.classList.remove('bg-accent', 'text-white');
                        el.classList.add('bg-white/[0.03]', 'border', 'border-zinc-800/50', 'text-zinc-400', 'hover:border-zinc-600');
                    }
                });
                updateFilterCount();
                loadAssets();
            }

            function clearFilters() {
                currentPrice = 'all';
                currentMinRating = 0;
                document.getElementById('mp-max-price').value = '';
                setPrice('all');
                setMinRating(0);
            }

            function updateFilterCount() {
                let count = 0;
                if (currentPrice !== 'all') count++;
                if (currentMinRating > 0) count++;
                if (document.getElementById('mp-max-price')?.value) count++;
                const badge = document.getElementById('active-filter-count');
                if (count > 0) {
                    badge.textContent = count;
                    badge.classList.remove('hidden');
                } else {
                    badge.classList.add('hidden');
                }
            }

            function setCategory(slug) {
                currentCategory = slug;
                currentPage = 1;
                const url = new URL(window.location);
                if (slug === 'all') url.searchParams.delete('category');
                else url.searchParams.set('category', slug);
                history.pushState({}, '', url);
                document.querySelectorAll('[id^="cat-"]').forEach(el => {
                    const isActive = el.id === 'cat-' + slug;
                    el.className = `inline-flex items-center gap-1.5 px-3.5 py-1.5 rounded-full text-xs font-medium transition-all ${isActive ? 'bg-accent text-white shadow-[0_0_12px_rgba(99,102,241,0.3)]' : 'bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200'}`;
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
                const maxPrice = document.getElementById('mp-max-price')?.value;
                if (maxPrice && currentPrice === 'paid') url += '&max_price=' + maxPrice;

                const res = await fetch(url);
                const data = await res.json();
                const el = document.getElementById('mp-grid');

                if (!data.assets?.length) {
                    el.innerHTML = `
                        <div class="col-span-full text-center py-20">
                            <div class="w-16 h-16 bg-zinc-800/50 rounded-2xl flex items-center justify-center mx-auto mb-4">
                                <i class="ph ph-storefront text-3xl text-zinc-600"></i>
                            </div>
                            <p class="text-zinc-500 text-sm">${q ? 'No results. Try a different search.' : 'No assets found with these filters.'}</p>
                            ${(currentPrice !== 'all' || currentMinRating > 0) ? '<button onclick="clearFilters()" class="mt-3 text-xs text-accent hover:text-accent-hover transition-colors">Clear filters</button>' : ''}
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
