use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="pt-28 pb-12 text-center px-6">
            <div class="max-w-2xl mx-auto">
                <h1 class="text-5xl md:text-6xl font-extrabold tracking-tight leading-[1.1] gradient-text">"Renzora Engine"</h1>
                <p class="mt-5 text-lg text-zinc-400 leading-relaxed">
                    "An open-source game engine built with Rust and Bevy. "
                    "Powerful editor, visual scripting, cross-platform export, "
                    "and a community marketplace."
                </p>
                <div class="mt-8 flex gap-3 justify-center flex-wrap">
                    <a href="/download" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                        <i class="ph ph-download-simple text-lg"></i>"Download"
                    </a>
                    <a href="/docs" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-surface-card text-zinc-50 border border-zinc-800 hover:border-zinc-600 transition-colors">
                        <i class="ph ph-book-open text-lg"></i>"Documentation"
                    </a>
                    <a href="https://github.com/renzora/engine" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-surface-card text-zinc-50 border border-zinc-800 hover:border-zinc-600 transition-colors">
                        <i class="ph ph-github-logo text-lg"></i>"Source"
                    </a>
                </div>
                <p class="mt-4 text-xs text-zinc-500">"r1-alpha4 — Early Access"</p>
            </div>
        </section>

        <section class="pb-16 w-full overflow-hidden">
            <img src="/assets/images/interface.png" alt="Renzora Engine editor" class="w-full max-w-full h-auto block" loading="lazy" />
        </section>

        <section class="pb-16">
            <div class="max-w-[1200px] mx-auto px-6">
                <h2 class="text-2xl font-bold text-center mb-10">"Everything you need to build your game"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
                    <FeatureCard icon="ph-cube" title="Visual Editor" description="Scene hierarchy, inspector, material graphs, terrain tools, and a fully dockable panel system." />
                    <FeatureCard icon="ph-tree-structure" title="Visual Scripting" description="Blueprint-style node graphs alongside Lua and Rhai scripting." />
                    <FeatureCard icon="ph-devices" title="Cross-Platform" description="Export to Windows, macOS, Linux, Android, iOS, tvOS, and Web." />
                    <FeatureCard icon="ph-wifi-high" title="Multiplayer" description="Dedicated server networking with state replication and client prediction." />
                    <FeatureCard icon="ph-storefront" title="Marketplace" description="Browse and sell plugins, assets, themes, and scripts. Earn credits." />
                    <FeatureCard icon="ph-mountains" title="Terrain" description="GPU-powered sculpting and painting with real-time preview." />
                    <FeatureCard icon="ph-drop" title="Materials" description="Node-based material editor with WGSL shader support." />
                    <FeatureCard icon="ph-code" title="Open Source" description="Built on Rust and Bevy. Inspect, extend, and contribute." />
                </div>
            </div>
        </section>

        <section class="pb-16">
            <div class="max-w-[1200px] mx-auto px-6">
                <h2 class="text-lg font-semibold mb-4">"Explore"</h2>
                <div class="grid grid-cols-2 sm:grid-cols-3 gap-3">
                    <ExploreCard icon="ph-storefront" name="Marketplace" desc="Plugins, assets, and themes" href="/marketplace" />
                    <ExploreCard icon="ph-book-open" name="Documentation" desc="Guides and API reference" href="/docs" />
                    <ExploreCard icon="ph-download-simple" name="Download" desc="Get the engine" href="/download" />
                </div>
            </div>
        </section>

        <section class="pb-20">
            <div class="max-w-[1200px] mx-auto px-6">
                <div class="text-center p-12 bg-surface-card border border-zinc-800 rounded-xl">
                    <h2 class="text-2xl font-bold">"Ready to build?"</h2>
                    <p class="text-zinc-400 mt-2 mb-6 text-sm">"Download the engine and create your first project in minutes."</p>
                    <div class="flex gap-3 justify-center flex-wrap">
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

#[component]
fn ExploreCard(icon: &'static str, name: &'static str, desc: &'static str, href: &'static str) -> impl IntoView {
    let icon_class = format!("ph {} text-2xl text-accent", icon);
    view! {
        <a href=href class="flex items-center gap-3 p-4 bg-surface-card border border-zinc-800 rounded-xl hover:border-zinc-600 transition-all group">
            <div class="w-10 h-10 rounded-xl bg-accent/10 flex items-center justify-center shrink-0">
                <i class=icon_class></i>
            </div>
            <div>
                <h3 class="text-sm font-semibold group-hover:text-accent transition-colors">{name}</h3>
                <p class="text-[11px] text-zinc-500">{desc}</p>
            </div>
        </a>
    }
}
