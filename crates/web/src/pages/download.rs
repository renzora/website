use leptos::prelude::*;
use leptos_meta::{Title, Meta};

#[component]
pub fn DownloadPage() -> impl IntoView {
    view! {
        <Title text="Download Renzora Engine, Free Open Source Bevy Editor" />
        <Meta name="description" content="Download Renzora Engine, the free and open-source Bevy editor, for Windows, macOS and Linux. A full 2D & 3D visual editor for the Bevy game engine, built in Rust." />

        // ── Hero ──
        <section class="relative min-h-[72vh] flex items-start justify-center overflow-hidden -mt-14 pt-36 px-6">
            <canvas id="hero-canvas" class="absolute inset-0 w-full h-full"></canvas>

            <div class="absolute top-1/4 left-1/4 w-96 h-96 bg-accent/20 rounded-full blur-[128px] animate-pulse pointer-events-none"></div>
            <div class="absolute bottom-1/4 right-1/4 w-80 h-80 bg-purple-600/15 rounded-full blur-[100px] pointer-events-none" style="animation: pulse 4s ease-in-out infinite 1s"></div>

            <div class="relative z-10 text-center max-w-3xl mx-auto">
                <div class="dl-hero-badge inline-flex items-center gap-2 px-3 py-1 rounded-full bg-accent/10 border border-accent/20 text-accent text-xs font-medium mb-4 backdrop-blur-sm">
                    <span class="w-1.5 h-1.5 rounded-full bg-accent animate-pulse"></span>
                    "r1-alpha6 · Early Access"
                </div>

                <h1 class="dl-hero-title text-5xl md:text-6xl lg:text-7xl font-extrabold tracking-tight leading-[1.05]">
                    "Renzora Engine"
                </h1>
                <p class="dl-hero-sub mt-4 text-sm text-zinc-500 uppercase tracking-widest font-medium">"Powered by Rust & Bevy 0.19"</p>
                <p class="dl-hero-sub mt-5 text-lg md:text-xl text-zinc-300 leading-relaxed max-w-2xl mx-auto">
                    "The first fully-featured game engine built on Bevy. Get the editor for Windows, macOS, and Linux, or clone the repo and build it with Cargo in minutes."
                </p>

                <div class="mt-10 flex gap-3 justify-center flex-wrap">
                    <a href="#install" class="dl-hero-cta group relative inline-flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-semibold bg-purple-600 text-white hover:bg-purple-500 transition-all hover:shadow-[0_0_30px_rgba(99,102,241,0.3)] hover:scale-[1.02]">
                        <i class="ph ph-terminal-window text-lg"></i>"Build from source"
                    </a>
                    <a href="#downloads" class="dl-hero-cta inline-flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-semibold bg-white/5 text-zinc-50 border border-zinc-700/50 hover:border-zinc-500 hover:bg-white/10 transition-all backdrop-blur-sm">
                        <i class="ph ph-download-simple text-lg"></i>"Download Engine"
                    </a>
                    <a href="https://github.com/renzora/engine" target="_blank" rel="noopener noreferrer" class="dl-hero-cta inline-flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-semibold bg-white/5 text-zinc-50 border border-zinc-700/50 hover:border-zinc-500 hover:bg-white/10 transition-all backdrop-blur-sm">
                        <i class="ph ph-github-logo text-lg"></i>"Source"
                    </a>
                </div>
            </div>
        </section>

        // ── Editor screenshot reveal ──
        <section class="relative -mt-10 pb-16 px-6 overflow-hidden">
            <div class="max-w-[1100px] mx-auto">
                <div class="dl-editor-reveal relative rounded-xl overflow-hidden border border-zinc-800/50 shadow-2xl shadow-black/50">
                    <div class="absolute inset-0 bg-gradient-to-t from-surface-panel via-transparent to-transparent z-10 pointer-events-none"></div>
                    <picture class="contents">
                        <source type="image/avif" sizes="(max-width: 1100px) 100vw, 1100px" srcset="/assets/previews/interface-640.avif 640w, /assets/previews/interface-1280.avif 1280w, /assets/previews/interface-1920.avif 1920w" />
                        <source type="image/webp" sizes="(max-width: 1100px) 100vw, 1100px" srcset="/assets/previews/interface-640.webp 640w, /assets/previews/interface-1280.webp 1280w, /assets/previews/interface-1920.webp 1920w" />
                        <img src="/assets/previews/interface-1280.webp" alt="The Renzora editor" class="w-full h-auto block" width="1600" height="858" fetchpriority="high" decoding="async" />
                    </picture>
                </div>
                <p class="text-center text-sm text-zinc-500 mt-4 max-w-2xl mx-auto">
                    "Running the download opens this: dockable panels, a scene hierarchy, a reflection-driven inspector, including your own custom components, and a live viewport."
                </p>
            </div>
        </section>

        // ── Install ──
        <section id="install" class="pb-20 px-6">
            <div class="max-w-[1000px] mx-auto">
                <div class="text-center mb-10">
                    <h2 class="text-3xl md:text-4xl font-bold">"Get the editor"</h2>
                    <p class="text-zinc-500 mt-3 text-base">"Two ways to start. Both give you the same editor."</p>
                </div>

                // Cargo (recommended)
                <div class="dl-install relative overflow-hidden p-6 md:p-8 rounded-2xl border border-accent/30 bg-gradient-to-br from-accent/[0.07] via-white/[0.02] to-purple-600/[0.06] mb-12">
                    <div class="absolute -top-16 -right-16 w-48 h-48 bg-accent/10 rounded-full blur-[80px] pointer-events-none"></div>
                    <div class="relative z-10">
                        <div class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-accent/15 border border-accent/30 text-accent text-[11px] font-semibold mb-4">
                            <i class="ph ph-star-four"></i>"Recommended for building games"
                        </div>
                        <div class="flex items-start gap-3 mb-3">
                            <div class="w-11 h-11 rounded-xl bg-accent/10 flex items-center justify-center shrink-0">
                                <i class="ph ph-terminal-window text-2xl text-accent"></i>
                            </div>
                            <div>
                                <h3 class="text-xl font-semibold">"Build from source"</h3>
                                <p class="text-sm text-zinc-400 mt-0.5">"Clone the engine and build it with Cargo. Needs a Rust toolchain and git on your PATH."</p>
                            </div>
                        </div>

                        <div class="rounded-xl bg-black/50 border border-zinc-800/60 overflow-hidden mt-5">
                            <div class="flex items-center gap-2 px-4 py-2.5 border-b border-zinc-800/60 bg-white/[0.02]">
                                <span class="w-2.5 h-2.5 rounded-full bg-red-500/70"></span>
                                <span class="w-2.5 h-2.5 rounded-full bg-amber-500/70"></span>
                                <span class="w-2.5 h-2.5 rounded-full bg-emerald-500/70"></span>
                                <span class="text-[11px] text-zinc-600 ml-2">"terminal"</span>
                            </div>
                            <div class="p-4 font-mono text-[13px] leading-relaxed space-y-2">
                                <p style="word-break:break-all">
                                    <span class="text-zinc-600 select-none">"$ "</span>
                                    <span class="text-emerald-400">"git"</span>
                                    <span class="text-zinc-200">" clone https://github.com/renzora/engine.git"</span>
                                </p>
                                <p>
                                    <span class="text-zinc-600 select-none">"$ "</span>
                                    <span class="text-emerald-400">"cd"</span>
                                    <span class="text-zinc-200">" engine"</span>
                                </p>
                                <p>
                                    <span class="text-zinc-600 select-none">"$ "</span>
                                    <span class="text-emerald-400">"cargo"</span>
                                    <span class="text-zinc-200">" renzora"</span>
                                    <span class="text-zinc-600">"        # build it and open the editor"</span>
                                </p>
                            </div>
                        </div>
                        <p class="text-xs text-zinc-500 mt-4">
                            <code class="text-zinc-300">"cargo renzora"</code>" compiles the engine and launches the editor. The first build takes a few minutes; after that it is incremental."
                        </p>
                    </div>
                </div>

                // Prebuilt downloads
                <div id="downloads" class="flex flex-wrap items-center justify-between gap-3 mb-3">
                    <h3 class="text-lg font-semibold flex items-center gap-2">
                        <div class="w-7 h-7 rounded-lg bg-accent/10 flex items-center justify-center">
                            <i class="ph ph-download-simple text-sm text-accent"></i>
                        </div>
                        "Download Engine"
                    </h3>
                    // Stable / nightly switch
                    <div class="flex items-center gap-1.5 p-1 bg-white/[0.02] rounded-xl border border-zinc-800/40">
                        <button type="button" id="ch-stable" onclick="dlChannel('stable')" class="px-5 py-2.5 rounded-lg text-sm font-semibold bg-sky-500 text-white transition-all">"Stable"</button>
                        <button type="button" id="ch-nightly" onclick="dlChannel('nightly')" class="px-5 py-2.5 rounded-lg text-sm font-semibold text-zinc-400 hover:text-zinc-200 transition-all">"Nightly"</button>
                    </div>
                </div>

                <p class="text-xs text-zinc-500 mb-5">
                    <span id="dl-release-label">"Latest stable release: "</span>
                    <span id="dl-release-tag" class="text-zinc-300">"r1-alpha6"</span>
                    <span id="dl-release-note" class="text-zinc-600"></span>
                </p>

                // One card per platform; the build (editor/runtime, architecture)
                // is picked from the card's dropdown. Options are server-rendered
                // for the stable channel so the page works without JS, and swapped
                // out by `dlRender` when the channel changes.
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                    <DownloadCard platform="windows" name="Windows" icon="ph-windows-logo" opt_label="Editor · x64" opt_asset="windows-x64.zip" />
                    <DownloadCard platform="macos" name="macOS" icon="ph-apple-logo" opt_label="Editor · Apple Silicon" opt_asset="macos-arm64.zip" />
                    <DownloadCard platform="linux" name="Linux" icon="ph-linux-logo" opt_label="Editor · ARM64" opt_asset="linux-arm64.zip" />
                    <DownloadCard platform="web" name="Web" icon="ph-globe" opt_label="" opt_asset="" />
                </div>

                <p class="text-xs text-zinc-500 mt-4 text-center">
                    "Each build is the engine binary with the "<code class="text-zinc-300">"renzora_editor"</code>" bundle beside it, just run it to open the editor."
                </p>
            </div>
        </section>

        // ── Other options ──
        <section class="pb-24 px-6">
            <div class="max-w-[1000px] mx-auto">
                <h2 class="text-lg font-semibold mb-5 flex items-center gap-2">
                    <div class="w-7 h-7 rounded-lg bg-emerald-500/10 flex items-center justify-center">
                        <i class="ph ph-git-branch text-sm text-emerald-400"></i>
                    </div>
                    "Other ways to get it"
                </h2>
                <div class="dl-options grid grid-cols-1 md:grid-cols-2 gap-4">
                    <a href="https://github.com/renzora/engine" target="_blank" rel="noopener noreferrer" class="group p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl hover:border-zinc-600 hover:bg-white/[0.04] transition-all flex items-center gap-4">
                        <div class="w-10 h-10 rounded-xl bg-zinc-800/80 flex items-center justify-center shrink-0 group-hover:scale-110 transition-transform">
                            <i class="ph ph-terminal text-xl text-zinc-400"></i>
                        </div>
                        <div>
                            <h4 class="text-sm font-semibold mb-0.5 group-hover:text-accent transition-colors">"Build from source"</h4>
                            <p class="text-xs text-zinc-500">"Clone the repo and compile the editor yourself with Cargo."</p>
                        </div>
                        <i class="ph ph-arrow-up-right text-zinc-600 ml-auto group-hover:text-accent transition-colors"></i>
                    </a>
                    <a href="https://github.com/renzora/engine/releases" target="_blank" rel="noopener noreferrer" class="group p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl hover:border-zinc-600 hover:bg-white/[0.04] transition-all flex items-center gap-4">
                        <div class="w-10 h-10 rounded-xl bg-zinc-800/80 flex items-center justify-center shrink-0 group-hover:scale-110 transition-transform">
                            <i class="ph ph-tag text-xl text-zinc-400"></i>
                        </div>
                        <div>
                            <h4 class="text-sm font-semibold mb-0.5 group-hover:text-accent transition-colors">"All releases"</h4>
                            <p class="text-xs text-zinc-500">"Browse every version and pre-release build on GitHub."</p>
                        </div>
                        <i class="ph ph-arrow-up-right text-zinc-600 ml-auto group-hover:text-accent transition-colors"></i>
                    </a>
                </div>
            </div>
        </section>


        // ── Support / Donate ──
        <section class="pb-24 px-6">
            <div class="max-w-[1000px] mx-auto">
                <div class="relative overflow-hidden rounded-3xl border border-rose-500/25 bg-gradient-to-br from-rose-500/[0.12] via-purple-600/[0.05] to-rose-500/[0.08] p-10 md:p-16 text-center">
                    <div class="absolute -top-20 left-1/2 -translate-x-1/2 w-[32rem] h-56 bg-rose-500/15 rounded-full blur-[110px] pointer-events-none"></div>
                    <div class="absolute -bottom-16 right-1/4 w-72 h-40 bg-purple-600/10 rounded-full blur-[90px] pointer-events-none"></div>
                    <div class="relative z-10">
                        <img src="/assets/previews/hazel.webp" alt="Hazel, the Renzora mascot" width="112" height="112" class="w-28 h-28 rounded-2xl object-cover mx-auto mb-5 shadow-lg shadow-rose-500/20 ring-1 ring-rose-500/30" />
                        <h2 class="text-3xl md:text-5xl font-extrabold tracking-tight">"Support Renzora"</h2>
                        <p class="text-zinc-300 mt-5 text-base md:text-lg leading-relaxed max-w-2xl mx-auto">
                            "Renzora Engine is free and open source, and it always will be. It's built in the open, with a lot of love, for anyone who dreams of making their own games."
                        </p>
                        <p class="text-zinc-400 mt-4 text-base leading-relaxed max-w-2xl mx-auto">
                            "If Renzora has helped you build something, taught you something, or just made your day a little easier, a donation, however small, genuinely means the world. It keeps the lights on and keeps the engine growing."
                        </p>
                        <a href="/donate" class="mt-8 inline-flex items-center gap-2.5 px-8 py-4 rounded-xl text-base font-semibold bg-rose-500 text-white hover:bg-rose-400 transition-all hover:shadow-[0_0_40px_rgba(244,63,94,0.4)] hover:scale-[1.03]">
                            <i class="ph ph-heart text-lg"></i>"Support Renzora's future"
                        </a>
                        <p class="text-sm text-zinc-400 mt-5 italic">"Thank you, truly. Hazel ♥"</p>
                    </div>
                </div>
            </div>
        </section>

        // ── Post-download signup prompt (revealed after a download starts) ──
        <div id="dl-modal" class="hidden fixed inset-0 z-[100] items-center justify-center p-4">
            <div class="absolute inset-0 bg-black/70 backdrop-blur-sm" onclick="dlClose()"></div>
            <div class="relative w-full max-w-md bg-surface-card border border-white/[0.08] rounded-2xl shadow-2xl shadow-black/60 p-6 max-h-[90vh] overflow-y-auto">
                <button onclick="dlClose()" class="absolute top-3 right-3 w-8 h-8 rounded-full bg-white/[0.05] hover:bg-white/[0.1] text-zinc-400 hover:text-white flex items-center justify-center transition-all" aria-label="Close">
                    <i class="ph ph-x"></i>
                </button>
                <div class="text-center">
                    <div class="w-14 h-14 rounded-2xl bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center mx-auto mb-4">
                        <i class="ph ph-check-circle text-3xl text-emerald-400"></i>
                    </div>
                    <h2 class="text-xl font-bold">"Your download is starting"</h2>
                    <p class="text-zinc-400 mt-2 text-sm">"Create a free account to get the most out of Renzora."</p>
                </div>
                <ul class="mt-6 space-y-3.5">
                    <li class="flex items-start gap-3">
                        <div class="w-8 h-8 rounded-lg bg-teal-500/10 flex items-center justify-center shrink-0"><i class="ph ph-storefront text-teal-400"></i></div>
                        <p class="text-sm text-zinc-300 leading-snug">"Download free assets, models and plugins from the marketplace"</p>
                    </li>
                    <li class="flex items-start gap-3">
                        <div class="w-8 h-8 rounded-lg bg-accent/10 flex items-center justify-center shrink-0"><i class="ph ph-upload-simple text-accent"></i></div>
                        <p class="text-sm text-zinc-300 leading-snug">"Publish and sell your own creations"</p>
                    </li>
                    <li class="flex items-start gap-3">
                        <div class="w-8 h-8 rounded-lg bg-sky-500/10 flex items-center justify-center shrink-0"><i class="ph ph-books text-sky-400"></i></div>
                        <p class="text-sm text-zinc-300 leading-snug">"Keep every purchase in your library, ready to re-download"</p>
                    </li>
                    <li class="flex items-start gap-3">
                        <div class="w-8 h-8 rounded-lg bg-amber-500/10 flex items-center justify-center shrink-0"><i class="ph ph-trophy text-amber-400"></i></div>
                        <p class="text-sm text-zinc-300 leading-snug">"Earn XP, level up, and unlock perks"</p>
                    </li>
                </ul>
                <a href="/register" class="mt-6 block text-center px-6 py-3 rounded-xl text-sm font-semibold bg-purple-600 text-white hover:bg-purple-500 transition-all">"Create your free account"</a>
                <div class="mt-3 flex items-center justify-center gap-3 text-xs">
                    <a href="/login" class="text-zinc-400 hover:text-white transition-colors">"Sign in"</a>
                    <span class="text-zinc-700">"·"</span>
                    <a href="/donate" class="text-rose-400 hover:text-rose-300 transition-colors inline-flex items-center gap-1"><i class="ph ph-heart"></i>"Donate"</a>
                    <span class="text-zinc-700">"·"</span>
                    <button onclick="dlClose()" class="text-zinc-500 hover:text-zinc-300 transition-colors">"Maybe later"</button>
                </div>
            </div>
        </div>

        // ── Post-download prompt logic ──
        <script>
            r#"
            function dlPrompt() {
                // Skip the signup prompt for people who are already logged in.
                if (document.cookie.match(/(^|;)\s*user\s*=/)) return;
                const m = document.getElementById('dl-modal');
                if (!m) return;
                m.classList.remove('hidden');
                m.classList.add('flex');
                document.body.style.overflow = 'hidden';
            }
            function dlClose() {
                const m = document.getElementById('dl-modal');
                if (!m) return;
                m.classList.add('hidden');
                m.classList.remove('flex');
                document.body.style.overflow = '';
            }
            document.addEventListener('keydown', function(e) { if (e.key === 'Escape') dlClose(); });

            // ── Download channel + per-platform build picker ──
            // Which builds each channel ships, per platform: [asset, label].
            // Stable only publishes three editor builds; the full architecture
            // matrix and the runtime bundles are nightly-only.
            var DL_BUILDS = {
                stable: {
                    windows: [['windows-x64.zip', 'Editor · x64']],
                    macos:   [['macos-arm64.zip', 'Editor · Apple Silicon']],
                    linux:   [['linux-arm64.zip', 'Editor · ARM64']],
                    web:     []
                },
                nightly: {
                    windows: [
                        ['windows-x64.zip', 'Editor · x64'],
                        ['windows-arm64.zip', 'Editor · ARM64'],
                        ['renzora-runtime-windows-x64.zip', 'Runtime · x64'],
                        ['renzora-runtime-windows-arm64.zip', 'Runtime · ARM64']
                    ],
                    macos: [
                        ['macos-arm64.zip', 'Editor · Apple Silicon'],
                        ['macos-x64.zip', 'Editor · Intel x64'],
                        ['renzora-runtime-macos-arm64.zip', 'Runtime · Apple Silicon'],
                        ['renzora-runtime-macos-x64.zip', 'Runtime · Intel x64']
                    ],
                    linux: [
                        ['linux-x64.zip', 'Editor · x64'],
                        ['linux-arm64.zip', 'Editor · ARM64'],
                        ['renzora-runtime-linux-x64.zip', 'Runtime · x64'],
                        ['renzora-runtime-linux-arm64.zip', 'Runtime · ARM64']
                    ],
                    web: [
                        ['web-wasm32.zip', 'Editor · wasm32'],
                        ['renzora-runtime-web-wasm32.zip', 'Runtime · wasm32'],
                        ['engine-source.zip', 'Engine source']
                    ]
                }
            };
            var DL_RELEASES = 'https://github.com/renzora/engine/releases';
            var DL_TAGS = { stable: 'r1-alpha6', nightly: null };
            var dlChannelName = 'stable';

            function dlAssetUrl(asset) {
                var tag = DL_TAGS[dlChannelName];
                if (!tag || !asset) return DL_RELEASES;
                return DL_RELEASES + '/download/' + tag + '/' + asset;
            }

            // Keep a card's button in step with its dropdown.
            function dlPick(sel) {
                var platform = sel.getAttribute('data-dl-select');
                var btn = document.querySelector('[data-dl-button="' + platform + '"]');
                if (btn) btn.href = dlAssetUrl(sel.value);
            }

            function dlRender() {
                var builds = DL_BUILDS[dlChannelName] || {};
                document.querySelectorAll('[data-dl-select]').forEach(function(sel) {
                    var platform = sel.getAttribute('data-dl-select');
                    var opts = builds[platform] || [];
                    var card = document.querySelector('[data-dl-card="' + platform + '"]');
                    var btn = document.querySelector('[data-dl-button="' + platform + '"]');
                    var prev = sel.value;

                    sel.innerHTML = '';
                    if (!opts.length) {
                        var none = document.createElement('option');
                        none.value = '';
                        none.textContent = 'Not in this release';
                        sel.appendChild(none);
                        sel.disabled = true;
                        if (card) card.classList.add('opacity-40');
                        if (btn) btn.href = DL_RELEASES;
                        return;
                    }

                    sel.disabled = false;
                    if (card) card.classList.remove('opacity-40');
                    opts.forEach(function(o) {
                        var el = document.createElement('option');
                        el.value = o[0];
                        el.textContent = o[1];
                        sel.appendChild(el);
                    });
                    // Keep the visitor's pick across a channel switch when that
                    // build exists on the new channel too.
                    sel.value = opts.some(function(o) { return o[0] === prev; }) ? prev : opts[0][0];
                    if (btn) btn.href = dlAssetUrl(sel.value);
                });
            }

            function dlChannel(ch) {
                dlChannelName = ch;
                var nightly = ch === 'nightly';
                [['ch-stable', !nightly], ['ch-nightly', nightly]].forEach(function(pair) {
                    var btn = document.getElementById(pair[0]);
                    if (!btn) return;
                    btn.classList.toggle('bg-sky-500', pair[1]);
                    btn.classList.toggle('text-white', pair[1]);
                    btn.classList.toggle('text-zinc-400', !pair[1]);
                    btn.classList.toggle('hover:text-zinc-200', !pair[1]);
                });

                var label = document.getElementById('dl-release-label');
                var tagEl = document.getElementById('dl-release-tag');
                var note = document.getElementById('dl-release-note');
                if (nightly) {
                    if (label) label.textContent = 'Latest nightly: ';
                    if (tagEl) tagEl.textContent = DL_TAGS.nightly || 'resolving…';
                    if (note) note.textContent = ' · built from main, expect breakage';
                    dlResolveNightly();
                } else {
                    if (label) label.textContent = 'Latest stable release: ';
                    if (tagEl) tagEl.textContent = DL_TAGS.stable;
                    if (note) note.textContent = '';
                }
                dlRender();
            }

            // Nightlies are tagged per-day, so the tag can't be baked into the page.
            // Resolve the newest pre-release once per tab; on any failure the cards
            // fall back to the releases list, which still gets people somewhere.
            var nightlyPending = false;
            async function dlResolveNightly() {
                if (DL_TAGS.nightly || nightlyPending) return;
                nightlyPending = true;
                var tag = null;
                try { tag = sessionStorage.getItem('renzora_nightly_tag'); } catch (e) {}
                if (!tag) {
                    try {
                        var res = await fetch('https://api.github.com/repos/renzora/engine/releases?per_page=20');
                        if (!res.ok) throw new Error('releases lookup failed');
                        var list = await res.json();
                        var latest = (list || []).find(function(r) { return r.prerelease && !r.draft; });
                        if (!latest) throw new Error('no pre-release found');
                        tag = latest.tag_name;
                        try { sessionStorage.setItem('renzora_nightly_tag', tag); } catch (e) {}
                    } catch (e) {
                        var t = document.getElementById('dl-release-tag');
                        if (t) t.textContent = 'see all releases';
                        nightlyPending = false;   // allow a retry on the next switch
                        return;
                    }
                }
                DL_TAGS.nightly = tag;
                nightlyPending = false;
                if (dlChannelName === 'nightly') {
                    var t2 = document.getElementById('dl-release-tag');
                    if (t2) t2.textContent = tag;
                    dlRender();
                }
            }
            "#
        </script>

        // ── Particle canvas ──
        <script>
            r#"
            // Particle canvas
            (function() {
                const canvas = document.getElementById('hero-canvas');
                if (!canvas) return;
                const ctx = canvas.getContext('2d');
                let w, h, rect, particles = [], mouse = { x: -1000, y: -1000 };

                function resize() {
                    // Batch layout reads before any write so we force at most one reflow.
                    const ow = canvas.offsetWidth, oh = canvas.offsetHeight;
                    rect = canvas.getBoundingClientRect();
                    w = canvas.width = ow;
                    h = canvas.height = oh;
                }
                resize();
                window.addEventListener('resize', resize);
                window.addEventListener('scroll', () => { rect = canvas.getBoundingClientRect(); }, { passive: true });

                canvas.addEventListener('mousemove', e => {
                    // Use the cached rect, never read layout during the move.
                    mouse.x = e.clientX - rect.left;
                    mouse.y = e.clientY - rect.top;
                });
                canvas.addEventListener('mouseleave', () => { mouse.x = -1000; mouse.y = -1000; });

                const count = Math.min(80, Math.floor(w * h / 15000));
                for (let i = 0; i < count; i++) {
                    particles.push({
                        x: Math.random() * w,
                        y: Math.random() * h,
                        vx: (Math.random() - 0.5) * 0.3,
                        vy: (Math.random() - 0.5) * 0.3,
                        r: Math.random() * 1.5 + 0.5,
                    });
                }

                function draw() {
                    ctx.clearRect(0, 0, w, h);
                    for (let i = 0; i < particles.length; i++) {
                        const p = particles[i];
                        p.x += p.vx;
                        p.y += p.vy;
                        if (p.x < 0) p.x = w;
                        if (p.x > w) p.x = 0;
                        if (p.y < 0) p.y = h;
                        if (p.y > h) p.y = 0;

                        const dx = p.x - mouse.x;
                        const dy = p.y - mouse.y;
                        const dist = Math.sqrt(dx * dx + dy * dy);
                        if (dist < 120) {
                            const force = (120 - dist) / 120 * 0.8;
                            p.x += dx / dist * force;
                            p.y += dy / dist * force;
                        }

                        ctx.beginPath();
                        ctx.arc(p.x, p.y, p.r, 0, Math.PI * 2);
                        ctx.fillStyle = 'rgba(99, 102, 241, 0.4)';
                        ctx.fill();

                        for (let j = i + 1; j < particles.length; j++) {
                            const p2 = particles[j];
                            const ddx = p.x - p2.x;
                            const ddy = p.y - p2.y;
                            const d = ddx * ddx + ddy * ddy;
                            if (d < 18000) {
                                ctx.beginPath();
                                ctx.moveTo(p.x, p.y);
                                ctx.lineTo(p2.x, p2.y);
                                const alpha = (1 - d / 18000) * 0.15;
                                ctx.strokeStyle = `rgba(99, 102, 241, ${alpha})`;
                                ctx.lineWidth = 0.5;
                                ctx.stroke();
                            }
                        }
                    }
                    requestAnimationFrame(draw);
                }
                draw();
            })();
            "#
        </script>

        <style>
            r#"
            @keyframes fadeSlideUp {
                from { opacity: 0; transform: translateY(16px); }
                to { opacity: 1; transform: translateY(0); }
            }

            /* Hero title shimmer */
            .dl-hero-title {
                background: linear-gradient(135deg, #fafafa 0%, #6366f1 40%, #a78bfa 60%, #fafafa 100%);
                background-size: 300% 300%;
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
                background-clip: text;
                animation: dl-shimmer 6s ease-in-out infinite;
            }
            @keyframes dl-shimmer {
                0%, 100% { background-position: 0% 50%; }
                50% { background-position: 100% 50%; }
            }

            /* Feature card glow on hover */
            .feature-card::before {
                content: '';
                position: absolute;
                inset: 0;
                border-radius: 0.75rem;
                opacity: 0;
                transition: opacity 0.3s;
                pointer-events: none;
            }
            .feature-card:hover::before { opacity: 1; }
            .feature-card.glow-indigo::before { background: radial-gradient(circle at 50% 0%, rgba(99,102,241,0.08), transparent 70%); }
            .feature-card.glow-violet::before { background: radial-gradient(circle at 50% 0%, rgba(139,92,246,0.08), transparent 70%); }
            .feature-card.glow-cyan::before { background: radial-gradient(circle at 50% 0%, rgba(6,182,212,0.08), transparent 70%); }
            .feature-card.glow-emerald::before { background: radial-gradient(circle at 50% 0%, rgba(16,185,129,0.08), transparent 70%); }
            .feature-card.glow-amber::before { background: radial-gradient(circle at 50% 0%, rgba(245,158,11,0.08), transparent 70%); }
            .feature-card.glow-rose::before { background: radial-gradient(circle at 50% 0%, rgba(244,63,94,0.08), transparent 70%); }
            .feature-card.glow-sky::before { background: radial-gradient(circle at 50% 0%, rgba(14,165,233,0.08), transparent 70%); }
            .feature-card.glow-orange::before { background: radial-gradient(circle at 50% 0%, rgba(249,115,22,0.08), transparent 70%); }

            .icon-indigo { color: #6366f1; background: rgba(99,102,241,0.1); }
            .icon-violet { color: #8b5cf6; background: rgba(139,92,246,0.1); }
            .icon-cyan { color: #06b6d4; background: rgba(6,182,212,0.1); }
            .icon-emerald { color: #10b981; background: rgba(16,185,129,0.1); }
            .icon-amber { color: #f59e0b; background: rgba(245,158,11,0.1); }
            .icon-rose { color: #f43f5e; background: rgba(244,63,94,0.1); }
            .icon-sky { color: #0ea5e9; background: rgba(14,165,233,0.1); }
            .icon-orange { color: #f97316; background: rgba(249,115,22,0.1); }
            "#
        </style>
    }
}

/// One platform's download card. `opt_label`/`opt_asset` seed the dropdown with
/// that platform's stable build so the card is usable before (or without) JS;
/// `dlRender` replaces the options when the channel changes. An empty
/// `opt_asset` means the platform has no build on the stable channel.
#[component]
fn DownloadCard(
    platform: &'static str,
    name: &'static str,
    icon: &'static str,
    opt_label: &'static str,
    opt_asset: &'static str,
) -> impl IntoView {
    let available = !opt_asset.is_empty();
    let icon_class = format!("ph {} text-xl {}", icon, if available { "text-zinc-200" } else { "text-zinc-600" });
    // Dim a platform with no build on the server-rendered (stable) channel, so
    // the first paint matches what `dlRender` would draw.
    let card_class = format!(
        "relative p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl flex flex-col items-center gap-3 text-center transition-all hover:border-accent/40 hover:bg-white/[0.04]{}",
        if available { "" } else { " opacity-40" }
    );
    let href = if available {
        format!("https://github.com/renzora/engine/releases/download/r1-alpha6/{opt_asset}")
    } else {
        "https://github.com/renzora/engine/releases".to_string()
    };
    view! {
        <div data-dl-card=platform class=card_class>
            <div class="w-12 h-12 rounded-2xl bg-white/[0.03] border border-zinc-800/30 flex items-center justify-center">
                <i class=icon_class></i>
            </div>
            <h3 class="text-base font-semibold">{name}</h3>

            <select
                data-dl-select=platform
                disabled=!available
                onchange="dlPick(this)"
                class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-xs outline-none focus:border-accent transition-colors cursor-pointer"
            >
                {if available {
                    view! { <option value=opt_asset>{opt_label}</option> }.into_any()
                } else {
                    view! { <option value="">"Not in this release"</option> }.into_any()
                }}
            </select>

            <a
                data-dl-button=platform
                href=href
                target="_blank"
                rel="noopener noreferrer"
                onclick="dlPrompt()"
                class="w-full inline-flex items-center justify-center gap-1.5 px-3 py-2.5 rounded-lg text-sm font-medium bg-purple-600 text-white hover:bg-purple-500 transition-all"
            >
                <i class="ph ph-download-simple"></i>"Download"
            </a>
        </div>
    }
}
