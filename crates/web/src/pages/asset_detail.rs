use leptos::prelude::*;

#[component]
pub fn AssetDetailPage() -> impl IntoView {
    view! {
        <section class="py-8 px-6 relative min-h-screen">
            // Background layer — inside the section so it's part of the document flow
            <div class="fixed inset-0 pointer-events-none overflow-hidden" style="z-index:0" id="asset-bg-layer">
                <canvas id="asset-canvas" class="absolute inset-0 w-full h-full" style="z-index:1"></canvas>
                <div class="absolute top-[10%] left-[15%] w-96 h-96 bg-accent/20 rounded-full blur-[80px] animate-pulse" style="z-index:0"></div>
                <div class="absolute bottom-[20%] right-[10%] w-80 h-80 bg-purple-600/15 rounded-full blur-[60px]" style="z-index:0;animation:pulse 4s ease-in-out infinite 1s"></div>
                <div id="asset-thumb-bg" class="absolute inset-0" style="z-index:0"></div>
                <div class="absolute inset-0 bg-[#060608]/15" style="z-index:2"></div>
            </div>
            <div class="max-w-[1100px] mx-auto relative" style="z-index:10" id="asset-detail">
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
                // If there's audio + image, merge them into a combined audio-with-artwork item
                galleryItems = [];
                const audioItems = mediaData.filter(m => m.media_type === 'audio');
                const otherItems = mediaData.filter(m => m.media_type !== 'audio');
                const coverImg = a.thumbnail_url || otherItems.find(m => m.media_type === 'image')?.url || '';

                if (audioItems.length > 0 && coverImg) {
                    // Combine: each audio track gets the cover image as background
                    audioItems.forEach(m => galleryItems.push({ type: 'audio', url: m.url, cover: coverImg }));
                    // Add remaining non-audio media
                    otherItems.forEach(m => galleryItems.push({ type: m.media_type, url: m.url, thumb: m.thumbnail_url }));
                } else {
                    if (a.thumbnail_url) galleryItems.push({ type: 'image', url: a.thumbnail_url });
                    mediaData.forEach(m => galleryItems.push({ type: m.media_type, url: m.url, thumb: m.thumbnail_url }));
                }
                if (!galleryItems.length) galleryItems.push({ type: 'placeholder' });

                const ratingAvg = reviewsData.rating_avg || 0;
                const ratingCount = reviewsData.rating_count || 0;
                const fullStars = Math.round(ratingAvg);
                const starsHtml = `<span class="text-amber-400">${'★'.repeat(fullStars)}</span><span class="text-zinc-700">${'☆'.repeat(5 - fullStars)}</span>`;
                const ratingLabel = `${ratingCount > 0 ? ratingAvg.toFixed(1) + ' ' : ''}(${ratingCount})`;

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

                // Inject blurred thumbnail into the background layer
                if (heroImg) {
                    const thumbBg = document.getElementById('asset-thumb-bg');
                    if (thumbBg) {
                        thumbBg.innerHTML = `
                            <div class="absolute inset-0 bg-cover bg-center blur-2xl scale-150 opacity-15" style="background-image:url('${heroImg}')"></div>
                            <div class="absolute inset-0 bg-gradient-to-b from-transparent via-[#060608]/60 to-[#060608]"></div>
                        `;
                    }
                }

                const el = document.getElementById('asset-detail');
                el.innerHTML = `

                    <a href="/marketplace" class="inline-flex items-center gap-1.5 text-sm text-zinc-500 hover:text-zinc-300 transition-colors mb-6">
                        <i class="ph ph-arrow-left"></i> Back to Marketplace
                    </a>

                    <!-- Gallery + Sidebar -->
                    <div class="flex flex-col lg:flex-row gap-8">
                        <div class="flex-1 min-w-0">
                            <!-- Live Preview or static gallery -->
                            ${(() => {
                                const cat = (a.category || '').toLowerCase();
                                const previewable = ['3d models', 'animations', 'materials & shaders', 'textures & hdris', 'particle effects'];
                                const hasLivePreview = previewable.some(c => cat.includes(c.split(' ')[0]));
                                if (!hasLivePreview) return '<div class="rounded-2xl overflow-hidden border border-zinc-800/50 bg-zinc-900 relative group/preview" id="main-preview">' + mainPreviewHtml + '</div>' + thumbsHtml;

                                let previewMode = 'shader';
                                if (cat.includes('3d') || cat.includes('model')) previewMode = 'model';
                                else if (cat.includes('anim')) previewMode = 'animation';
                                else if (cat.includes('material') || cat.includes('shader')) previewMode = 'shader';
                                else if (cat.includes('texture') || cat.includes('hdri')) previewMode = 'texture';
                                else if (cat.includes('particle')) previewMode = 'particle';

                                // Stash preview config for post-render init
                                // Use proxy endpoint to avoid CORS issues with CDN
                                window.__previewConfig = { mode: previewMode, fileUrl: '/api/marketplace/' + a.id + '/preview-file', category: cat };

                                const meshBtns = ['sphere','cube','plane','torus'].map(s =>
                                    '<button onclick="previewSetMesh(this.dataset.mesh)" class="px-2 py-0.5 rounded text-[11px] ' +
                                    (s === 'cube' ? 'bg-accent/20 text-accent' : 'text-zinc-500 hover:text-zinc-300') +
                                    ' transition-colors" data-mesh="' + s + '">' + s[0].toUpperCase() + s.slice(1) + '</button>'
                                ).join('');

                                return '<div class="mt-4" id="live-preview-section">' +
                                    '<div class="flex items-center justify-between mb-2">' +
                                        '<div class="flex items-center gap-2">' +
                                            '<i class="ph ph-play-circle text-accent"></i>' +
                                            '<span class="text-sm font-medium text-zinc-300">Live Preview</span>' +
                                            '<span class="px-1.5 py-0.5 rounded bg-accent/10 border border-accent/20 text-[10px] text-accent font-medium">BETA</span>' +
                                        '</div>' +
                                        '<div class="flex items-center gap-1.5" id="preview-mesh-controls" style="display:' +
                                            (previewMode === 'shader' || previewMode === 'material' ? '' : 'none') + '">' +
                                            '<span class="text-[11px] text-zinc-600 mr-1">Mesh:</span>' +
                                            meshBtns +
                                        '</div>' +
                                    '</div>' +
                                    '<div class="rounded-2xl overflow-hidden border border-zinc-800/50 bg-[#0f0f13] relative" style="aspect-ratio:16/9">' +
                                        '<canvas id="preview-canvas" class="w-full h-full"></canvas>' +
                                        '<div id="preview-loading" class="absolute inset-0 flex items-center justify-center bg-[#0f0f13]">' +
                                            '<div class="text-center">' +
                                                '<div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-accent rounded-full mb-2"></div>' +
                                                '<p class="text-xs text-zinc-600">Loading preview engine...</p>' +
                                            '</div>' +
                                        '</div>' +
                                    '</div>' +
                                    '<div id="preview-params" class="mt-2"></div>' +
                                '</div>';
                            })()}

                            <!-- Title + meta -->
                            <div class="mt-8">
                                <div class="flex items-center gap-3">
                                    <h1 class="text-3xl font-bold leading-tight">${a.name}</h1>
                                    ${isCreator ? `<a href="/marketplace/asset/${a.slug}/edit" class="inline-flex items-center gap-1 px-3 py-1.5 rounded-lg text-xs font-medium bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200 transition-colors"><i class="ph ph-pencil-simple"></i>Edit</a><button onclick="deleteAsset('${a.id}')" class="inline-flex items-center gap-1 px-3 py-1.5 rounded-lg text-xs font-medium bg-white/[0.03] border border-red-900/50 text-red-400 hover:border-red-700 hover:text-red-300 hover:bg-red-950/30 transition-colors"><i class="ph ph-trash"></i>Delete</button>` : ''}
                                </div>
                                <div class="flex items-center gap-4 mt-3 flex-wrap">
                                    <a href="/profile/${a.creator.username}" class="flex items-center gap-2 text-sm font-medium text-accent hover:text-accent-hover transition-colors">
                                        <div class="w-6 h-6 rounded-full bg-accent/10 flex items-center justify-center"><i class="ph ph-user text-xs text-accent"></i></div>
                                        ${a.creator.username}
                                    </a>
                                    <span class="text-sm text-zinc-600">·</span>
                                    <span class="px-2.5 py-1 rounded-full bg-white/[0.03] border border-zinc-800/50 text-xs text-zinc-400">${a.category}</span>
                                    ${(a.tags || []).map(t => `<span class="px-2 py-0.5 rounded-full bg-accent/10 border border-accent/20 text-[11px] text-accent">${t}</span>`).join('')}
                                    <span class="text-sm text-zinc-500">v${a.version}</span>
                                    <span class="text-sm text-zinc-600">·</span>
                                    <span class="text-sm">${starsHtml}</span>
                                    <span class="text-sm text-zinc-500">${ratingLabel}</span>
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
                                    ${true ? `
                                        <div class="flex items-center gap-2">
                                            <span class="text-lg">${starsHtml}</span>
                                            <span class="text-sm text-zinc-400">${ratingCount > 0 ? ratingAvg.toFixed(1) + ' out of 5' : 'No ratings yet'}</span>
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

                                ${token ? `
                                    ${isCreator ? `
                                        <button onclick="downloadAsset('${a.id}')" class="w-full mt-4 px-4 py-3 rounded-xl text-sm font-semibold bg-green-600 text-white hover:bg-green-500 transition-all hover:shadow-[0_0_20px_rgba(22,163,74,0.2)] flex items-center justify-center gap-2"><i class="ph ph-download-simple text-lg"></i>Download</button>
                                        <p class="text-xs text-zinc-500 text-center mt-2">This is your asset</p>
                                    ` : a.owned ? `
                                        <button onclick="downloadAsset('${a.id}')" class="w-full mt-4 px-4 py-3 rounded-xl text-sm font-semibold bg-green-600 text-white hover:bg-green-500 transition-all hover:shadow-[0_0_20px_rgba(22,163,74,0.2)] flex items-center justify-center gap-2"><i class="ph ph-download-simple text-lg"></i>Download</button>
                                        <a href="/library" class="w-full mt-2 px-4 py-2.5 rounded-xl text-sm font-medium bg-white/[0.03] border border-zinc-800/50 text-zinc-300 hover:border-zinc-600 hover:text-white transition-all flex items-center justify-center gap-2"><i class="ph ph-books text-lg"></i>Show in Library</a>
                                        <p class="text-xs text-green-400 text-center mt-2"><i class="ph ph-check-circle"></i> You own this asset</p>
                                    ` : a.price_credits === 0 ? `
                                        <button onclick="purchaseAsset('${a.id}')" class="w-full mt-4 px-4 py-3 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)] flex items-center justify-center gap-2"><i class="ph ph-download-simple text-lg"></i>Download for Free</button>
                                    ` : `
                                        <div class="mt-4 flex gap-2">
                                            <input type="text" id="promo-input" placeholder="Promo code" maxlength="32" class="flex-1 px-3 py-2.5 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-xs text-zinc-50 outline-none focus:border-accent/50 uppercase" />
                                        </div>
                                        <button onclick="purchaseAsset('${a.id}')" class="w-full mt-2 px-4 py-3 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)] flex items-center justify-center gap-2"><i class="ph ph-shopping-cart text-lg"></i>Buy for ${a.price_credits.toLocaleString()} credits</button>
                                    `}
                                ` : `
                                    <a href="/login" class="block w-full mt-4 px-4 py-3 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all text-center">Sign in to purchase</a>
                                `}

                                <div class="mt-6 pt-6 border-t border-zinc-800/50 space-y-3">
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Rating</span><span>${starsHtml} <span class="text-zinc-500">(${ratingCount})</span></span></div>
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Reviews</span><span class="text-zinc-300">${ratingCount}</span></div>
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Views</span><span class="text-zinc-300">${a.views.toLocaleString()}</span></div>
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Downloads</span><span class="text-zinc-300">${a.downloads.toLocaleString()}</span></div>
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Category</span><span class="text-zinc-300">${a.category}</span></div>
                                    ${(a.tags || []).length ? `<div class="text-sm"><span class="text-zinc-500 block mb-1.5">Tags</span><div class="flex flex-wrap gap-1">${a.tags.map(t => `<span class="px-2 py-0.5 rounded-full bg-accent/10 border border-accent/20 text-[10px] text-accent">${t}</span>`).join('')}</div></div>` : ''}
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Version</span><span class="text-zinc-300">${a.version}</span></div>
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Comments</span><span class="text-zinc-300">${commentsData.comments?.length || 0}</span></div>
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Published</span><span class="text-zinc-300">${fmtDate(a.created_at)}</span></div>
                                    <div class="flex justify-between text-sm"><span class="text-zinc-500">Updated</span><span class="text-zinc-300">${fmtDate(a.updated_at)}</span></div>
                                </div>

                                <!-- Rate this asset -->
                                ${token && !isCreator ? `
                                <div class="mt-6 pt-6 border-t border-zinc-800/50">
                                    <p class="text-xs text-zinc-500 mb-2">Rate this asset</p>
                                    <div class="flex gap-1" id="rate-stars">
                                        ${[1,2,3,4,5].map(n => `<button onclick="rateAsset('${a.id}',${n})" class="rate-star text-2xl text-zinc-700 hover:text-amber-400 transition-colors cursor-pointer" data-n="${n}" onmouseenter="hoverStar(${n})" onmouseleave="resetStars()">☆</button>`).join('')}
                                    </div>
                                </div>` : ''}

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

                                <!-- Share & Embed -->
                                <div class="mt-6 pt-6 border-t border-zinc-800/50">
                                    <p class="text-xs text-zinc-500 mb-3">Share</p>
                                    <div class="flex gap-2 mb-3">
                                        <a href="https://twitter.com/intent/tweet?text=${encodeURIComponent(a.name + ' on Renzora')}&url=${encodeURIComponent('https://renzora.com/marketplace/asset/' + a.slug)}" target="_blank" class="flex-1 px-3 py-2 rounded-lg bg-white/[0.03] border border-zinc-800/50 text-xs text-zinc-400 hover:text-white hover:border-zinc-600 transition-all flex items-center justify-center gap-1.5"><i class="ph ph-x-logo"></i>Post</a>
                                        <a href="https://www.reddit.com/submit?url=${encodeURIComponent('https://renzora.com/marketplace/asset/' + a.slug)}&title=${encodeURIComponent(a.name)}" target="_blank" class="flex-1 px-3 py-2 rounded-lg bg-white/[0.03] border border-zinc-800/50 text-xs text-zinc-400 hover:text-white hover:border-zinc-600 transition-all flex items-center justify-center gap-1.5"><i class="ph ph-reddit-logo"></i>Reddit</a>
                                        <button onclick="navigator.clipboard.writeText('https://renzora.com/marketplace/asset/${a.slug}');this.textContent='Copied!';setTimeout(()=>this.innerHTML='<i class=\\'ph ph-link\\'></i>Link',1500)" class="flex-1 px-3 py-2 rounded-lg bg-white/[0.03] border border-zinc-800/50 text-xs text-zinc-400 hover:text-white hover:border-zinc-600 transition-all flex items-center justify-center gap-1.5"><i class="ph ph-link"></i>Link</button>
                                    </div>
                                    <button onclick="toggleEmbed()" class="w-full px-3 py-2 rounded-lg bg-white/[0.03] border border-zinc-800/50 text-xs text-zinc-400 hover:text-white hover:border-zinc-600 transition-all flex items-center justify-center gap-1.5"><i class="ph ph-code"></i>Embed Preview</button>
                                    <div id="embed-code" class="hidden mt-2">
                                        <textarea readonly onclick="this.select()" class="w-full px-3 py-2 bg-zinc-900 border border-zinc-800 rounded-lg text-[10px] text-zinc-400 font-mono resize-none h-16">&lt;iframe src="https://renzora.com/embed/preview/${a.slug}" width="640" height="360" frameborder="0" allowfullscreen&gt;&lt;/iframe&gt;</textarea>
                                    </div>
                                </div>

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

                // Initialize live preview if section exists
                if (window.__previewConfig && document.getElementById('live-preview-section')) {
                    initLivePreview(window.__previewConfig);
                }
            })();

            // ── Live Preview ──
            window.previewSetMesh = function(shape) {
                if (!window.__previewWasm) return;
                window.__previewWasm.preview_set_mesh(shape);
                document.querySelectorAll('[data-mesh]').forEach(function(el) {
                    if (el.dataset.mesh === shape) {
                        el.className = el.className.replace('text-zinc-500 hover:text-zinc-300', '').replace('text-zinc-500', '');
                        el.classList.add('bg-accent/20', 'text-accent');
                    } else {
                        el.className = el.className.replace('bg-accent/20', '').replace('text-accent', '');
                        el.classList.add('text-zinc-500');
                    }
                });
            };
            window.previewSetParam = function(name, jsonValue) {
                if (window.__previewWasm) window.__previewWasm.preview_set_param(name, jsonValue);
            };
            window.previewSetColor = function(name, hex) {
                var r = parseInt(hex.slice(1,3),16)/255;
                var g = parseInt(hex.slice(3,5),16)/255;
                var b = parseInt(hex.slice(5,7),16)/255;
                if (window.__previewWasm) window.__previewWasm.preview_set_param(name, JSON.stringify({type:'Color',value:[r,g,b,1.0]}));
            };

            function buildPreviewParamUI(params, wasm) {
                var container = document.getElementById('preview-params');
                if (!container || !Object.keys(params).length) return;
                var html = '<div class="grid grid-cols-2 gap-2 p-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl">';
                for (var name in params) {
                    var p = params[name];
                    if (p.param_type === 'Float') {
                        var def = p.default_value && p.default_value.Float != null ? p.default_value.Float : 0;
                        var mn = p.min != null ? p.min : 0;
                        var mx = p.max != null ? p.max : 10;
                        html += '<div class="flex items-center gap-2"><label class="text-[11px] text-zinc-500 w-20 shrink-0 truncate" title="'+name+'">'+name+'</label><input type="range" min="'+mn+'" max="'+mx+'" step="0.01" value="'+def+'" oninput="previewSetParam(\''+name+'\', JSON.stringify({type:\'Float\',value:parseFloat(this.value)}))" class="flex-1 h-1 accent-accent bg-white/10 rounded-full appearance-none cursor-pointer" /><span class="text-[10px] text-zinc-600 w-8 text-right tabular-nums">'+def.toFixed(1)+'</span></div>';
                    } else if (p.param_type === 'Color') {
                        var c = p.default_value && p.default_value.Color ? p.default_value.Color : [1,1,1,1];
                        var hex = '#' + [c[0],c[1],c[2]].map(function(v){ return Math.round(v*255).toString(16).padStart(2,'0'); }).join('');
                        html += '<div class="flex items-center gap-2"><label class="text-[11px] text-zinc-500 w-20 shrink-0 truncate" title="'+name+'">'+name+'</label><input type="color" value="'+hex+'" oninput="previewSetColor(\''+name+'\', this.value)" class="w-6 h-6 rounded border-0 bg-transparent cursor-pointer" /></div>';
                    }
                }
                html += '</div>';
                container.innerHTML = html;
            }

            async function initLivePreview(config) {
                var section = document.getElementById('live-preview-section');
                if (!section) return;

                var observer = new IntersectionObserver(function(entries) {
                    if (entries[0].isIntersecting) {
                        observer.disconnect();
                        doInitPreview(config);
                    }
                }, { threshold: 0.1 });
                observer.observe(section);
            }

            async function doInitPreview(config) {
                try {
                    var wasm = await import('/assets/wasm/renzora_preview.js');
                    await wasm.default();
                    wasm.preview_init();
                    await new Promise(function(r){ setTimeout(r, 500); });

                    var mode = config.mode;
                    var fileUrl = config.fileUrl;
                    var category = config.category;

                    if (mode === 'shader' && fileUrl) {
                        var res = await fetch(fileUrl);
                        if (res.ok) {
                            var source = await res.text();
                            wasm.preview_load_shader(source, 'Fragment');
                            var paramsJson = wasm.preview_extract_params(source);
                            var params = JSON.parse(paramsJson || '{}');
                            buildPreviewParamUI(params, wasm);
                        }
                    } else if (mode === 'model' && fileUrl) {
                        wasm.preview_load_model(fileUrl);
                    } else if (mode === 'animation' && fileUrl) {
                        wasm.preview_load_animation(fileUrl);
                    } else if (mode === 'particle' && fileUrl) {
                        var res2 = await fetch(fileUrl);
                        if (res2.ok) { wasm.preview_load_particle(await res2.text()); }
                    } else if (mode === 'texture' && fileUrl) {
                        var texType = category.includes('hdri') ? 'hdri' : 'texture';
                        wasm.preview_load_texture(fileUrl, texType);
                    }

                    var loading = document.getElementById('preview-loading');
                    if (loading) loading.remove();
                    window.__previewWasm = wasm;
                } catch (err) {
                    console.warn('[preview] Failed to load:', err);
                    var sec = document.getElementById('live-preview-section');
                    if (sec) sec.style.display = 'none';
                }
            }

            (function __alreadyDefined(){})();

            function renderMainPreview(item) {
                if (!item || item.type === 'placeholder') {
                    return '<div class="aspect-video flex items-center justify-center"><i class="ph ph-package text-6xl text-zinc-700"></i></div>';
                }
                if (item.type === 'audio') {
                    const hasCover = item.cover;
                    const coverBg = hasCover
                        ? `<div class="absolute inset-0 bg-cover bg-center" style="background-image:url('${item.cover}')"></div><div class="absolute inset-0 bg-black/60 backdrop-blur-sm"></div>`
                        : `<div class="absolute inset-0 bg-gradient-to-b from-zinc-900 to-[#0a0a0b]"></div>`;
                    return `
                        <div class="aspect-video flex flex-col items-end justify-end relative overflow-hidden">
                            ${coverBg}
                            <audio id="audio-player" src="${item.url}" preload="metadata" crossorigin="anonymous" class="hidden"></audio>
                            <!-- Waveform canvas centered -->
                            <div class="absolute inset-0 z-[5] flex items-center justify-center pointer-events-none">
                                <canvas id="waveform-canvas" class="w-[85%] h-24 opacity-80 pointer-events-auto cursor-pointer" onclick="seekWaveform(event)"></canvas>
                            </div>
                            <div class="relative z-10 w-full px-5 pb-4">
                                <div class="flex items-center gap-3 w-full bg-black/40 backdrop-blur-md rounded-xl px-4 py-2.5">
                                    <button onclick="toggleAudioPlay()" id="audio-play-btn" class="w-10 h-10 rounded-full bg-accent hover:bg-accent-hover text-white flex items-center justify-center transition-colors shrink-0 shadow-lg shadow-accent/20">
                                        <svg id="audio-icon-play" class="w-5 h-5" viewBox="0 0 24 24" fill="currentColor"><polygon points="6,3 20,12 6,21"></polygon></svg>
                                        <svg id="audio-icon-pause" class="w-5 h-5 hidden" viewBox="0 0 24 24" fill="currentColor"><rect x="5" y="3" width="4" height="18"></rect><rect x="15" y="3" width="4" height="18"></rect></svg>
                                    </button>
                                    <div class="flex-1 min-w-0">
                                        <div class="relative w-full h-1 bg-white/10 rounded-full cursor-pointer" onclick="seekAudio(event)" id="audio-seek-bar">
                                            <div id="audio-progress" class="absolute left-0 top-0 h-full bg-accent rounded-full transition-all" style="width:0%"></div>
                                        </div>
                                        <div class="flex justify-between mt-1.5">
                                            <span id="audio-current" class="text-[11px] text-white/60 tabular-nums">0:00</span>
                                            <span id="audio-duration" class="text-[11px] text-white/60 tabular-nums">0:00</span>
                                        </div>
                                    </div>
                                    <div class="flex items-center gap-1.5 shrink-0 group/vol">
                                        <button onclick="toggleAudioVolume()" class="text-white/40 hover:text-white/80 transition-colors">
                                            <svg id="audio-vol-on" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="11,5 6,9 2,9 2,15 6,15 11,19" fill="currentColor"></polygon><path d="M19.07 4.93a10 10 0 010 14.14M15.54 8.46a5 5 0 010 7.07"></path></svg>
                                            <svg id="audio-vol-off" class="w-4 h-4 hidden" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="11,5 6,9 2,9 2,15 6,15 11,19" fill="currentColor"></polygon><line x1="23" y1="9" x2="17" y2="15"></line><line x1="17" y1="9" x2="23" y2="15"></line></svg>
                                        </button>
                                        <input type="range" min="0" max="100" value="100" id="audio-vol-slider" oninput="setAudioVolume(this.value)" class="w-16 h-1 accent-accent bg-white/10 rounded-full appearance-none cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-2.5 [&::-webkit-slider-thumb]:h-2.5 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-white" />
                                    </div>
                                </div>
                            </div>
                        </div>`;
                }
                if (item.type === 'video') {
                    // YouTube/external embed
                    if (item.url.includes('youtube.com') || item.url.includes('youtu.be')) {
                        const vid = item.url.match(/(?:v=|youtu\.be\/)([a-zA-Z0-9_-]+)/)?.[1];
                        return vid ? `<div class="aspect-video"><iframe src="https://www.youtube.com/embed/${vid}" class="w-full h-full" frameborder="0" allowfullscreen></iframe></div>` :
                            `<div class="aspect-video flex items-center justify-center text-zinc-600">Invalid video URL</div>`;
                    }
                    // Custom video player
                    return `
                        <div class="aspect-video relative bg-black group/vp" id="video-container">
                            <video id="video-player" src="${item.url}" ${item.thumb ? `poster="${item.thumb}"` : ''} preload="metadata" class="w-full h-full object-contain" onclick="toggleVideoPlay()" ondblclick="toggleVideoFullscreen()"></video>
                            <!-- Big play button overlay -->
                            <div id="video-big-play" class="absolute inset-0 flex items-center justify-center cursor-pointer" onclick="toggleVideoPlay()">
                                <div class="w-16 h-16 rounded-full bg-black/50 backdrop-blur-sm flex items-center justify-center hover:bg-accent/80 transition-colors">
                                    <svg class="w-7 h-7 text-white ml-1" viewBox="0 0 24 24" fill="currentColor"><polygon points="6,3 20,12 6,21"></polygon></svg>
                                </div>
                            </div>
                            <!-- Controls bar -->
                            <div id="video-controls" class="absolute bottom-0 inset-x-0 bg-gradient-to-t from-black/80 to-transparent pt-10 pb-3 px-4 opacity-0 group-hover/vp:opacity-100 transition-opacity duration-200">
                                <!-- Progress bar -->
                                <div class="relative w-full h-1 bg-white/10 rounded-full cursor-pointer mb-3 group/prog" onclick="seekVideo(event)" onmousemove="videoProgressHover(event)" onmouseleave="videoProgressLeave()">
                                    <div id="video-buffered" class="absolute left-0 top-0 h-full bg-white/10 rounded-full" style="width:0%"></div>
                                    <div id="video-progress" class="absolute left-0 top-0 h-full bg-accent rounded-full" style="width:0%"></div>
                                    <div id="video-hover-time" class="hidden absolute -top-7 bg-black/80 text-white text-[10px] px-1.5 py-0.5 rounded pointer-events-none tabular-nums"></div>
                                </div>
                                <div class="flex items-center gap-3">
                                    <button onclick="toggleVideoPlay()" class="text-white/80 hover:text-white transition-colors">
                                        <svg id="video-icon-play" class="w-5 h-5" viewBox="0 0 24 24" fill="currentColor"><polygon points="6,3 20,12 6,21"></polygon></svg>
                                        <svg id="video-icon-pause" class="w-5 h-5 hidden" viewBox="0 0 24 24" fill="currentColor"><rect x="5" y="3" width="4" height="18"></rect><rect x="15" y="3" width="4" height="18"></rect></svg>
                                    </button>
                                    <span id="video-time" class="text-[11px] text-white/60 tabular-nums">0:00 / 0:00</span>
                                    <div class="flex-1"></div>
                                    <div class="flex items-center gap-1.5 group/vvol">
                                        <button onclick="toggleVideoVolume()" class="text-white/60 hover:text-white transition-colors">
                                            <svg id="video-vol-on" class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="11,5 6,9 2,9 2,15 6,15 11,19" fill="currentColor"></polygon><path d="M19.07 4.93a10 10 0 010 14.14M15.54 8.46a5 5 0 010 7.07"></path></svg>
                                            <svg id="video-vol-off" class="w-4 h-4 hidden" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="11,5 6,9 2,9 2,15 6,15 11,19" fill="currentColor"></polygon><line x1="23" y1="9" x2="17" y2="15"></line><line x1="17" y1="9" x2="23" y2="15"></line></svg>
                                        </button>
                                        <input type="range" min="0" max="100" value="100" id="video-vol-slider" oninput="setVideoVolume(this.value)" class="w-16 h-1 accent-accent bg-white/10 rounded-full appearance-none cursor-pointer opacity-0 group-hover/vvol:opacity-100 transition-opacity [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-2.5 [&::-webkit-slider-thumb]:h-2.5 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-white" />
                                    </div>
                                    <button onclick="toggleVideoFullscreen()" class="text-white/60 hover:text-white transition-colors">
                                        <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M8 3H5a2 2 0 00-2 2v3m18 0V5a2 2 0 00-2-2h-3m0 18h3a2 2 0 002-2v-3M3 16v3a2 2 0 002 2h3"></path></svg>
                                    </button>
                                </div>
                            </div>
                        </div>`;
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
                // Re-init players based on type
                if (item.type === 'audio') initAudioPlayer();
                if (item.type === 'video') initVideoPlayer();
                document.querySelectorAll('.gallery-thumb').forEach(el => {
                    const i = parseInt(el.dataset.index);
                    el.className = el.className.replace(/border-accent|border-zinc-800\/50/g, '');
                    el.classList.add(i === index ? 'border-accent' : 'border-zinc-800/50');
                });
            }

            // ── Audio Player with Real-time Frequency Analyser ──
            let audioCtx = null;
            let analyser = null;
            let animFrameId = null;
            let analyserReady = false;

            function initAudioPlayer() {
                const audio = document.getElementById('audio-player');
                if (!audio) return;

                audio.addEventListener('loadedmetadata', () => {
                    const dur = document.getElementById('audio-duration');
                    if (dur && audio.duration && isFinite(audio.duration)) dur.textContent = fmtTime(audio.duration);
                });
                audio.addEventListener('durationchange', () => {
                    const dur = document.getElementById('audio-duration');
                    if (dur && audio.duration && isFinite(audio.duration)) dur.textContent = fmtTime(audio.duration);
                });
                audio.addEventListener('canplay', () => {
                    const dur = document.getElementById('audio-duration');
                    if (dur && audio.duration && isFinite(audio.duration) && dur.textContent === '0:00') dur.textContent = fmtTime(audio.duration);
                });
                audio.addEventListener('timeupdate', () => {
                    const cur = document.getElementById('audio-current');
                    const prog = document.getElementById('audio-progress');
                    if (cur) cur.textContent = fmtTime(audio.currentTime);
                    if (prog && audio.duration) prog.style.width = ((audio.currentTime / audio.duration) * 100) + '%';
                });
                audio.addEventListener('ended', () => {
                    document.getElementById('audio-icon-play')?.classList.remove('hidden');
                    document.getElementById('audio-icon-pause')?.classList.add('hidden');
                    cancelAnimationFrame(animFrameId);
                    drawIdleWaveform();
                });

                drawIdleWaveform();
            }

            function drawCanvas() {
                const canvas = document.getElementById('waveform-canvas');
                if (!canvas) return null;
                const ctx = canvas.getContext('2d');
                const dpr = window.devicePixelRatio || 1;
                const w = canvas.offsetWidth;
                const h = canvas.offsetHeight;
                canvas.width = w * dpr;
                canvas.height = h * dpr;
                ctx.scale(dpr, dpr);
                ctx.clearRect(0, 0, w, h);
                return { ctx, w, h };
            }

            function drawIdleWaveform() {
                const c = drawCanvas();
                if (!c) return;
                const { ctx, w, h } = c;
                const barW = 2, gap = 1.5, step = barW + gap;
                const bars = Math.floor(w / step);
                const mid = h / 2;
                for (let i = 0; i < bars; i++) {
                    const barH = 3 + Math.sin(i * 0.12) * 2;
                    ctx.fillStyle = 'rgba(255,255,255,0.06)';
                    ctx.beginPath();
                    ctx.roundRect(i * step, mid - barH / 2, barW, barH, 1);
                    ctx.fill();
                }
            }

            function connectAnalyser() {
                if (analyserReady) return true;
                const audio = document.getElementById('audio-player');
                if (!audio) return false;
                try {
                    audioCtx = new (window.AudioContext || window.webkitAudioContext)();
                    analyser = audioCtx.createAnalyser();
                    analyser.fftSize = 1024;
                    analyser.smoothingTimeConstant = 0.6;
                    analyser.minDecibels = -70;
                    analyser.maxDecibels = -5;
                    const src = audioCtx.createMediaElementSource(audio);
                    src.connect(analyser);
                    analyser.connect(audioCtx.destination);
                    analyserReady = true;
                    return true;
                } catch(e) {
                    console.warn('Analyser failed:', e.message);
                    return false;
                }
            }

            function drawWaveformFrame() {
                const audio = document.getElementById('audio-player');
                if (!audio || audio.paused || audio.ended) return;

                const c = drawCanvas();
                if (!c) return;
                const { ctx, w, h } = c;
                const barW = 2, gap = 1.5, step = barW + gap;
                const bars = Math.floor(w / step);
                const mid = h / 2;
                const progress = audio.duration ? audio.currentTime / audio.duration : 0;

                if (analyserReady && analyser) {
                    // Real frequency data
                    const bufLen = analyser.frequencyBinCount;
                    const freqData = new Uint8Array(bufLen);
                    analyser.getByteFrequencyData(freqData);

                    for (let i = 0; i < bars; i++) {
                        const x = i * step;
                        // Log-scale mapping: lower bars = bass, higher = treble
                        const freqIdx = Math.min(Math.floor(Math.pow(i / bars, 1.4) * bufLen), bufLen - 1);
                        const val = freqData[freqIdx] / 255;
                        const barH = Math.max(2, val * h * 0.9);

                        // Color by loudness: blue → accent → pink → red
                        let r, g, b;
                        if (val < 0.3) {
                            const t = val / 0.3;
                            r = Math.round(59 + 40 * t);
                            g = Math.round(130 - 28 * t);
                            b = Math.round(246 - 5 * t);
                        } else if (val < 0.6) {
                            const t = (val - 0.3) / 0.3;
                            r = Math.round(99 + 121 * t);
                            g = Math.round(102 - 42 * t);
                            b = Math.round(241 - 41 * t);
                        } else {
                            const t = (val - 0.6) / 0.4;
                            r = Math.round(220 + 35 * t);
                            g = Math.round(60 - 10 * t);
                            b = Math.round(200 - 120 * t);
                        }
                        ctx.fillStyle = `rgba(${r},${g},${b},${0.15 + val * 0.85})`;
                        ctx.beginPath();
                        ctx.roundRect(x, mid - barH / 2, barW, barH, 1);
                        ctx.fill();
                    }
                } else {
                    // Fallback animated waveform
                    const t = audio.currentTime;
                    const vol = audio.volume;
                    for (let i = 0; i < bars; i++) {
                        const x = i * step;
                        const wave = Math.sin(i * 0.08 + t * 3.5) * 0.3
                                   + Math.sin(i * 0.05 + t * 5.5) * 0.25
                                   + Math.sin(i * 0.15 + t * 2) * 0.25
                                   + Math.sin(i * 0.22 + t * 7) * 0.2;
                        const amp = Math.max(0, Math.min(1, (wave + 1) / 2)) * vol;
                        const barH = Math.max(2, amp * h * 0.8);
                        ctx.fillStyle = `rgba(99,102,241,${0.15 + amp * 0.85})`;
                        ctx.beginPath();
                        ctx.roundRect(x, mid - barH / 2, barW, barH, 1);
                        ctx.fill();
                    }
                }

                // Playhead
                if (audio.duration) {
                    const px = progress * w;
                    ctx.fillStyle = 'rgba(255,255,255,0.5)';
                    ctx.fillRect(px - 0.5, 0, 1, h);
                }

                animFrameId = requestAnimationFrame(drawWaveformFrame);
            }

            function seekWaveform(e) {
                const audio = document.getElementById('audio-player');
                if (!audio || !audio.duration) return;
                const rect = e.currentTarget.getBoundingClientRect();
                audio.currentTime = ((e.clientX - rect.left) / rect.width) * audio.duration;
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

                // Connect analyser on first play (needs user gesture for AudioContext)
                if (!analyserReady) connectAnalyser();
                if (audioCtx && audioCtx.state === 'suspended') audioCtx.resume();

                if (audio.paused) {
                    audio.play().then(() => {
                        document.getElementById('audio-icon-play')?.classList.add('hidden');
                        document.getElementById('audio-icon-pause')?.classList.remove('hidden');
                        const dur = document.getElementById('audio-duration');
                        if (dur && audio.duration && isFinite(audio.duration)) dur.textContent = fmtTime(audio.duration);
                        drawWaveformFrame();
                    }).catch(e => console.warn('Play failed:', e));
                } else {
                    audio.pause();
                    document.getElementById('audio-icon-play')?.classList.remove('hidden');
                    document.getElementById('audio-icon-pause')?.classList.add('hidden');
                    cancelAnimationFrame(animFrameId);
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
                audio.muted = !audio.muted;
                document.getElementById('audio-vol-on').classList.toggle('hidden', audio.muted);
                document.getElementById('audio-vol-off').classList.toggle('hidden', !audio.muted);
                const slider = document.getElementById('audio-vol-slider');
                if (slider) slider.value = audio.muted ? 0 : audio.volume * 100;
            }

            function setAudioVolume(val) {
                const audio = document.getElementById('audio-player');
                if (!audio) return;
                audio.volume = val / 100;
                audio.muted = val == 0;
                document.getElementById('audio-vol-on').classList.toggle('hidden', audio.muted);
                document.getElementById('audio-vol-off').classList.toggle('hidden', !audio.muted);
            }

            // ── Custom Video Player ──
            function initVideoPlayer() {
                const video = document.getElementById('video-player');
                if (!video) return;
                video.addEventListener('loadedmetadata', () => updateVideoTime());
                video.addEventListener('timeupdate', () => {
                    if (!video.duration) return;
                    const pct = (video.currentTime / video.duration) * 100;
                    document.getElementById('video-progress').style.width = pct + '%';
                    updateVideoTime();
                });
                video.addEventListener('progress', () => {
                    if (video.buffered.length > 0) {
                        const pct = (video.buffered.end(video.buffered.length - 1) / video.duration) * 100;
                        document.getElementById('video-buffered').style.width = pct + '%';
                    }
                });
                video.addEventListener('play', () => {
                    document.getElementById('video-big-play').classList.add('hidden');
                    document.getElementById('video-icon-play').classList.add('hidden');
                    document.getElementById('video-icon-pause').classList.remove('hidden');
                });
                video.addEventListener('pause', () => {
                    document.getElementById('video-icon-play').classList.remove('hidden');
                    document.getElementById('video-icon-pause').classList.add('hidden');
                });
                video.addEventListener('ended', () => {
                    document.getElementById('video-big-play').classList.remove('hidden');
                    document.getElementById('video-icon-play').classList.remove('hidden');
                    document.getElementById('video-icon-pause').classList.add('hidden');
                });
            }
            function updateVideoTime() {
                const video = document.getElementById('video-player');
                if (!video) return;
                document.getElementById('video-time').textContent = fmtTime(video.currentTime) + ' / ' + fmtTime(video.duration);
            }
            function toggleVideoPlay() {
                const video = document.getElementById('video-player');
                if (!video) return;
                if (video.paused) video.play(); else video.pause();
            }
            function seekVideo(e) {
                const video = document.getElementById('video-player');
                if (!video || !video.duration) return;
                const rect = e.currentTarget.getBoundingClientRect();
                const pct = (e.clientX - rect.left) / rect.width;
                video.currentTime = pct * video.duration;
            }
            function videoProgressHover(e) {
                const video = document.getElementById('video-player');
                const tooltip = document.getElementById('video-hover-time');
                if (!video || !video.duration || !tooltip) return;
                const rect = e.currentTarget.getBoundingClientRect();
                const pct = (e.clientX - rect.left) / rect.width;
                tooltip.textContent = fmtTime(pct * video.duration);
                tooltip.style.left = (pct * 100) + '%';
                tooltip.style.transform = 'translateX(-50%)';
                tooltip.classList.remove('hidden');
            }
            function videoProgressLeave() {
                const tooltip = document.getElementById('video-hover-time');
                if (tooltip) tooltip.classList.add('hidden');
            }
            function toggleVideoVolume() {
                const video = document.getElementById('video-player');
                if (!video) return;
                video.muted = !video.muted;
                document.getElementById('video-vol-on').classList.toggle('hidden', video.muted);
                document.getElementById('video-vol-off').classList.toggle('hidden', !video.muted);
                const slider = document.getElementById('video-vol-slider');
                if (slider) slider.value = video.muted ? 0 : video.volume * 100;
            }
            function setVideoVolume(val) {
                const video = document.getElementById('video-player');
                if (!video) return;
                video.volume = val / 100;
                video.muted = val == 0;
                document.getElementById('video-vol-on').classList.toggle('hidden', video.muted);
                document.getElementById('video-vol-off').classList.toggle('hidden', !video.muted);
            }
            function toggleVideoFullscreen() {
                const container = document.getElementById('video-container');
                if (!container) return;
                if (document.fullscreenElement) { document.exitFullscreen(); }
                else { container.requestFullscreen().catch(() => {}); }
            }

            // Auto-init players based on first gallery item
            setTimeout(() => {
                if (galleryItems[0]?.type === 'audio') initAudioPlayer();
                if (galleryItems[0]?.type === 'video') initVideoPlayer();
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

            function hoverStar(n) {
                document.querySelectorAll('.rate-star').forEach(s => {
                    const sn = parseInt(s.dataset.n);
                    s.textContent = sn <= n ? '★' : '☆';
                    s.classList.toggle('text-amber-400', sn <= n);
                    s.classList.toggle('text-zinc-700', sn > n);
                });
            }
            function resetStars() {
                document.querySelectorAll('.rate-star').forEach(s => {
                    s.textContent = '☆';
                    s.classList.remove('text-amber-400');
                    s.classList.add('text-zinc-700');
                });
            }
            async function rateAsset(id, rating) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }
                const res = await fetch('/api/marketplace/' + id + '/reviews', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ rating, title: '', content: '' })
                });
                if (res.ok) { window.location.reload(); }
                else { const d = await res.json().catch(() => ({})); alert(d.error || 'Failed to submit rating'); }
            }

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

            function toggleEmbed() {
                const el = document.getElementById('embed-code');
                el.classList.toggle('hidden');
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

            async function deleteAsset(assetId) {
                if (!confirm('Are you sure you want to delete this asset? This will permanently remove it and all associated files.')) return;
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                const res = await fetch('/api/marketplace/' + assetId + '/delete', {
                    method: 'DELETE',
                    headers: { 'Authorization': 'Bearer ' + token }
                });
                if (res.ok) {
                    window.location.href = '/marketplace';
                } else {
                    const data = await res.json().catch(() => ({}));
                    alert('Failed to delete: ' + (data.message || res.statusText));
                }
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

        // Particle canvas script
        <script>
            r#"
            (function() {
                const canvas = document.getElementById('asset-canvas');
                if (!canvas) return;
                const ctx = canvas.getContext('2d');
                let w, h, particles = [], mouse = { x: -1000, y: -1000 };

                function resize() {
                    w = canvas.width = window.innerWidth;
                    h = canvas.height = window.innerHeight;
                }
                resize();
                window.addEventListener('resize', resize);

                document.addEventListener('mousemove', e => { mouse.x = e.clientX; mouse.y = e.clientY; });
                document.addEventListener('mouseleave', () => { mouse.x = -1000; mouse.y = -1000; });

                const count = Math.min(100, Math.floor(w * h / 12000));
                for (let i = 0; i < count; i++) {
                    particles.push({
                        x: Math.random() * w,
                        y: Math.random() * h,
                        vx: (Math.random() - 0.5) * 0.3,
                        vy: (Math.random() - 0.5) * 0.3,
                        r: Math.random() * 1.8 + 0.5,
                    });
                }

                function draw() {
                    ctx.clearRect(0, 0, w, h);
                    for (let i = 0; i < particles.length; i++) {
                        const p = particles[i];
                        p.x += p.vx; p.y += p.vy;
                        if (p.x < 0) p.x = w; if (p.x > w) p.x = 0;
                        if (p.y < 0) p.y = h; if (p.y > h) p.y = 0;

                        const dx = p.x - mouse.x, dy = p.y - mouse.y;
                        const dist = Math.sqrt(dx * dx + dy * dy);
                        if (dist < 140) {
                            const force = (140 - dist) / 140 * 0.8;
                            p.x += dx / dist * force;
                            p.y += dy / dist * force;
                        }

                        ctx.beginPath();
                        ctx.arc(p.x, p.y, p.r, 0, Math.PI * 2);
                        ctx.fillStyle = 'rgba(99, 102, 241, 0.5)';
                        ctx.fill();

                        for (let j = i + 1; j < particles.length; j++) {
                            const p2 = particles[j];
                            const ddx = p.x - p2.x, ddy = p.y - p2.y;
                            const d = ddx * ddx + ddy * ddy;
                            if (d < 22000) {
                                ctx.beginPath();
                                ctx.moveTo(p.x, p.y);
                                ctx.lineTo(p2.x, p2.y);
                                ctx.strokeStyle = `rgba(99, 102, 241, ${(1 - d / 22000) * 0.18})`;
                                ctx.lineWidth = 0.6;
                                ctx.stroke();
                            }
                        }
                    }
                    requestAnimationFrame(draw);
                }
                draw();

                // Cleanup on navigation
                window.addEventListener('beforeunload', () => {
                    const bg = document.getElementById('asset-bg-layer');
                    if (bg) bg.remove();
                });
            })();
            "#
        </script>
    }
}
