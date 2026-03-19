use leptos::prelude::*;

/// Documentation landing page — shows all categories with descriptions.
#[component]
pub fn DocsPage() -> impl IntoView {
    view! {
        <div class="docs-layout">
            <DocsSidebar active_slug="" />
            <div class="docs-content">
                <div class="docs-hero">
                    <h1>"Documentation"</h1>
                    <p class="docs-hero-sub">"Everything you need to build games with Renzora Engine."</p>
                </div>

                <div class="docs-cards">
                    <a href="/docs/getting-started/installation" class="docs-card-lg">
                        <div class="docs-card-icon">"01"</div>
                        <div>
                            <h3>"Getting Started"</h3>
                            <p>"Install the engine, create your first project, and learn the editor basics."</p>
                        </div>
                    </a>
                    <a href="/docs/editor/scenes" class="docs-card-lg">
                        <div class="docs-card-icon">"02"</div>
                        <div>
                            <h3>"Editor Guide"</h3>
                            <p>"Master scenes, the inspector, materials, terrain, and the docking panel system."</p>
                        </div>
                    </a>
                    <a href="/docs/scripting/lua" class="docs-card-lg">
                        <div class="docs-card-icon">"03"</div>
                        <div>
                            <h3>"Scripting"</h3>
                            <p>"Write game logic with Lua, Rhai, or visual blueprints. Full API reference included."</p>
                        </div>
                    </a>
                    <a href="/docs/networking/overview" class="docs-card-lg">
                        <div class="docs-card-icon">"04"</div>
                        <div>
                            <h3>"Multiplayer"</h3>
                            <p>"Set up dedicated servers, replicate state, and handle player input."</p>
                        </div>
                    </a>
                    <a href="/docs/export/overview" class="docs-card-lg">
                        <div class="docs-card-icon">"05"</div>
                        <div>
                            <h3>"Export & Deploy"</h3>
                            <p>"Build for Windows, macOS, Linux, Android, iOS, and tvOS."</p>
                        </div>
                    </a>
                    <a href="/docs/extending/plugins" class="docs-card-lg">
                        <div class="docs-card-icon">"06"</div>
                        <div>
                            <h3>"Extending the Engine"</h3>
                            <p>"Build plugins, custom nodes, post-processing effects, and contribute to the source."</p>
                        </div>
                    </a>
                </div>
            </div>
        </div>
    }
}

/// Individual doc page — rendered from slug.
#[component]
pub fn DocArticle() -> impl IntoView {
    // For now, render placeholder content based on URL.
    // This will be replaced with DB-backed content later.
    view! {
        <div class="docs-layout">
            <DocsSidebar active_slug="" />
            <div class="docs-content">
                <article class="doc-article">
                    <p class="doc-breadcrumb">
                        <a href="/docs">"Docs"</a>
                        " / "
                        <span>"Page"</span>
                    </p>
                    <h1>"Documentation"</h1>
                    <p class="doc-lead">"This page is under construction. Check back soon."</p>
                </article>
            </div>
        </div>
    }
}

/// Sidebar navigation for docs — always visible.
#[component]
fn DocsSidebar(active_slug: &'static str) -> impl IntoView {
    view! {
        <aside class="docs-sidebar">
            <div class="docs-sidebar-inner">
                <SidebarSection
                    title="Getting Started"
                    links=vec![
                        ("/docs/getting-started/installation", "Installation"),
                        ("/docs/getting-started/first-project", "Your First Project"),
                        ("/docs/getting-started/editor-overview", "Editor Overview"),
                        ("/docs/getting-started/concepts", "Core Concepts"),
                    ]
                />
                <SidebarSection
                    title="Editor"
                    links=vec![
                        ("/docs/editor/scenes", "Scenes & Hierarchy"),
                        ("/docs/editor/inspector", "Inspector"),
                        ("/docs/editor/viewport", "Viewport & Camera"),
                        ("/docs/editor/materials", "Material Editor"),
                        ("/docs/editor/terrain", "Terrain"),
                        ("/docs/editor/animation", "Animation"),
                        ("/docs/editor/audio", "Audio"),
                        ("/docs/editor/layouts", "Layouts & Panels"),
                        ("/docs/editor/keybindings", "Keyboard Shortcuts"),
                    ]
                />
                <SidebarSection
                    title="Scripting"
                    links=vec![
                        ("/docs/scripting/overview", "Overview"),
                        ("/docs/scripting/lua", "Lua"),
                        ("/docs/scripting/rhai", "Rhai"),
                        ("/docs/scripting/blueprints", "Visual Blueprints"),
                        ("/docs/scripting/api-reference", "API Reference"),
                        ("/docs/scripting/events", "Events & Lifecycle"),
                        ("/docs/scripting/entities", "Entities & Components"),
                        ("/docs/scripting/input", "Input Handling"),
                        ("/docs/scripting/physics", "Physics"),
                        ("/docs/scripting/ui", "Game UI"),
                    ]
                />
                <SidebarSection
                    title="Networking"
                    links=vec![
                        ("/docs/networking/overview", "Overview"),
                        ("/docs/networking/server-setup", "Server Setup"),
                        ("/docs/networking/replication", "State Replication"),
                        ("/docs/networking/input", "Networked Input"),
                        ("/docs/networking/rooms", "Rooms & Lobbies"),
                    ]
                />
                <SidebarSection
                    title="Export & Deploy"
                    links=vec![
                        ("/docs/export/overview", "Overview"),
                        ("/docs/export/windows", "Windows"),
                        ("/docs/export/linux", "Linux"),
                        ("/docs/export/macos", "macOS"),
                        ("/docs/export/android", "Android"),
                        ("/docs/export/ios", "iOS & tvOS"),
                    ]
                />
                <SidebarSection
                    title="Extending"
                    links=vec![
                        ("/docs/extending/plugins", "Building Plugins"),
                        ("/docs/extending/custom-nodes", "Custom Blueprint Nodes"),
                        ("/docs/extending/post-processing", "Post-Processing Effects"),
                        ("/docs/extending/contributing", "Contributing to Renzora"),
                    ]
                />
                <SidebarSection
                    title="Marketplace"
                    links=vec![
                        ("/docs/marketplace/browsing", "Browsing & Installing"),
                        ("/docs/marketplace/publishing", "Publishing Assets"),
                        ("/docs/marketplace/credits", "Credits System"),
                    ]
                />
            </div>
        </aside>
    }
}

#[component]
fn SidebarSection(title: &'static str, links: Vec<(&'static str, &'static str)>) -> impl IntoView {
    view! {
        <div class="sidebar-section">
            <h4 class="sidebar-heading">{title}</h4>
            <ul class="sidebar-links">
                {links.into_iter().map(|(href, label)| {
                    view! {
                        <li><a href=href class="sidebar-link">{label}</a></li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
