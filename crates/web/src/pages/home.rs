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
                    <a href="/download" class="btn btn-primary">"Download for Windows"</a>
                    <a href="/docs" class="btn btn-secondary">"Documentation"</a>
                </div>
                <p class="hero-version">"v0.1.0 — Early Access"</p>
            </div>
        </section>

        <section class="features">
            <div class="container">
                <h2>"Built for creators"</h2>
                <div class="feature-grid">
                    <FeatureCard
                        title="Visual Editor"
                        description="A complete editor with scene hierarchy, inspector, material graphs, and terrain tools."
                        icon="editor"
                    />
                    <FeatureCard
                        title="Visual Scripting"
                        description="Blueprint-style visual scripting alongside Lua and Rhai for full flexibility."
                        icon="script"
                    />
                    <FeatureCard
                        title="Marketplace"
                        description="Discover plugins, themes, and assets from the community. Publish your own creations."
                        icon="marketplace"
                    />
                    <FeatureCard
                        title="Cross-Platform"
                        description="Export to Windows, macOS, Linux, Android, iOS, and tvOS from a single project."
                        icon="platforms"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(title: &'static str, description: &'static str, icon: &'static str) -> impl IntoView {
    view! {
        <div class="feature-card" data-icon=icon>
            <h3>{title}</h3>
            <p>{description}</p>
        </div>
    }
}
