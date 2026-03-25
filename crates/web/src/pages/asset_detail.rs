use leptos::prelude::*;

#[component]
pub fn AssetDetailPage() -> impl IntoView {
    view! {
        <section class="py-8 px-6 relative overflow-hidden">
            <div class="max-w-[1100px] mx-auto relative" id="asset-detail">
                <div class="text-center py-20">
                    <div class="inline-block animate-spin w-6 h-6 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                </div>
            </div>
        </section>
        <script>
            r##"
            function parseDate(s) {
                if (!s) return null;
                const iso = s.replace(/^(\d{4}-\d{2}-\d{2})\s+(\d{2}:\d{2}:\d{2}).*?\s+([+-]\d{2}):(\d{2}).*$/, '$1T$2$3:$4');
                const d = new Date(iso);
                return isNaN(d.getTime()) ? null : d;
            }
            function fmtDate(s) {
                const d = parseDate(s);
                if (!d) return 'Unknown';
                return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
            }

            let galleryItems = [];
            let activeGalleryIndex = 0;

            (async function() {
                const slug = window.location.pathname.split('/').pop();
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                const userCookie = document.cookie.match('(^|;)\\s*user\\s*=\\s*([^;]+)')?.pop();
                let currentUserId = null;
                let myRole = '';
                if (userCookie) { try { const u = JSON.parse(decodeURIComponent(userCookie)); currentUserId = u.id; myRole = u.role; } catch(e) {} }

                const fetchHeaders = token ? { 'Authorization': 'Bearer ' + token } : {};
                const res = await fetch('/api/marketplace/detail/' + slug, { headers: fetchHeaders });
                if (!res.ok) { document.getElementById('asset-detail').innerHTML = '<p class="text-center text-zinc-500 py-20">Asset not found.</p>'; return; }
                const a = await res.json();
                const isCreator = currentUserId && a.creator && a.creator.id === currentUserId;

                const [revRes, comRes, mediaRes] = await Promise.all([
                    fetch('/api/marketplace/' + a.id + '/reviews'),
                    fetch('/api/marketplace/' + a.id + '/comments'),
                    fetch('/api/marketplace/' + a.id + '/media'),
                ]);

                const reviewsData = revRes.ok ? await revRes.json() : { reviews: [], rating_avg: 0, rating_count: 0 };
                const commentsData = comRes.ok ? await comRes.json() : { comments: [] };
                const mediaData = mediaRes.ok ? await mediaRes.json() : [];

                // Build gallery: thumbnail first, then uploaded media
                galleryItems = [];
                if (a.thumbnail_url) galleryItems.push({ type: 'image', url: a.thumbnail_url });
                mediaData.forEach(m => galleryItems.push({ type: m.media_type, url: m.url, thumb: m.thumbnail_url }));
                if (!galleryItems.length) galleryItems.push({ type: 'placeholder' });

                const ratingAvg = reviewsData.rating_avg || 0;
                const ratingCount = reviewsData.rating_count || 0;
                const fullStars = Math.round(ratingAvg);
                const starsStr = '★'.repeat(fullStars) + '☆'.repeat(5 - fullStars);
                const ratingLabel = ratingCount === 0 ? 'No reviews yet' : `${ratingAvg.toFixed(1)} (${ratingCount} review${ratingCount !== 1 ? 's' : ''})`;

                // Reviews HTML
                let reviewsHtml = '';
                if (reviewsData.reviews?.length) {
                    reviewsHtml = reviewsData.reviews.map(r => {
                        const stars = '★'.repeat(r.rating) + '☆'.repeat(5 - r.rating);
                        return `
                        <div class="p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl">
                            <div class="flex justify-between items-start mb-2">
                                <div>
                                    <span class="text-amber-400 text-base">${stars}</span>
                                    ${r.title ? `<span class="text-base font-semibold ml-2">${r.title}</span>` : ''}
                                </div>
                                <div class="flex items-center gap-2 text-xs">
                                    <a href="/profile/${r.author_name}" class="text-accent hover:text-accent-hover font-medium">${r.author_name}</a>
                                    <span class="text-zinc-600">${fmtDate(r.created_at)}</span>
                                </div>
                            </div>
                            ${r.content ? `<p class="text-sm text-zinc-400 leading-relaxed">${r.content}</p>` : ''}
                            <div class="flex items-center gap-4 mt-3">
                                ${token ? `<button onclick="markHelpful('${a.id}','${r.id}')" class="text-xs text-zinc-500 hover:text-zinc-300 transition-colors"><i class="ph ph-thumbs-up"></i> Helpful (${r.helpful_count})</button>` : `<span class="text-xs text-zinc-600"><i class="ph ph-thumbs-up"></i> ${r.helpful_count}</span>`}
                                ${token ? `<button onclick="flagReview('${a.id}','${r.id}')" class="text-xs text-zinc-600 hover:text-red-400 transition-colors"><i class="ph ph-flag"></i> Report</button>` : ''}
                            </div>
                        </div>`;
                    }).join('');
                } else {
                    reviewsHtml = '<p class="text-sm text-zinc-600">No reviews yet. Be the first!</p>';
                }

                // Comments HTML
                let commentsHtml = '';
                if (commentsData.comments?.length) {
                    commentsHtml = commentsData.comments.map(c => {
                        const canDelete = currentUserId && (currentUserId === c.author_id || currentUserId === a.creator.id || myRole === 'admin');
                        return `
                        <div class="p-4 bg-white/[0.02] border border-zinc-800/50 rounded-xl">
                            <div class="flex justify-between items-start">
                                <div class="flex items-center gap-2">
                                    <a href="/profile/${c.author_name}" class="text-sm text-accent hover:text-accent-hover font-medium">${c.author_name}</a>
                                    <span class="text-xs text-zinc-600">${fmtDate(c.created_at)}</span>
                                </div>
                                ${canDelete ? `<button onclick="deleteComment('${c.id}','${a.id}')" class="text-xs text-red-400/60 hover:text-red-400 transition-colors"><i class="ph ph-trash"></i></button>` : ''}
                            </div>
                            <p class="text-sm text-zinc-300 mt-2 leading-relaxed">${c.content}</p>
                        </div>`;
                    }).join('');
                } else {
                    commentsHtml = '<p class="text-sm text-zinc-600">No comments yet.</p>';
                }

                // Gallery HTML
                const mainMedia = galleryItems[0];
                const mainPreviewHtml = renderMainPreview(mainMedia);
                const thumbsHtml = galleryItems.length > 1 ? `
                    <div class="flex gap-2 mt-3 overflow-x-auto pb-1" id="gallery-thumbs">
                        ${galleryItems.map((item, i) => {
                            const isVideo = item.type === 'video';
                            const isAudio = item.type === 'audio';
                            const thumbSrc = item.thumb || item.url;
                            return `<button onclick="setGalleryItem(${i})" class="gallery-thumb shrink-0 w-20 h-14 rounded-lg border-2 overflow-hidden relative transition-all ${i === 0 ? 'border-accent' : 'border-zinc-800/50 hover:border-zinc-600'}" data-index="${i}">
                                ${item.type === 'placeholder' ? '<div class="w-full h-full bg-zinc-800 flex items-center justify-center"><i class="ph ph-image text-zinc-600"></i></div>' :
                                  isAudio ? `<div class="w-full h-full bg-zinc-900 flex items-center justify-center"><i class="ph ph-music-note text-xl text-accent"></i></div>` :
                                  isVideo ? `<div class="w-full h-full bg-zinc-900 flex items-center justify-center"><i class="ph ph-play-circle text-xl text-zinc-400"></i></div>` :
                                  `<img src="${thumbSrc}" class="w-full h-full object-cover" />`}
                                ${isVideo ? '<div class="absolute bottom-0.5 right-0.5 bg-black/70 rounded px-1 text-[8px] text-white">VIDEO</div>' : ''}
                                ${isAudio ? '<div class="absolute bottom-0.5 right-0.5 bg-accent/80 rounded px-1 text-[8px] text-white">AUDIO</div>' : ''}
                            </button>`;
                        }).join('')}
                    </div>` : '';

                // Blurred hero background from thumbnail
                const heroImg = a.thumbnail_url || (galleryItems[0]?.type === 'image' ? galleryItems[0].url : '');

                const el = document.getElementById('asset-detail');
                el.innerHTML = `
                    ${heroImg ? `
                    <div class="absolute inset-x-0 top-0 h-[600px] overflow-hidden pointer-events-none z-0">
                        <div class="absolute inset-0 bg-cover bg-center blur-3xl scale-150 opacity-25" style="background-image:url('${heroImg}')"></div>
                        <div class="absolute inset-0 bg-gradient-to-b from-transparent via-[#060608]/60 to-[#060608]"></div>
                    </div>` : ''}

                    <a href="/marketplace" class="inline-flex items-center gap-1.5 text-sm text-zinc-500 hover:text-zinc-300 transition-colors mb-6 relative z-10">
                        <i class="ph ph-arrow-left"></i> Back to Marketplace
                    </a>

                    <!-- Gallery + Sidebar -->
                    <div class="flex flex-col lg:flex-row gap-8 relative z-10">
                        <div class="flex-1 min-w-0">
                            <!-- Main preview -->
                            <div class="rounded-2xl overflow-hidden border border-zinc-800/50 bg-zinc-900 relative group/preview" id="main-preview">
                                ${mainPreviewHtml}
                            </div>
                            ${thumbsHtml}

                            <!-- Title + meta -->
                            <div class="mt-8">
                                <div class="flex items-center gap-3">
                                    <h1 class="text-3xl font-bold leading-tight">${a.name}</h1>
                                    ${isCreator ? `<a href="/marketplace/asset/${a.slug}/edit" class="inline-flex items-center gap-1 px-3 py-1.5 rounded-lg text-xs font-medium bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200 transition-colors"><i class="ph ph-pencil-simple"></i>Edit</a>` : ''}
                                </div>
                                <div class="flex items-center gap-4 mt-3 flex-wrap">
                                    <a href="/profile/${a.creator.username}" class="flex items-center gap-2 text-sm font-medium text-accent hover:text-accent-hover transition-colors">
                                        <div class="w-6 h-6 rounded-full bg-accent/10 flex items-center justify-center"><i class="ph ph-user text-xs text-accent"></i></div>
                                        ${a.creator.username}
                                    </a>
                                    <span class="text-sm text-zinc-600">·</span>
                                    <span class="px-2.5 py-1 rounded-full bg-white/[0.03] border border-zinc-800/50 text-xs text-zinc-400">${a.category}</span>
                                    <span class="text-sm text-zinc-500">v${a.version}</span>
                                    ${ratingCount > 0 ? `
                                        <span class="text-sm text-zinc-600">·</span>
                                        <span class="text-amber-400 text-sm">${starsStr}</span>
                                        <span class="text-sm text-zinc-500">${ratingLabel}</span>
                                    ` : ''}
                                </div>
                            </div>

                            <!-- Description -->
                            <div class="mt-8">
                                <h2 class="text-lg font-semibold mb-3">About this asset</h2>
                                <p class="text-base text-zinc-400 leading-relaxed whitespace-pre-wrap">${a.description}</p>
                            </div>

                            <div class="mt-6 flex items-center gap-4 text-sm text-zinc-600">
                                <span><i class="ph ph-calendar-blank"></i> Published ${fmtDate(a.created_at)}</span>
                                <span><i class="ph ph-clock-clockwise"></i> Updated ${fmtDate(a.updated_at)}</span>
                                <span><i class="ph ph-eye"></i> ${a.views.toLocaleString()} views</span>
                                <span><i class="ph ph-download-simple"></i> ${a.downloads.toLocaleString()} downloads</span>
                            </div>

                            <!-- Reviews -->
                            <div class="mt-12" id="reviews">
                                <div class="flex items-center justify-between mb-6">
                                    <h2 class="text-lg font-semibold">Reviews</h2>
                                    ${ratingCount > 0 ? `
                                        <div class="flex items-center gap-2">
                                            <span class="text-amber-400 text-lg">${starsStr}</span>
                                            <span class="text-sm text-zinc-400">${ratingAvg.toFixed(1)} out of 5</span>
                                            <span class="text-sm text-zinc-600">(${ratingCount})</span>
                                        </div>
                                    ` : ''}
                                </div>
                                <div class="space-y-3 mb-6">${reviewsHtml}</div>

                                ${(token && !isCreator && (a.owned || a.price_credits === 0)) ? `
                                    <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-xl">
                                        <h3 class="text-base font-semibold mb-4">Write a Review</h3>
                                        <div class="flex items-center gap-2 mb-4" id="star-picker">
                                            ${[1,2,3,4,5].map(i => `<button onclick="setRating(${i})" class="text-4xl text-zinc-600 hover:text-amber-400 hover:scale-110 transition-all cursor-pointer" id="star-${i}">★</button>`).join('')}
                                            <span class="text-sm text-zinc-500 ml-3" id="rating-label">Select a rating</span>
                                        </div>
                                        <input type="text" id="review-title" placeholder="Review title (optional)" maxlength="128" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-sm text-zinc-50 outline-none focus:border-accent/50 mb-3" />
                                        <textarea id="review-content" rows="4" placeholder="Share your experience..." class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-sm text-zinc-50 outline-none focus:border-accent/50 resize-y mb-3"></textarea>
                                        <button onclick="submitReview('${a.id}')" class="px-5 py-2.5 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)]">Submit Review</button>
                                    </div>
                                ` : (!token ? `<p class="text-sm text-zinc-600"><a href="/login" class="text-accent">Sign in</a>${a.price_credits === 0 ? '' : ' and purchase'} to review.</p>` : '')}
                            </div>

                            <!-- Comments -->
                            <div class="mt-12" id="comments">
                                <h2 class="text-lg font-semibold mb-6">Comments <span class="text-zinc-600 font-normal">(${commentsData.comments?.length || 0})</span></h2>
                                <div class="space-y-3 mb-6">${commentsHtml}</div>
                                ${(token && (a.owned || isCreator || a.price_credits === 0)) ? `
                                    <div class="flex gap-3">
                                        <textarea id="comment-input" rows="2" placeholder="Leave a comment..." class="flex-1 px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-sm text-zinc-50 outline-none focus:border-accent/50 resize-y"></textarea>
                                        <button onclick="postComment('${a.id}')" class="px-5 py-2.5 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all self-end">Post</button>
                                    </div>
                                ` : (!token ? `<p class="text-sm text-zinc-600"><a href="/login" class="text-accent">Sign in</a>${a.price_credits === 0 ? '' : ' and purchase'} to comment.</p>` : (!a.owned && !isCreator ? '<p class="text-sm text-zinc-600">Purchase this asset to leave a comment.</p>' : ''))}
                            </div>
                        </div>

                        <!-- Sidebar -->
                        <div class="w-full lg:w-80 shrink-0">
                            <div class="bg-white/[0.02] border border-zinc-800/50 rounded-2xl p-6 sticky top-20">
                                <div class="text-3xl font-bold mb-1">${a.price_credits === 0 ? '<span class="text-green-400">Free</span>' : a.price_credits.toLocaleString() + ' <span class="text-lg font-normal text-zinc-500">credits</span>'}</div>

                                ${token ? `
                                    ${isCreator ? `
                                        <button onclick="downloadAsset('${a.id}')" class="w-full mt-4 px-4 py-3 rounded-xl text-sm font-semibold bg-green-600 text-white hover:bg-green-500 transition-all hover:shadow-[0_0_20px_rgba(22,163,74,0.2)] flex items-center justify-center gap-2"><i class="ph ph-download-simple text-lg"></i>Download</button>
                                        <p class="text-xs text-zinc-500 text-center mt-2">This is your asset</p>
                                    ` : a.owned ? `
                                        <button onclick="downloadAsset('${a.id}')" class="w-full mt-4 px-4 py-3 rounded-xl text-sm font-semibold bg-green-600 text-white hover:bg-green-500 transition-all hover:shadow-[0_0_20px_rgba(22,163,74,0.2)] flex items-center justify-center gap-2"><i class="ph ph-download-simple text-lg"></i>Download</button>
                                        <a href="/library" class="w-full mt-2 px-4 py-2.5 rounded-xl text-sm font-medium bg-white/[0.03] border border-zinc-800/50 text-zinc-300 hover:border-zinc-600 hover:text-white transition-all flex items-center justify-center gap-2"><i class="ph ph-books text-lg"></i>Show in Library</a>
                                        <p class="text-xs text-green-400 text-center mt-2"><i class="ph ph-check-circle"></i> You own this asset</p>
                                    ` : a.price_credits === 0 ? `
                                        <button onclick="purchaseAsset('${a.id}')" class="w-full mt-4 px-4 py-3 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)] flex items-center justify-center gap-2"><i class="ph ph-download-simple text-lg"></i>Get Free</button>
                                    ` : `
                                        <div class="mt-4 flex gap-2">
                                            <input type="text" id="promo-input" placeholder="Promo code" maxlength="32" class="flex-1 px-3 py-2.5 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-xs text-zinc-50 outline-none focus:border-accent/50 uppercase" />
                                        </div>
                                        <button onclick="purchaseAsset('${a.id}')" class="w-full mt-2 px-4 py-3 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)] flex items-center justify-center gap-2"><i class="ph ph-shopping-cart text-lg"></i>Purchase</button>
                                    `}
                                ` : `
                                    <a href="/login" class="block w-full mt-4 px-4 py-3 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all text-center">Sign in to purchase</a>
                                `}

                                <div class="mt-6 pt-6 border-t border-zinc-800/50 space-y-3">
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Rating</span><span class="text-amber-400">${ratingCount > 0 ? starsStr + ' ' + ratingAvg.toFixed(1) : '—'}</span></div>
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Reviews</span><span class="text-zinc-300">${ratingCount}</span></div>
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Views</span><span class="text-zinc-300">${a.views.toLocaleString()}</span></div>
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Downloads</span><span class="text-zinc-300">${a.downloads.toLocaleString()}</span></div>
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Category</span><span class="text-zinc-300">${a.category}</span></div>
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Version</span><span class="text-zinc-300">${a.version}</span></div>
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Comments</span><span class="text-zinc-300">${commentsData.comments?.length || 0}</span></div>
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Published</span><span class="text-zinc-300">${fmtDate(a.created_at)}</span></div>
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Updated</span><span class="text-zinc-300">${fmtDate(a.updated_at)}</span></div>
                                </div>

                                <div class="mt-6 pt-6 border-t border-zinc-800/50">
                                    <a href="/profile/${a.creator.username}" class="flex items-center gap-3 group">
                                        <div class="w-10 h-10 rounded-full bg-accent/10 flex items-center justify-center">
                                            <i class="ph ph-user text-accent"></i>
                                        </div>
                                        <div>
                                            <div class="text-sm font-medium group-hover:text-accent transition-colors">${a.creator.username}</div>
                                            <div class="text-xs text-zinc-600">${a.creator.role}</div>
                                        </div>
                                    </a>
                                </div>
                            </div>
                        </div>
                    </div>
                `;

                const urlParams = new URLSearchParams(window.location.search);
                const tab = urlParams.get('tab');
                if (tab) {
                    const target = document.getElementById(tab);
                    if (target) target.scrollIntoView({ behavior: 'smooth' });
                }
            })();

            function renderMainPreview(item) {
                if (!item || item.type === 'placeholder') {
                    return '<div class="aspect-video flex items-center justify-center"><i class="ph ph-package text-6xl text-zinc-700"></i></div>';
                }
                if (item.type === 'audio') {
                    const ext = item.url.split('.').pop()?.toUpperCase() || 'AUDIO';
                    return `
                        <div class="aspect-video flex flex-col items-center justify-center bg-gradient-to-b from-zinc-900 to-[#0a0a0b] relative">
                            <div class="absolute inset-0 flex items-center justify-center opacity-[0.03]">
                                <i class="ph ph-waveform text-[200px]"></i>
                            </div>
                            <div class="relative z-10 flex flex-col items-center gap-4 w-full max-w-md px-6">
                                <div class="w-20 h-20 rounded-full bg-accent/10 border border-accent/20 flex items-center justify-center mb-2">
                                    <i class="ph ph-music-note text-3xl text-accent"></i>
                                </div>
                                <span class="text-xs text-zinc-500 font-mono">${ext}</span>
                                <audio id="audio-player" src="${item.url}" preload="metadata" class="hidden"></audio>
                                <div class="w-full">
                                    <div class="flex items-center gap-3 w-full">
                                        <button onclick="toggleAudioPlay()" id="audio-play-btn" class="w-10 h-10 rounded-full bg-accent hover:bg-accent-hover text-white flex items-center justify-center transition-colors shrink-0">
                                            <i class="ph ph-play-fill text-lg" id="audio-play-icon"></i>
                                        </button>
                                        <div class="flex-1">
                                            <div class="relative w-full h-1.5 bg-zinc-800 rounded-full cursor-pointer group" onclick="seekAudio(event)">
                                                <div id="audio-progress" class="absolute left-0 top-0 h-full bg-accent rounded-full transition-all" style="width:0%"></div>
                                                <div id="audio-buffered" class="absolute left-0 top-0 h-full bg-zinc-700 rounded-full -z-10" style="width:0%"></div>
                                            </div>
                                            <div class="flex justify-between mt-1">
                                                <span id="audio-current" class="text-[10px] text-zinc-600 tabular-nums">0:00</span>
                                                <span id="audio-duration" class="text-[10px] text-zinc-600 tabular-nums">0:00</span>
                                            </div>
                                        </div>
                                        <button onclick="toggleAudioVolume()" class="text-zinc-500 hover:text-zinc-300 transition-colors shrink-0">
                                            <i class="ph ph-speaker-high text-lg" id="audio-vol-icon"></i>
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>`;
                }
                if (item.type === 'video') {
                    // YouTube/external embed or direct video
                    if (item.url.includes('youtube.com') || item.url.includes('youtu.be')) {
                        const vid = item.url.match(/(?:v=|youtu\.be\/)([a-zA-Z0-9_-]+)/)?.[1];
                        return vid ? `<div class="aspect-video"><iframe src="https://www.youtube.com/embed/${vid}" class="w-full h-full" frameborder="0" allowfullscreen></iframe></div>` :
                            `<div class="aspect-video flex items-center justify-center text-zinc-600">Invalid video URL</div>`;
                    }
                    return `<video src="${item.url}" controls ${item.thumb ? `poster="${item.thumb}"` : ''} class="w-full aspect-video object-contain bg-black" ondblclick="openAssetLightbox(activeGalleryIndex)"></video>`;
                }
                return `<div class="relative group/preview"><img src="${item.url}" class="w-full aspect-video object-cover cursor-pointer" onclick="openAssetLightbox(activeGalleryIndex)" /><button onclick="openAssetLightbox(activeGalleryIndex)" class="absolute top-3 right-3 w-8 h-8 rounded-lg bg-black/50 hover:bg-black/70 flex items-center justify-center text-white/70 hover:text-white text-sm opacity-0 group-hover/preview:opacity-100 transition-all backdrop-blur-sm"><i class="ph ph-arrows-out"></i></button></div>`;
            }

            function setGalleryItem(index) {
                // Stop any playing audio before switching
                const oldAudio = document.getElementById('audio-player');
                if (oldAudio) { oldAudio.pause(); }
                activeGalleryIndex = index;
                const item = galleryItems[index];
                document.getElementById('main-preview').innerHTML = renderMainPreview(item);
                // Re-init audio player if the new item is audio
                if (item.type === 'audio') initAudioPlayer();
                document.querySelectorAll('.gallery-thumb').forEach(el => {
                    const i = parseInt(el.dataset.index);
                    el.className = el.className.replace(/border-accent|border-zinc-800\/50/g, '');
                    el.classList.add(i === index ? 'border-accent' : 'border-zinc-800/50');
                });
            }

            // ── Audio Player ──
            function initAudioPlayer() {
                const audio = document.getElementById('audio-player');
                if (!audio) return;
                audio.addEventListener('loadedmetadata', () => {
                    document.getElementById('audio-duration').textContent = fmtTime(audio.duration);
                });
                audio.addEventListener('timeupdate', () => {
                    const pct = audio.duration ? (audio.currentTime / audio.duration) * 100 : 0;
                    document.getElementById('audio-progress').style.width = pct + '%';
                    document.getElementById('audio-current').textContent = fmtTime(audio.currentTime);
                });
                audio.addEventListener('ended', () => {
                    document.getElementById('audio-play-icon').className = 'ph ph-play-fill text-lg';
                });
                audio.addEventListener('progress', () => {
                    if (audio.buffered.length > 0) {
                        const pct = (audio.buffered.end(audio.buffered.length - 1) / audio.duration) * 100;
                        document.getElementById('audio-buffered').style.width = pct + '%';
                    }
                });
            }

            function fmtTime(sec) {
                if (!sec || isNaN(sec)) return '0:00';
                const m = Math.floor(sec / 60);
                const s = Math.floor(sec % 60);
                return m + ':' + (s < 10 ? '0' : '') + s;
            }

            function toggleAudioPlay() {
                const audio = document.getElementById('audio-player');
                if (!audio) return;
                const icon = document.getElementById('audio-play-icon');
                if (audio.paused) {
                    audio.play();
                    icon.className = 'ph ph-pause-fill text-lg';
                } else {
                    audio.pause();
                    icon.className = 'ph ph-play-fill text-lg';
                }
            }

            function seekAudio(e) {
                const audio = document.getElementById('audio-player');
                if (!audio || !audio.duration) return;
                const bar = e.currentTarget;
                const rect = bar.getBoundingClientRect();
                const pct = (e.clientX - rect.left) / rect.width;
                audio.currentTime = pct * audio.duration;
            }

            function toggleAudioVolume() {
                const audio = document.getElementById('audio-player');
                if (!audio) return;
                const icon = document.getElementById('audio-vol-icon');
                audio.muted = !audio.muted;
                icon.className = audio.muted ? 'ph ph-speaker-slash text-lg' : 'ph ph-speaker-high text-lg';
            }

            // Auto-init audio player if first gallery item is audio
            setTimeout(() => {
                if (galleryItems[0]?.type === 'audio') initAudioPlayer();
            }, 500);

            let assetLbIndex = 0;

            function openAssetLightbox(index) {
                assetLbIndex = index;
                let lb = document.getElementById('asset-lightbox');
                if (!lb) {
                    lb = document.createElement('div');
                    lb.id = 'asset-lightbox';
                    lb.className = 'fixed inset-0 z-[9999] bg-black/90 backdrop-blur-sm flex items-center justify-center';
                    lb.innerHTML = `
                        <button onclick="closeAssetLightbox()" class="absolute top-4 right-4 w-10 h-10 rounded-full bg-white/10 hover:bg-white/20 flex items-center justify-center text-white text-xl transition-all z-10"><i class="ph ph-x"></i></button>
                        <button onclick="assetLightboxNav(-1)" class="asset-lb-nav absolute left-4 top-1/2 -translate-y-1/2 w-10 h-10 rounded-full bg-white/10 hover:bg-white/20 flex items-center justify-center text-white text-xl transition-all z-10"><i class="ph ph-caret-left"></i></button>
                        <button onclick="assetLightboxNav(1)" class="asset-lb-nav absolute right-4 top-1/2 -translate-y-1/2 w-10 h-10 rounded-full bg-white/10 hover:bg-white/20 flex items-center justify-center text-white text-xl transition-all z-10"><i class="ph ph-caret-right"></i></button>
                        <div id="asset-lightbox-content" class="flex items-center justify-center p-4"></div>
                        <div id="asset-lightbox-counter" class="absolute bottom-4 left-1/2 -translate-x-1/2 text-sm text-zinc-400"></div>
                    `;
                    lb.addEventListener('click', function(e) { if (e.target === lb) closeAssetLightbox(); });
                    document.body.appendChild(lb);
                } else {
                    lb.classList.remove('hidden');
                }
                document.body.style.overflow = 'hidden';
                updateAssetLightbox();
            }

            function closeAssetLightbox() {
                const lb = document.getElementById('asset-lightbox');
                if (lb) lb.classList.add('hidden');
                document.body.style.overflow = '';
                const v = document.querySelector('#asset-lightbox-content video');
                if (v) v.pause();
            }

            function assetLightboxNav(dir) {
                const v = document.querySelector('#asset-lightbox-content video');
                if (v) v.pause();
                assetLbIndex = (assetLbIndex + dir + galleryItems.length) % galleryItems.length;
                updateAssetLightbox();
            }

            function updateAssetLightbox() {
                const item = galleryItems[assetLbIndex];
                const container = document.getElementById('asset-lightbox-content');
                if (!item || item.type === 'placeholder') {
                    container.innerHTML = '<div class="text-zinc-600 text-4xl"><i class="ph ph-image"></i></div>';
                } else if (item.type === 'video') {
                    if (item.url.includes('youtube.com') || item.url.includes('youtu.be')) {
                        const vid = item.url.match(/(?:v=|youtu\.be\/)([a-zA-Z0-9_-]+)/)?.[1];
                        container.innerHTML = vid ? `<div class="w-[80vw] max-w-[1000px] aspect-video"><iframe src="https://www.youtube.com/embed/${vid}?autoplay=1" class="w-full h-full rounded-xl" frameborder="0" allowfullscreen></iframe></div>` : '';
                    } else {
                        container.innerHTML = `<video src="${item.url}" ${item.thumb ? `poster="${item.thumb}"` : ''} class="max-w-full max-h-[85vh] rounded-xl" controls autoplay></video>`;
                    }
                } else {
                    container.innerHTML = `<img src="${item.url}" class="max-w-full max-h-[85vh] rounded-xl object-contain" />`;
                }
                const counter = document.getElementById('asset-lightbox-counter');
                counter.textContent = (assetLbIndex + 1) + ' / ' + galleryItems.length;
                const nav = document.querySelectorAll('.asset-lb-nav');
                nav.forEach(el => el.style.display = galleryItems.length > 1 ? '' : 'none');
            }

            document.addEventListener('keydown', function(e) {
                const lb = document.getElementById('asset-lightbox');
                if (!lb || lb.classList.contains('hidden')) return;
                if (e.key === 'Escape') closeAssetLightbox();
                if (e.key === 'ArrowLeft') assetLightboxNav(-1);
                if (e.key === 'ArrowRight') assetLightboxNav(1);
            });

            async function purchaseAsset(id) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }
                const promoInput = document.getElementById('promo-input');
                const promoCode = promoInput ? promoInput.value.trim() : '';
                const body = { asset_id: id };
                if (promoCode) body.promo_code = promoCode;
                const res = await fetch('/api/credits/purchase', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify(body)
                });
                const data = await res.json();
                if (res.ok) { fireConfetti(); setTimeout(() => window.location.reload(), 1500); } else { alert(data.error || 'Purchase failed'); }
            }

            function fireConfetti() {
                const colors = ['#6366f1', '#818cf8', '#a78bfa', '#22c55e', '#f59e0b', '#ec4899', '#06b6d4'];
                const container = document.createElement('div');
                container.style.cssText = 'position:fixed;inset:0;pointer-events:none;z-index:9999;overflow:hidden';
                document.body.appendChild(container);
                for (let i = 0; i < 80; i++) {
                    const p = document.createElement('div');
                    const size = Math.random() * 8 + 4;
                    const x = Math.random() * 100;
                    const color = colors[Math.floor(Math.random() * colors.length)];
                    const delay = Math.random() * 0.3;
                    const drift = (Math.random() - 0.5) * 200;
                    const shape = Math.random() > 0.5 ? '50%' : '0';
                    p.style.cssText = `position:absolute;top:-10px;left:${x}%;width:${size}px;height:${size}px;background:${color};border-radius:${shape};opacity:0.9;animation:confettiFall ${1.5 + Math.random()}s ease-out ${delay}s forwards`;
                    p.style.setProperty('--drift', drift + 'px');
                    container.appendChild(p);
                }
                setTimeout(() => container.remove(), 3000);
            }

            async function downloadAsset(id) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                const res = await fetch('/api/marketplace/' + id + '/download', {
                    headers: { 'Authorization': 'Bearer ' + token }
                });
                const data = await res.json();
                if (res.ok && data.download_url) {
                    window.open(data.download_url, '_blank');
                } else { alert(data.error || 'Download failed'); }
            }

            let selectedRating = 0;
            function setRating(n) {
                selectedRating = n;
                for (let i = 1; i <= 5; i++) {
                    const el = document.getElementById('star-' + i);
                    if (el) el.className = `text-4xl transition-all cursor-pointer ${i <= n ? 'text-amber-400 scale-110' : 'text-zinc-600 hover:text-amber-400 hover:scale-110'}`;
                }
                const label = document.getElementById('rating-label');
                const labels = ['', 'Poor', 'Fair', 'Good', 'Great', 'Excellent'];
                if (label) label.textContent = labels[n];
            }

            async function submitReview(id) {
                if (selectedRating < 1) { alert('Please select a rating'); return; }
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }
                const res = await fetch('/api/marketplace/' + id + '/reviews', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        rating: selectedRating,
                        title: document.getElementById('review-title')?.value || '',
                        content: document.getElementById('review-content')?.value || '',
                    })
                });
                if (res.ok) { window.location.reload(); } else { const d = await res.json(); alert(d.error || 'Failed to submit review'); }
            }

            async function flagReview(assetId, reviewId) {
                const reason = prompt('Why are you flagging this review?');
                if (!reason) return;
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                await fetch('/api/marketplace/' + assetId + '/reviews/flag', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ review_id: reviewId, reason })
                });
                alert('Review flagged for moderation');
            }

            async function markHelpful(assetId, reviewId) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }
                await fetch('/api/marketplace/' + assetId + '/reviews/helpful', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ review_id: reviewId })
                });
                window.location.reload();
            }

            async function postComment(id) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }
                const input = document.getElementById('comment-input');
                if (!input.value.trim()) return;
                const res = await fetch('/api/marketplace/' + id + '/comments', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ content: input.value })
                });
                if (res.ok) { input.value = ''; window.location.reload(); } else { const d = await res.json(); alert(d.error || 'Failed'); }
            }

            async function deleteComment(commentId, assetId) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                await fetch('/api/marketplace/comments/' + commentId, {
                    method: 'DELETE',
                    headers: { 'Authorization': 'Bearer ' + token }
                });
                window.location.reload();
            }
            "##
        </script>

        <style>
            r#"
            @keyframes confettiFall {
                0% { transform: translateY(0) translateX(0) rotate(0deg); opacity: 1; }
                100% { transform: translateY(100vh) translateX(var(--drift, 0px)) rotate(720deg); opacity: 0; }
            }
            .gallery-thumb { transition: border-color 0.2s, transform 0.2s; }
            .gallery-thumb:hover { transform: scale(1.05); }
            "#
        </style>
    }
}
