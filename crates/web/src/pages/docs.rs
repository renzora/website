use leptos::prelude::*;

#[component]
pub fn DocsPage() -> impl IntoView {
    let (search, set_search) = signal(String::new());

    view! {
        <section class="docs-page">
            <div class="container">
                <h1>"Documentation"</h1>
                <p class="docs-intro">
                    "Learn how to use Renzora Engine to build your game."
                </p>

                <div class="docs-search">
                    <input
                        type="text"
                        placeholder="Search documentation..."
                        class="search-input"
                        prop:value=search
                        on:input=move |ev| set_search.set(event_target_value(&ev))
                    />
                </div>

                <div class="docs-grid">
                    <DocSection
                        title="Getting Started"
                        description="Install the engine, create your first project, and learn the basics."
                        slug="getting-started"
                        pages=vec![
                            ("installation", "Installation"),
                            ("first-project", "Your First Project"),
                            ("editor-overview", "Editor Overview"),
                        ]
                    />
                    <DocSection
                        title="Editor Guide"
                        description="Master the editor: scenes, inspector, materials, terrain, and more."
                        slug="editor"
                        pages=vec![
                            ("scenes", "Working with Scenes"),
                            ("inspector", "Inspector Panel"),
                            ("materials", "Material Editor"),
                            ("terrain", "Terrain System"),
                        ]
                    />
                    <DocSection
                        title="Scripting"
                        description="Write game logic with Lua, Rhai, or visual blueprints."
                        slug="scripting"
                        pages=vec![
                            ("lua", "Lua Scripting"),
                            ("rhai", "Rhai Scripting"),
                            ("blueprints", "Visual Blueprints"),
                            ("api-reference", "API Reference"),
                        ]
                    />
                    <DocSection
                        title="Networking"
                        description="Build multiplayer games with dedicated server support."
                        slug="networking"
                        pages=vec![
                            ("setup", "Server Setup"),
                            ("replication", "State Replication"),
                            ("input", "Input Handling"),
                        ]
                    />
                    <DocSection
                        title="Export & Deploy"
                        description="Build and export your game for multiple platforms."
                        slug="export"
                        pages=vec![
                            ("windows", "Windows"),
                            ("linux", "Linux"),
                            ("macos", "macOS"),
                            ("android", "Android"),
                            ("ios", "iOS & tvOS"),
                        ]
                    />
                    <DocSection
                        title="Marketplace"
                        description="Publishing assets and using the marketplace."
                        slug="marketplace"
                        pages=vec![
                            ("browsing", "Browsing & Installing"),
                            ("publishing", "Publishing Assets"),
                            ("credits", "Credits System"),
                        ]
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn DocSection(
    title: &'static str,
    description: &'static str,
    slug: &'static str,
    pages: Vec<(&'static str, &'static str)>,
) -> impl IntoView {
    view! {
        <div class="doc-card">
            <h3>{title}</h3>
            <p>{description}</p>
            <ul class="doc-links">
                {pages.into_iter().map(|(page_slug, label)| {
                    let href = format!("/docs/{slug}/{page_slug}");
                    view! {
                        <li><a href=href>{label}</a></li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
