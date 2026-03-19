use leptos::prelude::*;

#[component]
pub fn EnginePage() -> impl IntoView {
    view! {
        <section class="pt-24 pb-12 text-center px-6">
            <div class="max-w-2xl mx-auto">
                <span class="text-xs font-medium text-accent uppercase tracking-wider">"Open Source Game Engine"</span>
                <h1 class="text-5xl md:text-6xl font-extrabold tracking-tight leading-[1.1] gradient-text mt-3">"Renzora Engine"</h1>
                <p class="mt-5 text-lg text-zinc-400 leading-relaxed">
                    "A modern game engine built with Rust and Bevy. "
                    "Create stunning 2D and 3D games with a powerful editor, "
                    "visual scripting, and cross-platform export."
                </p>
                <div class="mt-8 flex gap-3 justify-center">
                    <a href="/download" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                        <i class="ph ph-download-simple text-lg"></i>"Download"
                    </a>
                    <a href="/docs" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-surface-card text-zinc-50 border border-zinc-800 hover:border-zinc-600 transition-colors">
                        <i class="ph ph-book-open text-lg"></i>"Documentation"
                    </a>
                    <a href="https://github.com/renzora/engine" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-surface-card text-zinc-50 border border-zinc-800 hover:border-zinc-600 transition-colors">
                        <i class="ph ph-github-logo text-lg"></i>"Source Code"
                    </a>
                </div>
                <p class="mt-4 text-xs text-zinc-500">"r1-alpha4 — Early Access"</p>
            </div>
        </section>

        <section class="pb-16 w-full overflow-hidden">
            <img
                src="/assets/images/interface.png"
                alt="Renzora Engine editor"
                class="w-full max-w-full h-auto block"
                loading="lazy"
            />
        </section>

        <section class="pb-16">
            <div class="max-w-[1200px] mx-auto px-6">
                <h2 class="text-2xl font-bold text-center mb-10">"Everything you need"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
                    <FeatureCard icon="ph-cube" title="Visual Editor" description="Scene hierarchy, inspector, material graphs, terrain tools, and a fully dockable panel system." />
                    <FeatureCard icon="ph-tree-structure" title="Visual Scripting" description="Blueprint-style node graphs alongside Lua and Rhai scripting." />
                    <FeatureCard icon="ph-devices" title="Cross-Platform" description="Export to Windows, macOS, Linux, Android, iOS, tvOS, and Web." />
                    <FeatureCard icon="ph-wifi-high" title="Multiplayer" description="Dedicated server networking with state replication and client prediction." />
                    <FeatureCard icon="ph-mountains" title="Terrain" description="GPU-powered sculpting and painting with real-time preview." />
                    <FeatureCard icon="ph-film-strip" title="Animation" description="Keyframe animation editor with timeline and curve editing." />
                    <FeatureCard icon="ph-speaker-high" title="Audio" description="Spatial audio, mixer, and DAW-style audio pipeline." />
                    <FeatureCard icon="ph-drop" title="Materials" description="Node-based material editor with WGSL shader support." />
                    <FeatureCard icon="ph-code" title="Open Source" description="Built on Rust and Bevy. Inspect, extend, and contribute." />
                </div>
            </div>
        </section>

        <section class="pb-20">
            <div class="max-w-[1200px] mx-auto px-6">
                <div class="text-center p-12 bg-surface-card border border-zinc-800 rounded-xl">
                    <h2 class="text-2xl font-bold">"Ready to build?"</h2>
                    <p class="text-zinc-400 mt-2 mb-6 text-sm">"Download the engine and create your first project in minutes."</p>
                    <div class="flex gap-3 justify-center">
                        <a href="/download" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                            <i class="ph ph-download-simple text-lg"></i>"Download"
                        </a>
                        <a href="/docs/getting-started/installation" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-surface-card text-zinc-50 border border-zinc-800 hover:border-zinc-600 transition-colors">
                            <i class="ph ph-rocket-launch text-lg"></i>"Getting Started"
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(icon: &'static str, title: &'static str, description: &'static str) -> impl IntoView {
    let icon_class = format!("ph {} text-2xl text-accent", icon);
    view! {
        <div class="p-5 bg-surface-card border border-zinc-800 rounded-xl hover:border-accent/30 transition-colors">
            <i class=icon_class></i>
            <h3 class="text-sm font-semibold mt-3 mb-1">{title}</h3>
            <p class="text-xs text-zinc-400 leading-relaxed">{description}</p>
        </div>
    }
}
