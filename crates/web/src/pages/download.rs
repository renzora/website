use leptos::prelude::*;
use leptos_meta::{Title, Meta};

#[component]
pub fn DownloadPage() -> impl IntoView {
    view! {
        <Title text="Download Renzora — Free Open Source Bevy Editor" />
        <Meta name="description" content="Download Renzora, the free and open-source Bevy editor, for Windows, macOS and Linux. A full 2D & 3D visual editor for the Bevy game engine, built in Rust." />

        // ── Hero ──
        <section class="relative min-h-[72vh] flex items-start justify-center overflow-hidden -mt-14 pt-36 px-6">
            <canvas id="hero-canvas" class="absolute inset-0 w-full h-full"></canvas>

            <div class="absolute top-1/4 left-1/4 w-96 h-96 bg-accent/20 rounded-full blur-[128px] animate-pulse pointer-events-none"></div>
            <div class="absolute bottom-1/4 right-1/4 w-80 h-80 bg-purple-600/15 rounded-full blur-[100px] pointer-events-none" style="animation: pulse 4s ease-in-out infinite 1s"></div>

            <div class="relative z-10 text-center max-w-3xl mx-auto">
                <div class="dl-hero-badge inline-flex items-center gap-2 px-3 py-1 rounded-full bg-accent/10 border border-accent/20 text-accent text-xs font-medium mb-4 backdrop-blur-sm">
                    <span class="w-1.5 h-1.5 rounded-full bg-accent animate-pulse"></span>
                    "r1-alpha6 — Early Access"
                </div>

                <h1 class="dl-hero-title text-5xl md:text-6xl lg:text-7xl font-extrabold tracking-tight leading-[1.05]">
                    "Download Renzora"
                </h1>
                <p class="dl-hero-sub mt-4 text-sm text-zinc-500 uppercase tracking-widest font-medium">"Powered by Rust & Bevy 0.19"</p>
                <p class="dl-hero-sub mt-5 text-lg md:text-xl text-zinc-300 leading-relaxed max-w-2xl mx-auto">
                    "The first fully-featured game engine built on Bevy. Get the editor for Windows, macOS, and Linux — or install the Cargo CLI and scaffold your first game in minutes."
                </p>

                <div class="mt-10 flex gap-3 justify-center flex-wrap">
                    <a href="#install" class="dl-hero-cta group relative inline-flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-semibold bg-purple-600 text-white hover:bg-purple-500 transition-all hover:shadow-[0_0_30px_rgba(99,102,241,0.3)] hover:scale-[1.02]">
                        <i class="ph ph-terminal-window text-lg"></i>"Install with Cargo"
                    </a>
                    <a href="#downloads" class="dl-hero-cta inline-flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-semibold bg-white/5 text-zinc-50 border border-zinc-700/50 hover:border-zinc-500 hover:bg-white/10 transition-all backdrop-blur-sm">
                        <i class="ph ph-download-simple text-lg"></i>"Download prebuilt"
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
                    <img src="/assets/previews/interface.webp" alt="The Renzora editor" class="w-full h-auto block" width="1600" height="858" fetchpriority="high" decoding="async" data-zoom="1" />
                </div>
                <p class="text-center text-sm text-zinc-500 mt-4 max-w-2xl mx-auto">
                    "Running the download opens this: dockable panels, a scene hierarchy, a reflection-driven inspector — including your own custom components — and a live viewport."
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
                                <h3 class="text-xl font-semibold">"Install with Cargo"</h3>
                                <p class="text-sm text-zinc-400 mt-0.5">"The Renzora CLI, published on crates.io. Needs a Rust toolchain, plus Docker and git on your PATH."</p>
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
                                <p>
                                    <span class="text-zinc-600 select-none">"$ "</span>
                                    <span class="text-emerald-400">"cargo"</span>
                                    <span class="text-zinc-200">" install renzora"</span>
                                    <span class="text-zinc-600">"      # the published CLI"</span>
                                </p>
                                <p>
                                    <span class="text-zinc-600 select-none">"$ "</span>
                                    <span class="text-emerald-400">"renzora"</span>
                                    <span class="text-zinc-200">" new my-game"</span>
                                    <span class="text-zinc-600">"     # scaffold a project"</span>
                                </p>
                                <p>
                                    <span class="text-zinc-600 select-none">"$ "</span>
                                    <span class="text-emerald-400">"renzora"</span>
                                    <span class="text-zinc-200">" run"</span>
                                    <span class="text-zinc-600">"             # build it and open the editor"</span>
                                </p>
                            </div>
                        </div>
                        <p class="text-xs text-zinc-500 mt-4">
                            <code class="text-zinc-300">"renzora new"</code>" scaffolds a fresh project; "
                            <code class="text-zinc-300">"renzora run"</code>" compiles it and launches the editor on your game."
                        </p>
                    </div>
                </div>

                // Prebuilt downloads
                <div id="downloads" class="flex flex-wrap items-end justify-between gap-2 mb-5">
                    <h3 class="text-lg font-semibold flex items-center gap-2">
                        <div class="w-7 h-7 rounded-lg bg-accent/10 flex items-center justify-center">
                            <i class="ph ph-download-simple text-sm text-accent"></i>
                        </div>
                        "Or download a prebuilt editor"
                    </h3>
                    <p class="text-xs text-zinc-500">
                        "Latest release: "
                        <span id="release-version" class="text-zinc-300">"r1-alpha6"</span>
                    </p>
                </div>
                <div id="editor-downloads" class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <DownloadSkeleton />
                    <DownloadSkeleton />
                    <DownloadSkeleton />
                </div>
                <p class="text-xs text-zinc-500 mt-4 text-center">
                    "Each build is the engine binary with the "<code class="text-zinc-300">"renzora_editor"</code>" bundle beside it — just run it to open the editor."
                </p>
            </div>
        </section>

        // ── What's inside ──
        <section class="pb-20 px-6">
            <div class="max-w-[1150px] mx-auto">
                <div class="text-center mb-12">
                    <h2 class="text-3xl md:text-4xl font-bold">"What's inside"</h2>
                    <p class="text-zinc-500 mt-3 text-base">"All real — and almost every feature is its own plugin."</p>
                </div>
                <div class="dl-feature-grid grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                    <FeatureCard icon="ph-cube" title="Scene editor" description="Dockable panels, a scene hierarchy, and a reflection-driven inspector. Define custom components with derive(Inspectable) — they're editable and serialized with the scene." color="indigo" />
                    <FeatureCard icon="ph-code" title="Scripting" description="Write gameplay in Lua 5.4 or Rhai — chosen by file extension — or wire it up visually with Blueprint node graphs." color="violet" />
                    <FeatureCard icon="ph-puzzle-piece" title="Plugin system" description="Almost everything is a plugin across ~187 workspace crates. Drop hot-loadable cdylibs into plugins/ and register them with renzora::add!." color="emerald" />
                    <FeatureCard icon="ph-drop" title="Materials & shaders" description="A node-based material graph, custom WGSL shaders, and 50+ post-process effects." color="sky" />
                    <FeatureCard icon="ph-browsers" title="renzora_ember UI" description="Build game and editor interfaces from .html templates with reactive {{ }} bindings." color="amber" />
                    <FeatureCard icon="ph-soccer-ball" title="Physics" description="Rigid bodies, colliders, and joints powered by the Avian physics engine." color="rose" />
                    <FeatureCard icon="ph-devices" title="Cross-platform export" description="Ship to Windows, Linux, macOS, Android, iOS, and Web (WASM) from a single project." color="cyan" />
                    <FeatureCard icon="ph-git-branch" title="Open source" description="Built on Rust and Bevy 0.19, and fully open source under MIT/Apache." color="orange" />
                </div>
            </div>
        </section>

        // ── Showcase ──
        <section class="pb-20 px-6">
            <div class="max-w-[1150px] mx-auto">
                <div class="text-center mb-12">
                    <h2 class="text-3xl md:text-4xl font-bold">"See it in action"</h2>
                    <p class="text-zinc-500 mt-3 text-base">"Captured straight from the editor."</p>
                </div>
                <div class="dl-showcase grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-5">
                    <ShotCard img="viewport" title="Multiple viewports & gizmos" caption="Edit scenes across up to four viewports, each with move, rotate, and scale gizmos." />
                    <ShotCard img="code_editor" title="Lua & Rhai scripting" caption="A built-in code editor — scripts run as Lua 5.4 or Rhai, picked by file extension." />
                    <ShotCard img="material_graph" title="Node-based materials" caption="Compose PBR materials in a node graph, then drop to custom WGSL when you need to." />
                    <ShotCard img="renzora_ember" title="renzora_ember UI" caption="The toolkit behind both game and editor interfaces — .html templates with reactive bindings." />
                    <ShotCard img="debugging" title="11-panel debugger" caption="Profile frames, memory, render and ECS stats, physics, culling, and Lumen GI in real time." />
                    <ShotCard img="panels" title="A plugin for everything" caption="Add panels for Blueprint, terrain, particles, shaders, audio, and more — each its own plugin." />
                </div>
            </div>
        </section>

        // ── Stats ──
        <section class="pb-20 px-6">
            <div class="max-w-[1000px] mx-auto">
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                    <div class="dl-stat text-center p-6 rounded-xl bg-white/[0.02] border border-zinc-800/50">
                        <div class="dl-counter text-3xl font-bold text-accent" data-target="6">"0"</div>
                        <div class="text-xs text-zinc-500 mt-1 uppercase tracking-wider">"Export platforms"</div>
                    </div>
                    <div class="dl-stat text-center p-6 rounded-xl bg-white/[0.02] border border-zinc-800/50">
                        <div class="dl-counter text-3xl font-bold text-accent" data-target="3">"0"</div>
                        <div class="text-xs text-zinc-500 mt-1 uppercase tracking-wider">"Scripting options"</div>
                    </div>
                    <div class="dl-stat text-center p-6 rounded-xl bg-white/[0.02] border border-zinc-800/50">
                        <div class="dl-counter text-3xl font-bold text-accent" data-target="187" data-prefix="~">"0"</div>
                        <div class="text-xs text-zinc-500 mt-1 uppercase tracking-wider">"Workspace crates"</div>
                    </div>
                    <div class="dl-stat text-center p-6 rounded-xl bg-white/[0.02] border border-zinc-800/50">
                        <div class="dl-counter text-3xl font-bold text-accent" data-target="50" data-suffix="+">"0"</div>
                        <div class="text-xs text-zinc-500 mt-1 uppercase tracking-wider">"Post-process effects"</div>
                    </div>
                </div>
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

        // ── Release-fetch script (fills the prebuilt download cards) ──
        <script>
            r##"
            const PLATFORMS = [
                { key: 'windows', name: 'Windows', icon: 'ph-windows-logo', match: /windows.*\.zip$/i, req: 'Windows 10+, 64-bit', color: 'cyan' },
                { key: 'macos', name: 'macOS', icon: 'ph-apple-logo', match: /(macos|osx|darwin).*\.zip$/i, req: 'macOS 12 Monterey+', color: 'zinc' },
                { key: 'linux', name: 'Linux', icon: 'ph-linux-logo', match: /linux.*\.zip$/i, req: 'Ubuntu 22.04+, Fedora 38+', color: 'amber' },
            ];

            (async function() {
                let release = null;
                let assets = [];
                try {
                    const res = await fetch('https://api.github.com/repos/renzora/engine/releases/latest');
                    if (res.ok) { release = await res.json(); assets = release.assets || []; }
                } catch(e) {}

                if (!release) {
                    try {
                        const res = await fetch('https://api.github.com/repos/renzora/engine/releases');
                        if (res.ok) { const releases = await res.json(); if (releases.length) { release = releases[0]; assets = release.assets || []; } }
                    } catch(e) {}
                }

                const versionEl = document.getElementById('release-version');
                if (versionEl) {
                    if (release) {
                        versionEl.innerHTML = `<a href="${release.html_url}" target="_blank" rel="noopener noreferrer" class="text-accent hover:text-accent-hover">${release.tag_name}</a> — ${new Date(release.published_at).toLocaleDateString()}`;
                    }
                }

                function findAsset(pattern) { return assets.find(a => pattern.test(a.name)); }

                const el = document.getElementById('editor-downloads');
                if (!el) return;
                el.innerHTML = PLATFORMS.map((p, i) => {
                    const asset = findAsset(p.match);
                    const available = !!asset;
                    const url = asset ? asset.browser_download_url : '#';
                    const size = asset ? formatSize(asset.size) : '';
                    return `
                        <div class="relative p-6 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-center flex flex-col items-center gap-3 ${available ? 'hover:border-accent/40 hover:bg-white/[0.04] hover:shadow-lg hover:shadow-accent/5' : 'opacity-40'} transition-all" style="animation: fadeSlideUp 0.5s ease both; animation-delay: ${i * 100}ms">
                            <div class="w-14 h-14 rounded-2xl bg-white/[0.03] border border-zinc-800/30 flex items-center justify-center">
                                <i class="ph ${p.icon} text-2xl ${available ? 'text-zinc-200' : 'text-zinc-600'}"></i>
                            </div>
                            <h3 class="text-lg font-semibold">${p.name}</h3>
                            <p class="text-[11px] text-zinc-500">${p.req}</p>
                            ${available ? `
                                <a href="${url}" class="w-full mt-1 inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl text-sm font-medium bg-purple-600 text-white hover:bg-purple-500 transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)]">
                                    <i class="ph ph-download-simple"></i>Download
                                </a>
                                <span class="text-[10px] text-zinc-600">${size}</span>
                            ` : `
                                <span class="w-full mt-1 inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl text-sm font-medium bg-zinc-800/50 text-zinc-600 cursor-not-allowed">
                                    Coming soon
                                </span>
                            `}
                        </div>
                    `;
                }).join('');
            })();

            function formatSize(bytes) {
                if (bytes > 1e9) return (bytes / 1e9).toFixed(1) + ' GB';
                if (bytes > 1e6) return (bytes / 1e6).toFixed(1) + ' MB';
                return (bytes / 1e3).toFixed(0) + ' KB';
            }
            "##
        </script>

        // ── Particle canvas + anime.js entrance/scroll reveals ──
        <script>
            r#"
            // Particle canvas
            (function() {
                const canvas = document.getElementById('hero-canvas');
                if (!canvas) return;
                const ctx = canvas.getContext('2d');
                let w, h, particles = [], mouse = { x: -1000, y: -1000 };

                function resize() {
                    w = canvas.width = canvas.offsetWidth;
                    h = canvas.height = canvas.offsetHeight;
                }
                resize();
                window.addEventListener('resize', resize);

                canvas.addEventListener('mousemove', e => {
                    const rect = canvas.getBoundingClientRect();
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

            // anime.js animations
            (function() {
                if (typeof anime === 'undefined') return;

                anime.timeline({ easing: 'easeOutExpo' })
                    .add({ targets: '.dl-hero-badge', opacity: [0,1], translateY: [12,0], duration: 700 })
                    .add({ targets: '.dl-hero-title', opacity: [0,1], translateY: [40,0], duration: 1100 }, '-=500')
                    .add({ targets: '.dl-hero-sub', opacity: [0,1], translateY: [20,0], delay: anime.stagger(120), duration: 800 }, '-=800')
                    .add({ targets: '.dl-hero-cta', opacity: [0,1], translateY: [20,0], scale: [0.9,1], delay: anime.stagger(100), duration: 600 }, '-=500');

                function onReveal(selector, animProps, childSel) {
                    const obs = new IntersectionObserver((entries) => {
                        entries.forEach(entry => {
                            if (entry.isIntersecting) {
                                const t = childSel ? entry.target.querySelectorAll(childSel) : entry.target;
                                anime({ targets: t, ...animProps });
                                obs.unobserve(entry.target);
                            }
                        });
                    }, { threshold: 0.1, rootMargin: '0px 0px -40px 0px' });
                    document.querySelectorAll(selector).forEach(el => obs.observe(el));
                }

                onReveal('.dl-editor-reveal', { opacity: [0,1], translateY: [60,0], scale: [0.94,1], duration: 1000, easing: 'easeOutCubic' });
                onReveal('.dl-install', { opacity: [0,1], translateY: [30,0], duration: 800, easing: 'easeOutCubic' });
                onReveal('.dl-feature-grid', { opacity: [0,1], translateY: [50,0], scale: [0.85,1], delay: anime.stagger(70, {from: 'first'}), duration: 800, easing: 'easeOutElastic(1, 0.6)' }, '.feature-card');
                onReveal('.dl-showcase', { opacity: [0,1], translateY: [40,0], scale: [0.95,1], delay: anime.stagger(80), duration: 700, easing: 'easeOutCubic' }, '.dl-shot');
                onReveal('.dl-options', { opacity: [0,1], translateY: [20,0], delay: anime.stagger(120), duration: 700, easing: 'easeOutCubic' }, 'a');

                const counterObs = new IntersectionObserver((entries) => {
                    entries.forEach(entry => {
                        if (entry.isIntersecting) {
                            const el = entry.target;
                            const target = parseInt(el.dataset.target);
                            if (!target) return;
                            const prefix = el.dataset.prefix || '';
                            const suffix = el.dataset.suffix || '';
                            const obj = { val: 0 };
                            anime({
                                targets: obj, val: target, round: 1, duration: 1500, easing: 'easeOutExpo',
                                update: () => { el.textContent = prefix + obj.val + suffix; }
                            });
                            anime({ targets: el.closest('.dl-stat'), scale: [0.85, 1], opacity: [0, 1], duration: 600, easing: 'easeOutBack' });
                            counterObs.unobserve(el);
                        }
                    });
                }, { threshold: 0.5 });
                document.querySelectorAll('.dl-counter').forEach(el => counterObs.observe(el));

                document.querySelectorAll('.feature-card').forEach(card => {
                    card.addEventListener('mouseenter', () => { anime({ targets: card, scale: 1.03, duration: 200, easing: 'easeOutQuad' }); });
                    card.addEventListener('mouseleave', () => { anime({ targets: card, scale: 1, duration: 400, easing: 'easeOutElastic(1, 0.5)' }); });
                });
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
                opacity: 0;
            }
            @keyframes dl-shimmer {
                0%, 100% { background-position: 0% 50%; }
                50% { background-position: 100% 50%; }
            }

            /* Initial hidden states — anime.js reveals these */
            .dl-hero-badge { opacity: 0; }
            .dl-hero-sub { opacity: 0; }
            .dl-hero-cta { opacity: 0; }
            .dl-editor-reveal { opacity: 0; }
            .dl-install { opacity: 0; }
            .feature-card { opacity: 0; }
            .dl-shot { opacity: 0; }
            .dl-options a { opacity: 0; }

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

#[component]
fn FeatureCard(icon: &'static str, title: &'static str, description: &'static str, color: &'static str) -> impl IntoView {
    let icon_class = format!("ph {} text-2xl", icon);
    let icon_wrap_class = format!("w-10 h-10 rounded-xl flex items-center justify-center icon-{}", color);
    let card_class = format!("feature-card glow-{} relative p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl hover:border-zinc-700 transition-all group", color);
    view! {
        <div class=card_class>
            <div class=icon_wrap_class>
                <i class=icon_class></i>
            </div>
            <h3 class="text-sm font-semibold mt-3 mb-1">{title}</h3>
            <p class="text-xs text-zinc-500 leading-relaxed">{description}</p>
        </div>
    }
}

#[component]
fn ShotCard(img: &'static str, title: &'static str, caption: &'static str) -> impl IntoView {
    let src = format!("/assets/previews/{}.png", img);
    view! {
        <div class="dl-shot group relative rounded-xl overflow-hidden border border-zinc-800/50 bg-white/[0.02] hover:border-zinc-700 transition-all">
            <div class="relative overflow-hidden aspect-video bg-black/40">
                <img src=src alt=title loading="lazy" data-zoom="1" class="w-full h-full object-cover group-hover:scale-[1.03] transition-transform duration-500" />
            </div>
            <div class="p-4">
                <h3 class="text-sm font-semibold mb-1">{title}</h3>
                <p class="text-xs text-zinc-500 leading-relaxed">{caption}</p>
            </div>
        </div>
    }
}

#[component]
fn DownloadSkeleton() -> impl IntoView {
    view! {
        <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-xl animate-pulse">
            <div class="w-14 h-14 bg-zinc-800/50 rounded-2xl mx-auto mb-3"></div>
            <div class="h-4 w-20 bg-zinc-800/50 rounded mx-auto mb-2"></div>
            <div class="h-3 w-32 bg-zinc-800/50 rounded mx-auto mb-4"></div>
            <div class="h-10 bg-zinc-800/50 rounded-xl"></div>
        </div>
    }
}
