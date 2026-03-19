use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="hero">
            <div class="hero-content">
                <h1 class="hero-title">"Renzora Engine"</h1>
                <p class="hero-subtitle">
                    "A modern game engine built with Rust and Bevy. "
                    "Create stunning 2D and 3D games with a powerful editor, "
                    "visual scripting, and a thriving marketplace."
                </p>
                <div class="hero-actions">
                    <a href="https://github.com/renzora/engine/releases" class="btn btn-primary">"Download for Windows"</a>
                    <a href="/docs" class="btn btn-secondary">"Documentation"</a>
                </div>
                <p class="hero-version">"v0.1.0 — Early Access"</p>
            </div>
        </section>

        <section class="hero-preview">
            <div class="container">
                <div class="preview-window">
                    <div class="preview-chrome">
                        <span class="preview-dot red"></span>
                        <span class="preview-dot yellow"></span>
                        <span class="preview-dot green"></span>
                    </div>
                    <img
                        src="/assets/images/interface.png"
                        alt="Renzora Engine editor showing a 3D scene of Times Square"
                        class="preview-img"
                        loading="lazy"
                    />
                </div>
            </div>
        </section>

        <section class="features">
            <div class="container">
                <h2>"Built for creators"</h2>
                <div class="feature-grid">
                    <FeatureCard
                        title="Visual Editor"
                        description="Scene hierarchy, inspector, material graphs, terrain tools, and a fully dockable panel system."
                    />
                    <FeatureCard
                        title="Visual Scripting"
                        description="Blueprint-style node graphs alongside Lua and Rhai scripting for full flexibility."
                    />
                    <FeatureCard
                        title="Marketplace"
                        description="Discover and install plugins, themes, and assets. Publish your own and earn credits."
                    />
                    <FeatureCard
                        title="Cross-Platform"
                        description="Export to Windows, macOS, Linux, Android, iOS, and tvOS from a single project."
                    />
                    <FeatureCard
                        title="Multiplayer"
                        description="Built-in networking with dedicated server support, state replication, and client prediction."
                    />
                    <FeatureCard
                        title="Open Source"
                        description="Built on Rust and Bevy. Inspect the source, contribute, and extend every part of the engine."
                    />
                </div>
            </div>
        </section>

        <section class="cta">
            <div class="container">
                <div class="cta-card">
                    <h2>"Ready to build?"</h2>
                    <p>"Download the engine and follow the getting started guide to create your first project in minutes."</p>
                    <div class="hero-actions">
                        <a href="https://github.com/renzora/engine/releases" class="btn btn-primary">"Download"</a>
                        <a href="/docs/getting-started/installation" class="btn btn-secondary">"Getting Started Guide"</a>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="feature-card">
            <h3>{title}</h3>
            <p>{description}</p>
        </div>
    }
}
