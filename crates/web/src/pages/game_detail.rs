use leptos::prelude::*;

#[component]
pub fn GameDetailPage() -> impl IntoView {
    view! {
        <section class="relative py-8 px-6 min-h-[80vh]">
            <div class="max-w-[1000px] mx-auto">
                // Loading skeleton
                <div id="game-loading" class="flex justify-center py-20">
                    <div class="w-6 h-6 border-2 border-accent/30 border-t-accent rounded-full animate-spin"></div>
                </div>

                // Game content (hidden until loaded)
                <div id="game-content" class="hidden">
                    // Header
                    <div class="flex flex-col md:flex-row gap-6 mb-8">
                        // Thumbnail
                        <div class="w-full md:w-80 shrink-0">
                            <div id="game-thumbnail" class="aspect-[16/9] rounded-2xl overflow-hidden bg-zinc-900 border border-zinc-800/50"></div>
                        </div>
                        // Info
                        <div class="flex-1">
                            <div class="flex items-center gap-2 mb-2">
                                <span id="game-category" class="px-2 py-0.5 rounded-full bg-accent/10 border border-accent/20 text-accent text-[10px] font-medium"></span>
                                <span id="game-version" class="text-[10px] text-zinc-600"></span>
                            </div>
                            <h1 id="game-title" class="text-2xl md:text-3xl font-bold mb-2"></h1>
                            <p id="game-creator" class="text-sm text-zinc-500 mb-4"></p>

                            // Stats
                            <div class="flex items-center gap-4 mb-5">
                                <div id="game-rating" class="flex items-center gap-1 text-amber-400"></div>
                                <div id="game-downloads" class="text-xs text-zinc-500"></div>
                            </div>

                            // Price + action
                            <div class="flex items-center gap-3">
                                <div id="game-price" class="text-lg font-bold"></div>
                                <button id="game-action-btn" onclick="handleGameAction()" class="inline-flex items-center gap-2 px-6 py-2.5 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)]">
                                    "Loading..."
                                </button>
                            </div>
                            <p id="game-action-msg" class="hidden text-xs mt-2"></p>
                        </div>
                    </div>

                    // Media gallery
                    <div id="game-media" class="mb-8"></div>

                    // Description
                    <div class="mb-8">
                        <h2 class="text-lg font-semibold mb-3">"About this game"</h2>
                        <div id="game-description" class="text-sm text-zinc-400 leading-relaxed whitespace-pre-wrap"></div>
                    </div>

                    // Reviews
                    <div class="mb-8">
                        <h2 class="text-lg font-semibold mb-4">"Reviews"</h2>
                        <div id="game-reviews" class="space-y-3"></div>
                        <div id="game-no-reviews" class="hidden text-center py-8 text-zinc-600 text-sm">"No reviews yet"</div>
                    </div>
                </div>

                // Not found
                <div id="game-notfound" class="hidden text-center py-20">
                    <div class="text-4xl mb-3 opacity-30">"🎮"</div>
                    <p class="text-zinc-500">"Game not found"</p>
                    <a href="/games" class="inline-flex items-center gap-2 px-4 py-2 mt-4 rounded-lg text-sm font-medium bg-white/[0.05] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 transition-all">"Back to Game Store"</a>
                </div>
            </div>
        </section>

        <script>
            r##"
            let gameData = null;
            let gameOwned = false;
            let gameId = null;

            (async function() {
                const slug = window.location.pathname.split('/').pop();
                const loading = document.getElementById('game-loading');
                const content = document.getElementById('game-content');
                const notfound = document.getElementById('game-notfound');

                try {
                    const res = await fetch(`/api/games/detail/${slug}`, { credentials: 'include' });
                    loading.classList.add('hidden');

                    if (!res.ok) {
                        notfound.classList.remove('hidden');
                        return;
                    }

                    gameData = await res.json();
                    gameId = gameData.id;
                    gameOwned = gameData.owned === true;
                    content.classList.remove('hidden');

                    // Populate fields
                    document.getElementById('game-title').textContent = gameData.name;
                    document.getElementById('game-category').textContent = gameData.category;
                    document.getElementById('game-version').textContent = 'v' + gameData.version;
                    document.getElementById('game-creator').innerHTML = `by <a href="/profile/${gameData.creator.username}" class="text-accent hover:text-accent-hover transition-colors">${gameData.creator.username}</a>`;
                    document.getElementById('game-description').textContent = gameData.description;

                    // Thumbnail
                    const thumbEl = document.getElementById('game-thumbnail');
                    if (gameData.thumbnail_url) {
                        thumbEl.innerHTML = `<img src="${gameData.thumbnail_url}" alt="${gameData.name}" class="w-full h-full object-cover" />`;
                    } else {
                        thumbEl.innerHTML = `<div class="w-full h-full flex items-center justify-center"><span class="text-4xl opacity-20">🎮</span></div>`;
                    }

                    // Rating
                    const ratingEl = document.getElementById('game-rating');
                    if (gameData.downloads > 0 || true) {
                        const avg = gameData.rating_count > 0 ? (gameData.rating_sum / gameData.rating_count) : 0;
                        if (avg > 0) {
                            ratingEl.innerHTML = `<i class="ph-fill ph-star text-sm"></i><span class="text-sm font-medium">${avg.toFixed(1)}</span><span class="text-zinc-600 text-xs">(${gameData.rating_count})</span>`;
                        }
                    }

                    // Downloads
                    document.getElementById('game-downloads').innerHTML = `<i class="ph ph-download-simple"></i> ${gameData.downloads} downloads`;

                    // Price + action button
                    const priceEl = document.getElementById('game-price');
                    const actionBtn = document.getElementById('game-action-btn');

                    if (gameOwned) {
                        priceEl.textContent = '';
                        actionBtn.textContent = 'Download';
                        actionBtn.innerHTML = '<i class="ph ph-download-simple"></i> Download';
                    } else if (gameData.price_credits === 0) {
                        priceEl.innerHTML = '<span class="text-emerald-400">Free</span>';
                        actionBtn.innerHTML = '<i class="ph ph-plus"></i> Add to Library';
                    } else {
                        priceEl.innerHTML = `<span class="text-accent">${gameData.price_credits} credits</span>`;
                        actionBtn.innerHTML = '<i class="ph ph-shopping-cart"></i> Purchase';
                    }

                    // Load media
                    try {
                        const mediaRes = await fetch(`/api/games/${gameId}/media`);
                        const media = await mediaRes.json();
                        if (media.length > 0) {
                            const mediaEl = document.getElementById('game-media');
                            mediaEl.innerHTML = `
                                <h2 class="text-lg font-semibold mb-3">Screenshots & Videos</h2>
                                <div class="grid grid-cols-2 md:grid-cols-3 gap-3">
                                    ${media.map(m => `
                                        <div class="rounded-xl overflow-hidden border border-zinc-800/50 cursor-pointer hover:border-zinc-600 transition-all">
                                            ${m.media_type === 'video'
                                                ? `<video src="${m.url}" poster="${m.thumbnail_url || ''}" class="w-full aspect-video object-cover" controls></video>`
                                                : `<img src="${m.url}" class="w-full aspect-video object-cover hover:scale-105 transition-transform duration-300" loading="lazy" />`
                                            }
                                        </div>
                                    `).join('')}
                                </div>
                            `;
                        }
                    } catch(e) {}

                } catch(e) {
                    loading.classList.add('hidden');
                    notfound.classList.remove('hidden');
                }
            })();

            async function handleGameAction() {
                const btn = document.getElementById('game-action-btn');
                const msg = document.getElementById('game-action-msg');
                msg.classList.add('hidden');

                if (gameOwned) {
                    // Download
                    btn.innerHTML = '<div class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div> Downloading...';
                    try {
                        const res = await fetch(`/api/games/${gameId}/download`, { credentials: 'include' });
                        const data = await res.json();
                        if (data.download_url) {
                            window.open(data.download_url, '_blank');
                        }
                        btn.innerHTML = '<i class="ph ph-download-simple"></i> Download';
                    } catch(e) {
                        btn.innerHTML = '<i class="ph ph-download-simple"></i> Download';
                        msg.textContent = 'Download failed';
                        msg.className = 'text-xs mt-2 text-red-400';
                        msg.classList.remove('hidden');
                    }
                    return;
                }

                // Purchase / Add to library
                btn.innerHTML = '<div class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>';
                try {
                    const res = await fetch(`/api/games/${gameId}/purchase`, {
                        method: 'POST',
                        credentials: 'include',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ game_id: gameId }),
                    });
                    const data = await res.json();

                    if (res.ok) {
                        gameOwned = true;
                        btn.innerHTML = '<i class="ph ph-download-simple"></i> Download';
                        document.getElementById('game-price').textContent = '';
                        msg.textContent = data.message || 'Added to library!';
                        msg.className = 'text-xs mt-2 text-emerald-400';
                        msg.classList.remove('hidden');
                    } else {
                        msg.textContent = data.error || 'Purchase failed';
                        msg.className = 'text-xs mt-2 text-red-400';
                        msg.classList.remove('hidden');
                        btn.innerHTML = gameData.price_credits === 0
                            ? '<i class="ph ph-plus"></i> Add to Library'
                            : '<i class="ph ph-shopping-cart"></i> Purchase';
                    }
                } catch(e) {
                    msg.textContent = 'Connection failed';
                    msg.className = 'text-xs mt-2 text-red-400';
                    msg.classList.remove('hidden');
                    btn.innerHTML = '<i class="ph ph-shopping-cart"></i> Try Again';
                }
            }
            "##
        </script>
    }
}
