use leptos::prelude::*;

#[component]
pub fn MarketplacePage() -> impl IntoView {
    view! {
        <section class="py-8 px-6">
            <div class="max-w-[1200px] mx-auto">
                <div class="flex justify-between items-center mb-6">
                    <div>
                        <h1 class="text-2xl font-bold">"Marketplace"</h1>
                        <p class="text-zinc-500 text-sm mt-1">"Assets for every engine and every workflow."</p>
                    </div>
                    <a href="/marketplace/upload" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                        <i class="ph ph-upload-simple text-base"></i>"Publish an Asset"
                    </a>
                </div>

                // Search + filters
                <div class="flex flex-col sm:flex-row gap-3 mb-6">
                    <div class="relative flex-1">
                        <i class="ph ph-magnifying-glass absolute left-3 top-1/2 -translate-y-1/2 text-zinc-500"></i>
                        <input type="text" id="mp-search" placeholder="Search assets..." oninput="loadAssets()" class="w-full pl-9 pr-4 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors" />
                    </div>
                    <select id="mp-sort" onchange="loadAssets()" class="px-3 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm">
                        <option value="newest">"Newest"</option>
                        <option value="popular">"Most Popular"</option>
                        <option value="price_asc">"Price: Low to High"</option>
                        <option value="price_desc">"Price: High to Low"</option>
                    </select>
                </div>

                // Category chips
                <div id="mp-categories" class="flex gap-2 flex-wrap mb-6">"Loading categories..."</div>

                // Asset grid
                <div id="mp-grid" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">"Loading..."</div>

                // Pagination
                <div id="mp-pagination" class="flex justify-center gap-2 mt-8"></div>
            </div>
        </section>
        <script>
            r##"
            let currentCategory = new URLSearchParams(window.location.search).get('category') || 'all';
            let currentPage = 1;

            (async function() {
                // Fetch categories from API
                const catRes = await fetch('/api/marketplace/categories');
                const dbCats = catRes.ok ? await catRes.json() : [];
                const categories = [{slug: 'all', name: 'All', icon: 'ph-squares-four'}, ...dbCats.map(c => ({slug: c.slug, name: c.name, icon: c.icon}))];

                const catEl = document.getElementById('mp-categories');
                catEl.innerHTML = categories.map(c => `
                    <button onclick="setCategory('${c.slug}')" id="cat-${c.slug}" class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium transition-all ${c.slug === currentCategory ? 'bg-accent text-white' : 'bg-surface-card border border-zinc-800 text-zinc-400 hover:border-zinc-600'}">
                        <i class="ph ${c.icon}"></i>${c.name}
                    </button>
                `).join('');

                loadAssets();
            })();

            function setCategory(slug) {
                currentCategory = slug;
                currentPage = 1;
                // Update URL without reload
                const url = new URL(window.location);
                if (slug === 'all') url.searchParams.delete('category');
                else url.searchParams.set('category', slug);
                history.pushState({}, '', url);
                // Update chip styles
                document.querySelectorAll('[id^="cat-"]').forEach(el => {
                    const isActive = el.id === 'cat-' + slug;
                    el.className = `inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs font-medium transition-all ${isActive ? 'bg-accent text-white' : 'bg-surface-card border border-zinc-800 text-zinc-400 hover:border-zinc-600'}`;
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

                const res = await fetch(url);
                const data = await res.json();
                const el = document.getElementById('mp-grid');

                if (!data.assets?.length) {
                    el.innerHTML = `
                        <div class="col-span-full text-center py-16">
                            <i class="ph ph-storefront text-4xl text-zinc-700 mb-3"></i>
                            <p class="text-zinc-500 text-sm">No assets found. ${q ? 'Try a different search.' : 'Be the first to publish!'}</p>
                        </div>`;
                    document.getElementById('mp-pagination').innerHTML = '';
                    return;
                }

                el.innerHTML = data.assets.map(a => `
                    <a href="/marketplace/asset/${a.slug}" class="block group">
                        <div class="bg-surface-card border border-zinc-800 rounded-xl overflow-hidden hover:border-zinc-700 transition-all">
                            <div class="h-36 bg-surface flex items-center justify-center relative">
                                ${a.thumbnail_url ? `<img src="${a.thumbnail_url}" class="w-full h-full object-cover" />` : `<i class="ph ph-package text-3xl text-zinc-700"></i>`}
                                <span class="absolute top-2 right-2 text-[10px] px-1.5 py-0.5 rounded bg-black/50 text-zinc-300 backdrop-blur-sm">${a.category}</span>
                            </div>
                            <div class="p-3">
                                <h3 class="text-base font-semibold group-hover:text-accent transition-colors truncate">${a.name}</h3>
                                <p class="text-sm text-zinc-500 mt-1 line-clamp-2">${a.description}</p>
                                <div class="flex items-center justify-between mt-3">
                                    <span class="text-sm text-zinc-400">${a.creator_name}</span>
                                    <span class="text-sm font-semibold ${a.price_credits === 0 ? 'text-green-400' : 'text-zinc-50'}">${a.price_credits === 0 ? 'Free' : a.price_credits + ' credits'}</span>
                                </div>
                                <div class="flex items-center gap-3 mt-2 text-xs text-zinc-500">
                                    <span><i class="ph ph-download-simple"></i> ${a.downloads}</span>
                                    <span>v${a.version}</span>
                                </div>
                            </div>
                        </div>
                    </a>
                `).join('');

                // Pagination
                const totalPages = Math.ceil(data.total / data.per_page);
                const pagEl = document.getElementById('mp-pagination');
                if (totalPages <= 1) { pagEl.innerHTML = ''; return; }
                let pag = '';
                for (let i = 1; i <= totalPages; i++) {
                    pag += `<button onclick="goPage(${i})" class="w-8 h-8 rounded-lg text-xs font-medium ${i === currentPage ? 'bg-accent text-white' : 'bg-surface-card border border-zinc-800 text-zinc-400 hover:border-zinc-600'}">${i}</button>`;
                }
                pagEl.innerHTML = pag;
            }

            function goPage(p) { currentPage = p; loadAssets(); window.scrollTo(0, 0); }
            "##
        </script>
    }
}
