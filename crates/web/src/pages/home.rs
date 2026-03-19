use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="pt-24 pb-14 text-center px-6">
            <div class="max-w-3xl mx-auto">
                <h1 class="text-5xl md:text-6xl font-extrabold tracking-tight leading-[1.1] gradient-text">"Renzora"</h1>
                <p class="mt-4 text-xl text-zinc-400">"The game developer hub."</p>
                <p class="mt-3 text-sm text-zinc-500 leading-relaxed max-w-xl mx-auto">
                    "Browse and sell assets for any engine — Unreal, Unity, Godot, Renzora, and more. "
                    "Connect with developers, share your work, and find everything you need to build your game."
                </p>
                <div class="mt-8 flex gap-3 justify-center flex-wrap">
                    <a href="/marketplace" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                        <i class="ph ph-storefront text-lg"></i>"Browse Marketplace"
                    </a>
                    <a href="/forum" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-surface-card text-zinc-50 border border-zinc-800 hover:border-zinc-600 transition-colors">
                        <i class="ph ph-chat-circle text-lg"></i>"Community Forum"
                    </a>
                    <a href="/engine" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-surface-card text-zinc-50 border border-zinc-800 hover:border-zinc-600 transition-colors">
                        <i class="ph ph-rocket-launch text-lg"></i>"Renzora Engine"
                    </a>
                </div>
            </div>
        </section>

        <section class="pb-16">
            <div class="max-w-[1200px] mx-auto px-6">
                <h2 class="text-lg font-semibold mb-4">"Popular Categories"</h2>
                <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-3">
                    <CategoryCard icon="ph-cube" name="3D Models" href="/marketplace?category=3d-models" />
                    <CategoryCard icon="ph-image" name="Textures & HDRIs" href="/marketplace?category=textures" />
                    <CategoryCard icon="ph-music-notes" name="Music" href="/marketplace?category=music" />
                    <CategoryCard icon="ph-puzzle-piece" name="Plugins" href="/marketplace?category=plugins" />
                    <CategoryCard icon="ph-code" name="Scripts" href="/marketplace?category=scripts" />
                    <CategoryCard icon="ph-sparkle" name="VFX" href="/marketplace?category=particles" />
                </div>
            </div>
        </section>

        <section class="pb-16">
            <div class="max-w-[1200px] mx-auto px-6">
                <h2 class="text-lg font-semibold mb-4">"Supported Engines"</h2>
                <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
                    <EngineCard icon="ph-rocket-launch" name="Renzora" desc="Our open-source Rust engine" href="/engine" color="#6366f1" />
                    <EngineCard icon="ph-game-controller" name="Unreal Engine" desc="Epic's AAA powerhouse" href="/marketplace?category=unreal" color="#0ea5e9" />
                    <EngineCard icon="ph-circle-dashed" name="Unity" desc="The industry workhorse" href="/marketplace?category=unity" color="#a855f7" />
                    <EngineCard icon="ph-robot" name="Godot" desc="Free and open source" href="/marketplace?category=godot" color="#22c55e" />
                </div>
            </div>
        </section>

        <section class="pb-16">
            <div class="max-w-[1200px] mx-auto px-6">
                <h2 class="text-lg font-semibold mb-4">"Why Renzora?"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
                    <FeatureCard icon="ph-storefront" title="Multi-Engine Marketplace" description="Sell and buy assets for any game engine. One platform, every workflow." />
                    <FeatureCard icon="ph-users-three" title="Developer Community" description="Forums, profiles, articles, and collaboration tools for game devs." />
                    <FeatureCard icon="ph-coins" title="Credits System" description="Simple credits-based economy. Top up once, buy across the platform." />
                    <FeatureCard icon="ph-chart-line-up" title="Creator Dashboard" description="Track downloads, earnings, and audience. Get paid for your work." />
                    <FeatureCard icon="ph-rocket-launch" title="Renzora Engine" description="Our own open-source engine built with Rust and Bevy. Cross-platform." />
                    <FeatureCard icon="ph-code" title="Open Source" description="The platform and engine are open source. Contribute and extend." />
                </div>
            </div>
        </section>

        <section class="pb-20">
            <div class="max-w-[1200px] mx-auto px-6">
                <div class="text-center p-12 bg-surface-card border border-zinc-800 rounded-xl">
                    <h2 class="text-2xl font-bold">"Start creating"</h2>
                    <p class="text-zinc-400 mt-2 mb-6 text-sm">"Join the community, browse assets, or publish your own."</p>
                    <div class="flex gap-3 justify-center flex-wrap">
                        <a href="/register" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                            <i class="ph ph-user-plus text-lg"></i>"Create Account"
                        </a>
                        <a href="/marketplace" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-surface-card text-zinc-50 border border-zinc-800 hover:border-zinc-600 transition-colors">
                            <i class="ph ph-storefront text-lg"></i>"Explore Marketplace"
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn CategoryCard(icon: &'static str, name: &'static str, href: &'static str) -> impl IntoView {
    let icon_class = format!("ph {} text-2xl text-accent", icon);
    view! {
        <a href=href class="flex flex-col items-center gap-2 p-5 bg-surface-card border border-zinc-800 rounded-xl hover:border-accent transition-colors text-center group">
            <i class=icon_class></i>
            <span class="text-xs font-medium text-zinc-300 group-hover:text-zinc-50 transition-colors">{name}</span>
        </a>
    }
}

#[component]
fn EngineCard(icon: &'static str, name: &'static str, desc: &'static str, href: &'static str, color: &'static str) -> impl IntoView {
    let icon_class = format!("ph {} text-3xl", icon);
    let border_style = format!("border-color: {}20", color);
    let icon_style = format!("color: {}", color);
    let bg_style = format!("background: {}10", color);
    view! {
        <a href=href class="flex items-center gap-3 p-4 bg-surface-card border border-zinc-800 rounded-xl hover:border-zinc-600 transition-all group" style=border_style>
            <div class="w-12 h-12 rounded-xl flex items-center justify-center shrink-0" style=bg_style>
                <i class=icon_class style=icon_style></i>
            </div>
            <div>
                <h3 class="text-sm font-semibold group-hover:text-zinc-50 transition-colors">{name}</h3>
                <p class="text-[11px] text-zinc-500">{desc}</p>
            </div>
        </a>
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
