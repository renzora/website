use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="pt-28 pb-16 text-center px-6">
            <div class="max-w-2xl mx-auto">
                <h1 class="text-5xl md:text-6xl font-extrabold tracking-tight leading-[1.1] gradient-text">"Renzora Engine"</h1>
                <p class="mt-5 text-lg text-zinc-400 leading-relaxed">
                    "A modern game engine built with Rust and Bevy. "
                    "Create stunning 2D and 3D games with a powerful editor, "
                    "visual scripting, and a thriving community."
                </p>
                <div class="mt-9 flex gap-3 justify-center">
                    <a href="/download" class="inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                        <i class="ph ph-download-simple text-lg"></i>"Download"
                    </a>
                    <a href="/docs" class="inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-surface-card text-zinc-50 border border-zinc-800 hover:border-zinc-600 transition-colors">
                        <i class="ph ph-book-open text-lg"></i>"Documentation"
                    </a>
                </div>
                <p class="mt-4 text-xs text-zinc-500">"r1-alpha4 — Early Access"</p>
            </div>
        </section>

        <section class="pb-20 w-full overflow-hidden">
            <img
                src="/assets/images/interface.png"
                alt="Renzora Engine editor showing a 3D scene"
                class="w-full max-w-full h-auto block"
                loading="lazy"
            />
        </section>

        <section class="pb-20">
            <div class="max-w-[1200px] mx-auto px-6">
                <h2 class="text-3xl font-bold text-center mb-12">"Built for creators"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                    <FeatureCard
                        icon="ph-cube"
                        title="Visual Editor"
                        description="Scene hierarchy, inspector, material graphs, terrain tools, and a fully dockable panel system."
                    />
                    <FeatureCard
                        icon="ph-tree-structure"
                        title="Visual Scripting"
                        description="Blueprint-style node graphs alongside Lua and Rhai scripting for full flexibility."
                    />
                    <FeatureCard
                        icon="ph-storefront"
                        title="Marketplace"
                        description="Discover and install plugins, themes, and assets. Publish your own and earn credits."
                    />
                    <FeatureCard
                        icon="ph-devices"
                        title="Cross-Platform"
                        description="Export to Windows, macOS, Linux, Android, iOS, and tvOS from a single project."
                    />
                    <FeatureCard
                        icon="ph-wifi-high"
                        title="Multiplayer"
                        description="Built-in networking with dedicated server support, state replication, and client prediction."
                    />
                    <FeatureCard
                        icon="ph-code"
                        title="Open Source"
                        description="Built on Rust and Bevy. Inspect the source, contribute, and extend every part of the engine."
                    />
                </div>
            </div>
        </section>

        <section class="pb-20">
            <div class="max-w-[1200px] mx-auto px-6">
                <div class="text-center p-12 bg-surface-card border border-zinc-800 rounded-lg">
                    <h2 class="text-2xl font-bold">"Ready to build?"</h2>
                    <p class="text-zinc-400 mt-2 mb-6 text-sm">"Download the engine and follow the getting started guide to create your first project in minutes."</p>
                    <div class="flex gap-3 justify-center">
                        <a href="/download" class="inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                            <i class="ph ph-download-simple text-lg"></i>"Download"
                        </a>
                        <a href="/docs/getting-started/installation" class="inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-surface-card text-zinc-50 border border-zinc-800 hover:border-zinc-600 transition-colors">
                            <i class="ph ph-rocket-launch text-lg"></i>"Getting Started Guide"
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
        <div class="p-6 bg-surface-card border border-zinc-800 rounded-lg hover:border-accent transition-colors">
            <i class=icon_class></i>
            <h3 class="text-base font-semibold mt-3 mb-2">{title}</h3>
            <p class="text-sm text-zinc-400 leading-relaxed">{description}</p>
        </div>
    }
}
