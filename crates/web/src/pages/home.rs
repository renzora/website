use leptos::prelude::*;
use leptos_meta::{Title, Meta};

/// FAQ structured data (FAQPage). Mirrors the visible FAQ section below, the
/// answers target long-tail queries like "what is a Bevy editor" and "is
/// Renzora free", reinforcing relevance for the "Bevy editor" term.
const FAQ_JSONLD: &str = r#"{"@context":"https://schema.org","@type":"FAQPage","mainEntity":[{"@type":"Question","name":"What is Renzora?","acceptedAnswer":{"@type":"Answer","text":"Renzora is a free, open-source Bevy editor and game engine. It gives the Bevy ecosystem a full visual editor, scene tooling, an inspector, scripting, physics, real-time rendering and a marketplace, all built in Rust."}},{"@type":"Question","name":"Is Renzora free and open source?","acceptedAnswer":{"@type":"Answer","text":"Yes. Renzora is completely free and open source under the MIT and Apache-2.0 licenses. You can download it, read the source on GitHub, and extend it with plugins."}},{"@type":"Question","name":"What is a Bevy editor?","acceptedAnswer":{"@type":"Answer","text":"Bevy is a Rust game engine that ships as a code framework without an official visual editor. A Bevy editor like Renzora adds a graphical interface on top, letting you build scenes, edit components, write scripts and preview your game without writing all the boilerplate by hand."}},{"@type":"Question","name":"Which platforms can Renzora export to?","acceptedAnswer":{"@type":"Answer","text":"Renzora exports one project to six targets: Windows, macOS, Linux, Android, iOS and the web via WebAssembly."}},{"@type":"Question","name":"How do I script games in Renzora?","acceptedAnswer":{"@type":"Answer","text":"You can script gameplay in Lua 5.4 or Rhai, Renzora chooses the runtime by file extension, or use visual Blueprint node graphs. Nearly every feature is also a hot-loadable plugin."}}]}"#;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Renzora, The Open Source Bevy Editor" />
        <Meta name="description" content="Renzora is a free, open-source Bevy editor, a full 2D & 3D visual editor for Bevy with Lua & Rhai scripting, hot-loadable plugins, physics and real-time rendering. Built in Rust. Download for Windows, macOS, Linux and the web." />
        <div class="max-w-6xl mx-auto px-4 sm:px-6 py-6 space-y-12 sm:space-y-16">

            // ── Hero, gradient + ambient glows, no screenshot. The editor shots
            // live in the feature rows below (where they have context); keeping
            // the hero image-free makes the LCP a text element that paints at the
            // first render instead of waiting on an image decode.
            <section class="reveal relative overflow-hidden rounded-2xl border border-white/[0.08] min-h-[440px] flex items-center bg-surface-card">
                <div class="absolute inset-0 bg-gradient-to-br from-accent/15 via-transparent to-secondary/15 pointer-events-none"></div>
                <div class="absolute -top-24 left-1/4 w-96 h-96 rounded-full bg-accent/20 blur-[120px] pointer-events-none"></div>
                <div class="absolute -bottom-24 right-1/4 w-96 h-96 rounded-full bg-secondary/15 blur-[120px] pointer-events-none"></div>
                <div class="absolute top-0 left-1/2 -translate-x-1/2 w-2/3 h-px bg-gradient-to-r from-transparent via-accent/50 to-transparent"></div>
                <div class="relative z-10 p-8 sm:p-12 w-full">
                    <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-white/[0.08] border border-white/[0.12] text-xs font-medium text-zinc-100 backdrop-blur-sm">
                        <span class="w-1.5 h-1.5 rounded-full bg-secondary animate-pulse"></span>
                        "Open source · Rust · Bevy 0.19"
                    </div>
                    <h1 class="mt-5 text-4xl sm:text-5xl font-extrabold tracking-tight leading-[1.05] text-white drop-shadow-lg">
                        "The most complete "
                        <span class="bg-gradient-to-r from-accent to-secondary bg-clip-text text-transparent">"Bevy editor"</span>
                        ", fully open source."
                    </h1>
                    <p class="mt-5 text-base sm:text-lg text-zinc-200 leading-relaxed [text-shadow:0_1px_3px_rgba(0,0,0,0.5)]">
                        "Renzora is a free, open-source "
                        <span class="text-white font-medium">"Bevy editor"</span>
                        ", a complete 2D & 3D visual editor with Lua & Rhai scripting, physics and "
                        "real-time rendering, engineered in Rust on Bevy 0.19."
                    </p>
                    <div class="mt-8 flex flex-wrap gap-3">
                        <a href="/download" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl text-sm font-semibold bg-purple-600 text-white hover:bg-purple-500 transition-all hover:shadow-[0_0_30px_rgba(168,85,247,0.35)]">
                            <i class="ph ph-download-simple text-lg"></i>"Download the Engine"
                        </a>
                        <a href="/docs" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl text-sm font-semibold bg-white/[0.1] text-white border border-white/[0.15] hover:bg-white/[0.16] transition-all backdrop-blur-sm">
                            <i class="ph ph-book-open text-lg"></i>"Read the Docs"
                        </a>
                        <a href="https://github.com/renzora/engine" target="_blank" rel="noopener noreferrer" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl text-sm font-semibold bg-white/[0.1] text-white border border-white/[0.15] hover:bg-white/[0.16] transition-all backdrop-blur-sm">
                            <i class="ph ph-github-logo text-lg"></i>"Star on GitHub"
                        </a>
                    </div>
                    // Tech badges
                    <div class="mt-7 flex flex-wrap items-center gap-x-5 gap-y-2 text-xs text-zinc-300">
                        <span class="inline-flex items-center gap-1.5"><i class="ph ph-git-branch text-secondary"></i>"MIT / Apache 2.0"</span>
                        <span class="inline-flex items-center gap-1.5"><i class="ph ph-stack text-secondary"></i>"~187 workspace crates"</span>
                        <span class="inline-flex items-center gap-1.5"><i class="ph ph-code text-secondary"></i>"Lua · Rhai · Blueprints"</span>
                        <span class="inline-flex items-center gap-1.5"><i class="ph ph-devices text-secondary"></i>"6 export platforms"</span>
                    </div>
                </div>
            </section>

            // ── Stats strip ──
            <section class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-3 sm:gap-4">
                <StatCard target="187" suffix="+" label="Workspace crates" color="text-accent" />
                <StatCard target="6" suffix="" label="Export platforms" color="text-secondary" />
                <StatCard target="3" suffix="" label="Ways to script" color="text-emerald-400" />
                <StatCard target="11" suffix="" label="Debug panels" color="text-amber-400" />
                <StatCard target="4" suffix="" label="Max viewports" color="text-rose-400" />
                <StatCard target="50" suffix="+" label="Post effects" color="text-sky-400" />
            </section>

            // ── Eight systems, one engine (pillar cards) ──
            <section>
                <div class="text-center mb-10">
                    <span class="text-xs font-semibold uppercase tracking-widest text-accent">"Everything in one editor"</span>
                    <h2 class="text-2xl sm:text-3xl font-bold mt-2 text-white">"Eight systems, one engine"</h2>
                    <p class="text-zinc-500 mt-3 text-sm sm:text-base max-w-xl mx-auto">"Not a thin wrapper, a full production toolkit, with almost every feature shipping as its own plugin."</p>
                </div>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4">
                    <PillarCard icon="ph-stack" color="from-violet-500 to-purple-600" title="Scene Editor" desc="Dockable panels, nested hierarchy, and a reflection-driven inspector with custom components." />
                    <PillarCard icon="ph-code" color="from-fuchsia-500 to-violet-600" title="Scripting" desc="Lua 5.4 and Rhai chosen by file extension, plus visual Blueprint node graphs." />
                    <PillarCard icon="ph-puzzle-piece" color="from-cyan-400 to-sky-500" title="Plugin System" desc="Almost every feature is a plugin, hot-load cdylibs into plugins/ with renzora::add!." />
                    <PillarCard icon="ph-atom" color="from-emerald-400 to-teal-500" title="Physics" desc="Rigid bodies, colliders and queries powered by the Avian physics engine." />
                    <PillarCard icon="ph-paint-brush-broad" color="from-sky-400 to-blue-500" title="Materials & Shaders" desc="Node-based PBR material graph, custom WGSL, and 50+ post-process effects." />
                    <PillarCard icon="ph-browsers" color="from-rose-400 to-pink-500" title="Ember UI" desc="Build game and editor UI from .html templates with reactive bindings." />
                    <PillarCard icon="ph-devices" color="from-amber-400 to-orange-500" title="Cross-Platform" desc="Export to Windows, Linux, macOS, Android, iOS and the Web." />
                    <PillarCard icon="ph-bug" color="from-orange-400 to-red-500" title="Debugging" desc="An 11-panel debugger, profiler, memory, ECS, render and physics stats." />
                </div>
            </section>

            // ── Deep-dive feature rows ──
            <section>
                <div class="text-center max-w-2xl mx-auto mb-14">
                    <span class="text-xs font-semibold uppercase tracking-widest text-accent">"Under the hood"</span>
                    <h2 class="text-2xl sm:text-3xl font-bold mt-2 text-white">"A complete engine, engineered in Rust"</h2>
                    <p class="text-zinc-400 mt-3">"No glue scripts, no bolted-on runtimes. Every system below ships today, built on Rust and Bevy 0.19."</p>
                </div>

                <FeatureRow
                    color="violet"
                    icon="ph-stack"
                    eyebrow="Scene Editor"
                    title="A reflection-driven editor that understands your types"
                    body="Compose scenes across up to four viewports with transform gizmos, organize everything in a nested hierarchy, and edit any component in an inspector generated straight from your Rust types. Derive Inspectable on your own structs and they show up, fully editable and serialized with the scene."
                    chips=vec!["Custom components", "Up to 4 viewports", "Transform gizmos"]
                    img="/assets/previews/inspector.webp"
                    alt="The inspector showing a World Environment with transform, directional light, volumetric light and TAA components"
                    caption="The inspector stacks a World Environment's components, transform, directional light, volumetric god rays and TAA."
                    reversed=false
                />
                <FeatureRow
                    color="fuchsia"
                    icon="ph-sun"
                    eyebrow="Real-Time Rendering"
                    title="Bevy's renderer, tuned for beautiful real-time scenes"
                    body="Physically based shading, dynamic lighting and Lumen global illumination render your worlds as you build them. Light a moody neon cafe, a rain-slick street or a daytime cityscape and iterate live in the viewport, what you see is what ships."
                    chips=vec!["PBR shading", "Lumen GI", "Live viewport"]
                    img="/assets/previews/viewport.webp"
                    alt="A cinematic render of a Parisian cafe street with a blue scooter selected"
                    caption="A Parisian cafe street rendered in the viewport, with a single scooter selected and warm atmospheric lighting."
                    reversed=true
                />
                <FeatureRow
                    color="cyan"
                    icon="ph-code"
                    eyebrow="Scripting"
                    title="Script in Lua, Rhai or visual Blueprints"
                    body="Write gameplay logic in a built-in editor with full syntax highlighting. Renzora picks the runtime from the file extension, Lua 5.4 or Rhai, and visual Blueprint node graphs cover the same ground without code. Hook into lifecycle callbacks like on_update and drive entities directly."
                    chips=vec!["Lua 5.4", "Rhai", "Blueprint graphs"]
                    img="/assets/previews/code_editor.webp"
                    alt="The built-in code editor with several Lua scripts open, showing car_physics.lua"
                    caption="The built-in editor with several Lua scripts open, car_physics.lua handling steering, throttle, brake and handbrake input."
                    reversed=false
                />
                <FeatureRow
                    color="emerald"
                    icon="ph-puzzle-piece"
                    eyebrow="Plugin Architecture"
                    title="Almost everything is a plugin"
                    body="Renzora is built from roughly 187 workspace crates, and nearly every feature, from the material graph to the audio mixer, ships as its own plugin. Distribution plugins are hot-loadable cdylibs: drop one into plugins/, register it with renzora::add!, and it appears in the editor."
                    chips=vec!["~187 crates", "Hot-loadable cdylibs", "renzora::add!"]
                    img="/assets/previews/panels.webp"
                    alt="The Add Panel browser listing dockable panels grouped by category"
                    caption="The Add Panel browser, dockable panels grouped by Blueprint, Debug, Audio, Material, Particle, Scripting, Shader, Terrain and more."
                    reversed=true
                />
                <FeatureRow
                    color="amber"
                    icon="ph-atom"
                    eyebrow="Physics & Worlds"
                    title="Populate your world, then bring it to life with Avian"
                    body="Spawn lights, cameras, terrain, splines and 2D nodes from one searchable Add Entity menu, physics bodies included. The Avian physics engine drives rigid bodies, colliders and queries, with a dedicated physics debug view for when you need to see the simulation."
                    chips=vec!["Avian physics", "Rigid bodies & colliders", "Searchable palette"]
                    img="/assets/previews/add_entity.webp"
                    alt="The Add Entity menu with a category sidebar and a list of entity types including physics"
                    caption="The Add Entity menu, lights, cameras, terrain, 2D nodes and physics, all in one searchable list."
                    reversed=false
                />
                <FeatureRow
                    color="sky"
                    icon="ph-paint-brush-broad"
                    eyebrow="Materials & Shaders"
                    title="Author PBR materials as a graph, or drop down to WGSL"
                    body="Wire texture, normal-map and math nodes into a Surface Output exposing base color, metallic, roughness, normal, emissive, AO, clearcoat, anisotropy and more. Need full control? Write custom WGSL shaders and stack over fifty post-process effects on top."
                    chips=vec!["Node graph", "Custom WGSL", "50+ post effects"]
                    img="/assets/previews/material_graph.webp"
                    alt="A node-based material editor wiring texture and normal-map nodes into a PBR surface output"
                    caption="Sample Texture and Sample Normal Map nodes wired into a full PBR Surface Output."
                    reversed=true
                />
                <FeatureRow
                    color="rose"
                    icon="ph-browsers"
                    eyebrow="Ember UI"
                    title="Markup-driven UI for your game and the editor itself"
                    body="Renzora's Ember system builds interfaces from .html templates with reactive double-brace bindings, the same system powers in-game screens and the editor's own panels. Design a match lobby or a HUD visually, point it at an HTML template and a UI layout, then bind it to scripts."
                    chips=vec![".html templates", "Reactive bindings", "Game + editor UI"]
                    img="/assets/previews/ui.webp"
                    alt="The in-engine UI builder editing a match-lobby screen with HTML Template fields in the inspector"
                    caption="The in-engine UI builder editing a match-lobby screen, with HTML Template and UI Layout exposed in the inspector."
                    reversed=false
                />
                <FeatureRow
                    color="orange"
                    icon="ph-bug"
                    eyebrow="Debugging & Profiling"
                    title="Eleven debug panels, a console and a command palette"
                    body="Profile in real time with FPS, frame timing, memory and render stats, then dig into ECS stats, physics and culling debug, Lumen GI and scripting diagnostics, eleven panels in all. A filterable console with categorized logs and a command palette round out the toolkit."
                    chips=vec!["11 debug panels", "Live graphs", "Console & palette"]
                    img="/assets/previews/debugging.webp"
                    alt="The editor with a dense row of diagnostic panels: performance, system, render stats, memory and physics debug"
                    caption="Live diagnostics docked across the bottom, Performance, System, Render Stats, Memory, Physics and Camera debug, with graphs."
                    reversed=true
                />
            </section>

            // ── Cross-platform export band ──
            <section class="reveal relative overflow-hidden rounded-2xl border border-white/[0.08] p-8 sm:p-12">
                <div class="absolute inset-0 bg-gradient-to-br from-accent/10 via-transparent to-secondary/10 pointer-events-none"></div>
                <div class="absolute -top-24 left-1/2 -translate-x-1/2 w-96 h-48 bg-accent/15 rounded-full blur-[100px] pointer-events-none"></div>
                <div class="relative z-10">
                    <div class="text-center max-w-xl mx-auto mb-8">
                        <span class="text-xs font-semibold uppercase tracking-widest text-accent">"Cross-Platform Export"</span>
                        <h2 class="text-2xl sm:text-3xl font-bold mt-2 text-white">"One project. Six platforms."</h2>
                        <p class="text-zinc-400 mt-3">"Build once and export to desktop, mobile and the browser, Windows, Linux, macOS, Android, iOS and the Web via WebAssembly."</p>
                    </div>
                    <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-3">
                        <PlatformTile icon="ph-windows-logo" name="Windows" />
                        <PlatformTile icon="ph-linux-logo" name="Linux" />
                        <PlatformTile icon="ph-apple-logo" name="macOS" />
                        <PlatformTile icon="ph-android-logo" name="Android" />
                        <PlatformTile icon="ph-device-mobile" name="iOS" />
                        <PlatformTile icon="ph-globe" name="Web (WASM)" />
                    </div>
                </div>
            </section>

            // ── Screenshot gallery ──
            <section>
                <div class="flex items-end justify-between mb-6 flex-wrap gap-3">
                    <div>
                        <span class="text-xs font-semibold uppercase tracking-widest text-accent">"Inside the editor"</span>
                        <h2 class="text-xl sm:text-2xl font-bold mt-1 text-white">"More of the toolkit"</h2>
                    </div>
                    <p class="text-sm text-zinc-500 max-w-sm">"Hierarchy, audio mixing, the Hub Store, the asset browser and the console, every panel is real and dockable."</p>
                </div>
                <div class="grid grid-cols-2 lg:grid-cols-3 gap-3 sm:gap-4">
                    <GalleryShot img="/assets/previews/hierarchy.webp" label="Scene hierarchy with nested glTF imports and per-object visibility." />
                    <GalleryShot img="/assets/previews/renzora_ember.webp" label="The Ember UI toolkit: charts, inputs, timelines and inspector widgets." />
                    <GalleryShot img="/assets/previews/mixer.webp" label="An audio mixer with per-bus faders, pan, level meters and solo." />
                    <GalleryShot img="/assets/previews/marketplace.webp" label="The built-in Hub Store with free models and scripts to import." />
                    <GalleryShot img="/assets/previews/console.webp" label="A filterable console with categorized logs and a command bar." />
                    <GalleryShot img="/assets/previews/assets_panel.webp" label="A color-coded asset browser keeps your project organized." />
                </div>
            </section>

            // ── Fresh from the marketplace ──
            <section>
                <div class="flex items-center justify-between mb-5">
                    <h2 class="text-xl sm:text-2xl font-bold text-white">"Fresh from the marketplace"</h2>
                    <a href="/marketplace" class="text-sm font-medium text-accent hover:text-accent-hover transition-colors">"Browse all →"</a>
                </div>
                <div id="home-mp-grid" class="grid grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4">
                    <MpSkeleton /><MpSkeleton /><MpSkeleton /><MpSkeleton />
                </div>
            </section>

            // ── Explore ──
            <section>
                <h2 class="text-lg font-semibold mb-5 text-white">"Explore"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3 sm:gap-4">
                    <ExploreCard icon="ph-storefront" name="Hub Store" desc="Free models & scripts" href="/marketplace" />
                    <ExploreCard icon="ph-book-open" name="Documentation" desc="Guides & API reference" href="/docs" />
                    <ExploreCard icon="ph-download-simple" name="Download" desc="Get the engine" href="/download" />
                    <ExploreCard icon="ph-users-three" name="Community" desc="Devlogs & discussion" href="/community" />
                </div>
            </section>

            // ── FAQ ──
            <section>
                <div class="text-center mb-8">
                    <span class="text-xs font-semibold uppercase tracking-widest text-accent">"FAQ"</span>
                    <h2 class="text-2xl sm:text-3xl font-bold mt-2 text-white">"Frequently asked questions"</h2>
                </div>
                <div class="max-w-3xl mx-auto space-y-3">
                    <FaqItem q="What is Renzora?" a="Renzora is a free, open-source Bevy editor and game engine. It gives the Bevy ecosystem a full visual editor, scene tooling, an inspector, scripting, physics, real-time rendering and a marketplace, all built in Rust." />
                    <FaqItem q="Is Renzora free and open source?" a="Yes. Renzora is completely free and open source under the MIT and Apache-2.0 licenses. You can download it, read the source on GitHub, and extend it with plugins." />
                    <FaqItem q="What is a Bevy editor?" a="Bevy is a Rust game engine that ships as a code framework without an official visual editor. A Bevy editor like Renzora adds a graphical interface on top, letting you build scenes, edit components, write scripts and preview your game without writing all the boilerplate by hand." />
                    <FaqItem q="Which platforms can Renzora export to?" a="Renzora exports one project to six targets: Windows, macOS, Linux, Android, iOS and the web via WebAssembly." />
                    <FaqItem q="How do I script games in Renzora?" a="You can script gameplay in Lua 5.4 or Rhai, Renzora chooses the runtime by file extension, or use visual Blueprint node graphs. Nearly every feature is also a hot-loadable plugin." />
                </div>
                <script type="application/ld+json" inner_html=FAQ_JSONLD></script>
            </section>

            // ── Closing CTA ──
            <section class="reveal relative overflow-hidden rounded-2xl border border-white/[0.08] p-10 sm:p-14 text-center">
                <div class="absolute inset-0 bg-gradient-to-br from-accent/15 via-transparent to-secondary/15 pointer-events-none"></div>
                <div class="absolute top-0 left-1/2 -translate-x-1/2 w-64 h-px bg-gradient-to-r from-transparent via-accent/60 to-transparent"></div>
                <div class="relative z-10">
                    <h2 class="text-2xl sm:text-3xl font-bold text-white">"Build it in Renzora"</h2>
                    <p class="text-zinc-400 mt-3 mb-8 max-w-md mx-auto">"Download the open-source engine and spin up your first scene in minutes."</p>
                    <div class="flex gap-3 justify-center flex-wrap">
                        <a href="/download" class="inline-flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-semibold bg-purple-600 text-white hover:bg-purple-500 transition-all hover:shadow-[0_0_30px_rgba(168,85,247,0.35)]">
                            <i class="ph ph-download-simple text-lg"></i>"Download the Engine"
                        </a>
                        <a href="/docs/getting-started/installation" class="inline-flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-semibold bg-white/[0.06] text-zinc-50 border border-white/[0.1] hover:bg-white/[0.1] transition-all">
                            <i class="ph ph-rocket-launch text-lg"></i>"Getting Started"
                        </a>
                    </div>
                </div>
            </section>
        </div>

        <script>
            r#"
            // ── Fresh from the marketplace (real assets) ──
            (function() {
                const grid = document.getElementById('home-mp-grid');
                if (!grid) return;
                const gradients = [
                    'from-violet-600 to-purple-800',
                    'from-emerald-600 to-teal-800',
                    'from-sky-600 to-cyan-800',
                    'from-fuchsia-600 to-pink-800',
                ];
                function card(a, i) {
                    const price = a.price_credits === 0
                        ? '<span class="text-emerald-400 font-semibold">Free</span>'
                        : `<span class="inline-flex items-center gap-1 text-zinc-200 font-semibold"><i class="ph ph-diamond text-[11px] text-secondary"></i>${(a.price_credits||0).toLocaleString()}</span>`;
                    const rating = (a.rating_count > 0)
                        ? `<span class="inline-flex items-center gap-1 text-amber-400"><i class="ph ph-star text-[11px]"></i>${(a.rating_avg||0).toFixed(1)}</span>`
                        : `<span class="text-zinc-600">New</span>`;
                    const altText = (a.name || 'Marketplace asset').replace(/"/g, '&quot;');
                    const thumb = (a.thumbnail_url && String(a.thumbnail_url).trim())
                        ? `<img src="${a.thumbnail_url}" alt="${altText}" loading="lazy" class="absolute inset-0 w-full h-full object-cover" />`
                        : `<div class="absolute inset-0 bg-gradient-to-br ${gradients[i % gradients.length]}"></div>`;
                    const cat = (a.category || 'Asset').toString();
                    return `
                        <a href="/marketplace/asset/${a.slug}" class="group block rounded-xl overflow-hidden border border-white/[0.07] bg-white/[0.02] hover:border-accent/40 transition-all">
                            <div class="relative aspect-[4/3] overflow-hidden">
                                ${thumb}
                                <span class="absolute top-2 left-2 text-[9px] font-bold uppercase tracking-wider px-1.5 py-0.5 rounded bg-black/50 backdrop-blur-sm text-zinc-100 border border-white/10">${cat}</span>
                            </div>
                            <div class="p-3">
                                <p class="text-sm font-semibold text-white truncate group-hover:text-accent transition-colors">${a.name}</p>
                                <div class="mt-1.5 flex items-center justify-between text-[11px]">${rating}${price}</div>
                            </div>
                        </a>`;
                }
                fetch('/api/marketplace?page=1')
                    .then(r => r.ok ? r.json() : { assets: [] })
                    .then(data => {
                        const assets = (data.assets || []).slice(0, 4);
                        if (!assets.length) {
                            grid.innerHTML = '<a href="/marketplace" class="col-span-2 lg:col-span-4 rounded-xl border border-dashed border-white/[0.1] p-8 text-center text-sm text-zinc-500 hover:text-zinc-300 hover:border-white/20 transition-all">The marketplace is just getting started, browse what\'s there →</a>';
                            return;
                        }
                        grid.innerHTML = assets.map(card).join('');
                    })
                    .catch(() => {
                        grid.innerHTML = '<a href="/marketplace" class="col-span-2 lg:col-span-4 rounded-xl border border-dashed border-white/[0.1] p-8 text-center text-sm text-zinc-500 hover:text-zinc-300 transition-all">Browse the marketplace →</a>';
                    });
            })();
            "#
        </script>

        <style>
            r#"
            .icon-violet { color:#8b5cf6; background:rgba(139,92,246,0.12); }
            .icon-fuchsia { color:#d946ef; background:rgba(217,70,239,0.12); }
            .icon-cyan { color:#22d3ee; background:rgba(34,211,238,0.12); }
            .icon-emerald { color:#10b981; background:rgba(16,185,129,0.12); }
            .icon-amber { color:#f59e0b; background:rgba(245,158,11,0.12); }
            .icon-sky { color:#0ea5e9; background:rgba(14,165,233,0.12); }
            .icon-rose { color:#f43f5e; background:rgba(244,63,94,0.12); }
            .icon-orange { color:#f97316; background:rgba(249,115,22,0.12); }
            "#
        </style>
    }
}

#[component]
fn StatCard(target: &'static str, suffix: &'static str, label: &'static str, color: &'static str) -> impl IntoView {
    let value_class = format!("text-2xl sm:text-3xl font-extrabold {}", color);
    view! {
        <div class="rounded-xl border border-white/[0.07] bg-white/[0.02] p-4 sm:p-5 text-center">
            <div class=value_class>{target}{suffix}</div>
            <div class="text-[11px] text-zinc-500 mt-1.5 uppercase tracking-wider">{label}</div>
        </div>
    }
}

#[component]
fn PillarCard(icon: &'static str, color: &'static str, title: &'static str, desc: &'static str) -> impl IntoView {
    let icon_class = format!("ph {} text-white text-xl", icon);
    let icon_wrap = format!("w-11 h-11 rounded-xl bg-gradient-to-br {} flex items-center justify-center shadow-lg", color);
    view! {
        <div class="pillar-card rounded-xl border border-white/[0.07] bg-white/[0.02] p-5 hover:border-white/[0.14] hover:bg-white/[0.035] transition-all">
            <div class=icon_wrap>
                <i class=icon_class></i>
            </div>
            <h3 class="text-sm font-semibold text-white mt-3.5 mb-1.5">{title}</h3>
            <p class="text-xs text-zinc-500 leading-relaxed">{desc}</p>
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
    let text_col_class = format!("{}", text_order);
    let media_col_class = format!("{}", media_order);
    view! {
        <div class="feature-row grid lg:grid-cols-2 gap-8 lg:gap-12 items-center mb-16 lg:mb-24 last:mb-0">
            <div class=text_col_class>
                <div class="inline-flex items-center gap-2 mb-4">
                    <div class=icon_wrap_class>
                        <i class=icon_class></i>
                    </div>
                    <span class="text-xs font-semibold uppercase tracking-widest text-zinc-400">{eyebrow}</span>
                </div>
                <h2 class="text-xl sm:text-2xl font-bold tracking-tight leading-tight text-white">{title}</h2>
                <p class="text-zinc-400 mt-4 text-sm sm:text-base leading-relaxed">{body}</p>
                <div class="flex flex-wrap gap-2 mt-6">
                    {chips.into_iter().map(|c| view! {
                        <span class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full bg-white/[0.03] border border-white/[0.08] text-[11px] font-medium text-zinc-300">
                            <span class="w-1 h-1 rounded-full bg-accent"></span>{c}
                        </span>
                    }).collect_view()}
                </div>
            </div>
            <div class=media_col_class>
                <div class="group relative rounded-2xl overflow-hidden border border-white/[0.08] bg-surface-card shadow-2xl shadow-black/40">
                    <div class="pointer-events-none absolute inset-0 bg-gradient-to-tr from-accent/10 via-transparent to-secondary/10 z-10"></div>
                    <div class="pointer-events-none absolute inset-0 ring-1 ring-inset ring-white/5 rounded-2xl z-10"></div>
                    <picture class="contents">
                        <source srcset=img.replace(".webp", ".avif") type="image/avif" />
                        <img src=img alt=alt loading="lazy" data-zoom="1" class="relative w-full h-auto block transition-transform duration-500 group-hover:scale-[1.02]" />
                    </picture>
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
        <div class="flex flex-col items-center gap-3 p-6 rounded-xl bg-white/[0.02] border border-white/[0.07] hover:border-accent/40 hover:bg-white/[0.04] transition-all">
            <i class=icon_class></i>
            <span class="text-sm font-medium text-zinc-200">{name}</span>
        </div>
    }
}

#[component]
fn GalleryShot(img: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <figure class="group relative rounded-xl overflow-hidden border border-white/[0.07] bg-surface-card">
            <picture class="contents">
                <source srcset=img.replace(".webp", ".avif") type="image/avif" />
                <img src=img alt=label loading="lazy" data-zoom="1" class="w-full h-44 object-cover object-top transition-transform duration-500 group-hover:scale-105" />
            </picture>
            <figcaption class="absolute inset-x-0 bottom-0 p-3 text-xs text-zinc-200 leading-snug bg-gradient-to-t from-black/85 via-black/50 to-transparent opacity-0 group-hover:opacity-100 transition-opacity">{label}</figcaption>
        </figure>
    }
}

#[component]
fn ExploreCard(icon: &'static str, name: &'static str, desc: &'static str, href: &'static str) -> impl IntoView {
    let icon_class = format!("ph {} text-2xl text-accent", icon);
    view! {
        <a href=href class="flex items-center gap-4 p-5 bg-white/[0.02] border border-white/[0.07] rounded-xl hover:border-white/[0.16] hover:bg-white/[0.04] transition-all group">
            <div class="w-11 h-11 rounded-xl bg-accent/10 flex items-center justify-center shrink-0 group-hover:scale-110 transition-transform">
                <i class=icon_class></i>
            </div>
            <div>
                <h3 class="text-sm font-semibold text-white group-hover:text-accent transition-colors">{name}</h3>
                <p class="text-[11px] text-zinc-500">{desc}</p>
            </div>
        </a>
    }
}

#[component]
fn FaqItem(q: &'static str, a: &'static str) -> impl IntoView {
    view! {
        <details class="group rounded-xl border border-white/[0.07] bg-white/[0.02] px-5 py-4">
            <summary class="flex items-center justify-between gap-4 cursor-pointer list-none text-sm font-semibold text-white">
                {q}
                <i class="ph ph-plus text-zinc-500 shrink-0 group-open:rotate-45 transition-transform"></i>
            </summary>
            <p class="text-sm text-zinc-400 leading-relaxed mt-3">{a}</p>
        </details>
    }
}

#[component]
fn MpSkeleton() -> impl IntoView {
    view! {
        <div class="rounded-xl overflow-hidden border border-white/[0.07] bg-white/[0.02] animate-pulse">
            <div class="aspect-[4/3] bg-white/[0.04]"></div>
            <div class="p-3 space-y-2">
                <div class="h-3 bg-white/[0.06] rounded w-3/4"></div>
                <div class="h-2.5 bg-white/[0.04] rounded w-1/2"></div>
            </div>
        </div>
    }
}
