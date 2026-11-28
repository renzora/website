use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};
use renzora_common::ssr::AssetSsr;

/// JSON-escape a string for safe embedding in a JSON-LD literal.
fn json_escape(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    o.push('"');
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

#[component]
pub fn MarketplacePage() -> impl IntoView {
    // This page also answers /marketplace/asset/<slug>, where a listing opens
    // over the grid. On that path the server puts the asset in context, and the
    // head has to describe the LISTING rather than the marketplace: a crawler
    // and a shared link both land here, and neither would otherwise see
    // anything but the generic browse copy.
    //
    // This is what keeps the per-listing SEO after the standalone page went
    // away. Without it, folding the page into the overlay would quietly have
    // cost every listing its title, description and structured data.
    let asset = use_context::<AssetSsr>().filter(|a| a.found);
    let is_listing = asset.is_some();

    let asset_head = asset.clone().map(|a| {
        let title = format!("{}, Renzora Marketplace", a.name);
        let full = a.description.chars().count();
        let d: String = a.description.chars().take(155).collect();
        let desc = if full > 155 { format!("{d}…") } else { d };
        let canonical = format!("https://renzora.com/marketplace/asset/{}", a.slug);
        let price = if a.price_credits == 0 { "0".to_string() } else { a.price_credits.to_string() };
        let img = a.thumbnail_url.clone().unwrap_or_default();
        let ld = format!(
            "{{\"@context\":\"https://schema.org\",\"@type\":\"Product\",\"name\":{},\"description\":{},\"category\":{},\"image\":{},\"brand\":{{\"@type\":\"Brand\",\"name\":\"Renzora\"}},\"offers\":{{\"@type\":\"Offer\",\"price\":\"{}\",\"priceCurrency\":\"USD\",\"availability\":\"https://schema.org/InStock\",\"url\":{}}}}}",
            json_escape(&a.name), json_escape(&a.description), json_escape(&a.category), json_escape(&img), price, json_escape(&canonical)
        );
        view! {
            <Title text=title />
            <Meta name="description" content=desc />
            <Link rel="canonical" href=canonical />
            <script type="application/ld+json" inner_html=ld></script>
        }
    });

    // Crawlable copy for the listing, since the overlay itself is built by
    // script. Hidden from sight: the overlay renders the same thing properly a
    // moment later, and two copies on screen would be worse than none.
    let asset_body = asset.map(|a| {
        view! {
            <div class="sr-only">
                <h1>{a.name.clone()}</h1>
                <p>{a.description.clone()}</p>
                <p>{a.category.clone()}" · by "{a.seller.clone()}" · "{a.downloads}" downloads"</p>
            </div>
        }
    });

    view! {
        {(!is_listing).then(|| view! {
            <Title text="Renzora Marketplace, Models, Shaders & Scripts for Bevy" />
            <Meta name="description" content="Browse the Renzora marketplace: ready-made 3D models, shaders, scripts and assets for the Bevy editor. Free and paid packs you can import straight into your Bevy project." />
        })}
        {asset_head}
        {asset_body}

        <section class="min-h-[calc(100vh-3rem)] pb-20">

            // ── Hero ────────────────────────────────────────────────────────
            // A tilted wall of the marketplace's own thumbnails behind the
            // search. It starts as category-coloured tiles so it is never empty
            // or jumpy, and the first page of results swaps real art in (see
            // fillHeroMosaic). Fixed height, so nothing below it moves when it
            // fills.
            <div class="relative overflow-hidden h-[340px] sm:h-[420px]">
                <div id="mp-hero-mosaic" aria-hidden="true"
                     class="absolute -inset-x-20 -inset-y-16 grid grid-cols-4 sm:grid-cols-6 lg:grid-cols-8 auto-rows-[110px] sm:auto-rows-[150px] gap-3 -rotate-6 opacity-50 pointer-events-none"></div>
                <div class="absolute inset-0 pointer-events-none"
                     style="background: radial-gradient(ellipse at 50% 55%, rgba(8,5,14,0.92) 0, rgba(8,5,14,0.75) 45%, rgba(8,5,14,0.35) 100%), linear-gradient(180deg, rgba(8,5,14,0.55), transparent 25%, transparent 70%, #09040f)"></div>
                <div class="relative h-full max-w-3xl mx-auto px-4 sm:px-8 flex flex-col items-center justify-center gap-4 sm:gap-5 text-center">
                    <span class="px-3 py-1 rounded-full bg-purple-600 text-white text-[11px] sm:text-xs font-bold tracking-[0.08em]">"RENZORA MARKETPLACE"</span>
                    <h1 class="text-3xl sm:text-5xl lg:text-6xl font-bold tracking-tight leading-none text-zinc-50">"Plugins, shaders and assets, straight into your editor."</h1>
                    <label class="w-full flex items-center gap-3 px-4 sm:px-5 py-3 sm:py-4 rounded-2xl bg-[#160d26]/95 border border-accent/50 focus-within:border-accent transition-colors">
                        <i class="ph ph-magnifying-glass text-xl text-purple-300"></i>
                        <span class="sr-only">"Search the marketplace"</span>
                        <input type="search" id="mp-search" placeholder="Search assets" oninput="onMpSearch()"
                               class="flex-1 min-w-0 bg-transparent border-0 outline-none text-zinc-50 text-base placeholder:text-zinc-500" />
                    </label>
                </div>
            </div>

            <div class="max-w-[1376px] mx-auto px-4 sm:px-8 flex flex-col gap-6 sm:gap-8">

                // ── Category tiles ──────────────────────────────────────────
                // Filled from /api/marketplace/categories. Each starts on its
                // generated cover and takes a real thumbnail once the first
                // page shows one from that category.
                <nav id="mp-cat-strip" aria-label="Categories"
                     class="no-scrollbar grid grid-flow-col auto-cols-[140px] sm:auto-cols-[160px] gap-3 overflow-x-auto pb-1"></nav>

                // ── Tabs + tools ────────────────────────────────────────────
                <div id="mp-toolbar" class="sticky top-[60px] z-20 -mx-4 sm:-mx-8 px-4 sm:px-8 bg-[#09040f]/85 backdrop-blur-xl">
                    <div class="flex flex-wrap items-center gap-x-2 gap-y-2 shadow-[inset_0_-1px_0_rgba(255,255,255,0.07)]">
                        <div role="tablist" aria-label="Sort" class="flex items-center overflow-x-auto no-scrollbar">
                            <button type="button" role="tab" data-mp-sort="popular" onclick="setMpSort('popular')" class="mp-sort-tab">"Trending"</button>
                            <button type="button" role="tab" data-mp-sort="newest" onclick="setMpSort('newest')" class="mp-sort-tab">"New"</button>
                            <button type="button" role="tab" data-mp-sort="top_rated" onclick="setMpSort('top_rated')" class="mp-sort-tab">"Top rated"</button>
                            <button type="button" role="tab" data-mp-sort="most_viewed" onclick="setMpSort('most_viewed')" class="mp-sort-tab">"Most viewed"</button>
                        </div>
                        <span class="flex-1"></span>
                        <div class="flex items-center gap-2 py-2">
                            <button type="button" id="mp-free-toggle" onclick="toggleFreeOnly()" aria-pressed="false"
                                    class="px-3 py-2 rounded-xl text-sm font-medium border border-zinc-800/60 bg-white/[0.03] text-zinc-300 hover:border-zinc-600 transition-colors">"Free only"</button>
                            // In the bar rather than inside Filters. Which engine
                            // you are on decides what is installable at all, so it
                            // is not the same kind of question as a price range.
                            <select id="mp-engine" onchange="setEngineFilter(this.value)" title="Show only what this engine version can run" aria-label="Engine version"
                                    class="hidden sm:block px-3 py-2 bg-white/[0.03] border border-zinc-800/60 rounded-xl text-zinc-50 text-sm focus:border-accent/50 transition-all">
                                <option value="">"All engine versions"</option>
                            </select>
                            <button onclick="toggleAdvancedFilter()" id="adv-filter-btn" class="inline-flex items-center gap-1.5 px-3 py-2 bg-white/[0.03] border border-zinc-800/60 rounded-xl text-zinc-300 text-sm hover:border-zinc-600 transition-all">
                                <i class="ph ph-faders-horizontal text-base"></i><span class="hidden sm:inline">"Filters"</span>
                            </button>
                            <div class="flex items-center gap-0.5 p-0.5 bg-white/[0.02] rounded-lg border border-zinc-800/40">
                                <button onclick="setMpView('grid')" id="mp-view-grid" class="p-1.5 rounded-md text-zinc-300 bg-white/[0.06] transition-all" title="Grid" aria-label="Grid view">
                                    <i class="ph ph-grid-four text-sm"></i>
                                </button>
                                <button onclick="setMpView('list')" id="mp-view-list" class="p-1.5 rounded-md text-zinc-500 hover:text-zinc-300 transition-all" title="List" aria-label="List view">
                                    <i class="ph ph-list text-sm"></i>
                                </button>
                            </div>
                            <span id="mp-result-count" class="text-xs text-zinc-500 hidden md:block whitespace-nowrap"></span>
                            // Upload (signed-in only, revealed via JS)
                            <a id="publish-btn-hero" href="/marketplace/upload" class="hidden group inline-flex items-center gap-1.5 px-3 py-2 rounded-xl text-sm font-medium bg-teal-600 text-white hover:bg-teal-500 transition-all">
                                <i class="ph ph-plus-circle text-base group-hover:rotate-90 transition-transform duration-300"></i><span class="hidden sm:inline">"Upload"</span>
                            </a>
                        </div>
                    </div>
                    // Advanced filter panel
                    <div id="adv-filter-panel" class="hidden my-3 p-4 bg-[#120b1f] border border-zinc-800/50 rounded-xl">
                        <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
                            <div>
                                <label for="adv-max-price" class="text-[10px] text-zinc-500 uppercase tracking-wider font-medium mb-1.5 block">"Max price (credits)"</label>
                                <input type="number" id="adv-max-price" placeholder="Any" min="0" class="w-full px-2.5 py-1.5 bg-white/[0.03] border border-zinc-800/50 rounded-lg text-zinc-50 text-xs outline-none focus:border-accent/50 transition-all" />
                            </div>
                            <div>
                                <label for="adv-min-rating" class="text-[10px] text-zinc-500 uppercase tracking-wider font-medium mb-1.5 block">"Min Rating"</label>
                                <select id="adv-min-rating" class="w-full px-2.5 py-1.5 bg-white/[0.03] border border-zinc-800/50 rounded-lg text-zinc-50 text-xs focus:border-accent/50 transition-all">
                                    <option value="0">"Any"</option>
                                    <option value="3">"★★★ & up"</option>
                                    <option value="4">"★★★★ & up"</option>
                                    <option value="5">"★★★★★ only"</option>
                                </select>
                            </div>
                            <div>
                                <label for="adv-tag" class="text-[10px] text-zinc-500 uppercase tracking-wider font-medium mb-1.5 block">"Tag"</label>
                                <input type="text" id="adv-tag" placeholder="e.g. low-poly" class="w-full px-2.5 py-1.5 bg-white/[0.03] border border-zinc-800/50 rounded-lg text-zinc-50 text-xs outline-none focus:border-accent/50 transition-all" />
                            </div>
                            // The engine select sits in the bar on wider screens;
                            // a phone gets it here instead of not at all.
                            <div class="sm:hidden">
                                <label for="mp-engine-sm" class="text-[10px] text-zinc-500 uppercase tracking-wider font-medium mb-1.5 block">"Engine"</label>
                                <select id="mp-engine-sm" onchange="setEngineFilter(this.value)" class="w-full px-2.5 py-1.5 bg-white/[0.03] border border-zinc-800/50 rounded-lg text-zinc-50 text-xs">
                                    <option value="">"All engine versions"</option>
                                </select>
                            </div>
                        </div>
                        <div class="flex items-center gap-3 mt-3">
                            <button onclick="applyAdvancedFilter()" class="inline-flex items-center gap-1.5 px-4 py-1.5 rounded-lg text-xs font-medium bg-purple-600 text-white hover:bg-purple-500 transition-all">
                                <i class="ph ph-check text-sm"></i>"Apply"
                            </button>
                            <button onclick="clearAdvancedFilter()" class="inline-flex items-center gap-1.5 px-4 py-1.5 rounded-lg text-xs font-medium text-zinc-400 hover:text-zinc-200 bg-white/[0.03] border border-zinc-800/50 hover:border-zinc-600 transition-all">
                                "Clear"
                            </button>
                        </div>
                    </div>
                </div>

                // ── The wall ────────────────────────────────────────────────
                <div id="mp-grid" class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-3 sm:gap-4">
                    <div class="col-span-full text-center py-16">
                        <div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                    </div>
                </div>

                // Pagination
                <div id="mp-pagination" class="hidden flex items-center justify-center gap-3">
                    <button onclick="goPage(currentPage - 1)" id="mp-prev" aria-label="Previous page" class="inline-flex items-center justify-center w-8 h-8 rounded-lg bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:text-zinc-200 hover:border-zinc-600 transition-all disabled:opacity-30 disabled:pointer-events-none">
                        <i class="ph ph-caret-left text-sm"></i>
                    </button>
                    <div class="flex items-center gap-2 text-xs text-zinc-500">
                        "Page"
                        <input type="number" id="mp-page-input" min="1" value="1" onchange="goPage(parseInt(this.value)||1)" aria-label="Page" class="w-12 px-2 py-1 bg-white/[0.03] border border-zinc-800/50 rounded-lg text-zinc-50 text-xs text-center outline-none focus:border-accent/50 transition-all" />
                        "of "<span id="mp-total-pages">"1"</span>
                    </div>
                    <button onclick="goPage(currentPage + 1)" id="mp-next" aria-label="Next page" class="inline-flex items-center justify-center w-8 h-8 rounded-lg bg-white/[0.03] border border-zinc-800/50 text-zinc-400 hover:text-zinc-200 hover:border-zinc-600 transition-all disabled:opacity-30 disabled:pointer-events-none">
                        <i class="ph ph-caret-right text-sm"></i>
                    </button>
                </div>

                // ── Collection banner ───────────────────────────────────────
                // Shaders, drawn from whatever shader art the current page
                // already holds. Hidden until there are at least two to show.
                <button type="button" id="mp-collection" onclick="setCategory('shaders'); window.scrollTo({top: 0, behavior: 'smooth'})"
                        class="hidden relative w-full min-h-[200px] rounded-3xl overflow-hidden text-left items-center"
                        style="background: radial-gradient(circle at 85% 50%, rgba(34,211,238,0.33) 0, transparent 40%), linear-gradient(90deg, #1e1b4b, #4c1d95)">
                    <span id="mp-collection-art" aria-hidden="true" class="hidden md:grid absolute right-8 top-1/2 -translate-y-1/2 -rotate-6 grid-cols-4 gap-2.5"></span>
                    <span class="relative p-6 sm:p-8 max-w-lg flex flex-col gap-2">
                        <span class="text-xs font-bold tracking-[0.08em] text-cyan-300">"COLLECTION"</span>
                        <span class="text-2xl sm:text-3xl font-bold leading-tight text-white">"Post-processing shaders"</span>
                        <span class="text-sm text-zinc-200">"Camera effects you can drop on a scene, each with a live preview on its listing."</span>
                    </span>
                </button>
            </div>
        </section>

        // ── Quick look ──────────────────────────────────────────────────────
        // A listing opens over the grid instead of replacing it, so closing it
        // returns to the same scroll position, filters and page rather than a
        // reload of all three. Hidden until something opens it, and empty until
        // then too: the renderer fills `#mp-overlay-root`.
        <div id="mp-overlay" class="hidden fixed inset-0 z-[70]" role="dialog" aria-modal="true" aria-label="Asset details">
            <div class="absolute inset-0 bg-black/70 backdrop-blur-sm"></div>
            // The scroll container covers the whole viewport, scrim included, so
            // a click on "the outside" lands here and never on the scrim. That
            // is why closing is handled on this element and decided by what the
            // click was NOT inside, rather than by a listener on the backdrop.
            <div class="absolute inset-0 overflow-y-auto mp-scroll" id="mp-overlay-scroll">
                <div class="min-h-full px-4 py-6 sm:px-6 sm:py-10">
                    <div id="mp-overlay-panel" class="relative max-w-5xl mx-auto rounded-2xl border border-zinc-800/60 bg-surface shadow-2xl shadow-black/60">
                        <div class="px-4 sm:px-6 py-6" id="mp-overlay-root"></div>
                    </div>
                </div>
            </div>
        </div>

        // The same renderer the standalone listing page uses, so the overlay
        // cannot drift from it. stats-chart first, because the renderer mounts
        // the activity chart as soon as it has drawn.
        <script src=crate::assets::STATS_CHART_JS.as_str()></script>
        <script src=crate::assets::ASSET_DETAIL_JS.as_str()></script>

        <script>
            r##"
            // ── Marketplace logic ──
            let mpView = 'grid';
            let lastAssets = [];
            const mpParams = new URLSearchParams(window.location.search);
            const MP_SORTS = ['popular', 'newest', 'top_rated', 'most_viewed'];
            let currentCategory = mpParams.get('category') || 'all';
            let currentPage = 1;
            // Trending first: on a catalogue this size "newest" is mostly
            // whatever was uploaded in the last batch, which says nothing about
            // what is worth a look.
            let currentSort = MP_SORTS.includes(mpParams.get('sort')) ? mpParams.get('sort') : 'popular';
            let freeOnly = mpParams.get('free') === '1';
            let currentMinRating = 0;
            let advMaxPrice = null;
            let advTag = '';
            let advEngine = '';
            let catInfo = {};      // slug -> { name, icon }
            const catArt = {};     // slug -> a thumbnail seen in that category
            let heroFilled = false;
            let loadSeq = 0;

            function mpEsc(s) {
                return String(s ?? '').replace(/[&<>"']/g, c =>
                    ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
            }

            // ── Generated covers ──
            // A listing with no thumbnail gets a cover in its category's colour
            // with a monogram, instead of an empty grey box. Each colour carries
            // white text at 4.5:1, since the category chip sits on it too.
            const CAT_COLORS = {
                'plugins': '#7c3aed', 'plugin': '#7c3aed', 'shaders': '#0e7490', 'materials': '#a16207',
                'textures': '#4d7c0f', 'hdris': '#0369a1', '3d-models': '#c2410c', 'animations': '#be185d',
                '2d-art': '#9d174d', 'particles': '#b45309', 'sfx': '#be123c', 'music': '#9f1239',
                'scripts': '#15803d', 'script': '#15803d', 'blueprints': '#1d4ed8', 'projects': '#0f766e',
                'themes': '#6d28d9', 'fonts': '#854d0e', 'prefabs': '#c2410c', 'starter-templates': '#0f766e',
            };
            const catColor = slug => CAT_COLORS[slug] || '#6d28d9';
            const catName = slug => (catInfo[slug] && catInfo[slug].name) || slug;
            function coverStyle(slug) {
                return 'background: radial-gradient(circle at 1px 1px, rgba(255,255,255,0.16) 1px, transparent 0) 0 0 / 14px 14px, linear-gradient(135deg, ' + catColor(slug) + ', #0c0715)';
            }
            function monogram(name) {
                return String(name || '').split(/[\s_\-]+/).filter(Boolean).map(w => w[0]).join('').slice(0, 2).toUpperCase();
            }
            // The picture for a card: its thumbnail, else its generated cover.
            function artHtml(a, monoCls) {
                if (a.thumbnail_url) {
                    return `<img src="${mpEsc(a.thumbnail_url)}" alt="" loading="lazy" decoding="async" class="absolute inset-0 w-full h-full object-cover transition-transform duration-500 ease-out group-hover:scale-105" />`;
                }
                return `<span class="absolute inset-0 flex items-center justify-center transition-transform duration-500 ease-out group-hover:scale-105" style="${coverStyle(a.category)}"><span class="font-bold text-white/90 ${monoCls}">${mpEsc(monogram(a.name))}</span></span>`;
            }

            function setMpView(v) {
                mpView = v;
                document.getElementById('mp-view-grid').className = 'p-1.5 rounded-md transition-all ' + (v === 'grid' ? 'text-zinc-300 bg-white/[0.06]' : 'text-zinc-500 hover:text-zinc-300');
                document.getElementById('mp-view-list').className = 'p-1.5 rounded-md transition-all ' + (v === 'list' ? 'text-zinc-300 bg-white/[0.06]' : 'text-zinc-500 hover:text-zinc-300');
                if (lastAssets.length) renderMpAssets(lastAssets);
            }

            // Sort and the free toggle live in the address, so a reload or a
            // shared link keeps them. Replace rather than push: they refine the
            // page you are on, and Back should not walk through every one.
            function syncMpUrl() {
                if (/^\/marketplace\/asset\//.test(window.location.pathname)) return;
                const url = new URL(window.location);
                if (currentSort === 'popular') url.searchParams.delete('sort'); else url.searchParams.set('sort', currentSort);
                if (freeOnly) url.searchParams.set('free', '1'); else url.searchParams.delete('free');
                history.replaceState(history.state, '', url);
            }

            function syncSortTabs() {
                document.querySelectorAll('[data-mp-sort]').forEach(b =>
                    b.setAttribute('aria-selected', b.dataset.mpSort === currentSort ? 'true' : 'false'));
            }

            function syncFreeToggle() {
                const b = document.getElementById('mp-free-toggle');
                if (!b) return;
                b.setAttribute('aria-pressed', freeOnly ? 'true' : 'false');
                b.className = 'px-3 py-2 rounded-xl text-sm font-medium border transition-colors ' + (freeOnly
                    ? 'bg-purple-600 border-purple-600 text-white'
                    : 'border-zinc-800/60 bg-white/[0.03] text-zinc-300 hover:border-zinc-600');
            }

            function setMpSort(s) {
                if (!MP_SORTS.includes(s)) return;
                currentSort = s;
                currentPage = 1;
                syncSortTabs();
                syncMpUrl();
                loadAssets();
            }

            function toggleFreeOnly() {
                freeOnly = !freeOnly;
                currentPage = 1;
                syncFreeToggle();
                syncMpUrl();
                loadAssets();
            }

            let searchTimer = null;
            function onMpSearch() {
                clearTimeout(searchTimer);
                searchTimer = setTimeout(() => { currentPage = 1; loadAssets(); }, 180);
            }

            // ── Hero mosaic ──
            // Category-coloured tiles at first, so the hero never flashes empty,
            // then the first page's thumbnails once there are enough of them.
            const HERO_TILES = 24;
            function renderHeroPlaceholders() {
                const el = document.getElementById('mp-hero-mosaic');
                if (!el) return;
                const slugs = Object.keys(CAT_COLORS);
                el.innerHTML = Array.from({ length: HERO_TILES }, (_, i) =>
                    `<span class="rounded-2xl" style="${coverStyle(slugs[(i * 5) % slugs.length])}"></span>`).join('');
            }
            function fillHeroMosaic(assets) {
                if (heroFilled) return;
                const el = document.getElementById('mp-hero-mosaic');
                const thumbs = [...new Set(assets.map(a => a.thumbnail_url).filter(Boolean))];
                if (!el || thumbs.length < 6) return;
                heroFilled = true;
                el.innerHTML = Array.from({ length: HERO_TILES }, (_, i) =>
                    `<img src="${mpEsc(thumbs[i % thumbs.length])}" alt="" decoding="async" class="w-full h-full object-cover rounded-2xl" />`).join('');
            }

            // ── Category tiles ──
            function catTileClass(active) {
                return 'mp-cat-tile group relative h-[84px] sm:h-[92px] rounded-2xl overflow-hidden text-left ring-2 ring-inset transition-shadow '
                    + (active ? 'ring-accent' : 'ring-transparent');
            }
            function renderCatStrip(categories) {
                const el = document.getElementById('mp-cat-strip');
                if (!el) return;
                el.innerHTML = categories.map(c => {
                    const bg = c.slug === 'all'
                        ? 'background: conic-gradient(from 210deg, #7c3aed, #0e7490, #c2410c, #be185d, #7c3aed)'
                        : coverStyle(c.slug);
                    return `<button type="button" onclick="setCategory('${mpEsc(c.slug)}')" data-cat="${mpEsc(c.slug)}" class="${catTileClass(c.slug === currentCategory)}">
                        <span class="mp-cat-art absolute inset-0 bg-cover bg-center transition-transform duration-300 group-hover:scale-105" style="${bg}"></span>
                        <i class="ph ${mpEsc(c.icon)} absolute top-2.5 right-3 text-lg text-white/80"></i>
                        <span class="absolute inset-x-0 bottom-0 px-3 pt-7 pb-2.5 bg-gradient-to-b from-transparent to-black/85 text-white text-sm font-semibold truncate">${mpEsc(c.name)}</span>
                    </button>`;
                }).join('');
                for (const slug in catArt) paintCatArt(slug);
            }
            function paintCatArt(slug) {
                const art = document.querySelector(`[data-cat="${CSS.escape(slug)}"] .mp-cat-art`);
                if (art) art.style.background = `center / cover no-repeat url("${catArt[slug].replace(/"/g, '%22')}")`;
            }
            // A category tile takes the first real thumbnail seen in it, and
            // keeps it: later pages and filters only fill tiles still bare.
            function fillCategoryArt(assets) {
                for (const a of assets) {
                    if (!a.thumbnail_url || catArt[a.category]) continue;
                    catArt[a.category] = a.thumbnail_url;
                    paintCatArt(a.category);
                }
            }

            // ── Collection banner ──
            function renderCollection(assets) {
                const el = document.getElementById('mp-collection');
                const art = document.getElementById('mp-collection-art');
                if (!el || !art) return;
                const thumbs = [...new Set(assets.filter(a => a.category === 'shaders' && a.thumbnail_url).map(a => a.thumbnail_url))].slice(0, 4);
                const show = currentCategory === 'all' && thumbs.length >= 2;
                el.classList.toggle('hidden', !show);
                el.classList.toggle('flex', show);
                if (show) art.innerHTML = thumbs.map(t => `<img src="${mpEsc(t)}" alt="" loading="lazy" class="w-24 h-24 rounded-2xl object-cover" />`).join('');
            }

            (async function() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                const pubBtn = document.getElementById('publish-btn-hero');
                if (pubBtn && token) { pubBtn.classList.remove('hidden'); }

                syncSortTabs();
                syncFreeToggle();
                renderHeroPlaceholders();

                const [engRes, catRes] = await Promise.all([
                    fetch('/api/marketplace/engine-versions').catch(() => null),
                    fetch('/api/marketplace/categories').catch(() => null),
                ]);

                // The engine filter is additive, so a failure here leaves the
                // select on "All" and the grid unfiltered rather than breaking
                // browsing.
                if (engRes && engRes.ok) {
                    const engines = await engRes.json();
                    const opts = '<option value="">All engine versions</option>' + engines.map(e =>
                        `<option value="${mpEsc(e.version)}"${e.version === advEngine ? ' selected' : ''}>${mpEsc(e.version)}</option>`).join('');
                    ['mp-engine', 'mp-engine-sm'].forEach(id => { const s = document.getElementById(id); if (s) s.innerHTML = opts; });
                }

                const dbCats = (catRes && catRes.ok) ? await catRes.json() : [];
                // Every tile gets an icon, including categories the database has
                // none for.
                const categories = [{slug: 'all', name: 'All', icon: 'ph-squares-four'},
                    ...dbCats.map(c => ({slug: c.slug, name: c.name, icon: c.icon || 'ph-folder-simple'}))];
                categories.forEach(c => { catInfo[c.slug] = c; });
                renderCatStrip(categories);

                loadAssets();
            })();

            function setCategory(slug) {
                currentCategory = slug;
                currentPage = 1;
                const url = new URL(window.location);
                if (slug === 'all') url.searchParams.delete('category');
                else url.searchParams.set('category', slug);
                history.pushState({}, '', url);
                document.querySelectorAll('.mp-cat-tile').forEach(el => {
                    el.className = catTileClass(el.dataset.cat === slug);
                });
                loadAssets();
            }

            // Its own setter rather than part of Apply: it takes effect on
            // change, like the sort tabs do. Two selects (bar and phone panel)
            // drive it, so each mirrors the other.
            function setEngineFilter(v) {
                advEngine = v || '';
                ['mp-engine', 'mp-engine-sm'].forEach(id => { const s = document.getElementById(id); if (s) s.value = advEngine; });
                currentPage = 1;
                loadAssets();
            }

            function toggleAdvancedFilter() {
                const panel = document.getElementById('adv-filter-panel');
                const btn = document.getElementById('adv-filter-btn');
                panel.classList.toggle('hidden');
                const open = !panel.classList.contains('hidden');
                btn.classList.toggle('border-accent/50', open);
                btn.classList.toggle('text-purple-300', open);
            }

            function applyAdvancedFilter() {
                const maxPrice = document.getElementById('adv-max-price')?.value;
                const minRating = document.getElementById('adv-min-rating')?.value;
                const tag = document.getElementById('adv-tag')?.value?.trim();
                advMaxPrice = maxPrice ? parseInt(maxPrice) : null;
                advTag = tag || '';
                currentMinRating = parseInt(minRating) || 0;
                currentPage = 1;
                loadAssets();
            }

            function clearAdvancedFilter() {
                document.getElementById('adv-max-price').value = '';
                document.getElementById('adv-min-rating').value = '0';
                document.getElementById('adv-tag').value = '';
                advMaxPrice = null;
                advTag = '';
                currentMinRating = 0;
                currentPage = 1;
                loadAssets();
            }

            async function loadAssets() {
                const seq = ++loadSeq;
                const q = document.getElementById('mp-search')?.value || '';
                const cat = currentCategory === 'all' ? '' : currentCategory;
                let url = `/api/marketplace?page=${currentPage}&sort=${currentSort}`;
                if (q) url += '&q=' + encodeURIComponent(q);
                if (cat) url += '&category=' + encodeURIComponent(cat);
                if (freeOnly) url += '&free=true';
                if (currentMinRating > 0) url += '&min_rating=' + currentMinRating;
                if (advMaxPrice !== null) url += '&max_price=' + advMaxPrice;
                if (advTag) url += '&tag=' + encodeURIComponent(advTag);
                // Filters the grid AND relabels each card with the version that
                // engine would actually get, which is not always the newest one
                // published.
                if (advEngine) url += '&engine=' + encodeURIComponent(advEngine);

                const res = await fetch(url);
                const data = await res.json();
                // A slower, older request (typing fast in search) must not paint
                // over the newer one's results.
                if (seq !== loadSeq) return;
                const el = document.getElementById('mp-grid');

                const countEl = document.getElementById('mp-result-count');
                if (countEl) countEl.textContent = (data.total || 0).toLocaleString() + ' asset' + ((data.total || 0) !== 1 ? 's' : '');

                if (!data.assets?.length) {
                    el.className = 'grid grid-cols-1';
                    el.innerHTML = `
                        <div class="text-center py-20">
                            <div class="w-16 h-16 bg-zinc-800/50 rounded-2xl flex items-center justify-center mx-auto mb-4">
                                <i class="ph ph-storefront text-3xl text-zinc-600"></i>
                            </div>
                            <p class="text-zinc-500 text-sm">${q ? 'No results. Try a different search.' : 'No assets found with these filters.'}</p>
                        </div>`;
                    document.getElementById('mp-pagination').classList.add('hidden');
                    renderCollection([]);
                    return;
                }

                lastAssets = data.assets;
                renderMpAssets(data.assets);
                fillHeroMosaic(data.assets);
                fillCategoryArt(data.assets);
                renderCollection(data.assets);

                // Pagination
                const totalPages = Math.ceil(data.total / data.per_page);
                const pagEl = document.getElementById('mp-pagination');
                if (totalPages <= 1) {
                    pagEl.classList.add('hidden');
                } else {
                    pagEl.classList.remove('hidden');
                    document.getElementById('mp-page-input').value = currentPage;
                    document.getElementById('mp-page-input').max = totalPages;
                    document.getElementById('mp-total-pages').textContent = totalPages;
                    document.getElementById('mp-prev').disabled = currentPage <= 1;
                    document.getElementById('mp-next').disabled = currentPage >= totalPages;
                }
            }

            // Stars only once somebody has rated it. Where there is no rating,
            // downloads are the honest signal, and unlike empty stars they
            // actually differ between listings.
            function signalText(a) {
                return a.rating_count > 0
                    ? `★ ${a.rating_avg.toFixed(1)} (${a.rating_count})`
                    : `${a.downloads.toLocaleString()} download${a.downloads === 1 ? '' : 's'}`;
            }

            function renderMpAssets(assets) {
                const el = document.getElementById('mp-grid');
                if (mpView === 'list') {
                    el.className = 'divide-y divide-zinc-800/30 rounded-2xl border border-zinc-800/40 bg-white/[0.01] overflow-hidden';
                    el.innerHTML = assets.map((a, i) => {
                        const priceLabel = a.price_credits === 0 ? 'Free' : a.price_credits.toLocaleString() + ' credits';
                        return `<a href="/marketplace/asset/${mpEsc(a.slug)}" class="flex items-center gap-4 px-4 sm:px-5 py-3 hover:bg-white/[0.02] transition-all group" style="animation: mpFadeIn 0.2s ease both; animation-delay:${i * 10}ms">
                            <div class="relative w-20 h-14 rounded-lg overflow-hidden shrink-0 bg-zinc-900">${artHtml(a, 'text-lg')}</div>
                            <div class="flex-1 min-w-0">
                                <div class="text-sm font-semibold text-zinc-100 group-hover:text-purple-300 transition-colors truncate">${mpEsc(a.name)}</div>
                                <div class="flex items-center gap-2 mt-1 text-xs text-zinc-400 min-w-0">
                                    <span class="px-2 py-0.5 rounded-full text-[10.5px] font-semibold text-white shrink-0" style="background:${catColor(a.category)}">${mpEsc(catName(a.category))}</span>
                                    <span class="truncate">${mpEsc(a.creator_name)} · ${signalText(a)}</span>
                                </div>
                            </div>
                            <span class="text-sm font-semibold ${a.price_credits === 0 ? 'text-emerald-400' : 'text-amber-300'} min-w-[60px] text-right shrink-0">${priceLabel}</span>
                        </a>`;
                    }).join('');
                    return;
                }

                // The wall: square tiles, the art is the card. The name sits on
                // a scrim at the bottom; on hover the price makes way for Open.
                el.className = 'grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-3 sm:gap-4';
                el.innerHTML = assets.map((a, i) => {
                    const free = a.price_credits === 0;
                    return `
                    <a href="/marketplace/asset/${mpEsc(a.slug)}" class="group relative block aspect-square rounded-2xl overflow-hidden bg-zinc-900" style="animation: mpFadeIn 0.25s ease both; animation-delay: ${Math.min(i, 40) * 12}ms">
                        ${artHtml(a, 'text-4xl sm:text-5xl')}
                        <span class="absolute top-2.5 left-2.5 max-w-[70%] truncate px-2.5 py-1 rounded-full text-[11px] font-semibold text-white" style="background:${catColor(a.category)}">${mpEsc(catName(a.category))}</span>
                        ${free ? '' : `<span class="absolute top-2.5 right-2.5 px-2.5 py-1 rounded-full text-[11px] font-bold bg-amber-400 text-zinc-950">${a.price_credits.toLocaleString()} cr</span>`}
                        <span class="absolute inset-x-0 bottom-0 px-3 sm:px-3.5 pt-10 pb-3 bg-gradient-to-b from-transparent to-[#08050e]/95">
                            <span class="block text-sm sm:text-[15px] font-bold text-white truncate">${mpEsc(a.name)}</span>
                            <span class="flex items-center gap-2 mt-1 text-xs text-zinc-300">
                                <span class="truncate">${signalText(a)}</span>
                                <span class="flex-1"></span>
                                ${free ? '<span class="text-emerald-400 font-semibold sm:group-hover:hidden">Free</span>' : ''}
                                <span class="hidden sm:group-hover:inline-flex px-2.5 py-1 rounded-lg bg-purple-600 text-white font-semibold">Open</span>
                            </span>
                        </span>
                    </a>`;
                }).join('');
            }

            function goPage(p) {
                const totalPages = parseInt(document.getElementById('mp-total-pages')?.textContent) || 1;
                if (p < 1) p = 1;
                if (p > totalPages) p = totalPages;
                currentPage = p;
                loadAssets();
                window.scrollTo({top: 0, behavior: 'smooth'});
            }

            // ── Quick look ──────────────────────────────────────────────────
            //
            // Clicking a card opens the listing over the grid and pushes its
            // URL, so the address bar names what is on screen, the link is
            // shareable, and Back closes the overlay onto the grid exactly as it
            // was rather than reloading it.
            //
            // A middle click, a modifier click or a right click is left alone:
            // those mean "open a copy elsewhere", and an overlay cannot honour
            // that. Hence the guard below rather than a blanket preventDefault.

            // Responses warmed on hover, keyed by slug.
            //
            // A Response body can only be read once, so `take` hands the entry
            // over and drops it. A miss returns undefined and the renderer
            // fetches normally, which is also what happens when the pointer
            // moved too fast for the delay to fire.
            window.assetDetailPrefetch = (function () {
                const warm = new Map();
                let timer = null;
                const LIMIT = 30;

                // The slug, but only for a link to the LISTING itself.
                //
                // /marketplace/asset/<slug> is the listing; /files, /edit and
                // /releases/new underneath it are their own pages. Matching on
                // the prefix alone swallowed those too and asked the API for a
                // listing called "vibrance/files", which answered 404 and left
                // the overlay reading "Asset not found". Anything with a further
                // path segment navigates normally.
                function slugFrom(a) {
                    const href = (a && a.getAttribute('href')) || '';
                    const parts = href.split('/marketplace/asset/');
                    if (parts.length !== 2) return null;
                    const rest = parts[1].split(/[?#]/)[0];
                    return (rest && !rest.includes('/')) ? rest : null;
                }

                function warmUp(slug) {
                    if (!slug || warm.has(slug)) return;
                    // Cheap insurance against a long browse warming hundreds of
                    // listings: drop the oldest rather than grow without bound.
                    if (warm.size >= LIMIT) warm.delete(warm.keys().next().value);
                    const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                    const headers = token ? { 'Authorization': 'Bearer ' + token } : {};
                    warm.set(slug, fetch('/api/marketplace/detail/' + slug, { headers })
                        .catch(() => null));
                }

                return {
                    slugFrom,
                    // 120ms: long enough that sweeping the pointer across a row
                    // of cards on the way somewhere else costs nothing, short
                    // enough to be well ahead of a click.
                    hover(a) {
                        clearTimeout(timer);
                        const slug = slugFrom(a);
                        if (slug) timer = setTimeout(() => warmUp(slug), 120);
                    },
                    cancel() { clearTimeout(timer); },
                    take(slug) {
                        const p = warm.get(slug);
                        if (!p) return null;
                        warm.delete(slug);
                        // A failed prefetch must not become a failed render.
                        return p.then(r => (r && r.ok) ? r : fetch('/api/marketplace/detail/' + slug));
                    },
                };
            })();

            function assetLinkFrom(target) {
                const a = target.closest && target.closest('a[href^="/marketplace/asset/"]');
                return a || null;
            }

            document.addEventListener('mouseover', e => {
                const a = assetLinkFrom(e.target);
                if (a) window.assetDetailPrefetch.hover(a);
            }, { passive: true });
            document.addEventListener('mouseout', e => {
                if (assetLinkFrom(e.target)) window.assetDetailPrefetch.cancel();
            }, { passive: true });

            document.addEventListener('click', e => {
                // Anything that means "open elsewhere" stays a normal link.
                if (e.defaultPrevented || e.button !== 0 || e.metaKey || e.ctrlKey ||
                    e.shiftKey || e.altKey) return;
                const a = assetLinkFrom(e.target);
                if (!a || a.target === '_blank') return;
                const slug = window.assetDetailPrefetch.slugFrom(a);
                if (!slug) return;
                e.preventDefault();
                openQuickLook(slug, true);
            });

            let quickLookOpen = false;
            // Whether THIS overlay was opened by a click, which is what decides
            // how closing it should move history. See closeQuickLook.
            let quickLookPushed = false;

            function openQuickLook(slug, push) {
                const overlay = document.getElementById('mp-overlay');
                const root = document.getElementById('mp-overlay-root');
                if (!overlay || !root || !window.renderAssetDetail) {
                    // No renderer means no overlay, and this path has no page of
                    // its own to fall back to any more. Reload rather than
                    // swallow the click: the server will serve the grid and the
                    // listing opens over it.
                    window.location.href = '/marketplace/asset/' + slug;
                    return;
                }
                quickLookPushed = !!push;
                if (push) history.pushState({ quickLook: slug }, '', '/marketplace/asset/' + slug);

                root.innerHTML = '<div class="text-center py-20"><div class="inline-block animate-spin w-6 h-6 border-2 border-zinc-700 border-t-accent rounded-full"></div></div>';
                overlay.classList.remove('hidden');
                // The grid must not scroll behind the overlay, and must be
                // exactly where it was when this closes.
                document.body.style.overflow = 'hidden';
                document.getElementById('mp-overlay-scroll').scrollTop = 0;
                quickLookOpen = true;
                window.renderAssetDetail(slug, root);
            }

            function closeQuickLook(pop) {
                const overlay = document.getElementById('mp-overlay');
                if (!overlay || !quickLookOpen) return;
                overlay.classList.add('hidden');
                document.getElementById('mp-overlay-root').innerHTML = '';
                document.body.style.overflow = '';
                quickLookOpen = false;
                if (pop) return; // history already moved; nothing to undo.

                // Back only when there is something of ours to go back TO.
                // Opening from a click pushed an entry, so Back lands on the
                // grid. Landing straight on a listing URL pushed nothing, and
                // Back would leave the site entirely: the previous entry belongs
                // to wherever the link came from. Replace instead, so closing
                // reliably ends up on the marketplace.
                if (quickLookPushed) history.back();
                else history.replaceState({}, '', '/marketplace' + window.location.search);
            }

            // Click anywhere off the panel to close.
            //
            // Both the press and the release have to land outside it. Selecting
            // text in the listing and releasing past its edge is a drag, not a
            // dismissal, and closing on the release alone would throw the page
            // away mid-selection.
            (function () {
                const scroll = document.getElementById('mp-overlay-scroll');
                if (!scroll) return;
                let pressedOutside = false;
                const outside = t => !(t.closest && t.closest('#mp-overlay-panel'));
                scroll.addEventListener('mousedown', e => { pressedOutside = outside(e.target); });
                scroll.addEventListener('click', e => {
                    if (pressedOutside && outside(e.target)) closeQuickLook();
                    pressedOutside = false;
                });
            })();

            document.addEventListener('keydown', e => {
                if (e.key === 'Escape' && quickLookOpen) closeQuickLook();
            });

            // Back and forward. The URL is the state: an asset path means open,
            // anything else means closed.
            window.addEventListener('popstate', () => {
                const m = window.location.pathname.match(/^\/marketplace\/asset\/([^/?#]+)/);
                if (m) openQuickLook(m[1], false);
                else closeQuickLook(true);
            });

            // Landing straight on a listing URL. This page answers that path
            // now, so the grid loads behind and the listing opens over it,
            // exactly as it would have if you had clicked through. Without this
            // the URL would show the marketplace and nothing else, which is the
            // one thing it must not do.
            //
            // `false` for push: this URL is already the current entry, and
            // pushing it again would make Back a no-op.
            (function () {
                const m = window.location.pathname.match(/^\/marketplace\/asset\/([^/?#]+)/);
                if (m) openQuickLook(m[1], false);
            })();

            // Changing the release from inside the overlay.
            //
            // The renderer would otherwise navigate, which throws the grid away
            // for what is a change of contents within the thing already open.
            // replaceState rather than push: picking a version is refining what
            // you are looking at, not a new place, and pushing would make Back
            // walk every version you tried before reaching the grid.
            window.quickLookRerender = function (url) {
                if (!quickLookOpen) return false;
                history.replaceState({ quickLook: true }, '', url.toString());
                const root = document.getElementById('mp-overlay-root');
                const slug = window.location.pathname.split('/').pop();
                if (!root || !slug) return false;
                window.renderAssetDetail(slug, root);
                return true;
            };
            "##
        </script>

        <style>
            r#"
            .mp-scroll::-webkit-scrollbar { width: 6px; }
            .mp-scroll::-webkit-scrollbar-track { background: transparent; }
            .mp-scroll::-webkit-scrollbar-thumb { background: #27272a; border-radius: 3px; }
            .mp-scroll::-webkit-scrollbar-thumb:hover { background: #3f3f46; }
            .mp-scroll { scrollbar-width: thin; scrollbar-color: #27272a transparent; }
            .no-scrollbar::-webkit-scrollbar { display: none; }
            .no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }

            /* Sort tabs: an underline on the active one, nothing on the rest. */
            .mp-sort-tab {
                padding: 12px 14px; font-size: 14px; font-weight: 600; white-space: nowrap;
                color: #a1a1aa; border-bottom: 2px solid transparent; transition: color .15s;
            }
            .mp-sort-tab:hover { color: #e4e4e7; }
            .mp-sort-tab[aria-selected="true"] { color: #fafafa; border-bottom-color: #a855f7; }

            /* The renderer draws a "Back to Marketplace" link for the standalone
               page. In the overlay the marketplace is literally behind it and
               the close button is three pixels away, so it is noise here. */
            #mp-overlay .detail-back { display: none; }

            /* The listing page runs its own fixed background layer. Inside the
               overlay it would sit over the scrim and under nothing useful. */
            #mp-overlay #asset-bg-layer { display: none; }
            "#
        </style>
    }
}
