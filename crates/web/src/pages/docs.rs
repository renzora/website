use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

/// Documentation landing page.
#[component]
pub fn DocsPage() -> impl IntoView {
    view! {
        <div class="flex min-h-[calc(100vh-56px)]">
            <DocsSidebar />
            <div class="flex-1 min-w-0 px-8 py-10 lg:px-12 max-w-[860px]">
                <div class="mb-10">
                    <h1 class="text-4xl font-bold">"Documentation"</h1>
                    <p class="text-zinc-400 mt-2">"Everything you need to build games with Renzora Engine."</p>
                </div>

                <div class="flex flex-col gap-3">
                    <DocCard num="01" title="Getting Started" desc="Install the engine, create your first project, and learn the editor basics." href="/docs/getting-started/installation" />
                    <DocCard num="02" title="Editor Guide" desc="Master scenes, the inspector, materials, terrain, and the docking panel system." href="/docs/editor/scenes" />
                    <DocCard num="03" title="Scripting" desc="Write game logic with Lua, Rhai, or visual blueprints. Full API reference included." href="/docs/scripting/overview" />
                    <DocCard num="04" title="Multiplayer" desc="Set up dedicated servers, replicate state, and handle player input." href="/docs/networking/overview" />
                    <DocCard num="05" title="Export & Deploy" desc="Build for Windows, macOS, Linux, Android, iOS, and tvOS." href="/docs/export/overview" />
                    <DocCard num="06" title="Extending the Engine" desc="Build plugins, custom nodes, post-processing effects, and contribute to the source." href="/docs/extending/plugins" />
                </div>
            </div>
        </div>
    }
}

#[component]
fn DocCard(num: &'static str, title: &'static str, desc: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <a href=href class="flex items-center gap-5 p-5 bg-surface-card border border-zinc-800 rounded-lg hover:border-accent transition-colors">
            <div class="w-12 h-12 flex items-center justify-center bg-accent-subtle rounded-lg text-accent font-extrabold text-xl shrink-0">{num}</div>
            <div>
                <h3 class="text-sm font-semibold mb-0.5">{title}</h3>
                <p class="text-xs text-zinc-400 leading-relaxed">{desc}</p>
            </div>
        </a>
    }
}

/// Individual doc page rendered from URL params.
#[component]
pub fn DocArticle() -> impl IntoView {
    let params = use_params_map();
    let category = move || params.read().get("category");
    let slug = move || params.read().get("slug");

    let content = move || {
        let cat = category().unwrap_or_default();
        let sl = slug().unwrap_or_default();
        render_doc_content(&cat, &sl)
    };

    view! {
        <div class="flex min-h-[calc(100vh-56px)]">
            <DocsSidebar />
            <div class="flex-1 min-w-0 px-8 py-10 lg:px-12 max-w-[860px]">
                <article class="doc-article">
                    {content}
                </article>
            </div>
        </div>
    }
}

/// Sidebar navigation for docs.
#[component]
fn DocsSidebar() -> impl IntoView {
    view! {
        <aside class="docs-sidebar w-64 shrink-0 border-r border-zinc-800 bg-surface sticky top-14 h-[calc(100vh-56px)] overflow-y-auto hidden lg:block">
            <div class="p-4">
                <SidebarSection title="Getting Started" links=vec![
                    ("/docs/getting-started/installation", "Installation"),
                    ("/docs/getting-started/first-project", "Your First Project"),
                    ("/docs/getting-started/editor-overview", "Editor Overview"),
                    ("/docs/getting-started/concepts", "Core Concepts"),
                ] />
                <SidebarSection title="Editor" links=vec![
                    ("/docs/editor/scenes", "Scenes & Hierarchy"),
                    ("/docs/editor/inspector", "Inspector"),
                    ("/docs/editor/viewport", "Viewport & Camera"),
                    ("/docs/editor/materials", "Material Editor"),
                    ("/docs/editor/terrain", "Terrain"),
                    ("/docs/editor/animation", "Animation"),
                    ("/docs/editor/audio", "Audio"),
                    ("/docs/editor/layouts", "Layouts & Panels"),
                    ("/docs/editor/keybindings", "Keyboard Shortcuts"),
                ] />
                <SidebarSection title="Scripting" links=vec![
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
                ] />
                <SidebarSection title="Networking" links=vec![
                    ("/docs/networking/overview", "Overview"),
                    ("/docs/networking/server-setup", "Server Setup"),
                    ("/docs/networking/replication", "State Replication"),
                    ("/docs/networking/input", "Networked Input"),
                    ("/docs/networking/rooms", "Rooms & Lobbies"),
                ] />
                <SidebarSection title="Export & Deploy" links=vec![
                    ("/docs/export/overview", "Overview"),
                    ("/docs/export/windows", "Windows"),
                    ("/docs/export/linux", "Linux"),
                    ("/docs/export/macos", "macOS"),
                    ("/docs/export/android", "Android"),
                    ("/docs/export/ios", "iOS & tvOS"),
                ] />
                <SidebarSection title="Extending" links=vec![
                    ("/docs/extending/plugins", "Building Plugins"),
                    ("/docs/extending/custom-nodes", "Custom Blueprint Nodes"),
                    ("/docs/extending/post-processing", "Post-Processing Effects"),
                    ("/docs/extending/contributing", "Contributing to Renzora"),
                ] />
                <SidebarSection title="Marketplace" links=vec![
                    ("/docs/marketplace/browsing", "Browsing & Installing"),
                    ("/docs/marketplace/publishing", "Publishing Assets"),
                    ("/docs/marketplace/credits", "Credits System"),
                ] />
            </div>
        </aside>
    }
}

#[component]
fn SidebarSection(title: &'static str, links: Vec<(&'static str, &'static str)>) -> impl IntoView {
    view! {
        <div class="mb-6">
            <h4 class="text-[11px] font-semibold uppercase tracking-[0.08em] text-zinc-500 mb-2 px-2">{title}</h4>
            <ul class="flex flex-col gap-px">
                {links.into_iter().map(|(href, label)| {
                    view! {
                        <li><a href=href class="block px-2 py-1.5 text-[13px] text-zinc-400 rounded hover:text-zinc-50 hover:bg-white/5 transition-all">{label}</a></li>
                    }
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}

// ── Documentation Content ──
// Article helper classes (reused across all doc pages):
// h1: text-3xl font-bold mb-2
// h2: text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800
// h3: text-base font-semibold mt-7 mb-2
// p:  text-sm text-zinc-400 leading-relaxed mb-4
// ul/ol: pl-6 mb-4 list-disc/decimal
// li: text-sm text-zinc-400 leading-relaxed mb-1
// pre: p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono
// code (inline): bg-surface-card px-1.5 py-0.5 rounded text-[13px] font-mono
// blockquote: border-l-[3px] border-accent pl-4 py-3 my-4 bg-accent-subtle rounded-r-lg
// breadcrumb: text-xs text-zinc-500 mb-6
// a: text-accent hover:text-accent-hover

fn render_doc_content(category: &str, slug: &str) -> impl IntoView {
    match (category, slug) {
        ("getting-started", "installation") => doc_installation().into_any(),
        ("getting-started", "first-project") => doc_first_project().into_any(),
        ("getting-started", "editor-overview") => doc_editor_overview().into_any(),
        ("getting-started", "concepts") => doc_concepts().into_any(),
        ("editor", "scenes") => doc_scenes().into_any(),
        ("editor", "inspector") => doc_inspector().into_any(),
        ("editor", "viewport") => doc_viewport().into_any(),
        ("editor", "keybindings") => doc_keybindings().into_any(),
        ("scripting", "overview") => doc_scripting_overview().into_any(),
        ("scripting", "rhai") => doc_rhai().into_any(),
        ("scripting", "input") => doc_input().into_any(),
        ("scripting", "ui") => doc_game_ui().into_any(),
        ("export", "overview") => doc_export_overview().into_any(),
        ("networking", "overview") => doc_networking_overview().into_any(),
        _ => doc_placeholder(category, slug).into_any(),
    }
}

fn doc_placeholder(category: &str, slug: &str) -> impl IntoView {
    let title = slug.replace('-', " ");
    let cat = category.to_string();
    view! {
        <p class="text-xs text-zinc-500 mb-6"><a href="/docs" class="text-accent">"Docs"</a>" / "{cat}</p>
        <h1 class="text-3xl font-bold mb-2">{title}</h1>
        <p class="text-sm text-zinc-400">"This page is coming soon. Check back for updates."</p>
    }
}

fn doc_installation() -> impl IntoView {
    view! {
        <p class="text-xs text-zinc-500 mb-6"><a href="/docs" class="text-accent">"Docs"</a>" / Getting Started"</p>
        <h1 class="text-3xl font-bold mb-2">"Installation"</h1>
        <p class="text-sm text-zinc-400 leading-relaxed mb-8">"Get Renzora Engine running on your machine in a few minutes."</p>

        <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"System Requirements"</h2>
        <ul class="pl-6 mb-4 list-disc">
            <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"OS:"</strong>" Windows 10+, macOS 12+, or Ubuntu 22.04+"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"GPU:"</strong>" Any GPU with Vulkan, Metal, or DX12 support"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"RAM:"</strong>" 4 GB minimum, 8 GB recommended"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Disk:"</strong>" ~500 MB for the editor"</li>
        </ul>

        <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Download"</h2>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Head to the "<a href="/download" class="text-accent hover:text-accent-hover">"download page"</a>" and grab the installer for your platform."</p>

        <h3 class="text-base font-semibold mt-7 mb-2">"Windows"</h3>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Download the "<code class="bg-surface-card px-1.5 py-0.5 rounded text-[13px] font-mono">".exe"</code>" installer and run it. The editor will be added to your Start menu. Alternatively, download the portable "<code class="bg-surface-card px-1.5 py-0.5 rounded text-[13px] font-mono">".zip"</code>" and extract it anywhere."</p>

        <h3 class="text-base font-semibold mt-7 mb-2">"macOS"</h3>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Download the "<code class="bg-surface-card px-1.5 py-0.5 rounded text-[13px] font-mono">".dmg"</code>", open it, and drag Renzora to your Applications folder."</p>
        <div class="border-l-[3px] border-accent pl-4 py-3 my-4 bg-accent-subtle rounded-r-lg">
            <p class="text-sm text-zinc-300 mb-0">"On first launch, you may need to right-click and choose Open, then confirm in the security dialog."</p>
        </div>

        <h3 class="text-base font-semibold mt-7 mb-2">"Linux"</h3>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Download the "<code class="bg-surface-card px-1.5 py-0.5 rounded text-[13px] font-mono">".AppImage"</code>", make it executable, and run it:"</p>
        <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"chmod +x Renzora-r1-alpha4.AppImage\n./Renzora-r1-alpha4.AppImage"</pre>

        <h3 class="text-base font-semibold mt-7 mb-2">"Build from source"</h3>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"If you prefer to compile from source, you'll need Rust 1.85+ and Git:"</p>
        <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"git clone https://github.com/renzora/engine.git\ncd engine\ncargo run --release"</pre>

        <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"What's next?"</h2>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Now that you have the engine installed, "<a href="/docs/getting-started/first-project" class="text-accent hover:text-accent-hover">"create your first project"</a>"."</p>
    }
}

fn doc_first_project() -> impl IntoView {
    view! {
        <p class="text-xs text-zinc-500 mb-6"><a href="/docs" class="text-accent">"Docs"</a>" / Getting Started"</p>
        <h1 class="text-3xl font-bold mb-2">"Your First Project"</h1>
        <p class="text-sm text-zinc-400 leading-relaxed mb-8">"Create a new project, add some objects to your scene, and hit play."</p>

        <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Creating a project"</h2>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"When you launch Renzora, you'll see a project browser. Click "<strong class="text-zinc-50">"New Project"</strong>", give it a name, and choose a location on disk."</p>
        <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"my-game/\n\u{251C}\u{2500}\u{2500} project.toml      # project settings\n\u{251C}\u{2500}\u{2500} scenes/\n\u{2502}   \u{2514}\u{2500}\u{2500} main.ron      # your startup scene\n\u{251C}\u{2500}\u{2500} scripts/          # Lua/Rhai scripts\n\u{251C}\u{2500}\u{2500} textures/         # images & sprites\n\u{251C}\u{2500}\u{2500} audio/            # sound effects & music\n\u{2514}\u{2500}\u{2500} materials/        # material graph files"</pre>

        <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"The project.toml file"</h2>
        <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"[project]\nname = \"my-game\"\nversion = \"0.1.0\"\n\n[window]\nresolution = [1280, 720]\nfullscreen = false\ntitle = \"My Game\"\n\n[scene]\nmain = \"scenes/main.ron\""</pre>

        <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Adding objects to the scene"</h2>
        <ol class="pl-6 mb-4 list-decimal">
            <li class="text-sm text-zinc-400 leading-relaxed mb-1">"Click the "<strong class="text-zinc-50">"+"</strong>" button in the Hierarchy panel (left side)"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1">"Choose "<strong class="text-zinc-50">"3D \u{2192} Cube"</strong>" from the menu"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1">"A cube appears in the viewport. Use the gizmo to move it around."</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1">"Add a light: "<strong class="text-zinc-50">"+ \u{2192} Light \u{2192} Directional Light"</strong></li>
        </ol>

        <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Running your game"</h2>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Press "<code class="bg-surface-card px-1.5 py-0.5 rounded text-[13px] font-mono">"F5"</code>" to enter play mode. Press again to stop."</p>
        <div class="border-l-[3px] border-accent pl-4 py-3 my-4 bg-accent-subtle rounded-r-lg">
            <p class="text-sm text-zinc-300 mb-0"><strong>"Tip:"</strong>" Use "<code class="bg-surface-card px-1.5 py-0.5 rounded text-[13px] font-mono">"Shift+F5"</code>" to run scripts without switching to the game camera."</p>
        </div>

        <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Saving"</h2>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Press "<code class="bg-surface-card px-1.5 py-0.5 rounded text-[13px] font-mono">"Ctrl+S"</code>" to save. Scenes are stored as "<code class="bg-surface-card px-1.5 py-0.5 rounded text-[13px] font-mono">".ron"</code>" files."</p>

        <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"What's next?"</h2>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Learn the editor in the "<a href="/docs/getting-started/editor-overview" class="text-accent hover:text-accent-hover">"Editor Overview"</a>", or jump to "<a href="/docs/scripting/overview" class="text-accent hover:text-accent-hover">"Scripting"</a>"."</p>
    }
}

fn doc_editor_overview() -> impl IntoView {
    view! {
        <p class="text-xs text-zinc-500 mb-6"><a href="/docs" class="text-accent">"Docs"</a>" / Getting Started"</p>
        <h1 class="text-3xl font-bold mb-2">"Editor Overview"</h1>
        <p class="text-sm text-zinc-400 leading-relaxed mb-8">"A tour of the Renzora editor interface and its main panels."</p>

        <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Layout"</h2>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"The editor uses a dockable panel system. The default layout:"</p>
        <ul class="pl-6 mb-4 list-disc">
            <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Title bar"</strong>" \u{2014} file menu, workspace tabs, play controls, settings"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Viewport"</strong>" (center) \u{2014} your 3D/2D scene view"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Hierarchy"</strong>" (left) \u{2014} tree of all entities"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Inspector"</strong>" (right) \u{2014} properties of selected entity"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Asset browser"</strong>" (bottom) \u{2014} project files"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Console"</strong>" (bottom) \u{2014} logs and script output"</li>
        </ul>

        <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Workspaces"</h2>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Workspace tabs at the top switch between panel arrangements:"</p>
        <ul class="pl-6 mb-4 list-disc">
            <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Scene"</strong>" \u{2014} default layout for level editing"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Materials"</strong>" \u{2014} material graph editor with preview"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Animation"</strong>" \u{2014} timeline and keyframe editor"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Audio"</strong>" \u{2014} audio mixer (DAW-style)"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"UI"</strong>" \u{2014} game UI canvas editor"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Network"</strong>" \u{2014} multiplayer configuration"</li>
        </ul>

        <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"What's next?"</h2>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Learn about "<a href="/docs/getting-started/concepts" class="text-accent hover:text-accent-hover">"Core Concepts"</a>"."</p>
    }
}

fn doc_concepts() -> impl IntoView {
    view! {
        <p class="text-xs text-zinc-500 mb-6"><a href="/docs" class="text-accent">"Docs"</a>" / Getting Started"</p>
        <h1 class="text-3xl font-bold mb-2">"Core Concepts"</h1>
        <p class="text-sm text-zinc-400 leading-relaxed mb-8">"Entities, components, scenes, and scripts \u{2014} the building blocks of Renzora."</p>

        <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Entities"</h2>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"An entity is a thing in your game world. By themselves, entities are empty containers. They gain behavior through components."</p>

        <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Components"</h2>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Components are data attached to entities. A player character might have:"</p>
        <ul class="pl-6 mb-4 list-disc">
            <li class="text-sm text-zinc-400 leading-relaxed mb-1">"Transform \u{2014} position in the world"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1">"Mesh \u{2014} the 3D model"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1">"Material \u{2014} visual appearance"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1">"Rigid Body \u{2014} physics simulation"</li>
            <li class="text-sm text-zinc-400 leading-relaxed mb-1">"Script \u{2014} game logic"</li>
        </ul>

        <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Scenes"</h2>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"A scene is a collection of entities saved as a "<code class="bg-surface-card px-1.5 py-0.5 rounded text-[13px] font-mono">".ron"</code>" file. Your game can have multiple scenes. The startup scene is defined in "<code class="bg-surface-card px-1.5 py-0.5 rounded text-[13px] font-mono">"project.toml"</code>"."</p>

        <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Scripts"</h2>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Three scripting approaches: "<strong class="text-zinc-50">"Rhai"</strong>", "<strong class="text-zinc-50">"Lua"</strong>", and "<strong class="text-zinc-50">"Visual Blueprints"</strong>". Scripts run "<code class="bg-surface-card px-1.5 py-0.5 rounded text-[13px] font-mono">"on_ready()"</code>" when spawned and "<code class="bg-surface-card px-1.5 py-0.5 rounded text-[13px] font-mono">"on_update()"</code>" every frame."</p>

        <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"What's next?"</h2>
        <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Dive into the "<a href="/docs/editor/scenes" class="text-accent hover:text-accent-hover">"Editor Guide"</a>" or start "<a href="/docs/scripting/overview" class="text-accent hover:text-accent-hover">"Scripting"</a>"."</p>
    }
}

fn doc_scenes() -> impl IntoView { view! {
    <p class="text-xs text-zinc-500 mb-6"><a href="/docs" class="text-accent">"Docs"</a>" / Editor"</p>
    <h1 class="text-3xl font-bold mb-2">"Scenes & Hierarchy"</h1>
    <p class="text-sm text-zinc-400 leading-relaxed mb-8">"Organize your game world with the scene hierarchy."</p>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Adding entities"</h2>
    <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Click + in the Hierarchy to add: 3D objects, 2D sprites, lights, cameras, audio, physics, UI, or empty entities."</p>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Transform tools"</h2>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"W   Translate\nE   Rotate\nR   Scale\nG   Grab (Blender-style)\nQ   Select mode"</pre>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Parenting"</h2>
    <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Drag an entity onto another to make it a child. Children inherit their parent's transform."</p>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Scene operations"</h2>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"Ctrl+N   New scene\nCtrl+O   Open scene\nCtrl+S   Save\nCtrl+D   Duplicate\nDelete   Delete"</pre>
}}

fn doc_inspector() -> impl IntoView { view! {
    <p class="text-xs text-zinc-500 mb-6"><a href="/docs" class="text-accent">"Docs"</a>" / Editor"</p>
    <h1 class="text-3xl font-bold mb-2">"Inspector"</h1>
    <p class="text-sm text-zinc-400 leading-relaxed mb-8">"View and edit components on the selected entity."</p>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Common components"</h2>
    <ul class="pl-6 mb-4 list-disc">
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Transform"</strong>" \u{2014} position, rotation, scale"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Mesh"</strong>" \u{2014} 3D shape"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Material"</strong>" \u{2014} visual appearance"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Script"</strong>" \u{2014} game logic with exposed properties"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Rigid Body & Collider"</strong>" \u{2014} physics"</li>
    </ul>
    <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Click \"Add Component\" at the bottom to attach new components."</p>
}}

fn doc_viewport() -> impl IntoView { view! {
    <p class="text-xs text-zinc-500 mb-6"><a href="/docs" class="text-accent">"Docs"</a>" / Editor"</p>
    <h1 class="text-3xl font-bold mb-2">"Viewport & Camera"</h1>
    <p class="text-sm text-zinc-400 leading-relaxed mb-8">"Navigate the 3D scene."</p>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Navigation"</h2>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"Right-click + WASDQE   Fly\nAlt + Left-click       Orbit\nMiddle-click drag      Pan\nScroll wheel           Zoom\nF                      Focus selected\nShift                  Move faster"</pre>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"View presets"</h2>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"Numpad 1/3/7       Front / Right / Top\nCtrl+Numpad 1/3/7  Back / Left / Bottom\nNumpad 5            Toggle perspective/ortho\nZ                   Wireframe\nShift+Z             Toggle lighting\nH                   Toggle grid"</pre>
}}

fn doc_keybindings() -> impl IntoView { view! {
    <p class="text-xs text-zinc-500 mb-6"><a href="/docs" class="text-accent">"Docs"</a>" / Editor"</p>
    <h1 class="text-3xl font-bold mb-2">"Keyboard Shortcuts"</h1>
    <p class="text-sm text-zinc-400 leading-relaxed mb-8">"Quick reference for all editor shortcuts."</p>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Tools"</h2>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"Q   Select\nW   Translate\nE   Rotate\nR   Scale\nG   Grab (Blender-style)"</pre>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Selection"</h2>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"Click          Select\nCtrl+Click     Toggle\nShift+Click    Add\nEscape         Deselect\nCtrl+D         Duplicate\nDelete         Delete"</pre>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"File"</h2>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"Ctrl+N         New scene\nCtrl+O         Open\nCtrl+S         Save\nCtrl+Shift+S   Save as\nCtrl+Z         Undo\nCtrl+Y         Redo"</pre>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Play"</h2>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"F5             Play / Stop\nShift+F5       Scripts only"</pre>
}}

fn doc_scripting_overview() -> impl IntoView { view! {
    <p class="text-xs text-zinc-500 mb-6"><a href="/docs" class="text-accent">"Docs"</a>" / Scripting"</p>
    <h1 class="text-3xl font-bold mb-2">"Scripting Overview"</h1>
    <p class="text-sm text-zinc-400 leading-relaxed mb-8">"Add game logic with Rhai, Lua, or visual blueprints."</p>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Three ways to script"</h2>
    <ul class="pl-6 mb-4 list-disc">
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><a href="/docs/scripting/rhai" class="text-accent hover:text-accent-hover font-medium">"Rhai"</a>" \u{2014} lightweight, Rust-native scripting"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><a href="/docs/scripting/lua" class="text-accent hover:text-accent-hover font-medium">"Lua"</a>" \u{2014} industry-standard game scripting"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><a href="/docs/scripting/blueprints" class="text-accent hover:text-accent-hover font-medium">"Blueprints"</a>" \u{2014} visual node graphs, no coding"</li>
    </ul>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Built-in variables"</h2>
    <h3 class="text-base font-semibold mt-7 mb-2">"Time"</h3>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"delta      Seconds since last frame\nelapsed    Total seconds since start"</pre>
    <h3 class="text-base font-semibold mt-7 mb-2">"Transform"</h3>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"position_x/y/z   Entity position (read/write)\nrotation_x/y/z   Entity rotation\nscale_x/y/z      Entity scale"</pre>
    <h3 class="text-base font-semibold mt-7 mb-2">"Input"</h3>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"input_x/y                     WASD axes (-1 to 1)\nmouse_x/y                     Mouse position\nmouse_delta_x/y               Mouse movement\nmouse_button_left/right/middle Mouse buttons"</pre>
    <h3 class="text-base font-semibold mt-7 mb-2">"Gamepad"</h3>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"gamepad_left_x/y    Left stick\ngamepad_right_x/y   Right stick\ngamepad_south/east   Face buttons\ngamepad_left/right_trigger Triggers"</pre>
}}

fn doc_rhai() -> impl IntoView { view! {
    <p class="text-xs text-zinc-500 mb-6"><a href="/docs" class="text-accent">"Docs"</a>" / Scripting"</p>
    <h1 class="text-3xl font-bold mb-2">"Rhai Scripting"</h1>
    <p class="text-sm text-zinc-400 leading-relaxed mb-8">"Write game logic with Rhai \u{2014} a lightweight, Rust-native scripting language."</p>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Your first script"</h2>
    <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Create "<code class="bg-surface-card px-1.5 py-0.5 rounded text-[13px] font-mono">"scripts/player.rhai"</code>":"</p>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"fn on_ready() {\n    print(\"Player spawned!\");\n}\n\nfn on_update() {\n    let speed = 5.0;\n    position_x += input_x * speed * delta;\n    position_z += input_y * speed * delta;\n}"</pre>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Script properties"</h2>
    <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Expose variables to the Inspector:"</p>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"fn props() {\n    #{\n        speed: #{ default: 5.0, min: 0.0, max: 100.0 },\n        jump_force: #{ default: 10.0, min: 0.0, max: 50.0 },\n        can_fly: #{ default: false }\n    }\n}"</pre>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Collisions"</h2>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"fn on_update() {\n    if collisions_entered > 0 {\n        print(\"Hit something!\");\n    }\n}"</pre>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Syntax basics"</h2>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"let x = 42;\nlet name = \"hello\";\n\nfn add(a, b) { a + b }\n\nif x > 10 { print(\"big\"); }\n\nfor i in 0..10 { print(i); }"</pre>
}}

fn doc_input() -> impl IntoView { view! {
    <p class="text-xs text-zinc-500 mb-6"><a href="/docs" class="text-accent">"Docs"</a>" / Scripting"</p>
    <h1 class="text-3xl font-bold mb-2">"Input Handling"</h1>
    <p class="text-sm text-zinc-400 leading-relaxed mb-8">"Read keyboard, mouse, and gamepad input."</p>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Keyboard"</h2>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"fn on_update() {\n    let speed = 5.0;\n    position_x += input_x * speed * delta;\n    position_z += input_y * speed * delta;\n}"</pre>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Mouse"</h2>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"fn on_update() {\n    let dx = mouse_delta_x;\n    let dy = mouse_delta_y;\n    if mouse_button_left {\n        print(\"Shooting!\");\n    }\n}"</pre>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Gamepad"</h2>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"fn on_update() {\n    position_x += gamepad_left_x * 5.0 * delta;\n    if gamepad_south { print(\"Jump!\"); }\n    if gamepad_right_trigger > 0.5 { print(\"Fire!\"); }\n}"</pre>
}}

fn doc_game_ui() -> impl IntoView { view! {
    <p class="text-xs text-zinc-500 mb-6"><a href="/docs" class="text-accent">"Docs"</a>" / Scripting"</p>
    <h1 class="text-3xl font-bold mb-2">"Game UI"</h1>
    <p class="text-sm text-zinc-400 leading-relaxed mb-8">"Control in-game UI elements from scripts."</p>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Script functions"</h2>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"ui_show(\"health_bar\");\nui_hide(\"main_menu\");\nui_toggle(\"inventory\");\nui_set_text(\"score\", \"Score: 1500\");\nui_set_progress(\"loading\", 0.75);\nui_set_health(\"hp\", 80, 100);\nui_set_slider(\"volume\", 0.5);\nui_set_checkbox(\"vsync\", true);\nui_set_color(\"flash\", 255, 0, 0, 128);\nui_set_theme(\"dark\");"</pre>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Widget types"</h2>
    <ul class="pl-6 mb-4 list-disc">
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Layout:"</strong>" Canvas, Panel, Scroll Area, Grid"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Basic:"</strong>" Text, Image, Button"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Input:"</strong>" Slider, Checkbox, Toggle, Radio, Dropdown, Text Input"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Display:"</strong>" Progress Bar, Health Bar, Tab Bar, Spinner"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Overlay:"</strong>" Tooltip, Modal, Draggable Window"</li>
    </ul>
}}

fn doc_export_overview() -> impl IntoView { view! {
    <p class="text-xs text-zinc-500 mb-6"><a href="/docs" class="text-accent">"Docs"</a>" / Export & Deploy"</p>
    <h1 class="text-3xl font-bold mb-2">"Export Overview"</h1>
    <p class="text-sm text-zinc-400 leading-relaxed mb-8">"Build your game for 10 platforms from a single project."</p>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Supported platforms"</h2>
    <ul class="pl-6 mb-4 list-disc">
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Windows"</strong>" (x64) \u{2192} .exe"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Linux"</strong>" (x64) \u{2192} executable"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"macOS"</strong>" (Intel + Apple Silicon) \u{2192} .app"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Android"</strong>" (ARM64, x86_64) \u{2192} .apk"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"iOS"</strong>" (ARM64) \u{2192} .ipa"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"tvOS"</strong>" (ARM64) \u{2192} .ipa"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"Web"</strong>" (WASM) \u{2192} .wasm"</li>
    </ul>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"How to export"</h2>
    <ol class="pl-6 mb-4 list-decimal">
        <li class="text-sm text-zinc-400 leading-relaxed mb-1">"Open your project"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1">"File \u{2192} Export"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1">"Select platform, configure settings"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1">"Click Export \u{2014} assets are packed into a .rpak archive"</li>
    </ol>
}}

fn doc_networking_overview() -> impl IntoView { view! {
    <p class="text-xs text-zinc-500 mb-6"><a href="/docs" class="text-accent">"Docs"</a>" / Networking"</p>
    <h1 class="text-3xl font-bold mb-2">"Networking Overview"</h1>
    <p class="text-sm text-zinc-400 leading-relaxed mb-8">"Build multiplayer games with dedicated server support."</p>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Architecture"</h2>
    <p class="text-sm text-zinc-400 leading-relaxed mb-4">"Dedicated server model \u{2014} the server runs the authoritative simulation, clients send input and receive state."</p>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Transport"</h2>
    <ul class="pl-6 mb-4 list-disc">
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"UDP"</strong>" \u{2014} lowest latency"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"WebTransport"</strong>" \u{2014} modern web protocol"</li>
        <li class="text-sm text-zinc-400 leading-relaxed mb-1"><strong class="text-zinc-50">"WebSocket"</strong>" \u{2014} widest compatibility"</li>
    </ul>
    <h2 class="text-xl font-semibold mt-10 mb-3 pb-2 border-b border-zinc-800">"Configuration"</h2>
    <pre class="p-4 bg-surface-card border border-zinc-800 rounded-lg text-[13px] leading-relaxed mb-5 overflow-x-auto font-mono text-zinc-300">"[network]\nserver_addr = \"127.0.0.1\"\nport = 5000\ntransport = \"udp\"\ntick_rate = 60\nmax_clients = 16"</pre>
}}
