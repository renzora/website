use leptos::prelude::*;

#[component]
pub fn ShopPage() -> impl IntoView {
    view! {
        // The page starts empty — no site chrome colors leak in
        <div id="shop-root">
            <div class="flex items-center justify-center min-h-screen">
                <div class="inline-block animate-spin w-6 h-6 border-2 border-zinc-700 border-t-white rounded-full"></div>
            </div>
        </div>
        <script>
            r##"
            (async function() {
                const username = window.location.pathname.split('/').pop();
                const res = await fetch('/api/profiles/shop/' + username);

                if (!res.ok) {
                    document.getElementById('shop-root').innerHTML = `
                        <div class="flex flex-col items-center justify-center min-h-[60vh] text-zinc-500">
                            <i class="ph ph-storefront text-5xl mb-3 opacity-40"></i>
                            <p class="text-lg font-medium">Shop not found</p>
                            <p class="text-sm mt-1">This user hasn't set up their storefront yet.</p>
                            <a href="/profile/${username}" class="text-accent mt-4 text-sm hover:underline">View profile instead</a>
                        </div>`;
                    return;
                }

                const s = await res.json();

                // Google Fonts import for custom font
                const fontLink = document.createElement('link');
                fontLink.rel = 'stylesheet';
                fontLink.href = `https://fonts.googleapis.com/css2?family=${encodeURIComponent(s.font)}:wght@300;400;500;600;700&display=swap`;
                document.head.appendChild(fontLink);

                // Build item cards
                const allItems = [
                    ...s.assets.map(a => ({ ...a, type: 'asset', href: `/marketplace/asset/${a.slug}` })),
                    ...s.games.map(g => ({ ...g, type: 'game', href: `/games/${g.slug}` })),
                ];

                const isGrid = s.layout !== 'list';
                const gridClass = isGrid
                    ? 'grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-5'
                    : 'flex flex-col gap-4';

                const cardsHtml = allItems.length ? allItems.map(item => {
                    const priceHtml = item.price_credits === 0
                        ? `<span style="color: #4ade80; font-weight: 600;">Free</span>`
                        : `<span style="color: ${s.accent_color}; font-weight: 600;">${item.price_credits} credits</span>`;
                    const typeIcon = item.type === 'game' ? 'ph-game-controller' : 'ph-package';
                    const thumb = item.thumbnail_url
                        ? `<img src="${item.thumbnail_url}" class="w-full h-full object-cover transition-transform duration-300 group-hover:scale-105" />`
                        : `<div class="w-full h-full flex items-center justify-center" style="background: ${s.accent_color}10"><i class="ph ${typeIcon}" style="font-size: 2rem; color: ${s.accent_color}40"></i></div>`;

                    if (!isGrid) {
                        // List layout
                        return `
                        <a href="${item.href}" class="shop-card group flex overflow-hidden rounded-xl transition-all duration-200 hover:translate-y-[-2px]"
                           style="background: ${s.card_bg}; border: 1px solid ${s.card_border};">
                            <div class="w-48 h-28 shrink-0 overflow-hidden">${thumb}</div>
                            <div class="flex-1 p-4 flex flex-col justify-between">
                                <div>
                                    <div class="flex items-center gap-2 mb-1">
                                        <span class="text-xs px-1.5 py-0.5 rounded" style="background: ${s.accent_color}15; color: ${s.accent_color}">${item.category}</span>
                                        <span class="text-xs" style="opacity: 0.4"><i class="ph ${typeIcon}"></i> ${item.type}</span>
                                    </div>
                                    <h3 class="font-semibold group-hover:opacity-80 transition-opacity">${item.name}</h3>
                                    <p class="text-sm mt-0.5 line-clamp-1" style="opacity: 0.5">${item.description}</p>
                                </div>
                                <div class="flex items-center gap-4 text-xs mt-2" style="opacity: 0.5">
                                    <span><i class="ph ph-eye"></i> ${item.views}</span>
                                    <span><i class="ph ph-download-simple"></i> ${item.downloads}</span>
                                    ${priceHtml}
                                </div>
                            </div>
                        </a>`;
                    }

                    // Grid layout
                    return `
                    <a href="${item.href}" class="shop-card group overflow-hidden rounded-xl transition-all duration-200 hover:translate-y-[-4px]"
                       style="background: ${s.card_bg}; border: 1px solid ${s.card_border};">
                        <div class="aspect-[16/10] overflow-hidden relative">
                            ${thumb}
                            <span class="absolute top-2 left-2 text-[10px] px-1.5 py-0.5 rounded backdrop-blur-sm"
                                  style="background: ${s.card_bg}CC; color: ${s.accent_color}">${item.category}</span>
                            <span class="absolute top-2 right-2 text-[10px] px-1.5 py-0.5 rounded backdrop-blur-sm"
                                  style="background: ${s.card_bg}CC"><i class="ph ${typeIcon}"></i></span>
                        </div>
                        <div class="p-3.5">
                            <h3 class="font-semibold text-sm truncate group-hover:opacity-80 transition-opacity">${item.name}</h3>
                            <p class="text-xs mt-1 line-clamp-2" style="opacity: 0.5; line-height: 1.5">${item.description}</p>
                            <div class="flex items-center justify-between mt-3 text-xs">
                                <div class="flex items-center gap-3" style="opacity: 0.4">
                                    <span><i class="ph ph-eye"></i> ${item.views}</span>
                                    <span><i class="ph ph-download-simple"></i> ${item.downloads}</span>
                                </div>
                                ${priceHtml}
                            </div>
                        </div>
                    </a>`;
                }).join('') : `<div class="col-span-full text-center py-16" style="opacity: 0.4"><i class="ph ph-storefront text-4xl mb-2"></i><p>No items yet</p></div>`;

                // Badges
                const badgesHtml = s.badges.length ? `<div class="flex flex-wrap gap-2 mt-3">${s.badges.map(b =>
                    `<span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-[11px] font-medium" style="border: 1px solid ${b.color}30; color: ${b.color}; background: ${b.color}10"><i class="ph ${b.icon}"></i>${b.name}</span>`
                ).join('')}</div>` : '';

                // Avatar
                const avatar = s.avatar_url
                    ? `<img src="${s.avatar_url}" class="w-full h-full object-cover rounded-full" />`
                    : `<i class="ph ph-user text-2xl" style="color: ${s.accent_color}"></i>`;

                // Stat counts
                const assetCount = s.assets.length;
                const gameCount = s.games.length;
                const totalDownloads = allItems.reduce((n, i) => n + i.downloads, 0);

                const bgStyle = s.bg_image
                    ? `background: url('${s.bg_image}') center/cover no-repeat fixed; `
                    : `background: ${s.bg_color}; `;

                const root = document.getElementById('shop-root');
                root.innerHTML = `
                    <div id="shop-page" style="${bgStyle} color: ${s.text_color}; font-family: '${s.font}', sans-serif; font-size: ${s.font_size}; cursor: ${s.cursor}; min-height: 100vh;">

                        <!-- Hero banner -->
                        <div class="relative overflow-hidden" style="background: linear-gradient(135deg, ${s.banner_color}, ${s.accent_color}40)">
                            <div class="absolute inset-0" style="background: linear-gradient(180deg, transparent 40%, ${s.bg_image ? 'rgba(0,0,0,0.7)' : s.bg_color} 100%)"></div>
                            <div class="relative max-w-6xl mx-auto px-6 py-16 flex flex-col sm:flex-row items-center gap-6">
                                <div class="w-24 h-24 rounded-full overflow-hidden flex items-center justify-center shrink-0" style="background: ${s.card_bg}; border: 3px solid ${s.accent_color}40; box-shadow: 0 0 30px ${s.accent_color}20">
                                    ${avatar}
                                </div>
                                <div class="text-center sm:text-left">
                                    <h1 class="text-3xl font-bold">${s.username}</h1>
                                    ${s.tagline ? `<p class="text-lg mt-1" style="opacity: 0.7">${s.tagline}</p>` : ''}
                                    ${s.bio ? `<p class="mt-2 max-w-lg" style="opacity: 0.5">${s.bio}</p>` : ''}
                                    ${badgesHtml}
                                    <div class="flex flex-wrap gap-6 mt-4 text-sm" style="opacity: 0.5">
                                        ${assetCount ? `<span><i class="ph ph-package"></i> ${assetCount} asset${assetCount !== 1 ? 's' : ''}</span>` : ''}
                                        ${gameCount ? `<span><i class="ph ph-game-controller"></i> ${gameCount} game${gameCount !== 1 ? 's' : ''}</span>` : ''}
                                        <span><i class="ph ph-download-simple"></i> ${totalDownloads.toLocaleString()} downloads</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <!-- Content -->
                        <div class="max-w-6xl mx-auto px-6 py-10">
                            <div class="${gridClass}">
                                ${cardsHtml}
                            </div>
                        </div>

                        <!-- Footer -->
                        <div class="text-center py-8 text-xs" style="opacity: 0.25">
                            <a href="/profile/${s.username}" class="hover:underline">${s.username}'s profile</a>
                            <span class="mx-2">·</span>
                            <a href="/marketplace" class="hover:underline">Marketplace</a>
                        </div>
                    </div>

                    <style>
                        #shop-page .shop-card { box-shadow: 0 2px 8px ${s.card_border}40; }
                        #shop-page .shop-card:hover { box-shadow: 0 8px 24px ${s.accent_color}15; border-color: ${s.accent_color}40 !important; }
                        #shop-page a { color: inherit; text-decoration: none; }
                        #shop-page .line-clamp-1 { display: -webkit-box; -webkit-line-clamp: 1; -webkit-box-orient: vertical; overflow: hidden; }
                        #shop-page .line-clamp-2 { display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; }
                        ${s.css}
                    </style>
                `;
            })();
            "##
        </script>
    }
}
