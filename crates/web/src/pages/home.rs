use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        // ── Hero section with animated particle canvas ──
        <section class="relative min-h-[88vh] flex items-start justify-center overflow-hidden -mt-14 pt-36">
            // Animated background
            <canvas id="hero-canvas" class="absolute inset-0 w-full h-full"></canvas>

            // Glow orbs
            <div class="absolute top-1/4 left-1/4 w-96 h-96 bg-accent/20 rounded-full blur-[128px] animate-pulse pointer-events-none"></div>
            <div class="absolute bottom-1/4 right-1/4 w-80 h-80 bg-purple-600/15 rounded-full blur-[100px] pointer-events-none" style="animation: pulse 4s ease-in-out infinite 1s"></div>

            <div class="hero-content relative z-10 text-center px-6 max-w-3xl mx-auto">
                // Version badge
                <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-accent/10 border border-accent/20 text-accent text-xs font-medium mb-4 backdrop-blur-sm">
                    <span class="w-1.5 h-1.5 rounded-full bg-accent animate-pulse"></span>
                    "r1-alpha5 — Early Access"
                </div>

                <h1 class="text-6xl md:text-7xl lg:text-8xl font-extrabold tracking-tight leading-[1.05]">
                    <span class="hero-title">"Renzora Engine"</span>
                </h1>
                <p class="mt-4 text-sm text-zinc-500 uppercase tracking-widest font-medium">"Powered by Rust & Bevy 0.18"</p>
                <p class="mt-5 text-lg md:text-xl text-zinc-300 leading-relaxed max-w-2xl mx-auto">
                    "The first fully-featured game engine built on Bevy — a complete visual editor, "
                    "scripting, physics and real-time rendering, engineered in Rust and "
                    <span class="text-zinc-100 font-medium">"fully open source."</span>
                </p>

                <div class="cta mt-10 flex gap-3 justify-center flex-wrap">
                    <a href="/download" class="group relative inline-flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_30px_rgba(99,102,241,0.3)] hover:scale-[1.02]">
                        <i class="ph ph-download-simple text-lg"></i>"Download Engine"
                        <span class="absolute inset-0 rounded-xl bg-white/10 opacity-0 group-hover:opacity-100 transition-opacity"></span>
                    </a>
                    <a href="/docs" class="inline-flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-semibold bg-white/5 text-zinc-50 border border-zinc-700/50 hover:border-zinc-500 hover:bg-white/10 transition-all backdrop-blur-sm">
                        <i class="ph ph-book-open text-lg"></i>"Documentation"
                    </a>
                    <a href="https://github.com/renzora/engine" target="_blank" rel="noopener noreferrer" class="inline-flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-semibold bg-white/5 text-zinc-50 border border-zinc-700/50 hover:border-zinc-500 hover:bg-white/10 transition-all backdrop-blur-sm">
                        <i class="ph ph-github-logo text-lg"></i>"Source"
                    </a>
                </div>

                // Tech badges
                <div class="mt-8 flex flex-wrap items-center justify-center gap-x-5 gap-y-2 text-xs text-zinc-500">
                    <span class="inline-flex items-center gap-1.5"><i class="ph ph-git-branch text-accent"></i>"MIT / Apache 2.0"</span>
                    <span class="inline-flex items-center gap-1.5"><i class="ph ph-stack text-accent"></i>"~187 workspace crates"</span>
                    <span class="inline-flex items-center gap-1.5"><i class="ph ph-code text-accent"></i>"Lua · Rhai · Blueprints"</span>
                    <span class="inline-flex items-center gap-1.5"><i class="ph ph-devices text-accent"></i>"6 export platforms"</span>
                </div>
            </div>

            // Scroll indicator
            <div class="absolute bottom-8 left-1/2 -translate-x-1/2 z-10">
                <div class="w-5 h-8 rounded-full border-2 border-zinc-600 flex justify-center pt-1.5">
                    <div class="w-1 h-2 rounded-full bg-zinc-500 scroll-dot"></div>
                </div>
            </div>
        </section>

        // ── Hero editor screenshot ──
        <section class="relative -mt-16 pb-24 w-full overflow-hidden">
            <div class="max-w-[1200px] mx-auto px-6">
                <div class="relative rounded-2xl overflow-hidden border border-zinc-800/60 shadow-2xl shadow-black/60 editor-reveal">
                    <div class="absolute inset-0 bg-gradient-to-t from-surface-panel via-transparent to-transparent z-10 pointer-events-none"></div>
                    <div class="absolute inset-0 ring-1 ring-inset ring-white/5 rounded-2xl z-10 pointer-events-none"></div>
                    <img src="/assets/previews/interface.png" alt="The Renzora editor with a Times Square scene, hierarchy, inspector and asset browser" class="w-full h-auto block" loading="lazy" data-zoom="1" />
                </div>
                <p class="text-center text-sm text-zinc-500 mt-4">"Build real-time 3D worlds in one professional editor — hierarchy, transform gizmos, a reflection-driven inspector and a live performance panel, side by side."</p>
            </div>
        </section>

        // ── At-a-glance pillars ──
        <section class="pb-24">
            <div class="max-w-[1200px] mx-auto px-6">
                <div class="text-center mb-14">
                    <span class="text-xs font-semibold uppercase tracking-widest text-accent">"Everything in one editor"</span>
                    <h2 class="text-3xl md:text-4xl font-bold mt-3">"Eight systems, one engine"</h2>
                    <p class="text-zinc-500 mt-3 text-base max-w-xl mx-auto">"Not a thin wrapper — a full production toolkit, with almost every feature shipping as its own plugin."</p>
                </div>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 feature-grid">
                    <FeatureCard icon="ph-stack" title="Scene Editor" description="Dockable panels, nested hierarchy, and a reflection-driven inspector with custom components." color="indigo" />
                    <FeatureCard icon="ph-code" title="Scripting" description="Lua 5.4 and Rhai chosen by file extension, plus visual Blueprint node graphs." color="violet" />
                    <FeatureCard icon="ph-puzzle-piece" title="Plugin System" description="Almost every feature is a plugin — hot-load cdylibs into plugins/ with renzora::add!." color="cyan" />
                    <FeatureCard icon="ph-atom" title="Physics" description="Rigid bodies, colliders and queries powered by the Avian physics engine." color="emerald" />
                    <FeatureCard icon="ph-paint-brush-broad" title="Materials & Shaders" description="Node-based PBR material graph, custom WGSL, and 50+ post-process effects." color="sky" />
                    <FeatureCard icon="ph-browsers" title="Ember UI" description="Build game and editor UI from .html templates with reactive bindings." color="rose" />
                    <FeatureCard icon="ph-devices" title="Cross-Platform" description="Export to Windows, Linux, macOS, Android, iOS and the Web." color="amber" />
                    <FeatureCard icon="ph-bug" title="Debugging" description="An 11-panel debugger — profiler, memory, ECS, render and physics stats." color="orange" />
                </div>
            </div>
        </section>

        // ── Deep-dive feature rows ──
        <section class="pb-8">
            <div class="max-w-[1200px] mx-auto px-6">
                <div class="text-center max-w-2xl mx-auto mb-20">
                    <span class="text-xs font-semibold uppercase tracking-widest text-accent">"Under the hood"</span>
                    <h2 class="text-3xl md:text-4xl font-bold mt-3">"A complete engine, engineered in Rust"</h2>
                    <p class="text-zinc-400 mt-4">"No glue scripts, no bolted-on runtimes. Every system below ships today, built on Rust and Bevy 0.18."</p>
                </div>

                <FeatureRow
                    color="indigo"
                    icon="ph-stack"
                    eyebrow="Scene Editor"
                    title="A reflection-driven editor that understands your types"
                    body="Compose scenes across up to four viewports with transform gizmos, organize everything in a nested hierarchy, and edit any component in an inspector generated straight from your Rust types. Derive Inspectable on your own structs and they show up — fully editable and serialized with the scene."
                    chips=vec!["Custom components", "Up to 4 viewports", "Transform gizmos"]
                    img="/assets/previews/inspector.png"
                    alt="The inspector showing a World Environment with transform, directional light, volumetric light and TAA components"
                    caption="The inspector stacks a World Environment's components — transform, directional light, volumetric god rays and TAA."
                    reversed=false
                />

                <FeatureRow
                    color="violet"
                    icon="ph-sun"
                    eyebrow="Real-Time Rendering"
                    title="Bevy's renderer, tuned for beautiful real-time scenes"
                    body="Physically based shading, dynamic lighting and Lumen global illumination render your worlds as you build them. Light a moody neon cafe, a rain-slick street or a daytime cityscape and iterate live in the viewport — what you see is what ships."
                    chips=vec!["PBR shading", "Lumen GI", "Live viewport"]
                    img="/assets/previews/viewport.png"
                    alt="A cinematic render of a Parisian cafe street with a blue scooter selected"
                    caption="A Parisian cafe street rendered in the viewport, with a single scooter selected and warm atmospheric lighting."
                    reversed=true
                />

                <FeatureRow
                    color="cyan"
                    icon="ph-code"
                    eyebrow="Scripting"
                    title="Script in Lua, Rhai or visual Blueprints"
                    body="Write gameplay logic in a built-in editor with full syntax highlighting. Renzora picks the runtime from the file extension — Lua 5.4 or Rhai — and visual Blueprint node graphs cover the same ground without code. Hook into lifecycle callbacks like on_update and drive entities directly."
                    chips=vec!["Lua 5.4", "Rhai", "Blueprint graphs"]
                    img="/assets/previews/code_editor.png"
                    alt="The built-in code editor with several Lua scripts open, showing car_physics.lua"
                    caption="The built-in editor with several Lua scripts open — car_physics.lua handling steering, throttle, brake and handbrake input."
                    reversed=false
                />

                <FeatureRow
                    color="emerald"
                    icon="ph-puzzle-piece"
                    eyebrow="Plugin Architecture"
                    title="Almost everything is a plugin"
                    body="Renzora is built from roughly 187 workspace crates, and nearly every feature — from the material graph to the audio mixer — ships as its own plugin. Distribution plugins are hot-loadable cdylibs: drop one into plugins/, register it with renzora::add!, and it appears in the editor."
                    chips=vec!["~187 crates", "Hot-loadable cdylibs", "renzora::add!"]
                    img="/assets/previews/panels.png"
                    alt="The Add Panel browser listing dockable panels grouped by category"
                    caption="The Add Panel browser — dockable panels grouped by Blueprint, Debug, Audio, Material, Particle, Scripting, Shader, Terrain and more."
                    reversed=true
                />

                <FeatureRow
                    color="amber"
                    icon="ph-atom"
                    eyebrow="Physics & Worlds"
                    title="Populate your world, then bring it to life with Avian"
                    body="Spawn lights, cameras, terrain, splines and 2D nodes from one searchable Add Entity menu — physics bodies included. The Avian physics engine drives rigid bodies, colliders and queries, with a dedicated physics debug view for when you need to see the simulation."
                    chips=vec!["Avian physics", "Rigid bodies & colliders", "Searchable palette"]
                    img="/assets/previews/add_entity.png"
                    alt="The Add Entity menu with a category sidebar and a list of entity types including physics"
                    caption="The Add Entity menu — lights, cameras, terrain, 2D nodes and physics, all in one searchable list."
                    reversed=false
                />

                <FeatureRow
                    color="sky"
                    icon="ph-paint-brush-broad"
                    eyebrow="Materials & Shaders"
                    title="Author PBR materials as a graph — or drop down to WGSL"
                    body="Wire texture, normal-map and math nodes into a Surface Output exposing base color, metallic, roughness, normal, emissive, AO, clearcoat, anisotropy and more. Need full control? Write custom WGSL shaders and stack over fifty post-process effects on top."
                    chips=vec!["Node graph", "Custom WGSL", "50+ post effects"]
                    img="/assets/previews/material_graph.png"
                    alt="A node-based material editor wiring texture and normal-map nodes into a PBR surface output"
                    caption="Sample Texture and Sample Normal Map nodes wired into a full PBR Surface Output."
                    reversed=true
                />

                <FeatureRow
                    color="rose"
                    icon="ph-browsers"
                    eyebrow="Ember UI"
                    title="Markup-driven UI for your game and the editor itself"
                    body="Renzora's Ember system builds interfaces from .html templates with reactive double-brace bindings — the same system powers in-game screens and the editor's own panels. Design a match lobby or a HUD visually, point it at an HTML template and a UI layout, then bind it to scripts."
                    chips=vec![".html templates", "Reactive bindings", "Game + editor UI"]
                    img="/assets/previews/ui.png"
                    alt="The in-engine UI builder editing a match-lobby screen with HTML Template fields in the inspector"
                    caption="The in-engine UI builder editing a match-lobby screen, with HTML Template and UI Layout exposed in the inspector."
                    reversed=false
                />

                <FeatureRow
                    color="orange"
                    icon="ph-bug"
                    eyebrow="Debugging & Profiling"
                    title="Eleven debug panels, a console and a command palette"
                    body="Profile in real time with FPS, frame timing, memory and render stats, then dig into ECS stats, physics and culling debug, Lumen GI and scripting diagnostics — eleven panels in all. A filterable console with categorized logs and a command palette round out the toolkit."
                    chips=vec!["11 debug panels", "Live graphs", "Console & palette"]
                    img="/assets/previews/debugging.png"
                    alt="The editor with a dense row of diagnostic panels: performance, system, render stats, memory and physics debug"
                    caption="Live diagnostics docked across the bottom — Performance, System, Render Stats, Memory, Physics and Camera debug, with graphs."
                    reversed=true
                />
            </div>
        </section>

        // ── Cross-platform export band ──
        <section class="pb-24">
            <div class="max-w-[1200px] mx-auto px-6">
                <div class="relative overflow-hidden rounded-2xl border border-zinc-800/60 p-10 md:p-14">
                    <div class="absolute inset-0 bg-gradient-to-br from-accent/10 via-transparent to-purple-600/10 pointer-events-none"></div>
                    <div class="absolute -top-24 left-1/2 -translate-x-1/2 w-96 h-48 bg-accent/15 rounded-full blur-[100px] pointer-events-none"></div>
                    <div class="relative z-10">
                        <div class="text-center max-w-xl mx-auto mb-10">
                            <span class="text-xs font-semibold uppercase tracking-widest text-accent">"Cross-Platform Export"</span>
                            <h2 class="text-3xl md:text-4xl font-bold mt-3">"One project. Six platforms."</h2>
                            <p class="text-zinc-400 mt-4">"Build once and export to desktop, mobile and the browser — Windows, Linux, macOS, Android, iOS and the Web via WebAssembly."</p>
                        </div>
                        <div class="platform-grid grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-3">
                            <PlatformTile icon="ph-windows-logo" name="Windows" />
                            <PlatformTile icon="ph-linux-logo" name="Linux" />
                            <PlatformTile icon="ph-apple-logo" name="macOS" />
                            <PlatformTile icon="ph-android-logo" name="Android" />
                            <PlatformTile icon="ph-device-mobile" name="iOS" />
                            <PlatformTile icon="ph-globe" name="Web (WASM)" />
                        </div>
                    </div>
                </div>
            </div>
        </section>

        // ── Screenshot gallery ──
        <section class="pb-24">
            <div class="max-w-[1200px] mx-auto px-6">
                <div class="flex items-end justify-between mb-8 flex-wrap gap-3">
                    <div>
                        <span class="text-xs font-semibold uppercase tracking-widest text-accent">"Inside the editor"</span>
                        <h2 class="text-2xl md:text-3xl font-bold mt-2">"More of the toolkit"</h2>
                    </div>
                    <p class="text-sm text-zinc-500 max-w-sm">"Hierarchy, audio mixing, the Hub Store, the asset browser and the console — every panel is real and dockable."</p>
                </div>
                <div class="gallery-grid grid grid-cols-2 lg:grid-cols-3 gap-4">
                    <GalleryShot img="/assets/previews/hierarchy.png" label="Scene hierarchy with nested glTF imports and per-object visibility." />
                    <GalleryShot img="/assets/previews/renzora_ember.png" label="The Ember UI toolkit: charts, inputs, timelines and inspector widgets." />
                    <GalleryShot img="/assets/previews/mixer.png" label="An audio mixer with per-bus faders, pan, level meters and solo." />
                    <GalleryShot img="/assets/previews/marketplace.png" label="The built-in Hub Store with free models and scripts to import." />
                    <GalleryShot img="/assets/previews/console.png" label="A filterable console with categorized logs and a command bar." />
                    <GalleryShot img="/assets/previews/assets_panel.png" label="A color-coded asset browser keeps your project organized." />
                </div>
            </div>
        </section>

        // ── Honest stats strip ──
        <section class="pb-24">
            <div class="max-w-[1200px] mx-auto px-6">
                <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
                    <StatCard target="187" suffix="+" label="Workspace Crates" />
                    <StatCard target="6" suffix="" label="Export Platforms" />
                    <StatCard target="3" suffix="" label="Ways to Script" />
                    <StatCard target="11" suffix="" label="Debug Panels" />
                    <StatCard target="4" suffix="" label="Max Viewports" />
                    <StatCard target="50" suffix="+" label="Post Effects" />
                </div>
            </div>
        </section>

        // ── Explore ──
        <section class="pb-24">
            <div class="max-w-[1200px] mx-auto px-6">
                <h2 class="text-lg font-semibold mb-5">"Explore"</h2>
                <div class="explore-grid grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                    <ExploreCard icon="ph-storefront" name="Hub Store" desc="Free models & scripts" href="/marketplace" />
                    <ExploreCard icon="ph-book-open" name="Documentation" desc="Guides & API reference" href="/docs" />
                    <ExploreCard icon="ph-download-simple" name="Download" desc="Get the engine" href="/download" />
                    <ExploreCard icon="ph-users-three" name="Community" desc="Devlogs & discussion" href="/community" />
                </div>
            </div>
        </section>

        // ── Closing CTA ──
        <section class="pb-24">
            <div class="max-w-[1200px] mx-auto px-6">
                <div class="cta-section relative overflow-hidden text-center p-16 rounded-2xl border border-zinc-800/50">
                    <div class="absolute inset-0 bg-gradient-to-br from-accent/5 via-transparent to-purple-600/5"></div>
                    <div class="absolute top-0 left-1/2 -translate-x-1/2 w-64 h-px bg-gradient-to-r from-transparent via-accent/50 to-transparent"></div>
                    <div class="relative z-10">
                        <h2 class="text-3xl md:text-4xl font-bold">"Build it in Renzora"</h2>
                        <p class="text-zinc-400 mt-3 mb-8 text-base max-w-md mx-auto">"Download the open-source engine and spin up your first scene in minutes."</p>
                        <div class="flex gap-3 justify-center flex-wrap">
                            <a href="/download" class="group relative inline-flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_30px_rgba(99,102,241,0.3)]">
                                <i class="ph ph-download-simple text-lg"></i>"Download Engine"
                            </a>
                            <a href="/docs/getting-started/installation" class="inline-flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-semibold bg-white/5 text-zinc-50 border border-zinc-700/50 hover:border-zinc-500 transition-all">
                                <i class="ph ph-rocket-launch text-lg"></i>"Getting Started"
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <script>
            r#"
            // ── Particle canvas ──
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

                        // Mouse repulsion
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

                        // Draw connections
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

            // ── anime.js powered animations ──
            (function() {
                // Hero entrance — staggered fade up
                anime.timeline({ easing: 'easeOutExpo' })
                    .add({ targets: '.hero-title', opacity: [0,1], translateY: [40,0], duration: 1200 })
                    .add({ targets: '.hero-content p', opacity: [0,1], translateY: [20,0], duration: 800 }, '-=800')
                    .add({ targets: '.hero-content .cta a', opacity: [0,1], translateY: [20,0], scale: [0.9,1], delay: anime.stagger(100), duration: 600 }, '-=500')
                    .add({ targets: '.scroll-dot', opacity: [0,1], duration: 600 }, '-=300');

                // Scroll observer using anime.js
                function onReveal(selector, animProps, childSelector) {
                    const obs = new IntersectionObserver((entries) => {
                        entries.forEach(entry => {
                            if (entry.isIntersecting) {
                                if (childSelector) {
                                    anime({ targets: entry.target.querySelectorAll(childSelector), ...animProps });
                                } else {
                                    anime({ targets: entry.target, ...animProps });
                                }
                                obs.unobserve(entry.target);
                            }
                        });
                    }, { threshold: 0.1, rootMargin: '0px 0px -40px 0px' });
                    document.querySelectorAll(selector).forEach(el => obs.observe(el));
                }

                // Pillar cards — elastic stagger
                onReveal('.feature-grid', {
                    opacity: [0,1],
                    translateY: [50,0],
                    scale: [0.85,1],
                    delay: anime.stagger(70, {from: 'first'}),
                    duration: 800,
                    easing: 'easeOutElastic(1, 0.6)'
                }, '.feature-card');

                // Hero editor screenshot — dramatic scale up
                onReveal('.editor-reveal', {
                    opacity: [0,1],
                    translateY: [60,0],
                    scale: [0.94,1],
                    duration: 1000,
                    easing: 'easeOutCubic'
                });

                // Feature rows — slide the two columns up
                onReveal('.feature-row', {
                    opacity: [0,1],
                    translateY: [60,0],
                    scale: [0.97,1],
                    delay: anime.stagger(140),
                    duration: 900,
                    easing: 'easeOutCubic'
                }, ':scope > div');

                // Platform tiles — bounce in
                onReveal('.platform-grid', {
                    opacity: [0,1],
                    translateY: [30,0],
                    scale: [0.8,1],
                    delay: anime.stagger(70),
                    duration: 600,
                    easing: 'easeOutBack'
                }, '.platform-tile');

                // Gallery — pop in
                onReveal('.gallery-grid', {
                    opacity: [0,1],
                    translateY: [40,0],
                    scale: [0.92,1],
                    delay: anime.stagger(70),
                    duration: 700,
                    easing: 'easeOutCubic'
                }, '.gallery-item');

                // Stats counters — animated numbers + bounce in
                const counterObs = new IntersectionObserver((entries) => {
                    entries.forEach(entry => {
                        if (entry.isIntersecting) {
                            const el = entry.target;
                            const target = parseInt(el.dataset.target);
                            if (!target) return;
                            const suffix = el.dataset.suffix || '';
                            const obj = { val: 0 };
                            anime({
                                targets: obj,
                                val: target,
                                round: 1,
                                duration: 1500,
                                easing: 'easeOutExpo',
                                update: () => { el.textContent = obj.val + suffix; }
                            });
                            anime({
                                targets: el.closest('.rounded-xl'),
                                scale: [0.8, 1],
                                opacity: [0, 1],
                                duration: 600,
                                easing: 'easeOutBack'
                            });
                            counterObs.unobserve(el);
                        }
                    });
                }, { threshold: 0.5 });
                document.querySelectorAll('.counter').forEach(el => counterObs.observe(el));

                // Explore cards — slide in from left
                onReveal('.explore-grid', {
                    opacity: [0,1],
                    translateX: [-40,0],
                    delay: anime.stagger(100),
                    duration: 700,
                    easing: 'easeOutCubic'
                }, 'a');

                // CTA section — fade up
                onReveal('.cta-section', {
                    opacity: [0,1],
                    translateY: [30,0],
                    duration: 800,
                    easing: 'easeOutCubic'
                });

                // Pillar cards — playful hover tilt
                document.querySelectorAll('.feature-card').forEach(card => {
                    card.addEventListener('mouseenter', () => {
                        anime({ targets: card, scale: 1.03, duration: 200, easing: 'easeOutQuad' });
                    });
                    card.addEventListener('mouseleave', () => {
                        anime({ targets: card, scale: 1, duration: 400, easing: 'easeOutElastic(1, 0.5)' });
                    });
                });

                // Explore cards — bounce icon on hover
                document.querySelectorAll('.explore-card').forEach(card => {
                    card.addEventListener('mouseenter', () => {
                        anime({ targets: card.querySelector('.shrink-0'), scale: 1.15, rotate: '8deg', duration: 300, easing: 'easeOutBack' });
                    });
                    card.addEventListener('mouseleave', () => {
                        anime({ targets: card.querySelector('.shrink-0'), scale: 1, rotate: '0deg', duration: 500, easing: 'easeOutElastic(1, 0.4)' });
                    });
                });

                // Download buttons — pulse glow
                document.querySelectorAll('a[href="/download"]').forEach(btn => {
                    anime({
                        targets: btn,
                        boxShadow: ['0 0 0px rgba(99,102,241,0)', '0 0 25px rgba(99,102,241,0.3)', '0 0 0px rgba(99,102,241,0)'],
                        duration: 2500,
                        loop: true,
                        easing: 'easeInOutSine'
                    });
                });
            })();
            "#
        </script>

        <style>
            r#"
            /* Hero title shimmer */
            .hero-title {
                background: linear-gradient(
                    135deg,
                    #fafafa 0%,
                    #6366f1 40%,
                    #a78bfa 60%,
                    #fafafa 100%
                );
                background-size: 300% 300%;
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
                background-clip: text;
                animation: shimmer 6s ease-in-out infinite;
            }
            @keyframes shimmer {
                0%, 100% { background-position: 0% 50%; }
                50% { background-position: 100% 50%; }
            }

            /* Scroll indicator bounce */
            .scroll-dot {
                animation: scrollBounce 2s ease-in-out infinite;
            }
            @keyframes scrollBounce {
                0%, 100% { transform: translateY(0); opacity: 1; }
                50% { transform: translateY(6px); opacity: 0.3; }
            }

            /* Initial hidden states — anime.js reveals these */
            .feature-card { opacity: 0; }
            .editor-reveal { opacity: 0; }
            .hero-title { opacity: 0; }
            .hero-content p { opacity: 0; }
            .hero-content .cta a { opacity: 0; }
            .feature-row > div { opacity: 0; }
            .platform-tile { opacity: 0; }
            .gallery-item { opacity: 0; }

            /* Pillar card glow on hover */
            .feature-card::before {
                content: '';
                position: absolute;
                inset: 0;
                border-radius: 0.75rem;
                opacity: 0;
                transition: opacity 0.3s;
                pointer-events: none;
            }
            .feature-card:hover::before {
                opacity: 1;
            }
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
fn FeatureRow(
    color: &'static str,
    icon: &'static str,
    eyebrow: &'static str,
    title: &'static str,
    body: &'static str,
    chips: Vec<&'static str>,
    img: &'static str,
    alt: &'static str,
    caption: &'static str,
    reversed: bool,
) -> impl IntoView {
    let icon_class = format!("ph {} text-lg", icon);
    let icon_wrap_class = format!("w-9 h-9 rounded-lg flex items-center justify-center icon-{}", color);
    let text_order = if reversed { "lg:order-2" } else { "lg:order-1" };
    let media_order = if reversed { "lg:order-1" } else { "lg:order-2" };
    let text_col_class = format!("fr-text {}", text_order);
    let media_col_class = format!("fr-media {}", media_order);
    view! {
        <div class="feature-row grid lg:grid-cols-2 gap-10 lg:gap-14 items-center mb-20 lg:mb-28">
            <div class=text_col_class>
                <div class="inline-flex items-center gap-2 mb-4">
                    <div class=icon_wrap_class>
                        <i class=icon_class></i>
                    </div>
                    <span class="text-xs font-semibold uppercase tracking-widest text-zinc-400">{eyebrow}</span>
                </div>
                <h2 class="text-2xl md:text-3xl font-bold tracking-tight leading-tight">{title}</h2>
                <p class="text-zinc-400 mt-4 text-base leading-relaxed">{body}</p>
                <div class="flex flex-wrap gap-2 mt-6">
                    {chips.into_iter().map(|c| view! {
                        <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-white/[0.03] border border-zinc-800 text-[11px] font-medium text-zinc-300">
                            <span class="w-1 h-1 rounded-full bg-accent"></span>{c}
                        </span>
                    }).collect_view()}
                </div>
            </div>
            <div class=media_col_class>
                <div class="group relative rounded-2xl overflow-hidden border border-zinc-800/60 bg-surface-card shadow-2xl shadow-black/40">
                    <div class="pointer-events-none absolute inset-0 bg-gradient-to-tr from-indigo-500/10 via-transparent to-purple-600/10 z-10"></div>
                    <div class="pointer-events-none absolute inset-0 ring-1 ring-inset ring-white/5 rounded-2xl z-10"></div>
                    <img src=img alt=alt loading="lazy" data-zoom="1" class="relative w-full h-auto block transition-transform duration-500 group-hover:scale-[1.02]" />
                </div>
                <p class="mt-3 text-xs text-zinc-500 italic leading-relaxed">{caption}</p>
            </div>
        </div>
    }
}

#[component]
fn PlatformTile(icon: &'static str, name: &'static str) -> impl IntoView {
    let icon_class = format!("ph {} text-3xl text-accent", icon);
    view! {
        <div class="platform-tile flex flex-col items-center gap-3 p-6 rounded-xl bg-white/[0.02] border border-zinc-800/50 hover:border-accent/40 hover:bg-white/[0.04] transition-all">
            <i class=icon_class></i>
            <span class="text-sm font-medium text-zinc-200">{name}</span>
        </div>
    }
}

#[component]
fn GalleryShot(img: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <figure class="gallery-item group relative rounded-xl overflow-hidden border border-zinc-800/50 bg-surface-card">
            <img src=img alt=label loading="lazy" data-zoom="1" class="w-full h-44 object-cover object-top transition-transform duration-500 group-hover:scale-105" />
            <figcaption class="absolute inset-x-0 bottom-0 p-3 text-xs text-zinc-200 leading-snug bg-gradient-to-t from-black/85 via-black/50 to-transparent opacity-0 group-hover:opacity-100 transition-opacity">{label}</figcaption>
        </figure>
    }
}

#[component]
fn StatCard(target: &'static str, suffix: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <div class="text-center p-6 rounded-xl bg-white/[0.02] border border-zinc-800/50">
            <div class="text-3xl md:text-4xl font-bold text-accent counter" data-target=target data-suffix=suffix>"0"</div>
            <div class="text-xs text-zinc-500 mt-2 uppercase tracking-wider">{label}</div>
        </div>
    }
}

#[component]
fn ExploreCard(icon: &'static str, name: &'static str, desc: &'static str, href: &'static str) -> impl IntoView {
    let icon_class = format!("ph {} text-2xl text-accent", icon);
    view! {
        <a href=href class="explore-card flex items-center gap-4 p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl hover:border-zinc-600 hover:bg-white/[0.04] transition-all group">
            <div class="w-11 h-11 rounded-xl bg-accent/10 flex items-center justify-center shrink-0 group-hover:scale-110 transition-transform">
                <i class=icon_class></i>
            </div>
            <div>
                <h3 class="text-sm font-semibold group-hover:text-accent transition-colors">{name}</h3>
                <p class="text-[11px] text-zinc-500">{desc}</p>
            </div>
        </a>
    }
}
