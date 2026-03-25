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
                                <div id="game-views" class="text-xs text-zinc-500"></div>
                                <div id="game-downloads" class="text-xs text-zinc-500"></div>
                            </div>

                            // Price + action
                            <div class="flex items-center gap-3">
                                <div id="game-price" class="text-lg font-bold"></div>
                                <button id="game-action-btn" onclick="handleGameAction()" class="inline-flex items-center gap-2 px-6 py-2.5 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)]">
                                    "Loading..."
                                </button>
                                <button id="wishlist-btn" onclick="toggleWishlist()" class="hidden inline-flex items-center gap-1.5 px-4 py-2.5 rounded-xl text-sm font-medium border border-zinc-800 text-zinc-400 hover:border-pink-500 hover:text-pink-400 transition-all">
                                    <i id="wishlist-icon" class="ph ph-heart text-base"></i>
                                    <span id="wishlist-text">"Wishlist"</span>
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

        // Lightbox overlay
        <div id="game-lightbox" class="hidden fixed inset-0 z-[9999] bg-black/90 backdrop-blur-sm flex items-center justify-center" onclick="if(event.target===this)closeGameLightbox()">
            <button onclick="closeGameLightbox()" class="absolute top-4 right-4 w-10 h-10 rounded-full bg-white/10 hover:bg-white/20 flex items-center justify-center text-white text-xl transition-all z-10">
                <i class="ph ph-x"></i>
            </button>
            <button onclick="gameLightboxNav(-1)" class="game-lb-nav absolute left-4 top-1/2 -translate-y-1/2 w-10 h-10 rounded-full bg-white/10 hover:bg-white/20 flex items-center justify-center text-white text-xl transition-all z-10">
                <i class="ph ph-caret-left"></i>
            </button>
            <button onclick="gameLightboxNav(1)" class="game-lb-nav absolute right-4 top-1/2 -translate-y-1/2 w-10 h-10 rounded-full bg-white/10 hover:bg-white/20 flex items-center justify-center text-white text-xl transition-all z-10">
                <i class="ph ph-caret-right"></i>
            </button>
            <div id="game-lightbox-content" class="flex items-center justify-center p-4"></div>
            <div id="game-lightbox-counter" class="absolute bottom-4 left-1/2 -translate-x-1/2 text-sm text-zinc-400"></div>
        </div>

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

                    // Views & Downloads
                    document.getElementById('game-views').innerHTML = `<i class="ph ph-eye"></i> ${gameData.views.toLocaleString()} views`;
                    document.getElementById('game-downloads').innerHTML = `<i class="ph ph-download-simple"></i> ${gameData.downloads.toLocaleString()} downloads`;

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

                    // Show wishlist button if logged in and not owned
                    if (token && !gameOwned) {
                        const wBtn = document.getElementById('wishlist-btn');
                        wBtn.classList.remove('hidden');
                        // Check if already wishlisted
                        try {
                            const wRes = await fetch('/api/games/wishlist', { headers: { 'Authorization': 'Bearer ' + token } });
                            if (wRes.ok) {
                                const wData = await wRes.json();
                                const isWishlisted = wData.some(w => w.game_id === gameId);
                                if (isWishlisted) {
                                    document.getElementById('wishlist-icon').className = 'ph-fill ph-heart text-base text-pink-400';
                                    document.getElementById('wishlist-text').textContent = 'Wishlisted';
                                    wBtn.classList.add('border-pink-500', 'text-pink-400');
                                    wBtn.classList.remove('border-zinc-800', 'text-zinc-400');
                                }
                            }
                        } catch(e) {}
                    }

                    // Load media
                    let gameMedia = [];
                    try {
                        const mediaRes = await fetch(`/api/games/${gameId}/media`);
                        const media = await mediaRes.json();
                        gameMedia = media;
                        if (media.length > 0) {
                            const mediaEl = document.getElementById('game-media');
                            mediaEl.innerHTML = `
                                <h2 class="text-lg font-semibold mb-3">Screenshots & Videos</h2>
                                <div class="grid grid-cols-2 md:grid-cols-3 gap-3">
                                    ${media.map((m, i) => `
                                        <div class="rounded-xl overflow-hidden border border-zinc-800/50 cursor-pointer hover:border-zinc-600 transition-all" onclick="openGameLightbox(${i})">
                                            ${m.media_type === 'video'
                                                ? `<div class="relative"><video src="${m.url}" poster="${m.thumbnail_url || ''}" class="w-full aspect-video object-cover" preload="metadata"></video><div class="absolute inset-0 flex items-center justify-center"><div class="w-12 h-12 rounded-full bg-black/60 flex items-center justify-center backdrop-blur-sm"><i class="ph-fill ph-play text-xl text-white"></i></div></div></div>`
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

            let gameLightboxIndex = 0;

            function openGameLightbox(index) {
                gameLightboxIndex = index;
                const lb = document.getElementById('game-lightbox');
                lb.classList.remove('hidden');
                document.body.style.overflow = 'hidden';
                updateGameLightbox();
            }

            function closeGameLightbox() {
                document.getElementById('game-lightbox').classList.add('hidden');
                document.body.style.overflow = '';
                // pause any playing video
                const v = document.querySelector('#game-lightbox-content video');
                if (v) v.pause();
            }

            function gameLightboxNav(dir) {
                const v = document.querySelector('#game-lightbox-content video');
                if (v) v.pause();
                gameLightboxIndex = (gameLightboxIndex + dir + gameMedia.length) % gameMedia.length;
                updateGameLightbox();
            }

            function updateGameLightbox() {
                const m = gameMedia[gameLightboxIndex];
                const container = document.getElementById('game-lightbox-content');
                if (m.media_type === 'video') {
                    container.innerHTML = `<video src="${m.url}" poster="${m.thumbnail_url || ''}" class="max-w-full max-h-[85vh] rounded-xl" controls autoplay></video>`;
                } else {
                    container.innerHTML = `<img src="${m.url}" class="max-w-full max-h-[85vh] rounded-xl object-contain" />`;
                }
                const counter = document.getElementById('game-lightbox-counter');
                counter.textContent = (gameLightboxIndex + 1) + ' / ' + gameMedia.length;
                // show/hide nav
                const nav = document.querySelectorAll('.game-lb-nav');
                nav.forEach(el => el.style.display = gameMedia.length > 1 ? '' : 'none');
            }

            document.addEventListener('keydown', function(e) {
                const lb = document.getElementById('game-lightbox');
                if (!lb || lb.classList.contains('hidden')) return;
                if (e.key === 'Escape') closeGameLightbox();
                if (e.key === 'ArrowLeft') gameLightboxNav(-1);
                if (e.key === 'ArrowRight') gameLightboxNav(1);
            });

            async function toggleWishlist() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }
                try {
                    const res = await fetch('/api/games/wishlist/' + gameId, {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token }
                    });
                    if (res.ok) {
                        const data = await res.json();
                        const icon = document.getElementById('wishlist-icon');
                        const text = document.getElementById('wishlist-text');
                        const btn = document.getElementById('wishlist-btn');
                        if (data.wishlisted) {
                            icon.className = 'ph-fill ph-heart text-base text-pink-400';
                            text.textContent = 'Wishlisted';
                            btn.classList.add('border-pink-500', 'text-pink-400');
                            btn.classList.remove('border-zinc-800', 'text-zinc-400');
                        } else {
                            icon.className = 'ph ph-heart text-base';
                            text.textContent = 'Wishlist';
                            btn.classList.remove('border-pink-500', 'text-pink-400');
                            btn.classList.add('border-zinc-800', 'text-zinc-400');
                        }
                    }
                } catch(e) {}
            }

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
