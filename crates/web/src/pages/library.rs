use leptos::prelude::*;

#[component]
pub fn LibraryPage() -> impl IntoView {
    view! {
        <section class="py-8 px-6 min-h-[80vh]">
            <div class="max-w-[1200px] mx-auto">
                <div class="flex justify-between items-center mb-8">
                    <div>
                        <h1 class="text-2xl font-bold">"My Library"</h1>
                        <p class="text-zinc-500 text-sm mt-1">"Assets you own. Download or install them to your project."</p>
                    </div>
                    <a href="/marketplace" class="inline-flex items-center gap-2 px-4 py-2 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)]">
                        <i class="ph ph-storefront text-base"></i>"Browse Marketplace"
                    </a>
                </div>

                // Filter bar + view toggle
                <div class="flex flex-col sm:flex-row gap-3 mb-6">
                    <div class="relative flex-1">
                        <i class="ph ph-magnifying-glass absolute left-3 top-1/2 -translate-y-1/2 text-zinc-500"></i>
                        <input type="text" id="lib-search" placeholder="Filter your assets..." oninput="filterLibrary()" class="w-full pl-9 pr-4 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 focus:bg-white/[0.05] transition-all" />
                    </div>
                    <select id="lib-category" onchange="filterLibrary()" class="px-3 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm">
                        <option value="all">"All Categories"</option>
                    </select>
                    <div class="flex gap-1 bg-white/[0.03] border border-zinc-800/50 rounded-xl p-1">
                        <button onclick="setView('grid')" id="view-grid" class="px-2.5 py-1.5 rounded-lg text-sm transition-all bg-accent text-white">
                            <i class="ph ph-grid-four"></i>
                        </button>
                        <button onclick="setView('list')" id="view-list" class="px-2.5 py-1.5 rounded-lg text-sm transition-all text-zinc-400 hover:text-zinc-200">
                            <i class="ph ph-list"></i>
                        </button>
                    </div>
                </div>

                // Asset grid/list
                <div id="lib-grid" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
                    <div class="col-span-full text-center py-16">
                        <div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                    </div>
                </div>

                // Sign-in prompt
                <div id="lib-signin" class="hidden text-center py-20">
                    <div class="w-16 h-16 bg-zinc-800/50 rounded-2xl flex items-center justify-center mx-auto mb-4">
                        <i class="ph ph-user text-3xl text-zinc-600"></i>
                    </div>
                    <p class="text-zinc-500 text-sm mb-5">"Sign in to view your library."</p>
                    <a href="/login" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">"Sign In"</a>
                </div>
            </div>
        </section>
        <script>
            r##"
            let allAssets = [];
            let currentView = 'grid';

            (async function() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) {
                    document.getElementById('lib-grid').classList.add('hidden');
                    document.getElementById('lib-signin').classList.remove('hidden');
                    return;
                }

                try {
                    const res = await fetch('/api/marketplace/purchased', {
                        headers: { 'Authorization': 'Bearer ' + token }
                    });
                    if (!res.ok) throw new Error('Failed to load library');
                    const data = await res.json();
                    allAssets = data.assets || [];

                    const cats = [...new Set(allAssets.map(a => a.category))].sort();
                    const catEl = document.getElementById('lib-category');
                    cats.forEach(c => {
                        const opt = document.createElement('option');
                        opt.value = c;
                        opt.textContent = c.charAt(0).toUpperCase() + c.slice(1);
                        catEl.appendChild(opt);
                    });

                    renderLibrary(allAssets);
                } catch (e) {
                    document.getElementById('lib-grid').innerHTML = `
                        <div class="col-span-full text-center py-16">
                            <i class="ph ph-warning text-4xl text-zinc-700 mb-3"></i>
                            <p class="text-zinc-500 text-sm">${e.message}</p>
                        </div>`;
                }
            })();

            function setView(view) {
                currentView = view;
                document.getElementById('view-grid').className = `px-2.5 py-1.5 rounded-lg text-sm transition-all ${view === 'grid' ? 'bg-accent text-white' : 'text-zinc-400 hover:text-zinc-200'}`;
                document.getElementById('view-list').className = `px-2.5 py-1.5 rounded-lg text-sm transition-all ${view === 'list' ? 'bg-accent text-white' : 'text-zinc-400 hover:text-zinc-200'}`;
                filterLibrary();
            }

            function filterLibrary() {
                const q = (document.getElementById('lib-search')?.value || '').toLowerCase();
                const cat = document.getElementById('lib-category')?.value || 'all';
                const filtered = allAssets.filter(a => {
                    const matchQ = !q || a.name.toLowerCase().includes(q) || a.description.toLowerCase().includes(q);
                    const matchCat = cat === 'all' || a.category === cat;
                    return matchQ && matchCat;
                });
                renderLibrary(filtered);
            }

            function renderLibrary(assets) {
                const el = document.getElementById('lib-grid');
                if (!assets.length) {
                    el.className = 'grid grid-cols-1 gap-4';
                    el.innerHTML = `
                        <div class="text-center py-20">
                            <div class="w-16 h-16 bg-zinc-800/50 rounded-2xl flex items-center justify-center mx-auto mb-4">
                                <i class="ph ph-books text-3xl text-zinc-600"></i>
                            </div>
                            <p class="text-zinc-500 text-sm mb-4">No assets in your library yet.</p>
                            <a href="/marketplace" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">Browse Marketplace</a>
                        </div>`;
                    return;
                }

                if (currentView === 'list') {
                    el.className = 'flex flex-col gap-2';
                    el.innerHTML = assets.map((a, i) => `
                        <div class="flex items-center gap-4 p-4 bg-white/[0.02] border border-zinc-800/50 rounded-xl hover:border-zinc-700 hover:bg-white/[0.04] transition-all group" style="animation: fadeSlideUp 0.3s ease both; animation-delay: ${i * 30}ms">
                            <div class="w-14 h-14 rounded-lg bg-surface-panel border border-zinc-800/50 flex items-center justify-center shrink-0 overflow-hidden">
                                ${a.thumbnail_url ? `<img src="${a.thumbnail_url}" class="w-full h-full object-cover" />` : `<i class="ph ph-package text-xl text-zinc-700"></i>`}
                            </div>
                            <div class="flex-1 min-w-0">
                                <a href="/marketplace/asset/${a.slug}" class="text-sm font-semibold group-hover:text-accent transition-colors truncate block">${a.name}</a>
                                <div class="flex items-center gap-2 mt-0.5 text-[11px] text-zinc-600">
                                    <span>${a.category}</span>
                                    <span>·</span>
                                    <span>${a.creator_name}</span>
                                    <span>·</span>
                                    <span>v${a.version}</span>
                                    ${a.rating_count > 0 ? `<span>·</span><span class="text-amber-400">${'★'.repeat(Math.round(a.rating_avg))}${'☆'.repeat(5 - Math.round(a.rating_avg))}</span><span>(${a.rating_count})</span>` : ''}
                                </div>
                            </div>
                            <div class="flex gap-2 shrink-0">
                                <a href="/marketplace/asset/${a.slug}?tab=reviews" class="inline-flex items-center justify-center gap-1.5 px-3 py-2 rounded-lg text-xs font-medium bg-white/[0.03] border border-zinc-800/50 text-amber-400 hover:border-amber-500/30 hover:text-amber-300 transition-all" title="Review">
                                    <i class="ph ph-star"></i> Review
                                </a>
                                <button onclick="downloadAsset('${a.id}', '${a.name.replace(/'/g, "\\'")}')" class="inline-flex items-center justify-center gap-1.5 px-3 py-2 rounded-lg text-xs font-medium bg-green-600 text-white hover:bg-green-500 transition-all hover:shadow-[0_0_15px_rgba(22,163,74,0.2)]">
                                    <i class="ph ph-download-simple"></i> Download
                                </button>
                            </div>
                        </div>
                    `).join('');
                } else {
                    el.className = 'grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4';
                    el.innerHTML = assets.map((a, i) => `
                        <div class="bg-white/[0.02] border border-zinc-800/50 rounded-xl overflow-hidden hover:border-zinc-700 hover:bg-white/[0.04] transition-all group" style="animation: fadeSlideUp 0.4s ease both; animation-delay: ${i * 50}ms">
                            <div class="h-36 bg-surface-panel flex items-center justify-center relative overflow-hidden">
                                ${a.thumbnail_url ? `<img src="${a.thumbnail_url}" class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500" />` : `<i class="ph ph-package text-3xl text-zinc-700"></i>`}
                                <span class="absolute top-2 right-2 text-[10px] px-2 py-0.5 rounded-full bg-black/60 text-zinc-300 backdrop-blur-md border border-white/5">${a.category}</span>
                            </div>
                            <div class="p-4">
                                <h3 class="text-sm font-semibold truncate">${a.name}</h3>
                                <p class="text-xs text-zinc-500 mt-1 line-clamp-2">${a.description}</p>
                                <div class="flex items-center justify-between mt-3 text-xs text-zinc-500">
                                    <span>${a.creator_name}</span>
                                    <span>v${a.version}</span>
                                </div>
                                ${a.rating_count > 0 ? `
                                    <div class="flex items-center gap-1.5 mt-2 text-[11px]">
                                        <span class="text-amber-400">${'★'.repeat(Math.round(a.rating_avg))}${'☆'.repeat(5 - Math.round(a.rating_avg))}</span>
                                        <span class="text-zinc-600">(${a.rating_count})</span>
                                    </div>
                                ` : ''}
                                <div class="flex gap-2 mt-3">
                                    <a href="/marketplace/asset/${a.slug}?tab=reviews" class="flex-1 inline-flex items-center justify-center gap-1.5 px-3 py-2 rounded-lg text-xs font-medium bg-white/[0.03] border border-zinc-800/50 text-amber-400 hover:border-amber-500/30 hover:text-amber-300 transition-all">
                                        <i class="ph ph-star"></i> Review
                                    </a>
                                    <button onclick="downloadAsset('${a.id}', '${a.name.replace(/'/g, "\\'")}')" class="flex-1 inline-flex items-center justify-center gap-1.5 px-3 py-2 rounded-lg text-xs font-medium bg-green-600 text-white hover:bg-green-500 transition-all hover:shadow-[0_0_15px_rgba(22,163,74,0.2)]">
                                        <i class="ph ph-download-simple"></i> Download
                                    </button>
                                </div>
                            </div>
                        </div>
                    `).join('');
                }
            }

            async function downloadAsset(id, name) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location = '/login'; return; }
                try {
                    const res = await fetch('/api/marketplace/' + id + '/download', {
                        headers: { 'Authorization': 'Bearer ' + token }
                    });
                    if (!res.ok) throw new Error('Download failed');
                    const data = await res.json();
                    const link = document.createElement('a');
                    link.href = data.download_url;
                    link.download = name;
                    document.body.appendChild(link);
                    link.click();
                    link.remove();
                } catch (e) {
                    alert('Download failed: ' + e.message);
                }
            }
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
