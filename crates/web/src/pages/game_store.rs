use leptos::prelude::*;

#[component]
pub fn GameStorePage() -> impl IntoView {
    view! {
        <section class="relative py-8 px-6 min-h-[80vh] bg-gradient-to-b from-[#0a0a12] via-[#060608] to-[#060608]">
            <div class="max-w-[1200px] mx-auto">
                // Hero
                <div class="relative mb-10 py-10 text-center overflow-hidden">
                    <div class="absolute inset-0 bg-gradient-to-b from-accent/5 via-transparent to-transparent rounded-2xl pointer-events-none"></div>
                    <div class="absolute top-0 left-1/2 -translate-x-1/2 w-64 h-px bg-gradient-to-r from-transparent via-accent/30 to-transparent"></div>
                    <div class="relative z-10">
                        <h1 class="text-3xl md:text-4xl font-bold">"Game Store"</h1>
                        <p class="text-zinc-500 text-sm mt-2 max-w-md mx-auto">"Discover, purchase, and play games built with Renzora Engine."</p>
                        <a id="publish-btn" href="/login" class="hidden inline-flex items-center gap-2 px-4 py-2 mt-4 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)]">
                            <i class="ph ph-upload-simple text-base"></i>"Publish a Game"
                        </a>
                    </div>
                </div>

                // Search + sort
                <div class="flex flex-col sm:flex-row gap-3 mb-4">
                    <div class="relative flex-1">
                        <i class="ph ph-magnifying-glass absolute left-3 top-1/2 -translate-y-1/2 text-zinc-500"></i>
                        <input type="text" id="gs-search" placeholder="Search games..." oninput="loadGames()" class="w-full pl-9 pr-4 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 focus:bg-white/[0.05] transition-all" />
                    </div>
                    <select id="gs-sort" onchange="loadGames()" class="px-3 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm focus:border-accent/50 transition-all">
                        <option value="newest">"Newest"</option>
                        <option value="popular">"Most Popular"</option>
                        <option value="top_rated">"Top Rated"</option>
                        <option value="price_asc">"Price: Low to High"</option>
                        <option value="price_desc">"Price: High to Low"</option>
                    </select>
                </div>

                // Categories
                <div id="gs-categories" class="flex gap-2 flex-wrap mb-6">
                    <button onclick="setCategory('all')" class="gs-cat-btn active px-3 py-1.5 rounded-lg text-xs font-medium bg-accent text-white transition-all">"All"</button>
                </div>

                // Game grid
                <div id="gs-grid" class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4 mb-8"></div>

                // Pagination
                <div id="gs-pagination" class="flex items-center justify-center gap-3 mb-8"></div>

                // Empty state
                <div id="gs-empty" class="hidden text-center py-16">
                    <div class="text-4xl mb-3 opacity-30">"🎮"</div>
                    <p class="text-zinc-500">"No games found"</p>
                </div>

                // Loading
                <div id="gs-loading" class="flex justify-center py-16">
                    <div class="w-6 h-6 border-2 border-accent/30 border-t-accent rounded-full animate-spin"></div>
                </div>
            </div>
        </section>

        <script>
            r##"
            let gsPage = 1, gsCategory = 'all';

            async function loadGames() {
                const search = document.getElementById('gs-search').value;
                const sort = document.getElementById('gs-sort').value;
                const grid = document.getElementById('gs-grid');
                const empty = document.getElementById('gs-empty');
                const loading = document.getElementById('gs-loading');
                const pagination = document.getElementById('gs-pagination');

                loading.classList.remove('hidden');
                grid.innerHTML = '';
                empty.classList.add('hidden');

                const params = new URLSearchParams();
                if (search) params.set('q', search);
                if (gsCategory !== 'all') params.set('category', gsCategory);
                params.set('sort', sort);
                params.set('page', gsPage);

                try {
                    const res = await fetch('/api/games?' + params.toString());
                    const data = await res.json();
                    loading.classList.add('hidden');

                    if (!data.games || data.games.length === 0) {
                        empty.classList.remove('hidden');
                        pagination.innerHTML = '';
                        return;
                    }

                    grid.innerHTML = data.games.map((game, i) => {
                        const rating = game.rating_count > 0 ? game.rating_avg.toFixed(1) : null;
                        const isFree = game.price_credits === 0;
                        return `
                            <a href="/games/${game.slug}" class="group rounded-xl overflow-hidden bg-white/[0.02] border border-zinc-800/50 hover:border-accent/30 transition-all" style="animation: fadeSlideUp 0.4s ease both; animation-delay: ${i * 50}ms">
                                <div class="aspect-[16/9] bg-zinc-900 relative overflow-hidden">
                                    ${game.thumbnail_url
                                        ? `<div class="absolute inset-0 bg-cover bg-center blur-2xl scale-110 opacity-50" style="background-image:url('${game.thumbnail_url}')"></div><img src="${game.thumbnail_url}" alt="${game.name}" class="relative z-10 w-full h-full object-cover group-hover:scale-105 transition-transform duration-300" loading="lazy" />`
                                        : `<div class="w-full h-full flex items-center justify-center"><span class="text-3xl opacity-20">🎮</span></div>`
                                    }
                                    ${isFree ? `<div class="absolute top-2 right-2 z-20 px-2 py-0.5 rounded-full bg-emerald-500/20 text-emerald-400 text-[10px] font-bold backdrop-blur-sm">FREE</div>` : ''}
                                    ${game.category ? `<div class="absolute top-2 left-2 z-20 px-2 py-0.5 rounded-full bg-black/40 text-white/70 text-[10px] font-medium backdrop-blur-sm">${game.category}</div>` : ''}
                                </div>
                                <div class="p-3">
                                    <h3 class="text-sm font-semibold truncate mb-0.5">${game.name}</h3>
                                    <p class="text-[11px] text-zinc-500 truncate mb-2">${game.creator_name}</p>
                                    <div class="flex items-center justify-between">
                                        <div class="flex items-center gap-2">
                                            ${rating ? `<div class="flex items-center gap-1 text-amber-400"><i class="ph-fill ph-star text-[10px]"></i><span class="text-[10px] font-medium">${rating}</span></div>` : ''}
                                            ${game.downloads > 0 ? `<span class="text-[10px] text-zinc-600"><i class="ph ph-download-simple"></i> ${game.downloads}</span>` : ''}
                                        </div>
                                        ${!isFree ? `<span class="text-[10px] font-medium text-accent">${game.price_credits} credits</span>` : ''}
                                    </div>
                                </div>
                            </a>
                        `;
                    }).join('');

                    // Pagination
                    const totalPages = Math.ceil(data.total / data.per_page);
                    if (totalPages > 1) {
                        pagination.innerHTML = `
                            <button onclick="gsPage = Math.max(1, gsPage - 1); loadGames()" class="px-3 py-1.5 rounded-lg text-xs font-medium ${gsPage <= 1 ? 'text-zinc-600 cursor-not-allowed' : 'text-zinc-400 hover:text-zinc-200 bg-white/[0.03] border border-zinc-800/50 hover:border-zinc-600'} transition-all" ${gsPage <= 1 ? 'disabled' : ''}>Previous</button>
                            <span class="text-xs text-zinc-500">Page ${gsPage} of ${totalPages}</span>
                            <button onclick="gsPage++; loadGames()" class="px-3 py-1.5 rounded-lg text-xs font-medium ${gsPage >= totalPages ? 'text-zinc-600 cursor-not-allowed' : 'text-zinc-400 hover:text-zinc-200 bg-white/[0.03] border border-zinc-800/50 hover:border-zinc-600'} transition-all" ${gsPage >= totalPages ? 'disabled' : ''}>Next</button>
                        `;
                    } else {
                        pagination.innerHTML = '';
                    }
                } catch (e) {
                    loading.classList.add('hidden');
                    empty.classList.remove('hidden');
                }
            }

            function setCategory(cat) {
                gsCategory = cat;
                gsPage = 1;
                document.querySelectorAll('.gs-cat-btn').forEach(b => {
                    b.classList.remove('bg-accent', 'text-white');
                    b.classList.add('bg-white/[0.03]', 'border', 'border-zinc-800/50', 'text-zinc-400');
                });
                event.target.classList.remove('bg-white/[0.03]', 'border', 'border-zinc-800/50', 'text-zinc-400');
                event.target.classList.add('bg-accent', 'text-white');
                loadGames();
            }

            // Load categories
            (async function() {
                try {
                    const res = await fetch('/api/games/categories');
                    const cats = await res.json();
                    const container = document.getElementById('gs-categories');
                    cats.forEach(cat => {
                        const btn = document.createElement('button');
                        btn.className = 'gs-cat-btn px-3 py-1.5 rounded-lg text-xs font-medium bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 transition-all';
                        btn.textContent = cat.name;
                        btn.onclick = () => setCategory(cat.slug);
                        container.appendChild(btn);
                    });
                } catch(e) {}

                // Check auth for publish button
                try {
                    const res = await fetch('/api/auth/me', { credentials: 'include' });
                    if (res.ok) {
                        const btn = document.getElementById('publish-btn');
                        btn.href = '/games/upload';
                        btn.classList.remove('hidden');
                    }
                } catch(e) {}

                loadGames();
            })();
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
