use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

/// Documentation landing page.
#[component]
pub fn DocsPage() -> impl IntoView {
    view! {
        <div class="docs-layout">
            <DocsSidebar />
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
                    <a href="/docs/scripting/overview" class="docs-card-lg">
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
        <div class="docs-layout">
            <DocsSidebar />
            <div class="docs-content">
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

// ── Documentation Content ──

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
        <p class="doc-breadcrumb"><a href="/docs">"Docs"</a>" / "{cat}</p>
        <h1>{title}</h1>
        <p class="doc-lead">"This page is coming soon. Check back for updates."</p>
    }
}

fn doc_installation() -> impl IntoView {
    view! {
        <p class="doc-breadcrumb"><a href="/docs">"Docs"</a>" / Getting Started"</p>
        <h1>"Installation"</h1>
        <p class="doc-lead">"Get Renzora Engine running on your machine in a few minutes."</p>

        <h2>"System Requirements"</h2>
        <ul>
            <li><strong>"OS:"</strong>" Windows 10+, macOS 12+, or Ubuntu 22.04+ (other Linux distros work too)"</li>
            <li><strong>"GPU:"</strong>" Any GPU with Vulkan, Metal, or DX12 support"</li>
            <li><strong>"RAM:"</strong>" 4 GB minimum, 8 GB recommended"</li>
            <li><strong>"Disk:"</strong>" ~500 MB for the editor"</li>
        </ul>

        <h2>"Download"</h2>
        <p>"Head to the "<a href="/download">"download page"</a>" and grab the installer for your platform."</p>

        <h3>"Windows"</h3>
        <p>"Download the "<code>".exe"</code>" installer and run it. The editor will be added to your Start menu."</p>
        <p>"Alternatively, download the portable "<code>".zip"</code>" and extract it anywhere."</p>

        <h3>"macOS"</h3>
        <p>"Download the "<code>".dmg"</code>", open it, and drag Renzora to your Applications folder."</p>
        <blockquote><p>"On first launch, you may need to right-click and choose Open, then confirm in the security dialog."</p></blockquote>

        <h3>"Linux"</h3>
        <p>"Download the "<code>".AppImage"</code>", make it executable, and run it:"</p>
        <pre><code>"chmod +x Renzora-r1-alpha4.AppImage\n./Renzora-r1-alpha4.AppImage"</code></pre>
        <p>"Debian/Ubuntu users can also use the "<code>".deb"</code>" package."</p>

        <h3>"Build from source"</h3>
        <p>"If you prefer to compile from source, you'll need Rust 1.85+ and Git:"</p>
        <pre><code>"git clone https://github.com/renzora/engine.git\ncd engine\ncargo run --release"</code></pre>

        <h2>"What's next?"</h2>
        <p>"Now that you have the engine installed, "<a href="/docs/getting-started/first-project">"create your first project"</a>"."</p>
    }
}

fn doc_first_project() -> impl IntoView {
    view! {
        <p class="doc-breadcrumb"><a href="/docs">"Docs"</a>" / Getting Started"</p>
        <h1>"Your First Project"</h1>
        <p class="doc-lead">"Create a new project, add some objects to your scene, and hit play."</p>

        <h2>"Creating a project"</h2>
        <p>"When you launch Renzora, you'll see a project browser. Click "<strong>"New Project"</strong>", give it a name, and choose a location on disk."</p>
        <p>"This creates a project folder with the following structure:"</p>
        <pre><code>"my-game/\n├── project.toml      # project settings\n├── scenes/\n│   └── main.ron      # your startup scene\n├── scripts/          # Lua/Rhai scripts\n├── textures/         # images & sprites\n├── audio/            # sound effects & music\n└── materials/        # material graph files"</code></pre>

        <h2>"The project.toml file"</h2>
        <p>"This is your project's configuration. It looks like this:"</p>
        <pre><code>"[project]\nname = \"my-game\"\nversion = \"0.1.0\"\n\n[window]\nresolution = [1280, 720]\nfullscreen = false\ntitle = \"My Game\"\n\n[scene]\nmain = \"scenes/main.ron\""</code></pre>

        <h2>"Adding objects to the scene"</h2>
        <p>"The editor opens with an empty scene. Let's add something:"</p>
        <ol>
            <li>"Click the "<strong>"+"</strong>" button in the Hierarchy panel (left side)"</li>
            <li>"Choose "<strong>"3D → Cube"</strong>" from the menu"</li>
            <li>"A cube appears in the viewport. Use the gizmo to move it around."</li>
            <li>"Add a light: "<strong>"+ → Light → Directional Light"</strong></li>
        </ol>

        <h2>"Running your game"</h2>
        <p>"Press "<code>"F5"</code>" (or click the play button in the title bar) to enter play mode. The viewport switches to the game camera and your scripts start running."</p>
        <p>"Press "<code>"F5"</code>" again to stop and return to the editor."</p>
        <blockquote><p><strong>"Tip:"</strong>" Use "<code>"Shift+F5"</code>" to run scripts without switching to the game camera. This is useful for testing script logic while still having editor controls."</p></blockquote>

        <h2>"Saving"</h2>
        <p>"Press "<code>"Ctrl+S"</code>" to save your scene. Scenes are stored as "<code>".ron"</code>" files in the "<code>"scenes/"</code>" folder."</p>

        <h2>"What's next?"</h2>
        <p>"Learn the editor interface in the "<a href="/docs/getting-started/editor-overview">"Editor Overview"</a>", or jump straight to "<a href="/docs/scripting/overview">"Scripting"</a>" to add game logic."</p>
    }
}

fn doc_editor_overview() -> impl IntoView {
    view! {
        <p class="doc-breadcrumb"><a href="/docs">"Docs"</a>" / Getting Started"</p>
        <h1>"Editor Overview"</h1>
        <p class="doc-lead">"A tour of the Renzora editor interface and its main panels."</p>

        <h2>"Layout"</h2>
        <p>"The editor uses a dockable panel system. You can drag panels, split them, and arrange them however you like. The default layout has:"</p>
        <ul>
            <li><strong>"Title bar"</strong>" — file menu, workspace tabs, play controls, settings, and sign-in"</li>
            <li><strong>"Viewport"</strong>" (center) — your 3D/2D scene view"</li>
            <li><strong>"Hierarchy"</strong>" (left) — tree of all entities in the scene"</li>
            <li><strong>"Inspector"</strong>" (right) — properties of the selected entity"</li>
            <li><strong>"Asset browser"</strong>" (bottom) — browse and manage project files"</li>
            <li><strong>"Console"</strong>" (bottom) — logs and script output"</li>
        </ul>

        <h2>"Workspaces"</h2>
        <p>"The title bar has workspace tabs at the top. Each workspace is a different panel arrangement optimized for a task:"</p>
        <ul>
            <li><strong>"Scene"</strong>" — default layout for level editing"</li>
            <li><strong>"Materials"</strong>" — material graph editor with preview"</li>
            <li><strong>"Animation"</strong>" — timeline and keyframe editor"</li>
            <li><strong>"Audio"</strong>" — audio mixer (DAW-style)"</li>
            <li><strong>"UI"</strong>" — game UI canvas editor"</li>
            <li><strong>"Network"</strong>" — multiplayer configuration"</li>
        </ul>
        <p>"You can create custom workspaces and save your own layouts."</p>

        <h2>"Key panels"</h2>
        <p>"Renzora has 30+ panels. Here are the most important ones:"</p>

        <h3>"Viewport"</h3>
        <p>"The main scene view. Navigate with WASD to fly, Alt+left-click to orbit, and scroll to zoom. It automatically switches between 3D and 2D mode based on the selected entity."</p>

        <h3>"Hierarchy"</h3>
        <p>"Shows every entity in the scene as a tree. Drag entities to reparent them. Right-click for options like duplicate, delete, and rename. Use the + button to add new entities."</p>

        <h3>"Inspector"</h3>
        <p>"Shows all components on the selected entity: transform, mesh, material, physics, scripts, and more. Click \"Add Component\" to attach new functionality."</p>

        <h3>"Asset Browser"</h3>
        <p>"A file browser for your project. Drag textures onto materials, scripts onto entities, and scenes into the hierarchy. Supports thumbnails for images and models."</p>

        <h2>"What's next?"</h2>
        <p>"Learn about "<a href="/docs/getting-started/concepts">"Core Concepts"</a>" like entities, components, and scenes."</p>
    }
}

fn doc_concepts() -> impl IntoView {
    view! {
        <p class="doc-breadcrumb"><a href="/docs">"Docs"</a>" / Getting Started"</p>
        <h1>"Core Concepts"</h1>
        <p class="doc-lead">"The fundamental building blocks of Renzora — entities, components, scenes, and scripts."</p>

        <h2>"Entities"</h2>
        <p>"An entity is a thing in your game world. A character, a light, a tree, a camera — they're all entities. By themselves, entities are empty containers. They only gain behavior through components."</p>

        <h2>"Components"</h2>
        <p>"Components are data attached to entities. A "<strong>"Transform"</strong>" component gives an entity a position, rotation, and scale. A "<strong>"Mesh"</strong>" component gives it a 3D shape. A "<strong>"Rigid Body"</strong>" gives it physics."</p>
        <p>"You build game objects by combining components. A player character might have:"</p>
        <ul>
            <li>"Transform — position in the world"</li>
            <li>"Mesh — the 3D model"</li>
            <li>"Material — the visual appearance"</li>
            <li>"Rigid Body — physics simulation"</li>
            <li>"Collider — collision detection"</li>
            <li>"Script — game logic (movement, health, etc.)"</li>
        </ul>

        <h2>"Scenes"</h2>
        <p>"A scene is a collection of entities saved as a "<code>".ron"</code>" file. Your game can have multiple scenes — a main menu, a gameplay level, a settings screen."</p>
        <p>"The startup scene is defined in "<code>"project.toml"</code>" and loads automatically when the game runs."</p>

        <h2>"Scripts"</h2>
        <p>"Scripts add custom behavior to entities. Renzora supports three scripting approaches:"</p>
        <ul>
            <li><strong>"Rhai"</strong>" — a lightweight scripting language designed for Rust. Great for gameplay logic."</li>
            <li><strong>"Lua"</strong>" — the industry-standard game scripting language. Familiar to most game developers."</li>
            <li><strong>"Blueprints"</strong>" — visual node graphs for logic. No coding required."</li>
        </ul>
        <p>"Scripts run two key functions: "<code>"on_ready()"</code>" when the entity spawns, and "<code>"on_update()"</code>" every frame."</p>

        <h2>"Materials"</h2>
        <p>"Materials define how surfaces look. Renzora uses a node-based material editor — you connect texture nodes, math nodes, and shader properties in a visual graph. Materials are saved as "<code>".material"</code>" files."</p>

        <h2>"The game loop"</h2>
        <p>"Every frame, the engine:"</p>
        <ol>
            <li>"Processes input (keyboard, mouse, gamepad)"</li>
            <li>"Runs scripts ("<code>"on_update"</code>" on every active script)"</li>
            <li>"Steps the physics simulation"</li>
            <li>"Updates transforms and animations"</li>
            <li>"Renders the frame"</li>
        </ol>
        <p>"You don't need to manage this loop yourself — just write your script logic and the engine handles the rest."</p>

        <h2>"What's next?"</h2>
        <p>"Dive into the "<a href="/docs/editor/scenes">"Editor Guide"</a>" to learn the tools, or start "<a href="/docs/scripting/overview">"Scripting"</a>" to add game logic."</p>
    }
}

fn doc_scenes() -> impl IntoView {
    view! {
        <p class="doc-breadcrumb"><a href="/docs">"Docs"</a>" / Editor"</p>
        <h1>"Scenes & Hierarchy"</h1>
        <p class="doc-lead">"Organize your game world with the scene hierarchy."</p>

        <h2>"The Hierarchy panel"</h2>
        <p>"The Hierarchy panel (left side of the editor) shows a tree of every entity in the current scene. Entities can be nested inside other entities to create parent-child relationships."</p>

        <h2>"Adding entities"</h2>
        <p>"Click the "<strong>"+"</strong>" button at the top of the Hierarchy to add new entities:"</p>
        <ul>
            <li><strong>"3D"</strong>" — Cube, Sphere, Plane, Cylinder, Capsule, and more"</li>
            <li><strong>"2D"</strong>" — Sprite, TileMap"</li>
            <li><strong>"Light"</strong>" — Directional, Point, Spot"</li>
            <li><strong>"Camera"</strong>" — 3D or 2D camera"</li>
            <li><strong>"Audio"</strong>" — Audio emitter, listener"</li>
            <li><strong>"Physics"</strong>" — Rigid body, collider"</li>
            <li><strong>"UI"</strong>" — Game UI canvas"</li>
            <li><strong>"Empty"</strong>" — an empty entity (use as an organizer)"</li>
        </ul>

        <h2>"Selecting and transforming"</h2>
        <p>"Click an entity in the Hierarchy or viewport to select it. The gizmo appears for moving, rotating, and scaling:"</p>
        <ul>
            <li><code>"W"</code>" — Translate (move)"</li>
            <li><code>"E"</code>" — Rotate"</li>
            <li><code>"R"</code>" — Scale"</li>
            <li><code>"Q"</code>" — Select mode (no gizmo)"</li>
        </ul>
        <p>"You can also use Blender-style modal transforms: press "<code>"G"</code>" to grab, "<code>"R"</code>" to rotate, or "<code>"S"</code>" to scale, then move the mouse."</p>

        <h2>"Parenting"</h2>
        <p>"Drag an entity onto another in the Hierarchy to make it a child. Children inherit their parent's transform — if you move the parent, the children move too."</p>
        <p>"This is useful for:"</p>
        <ul>
            <li>"Attaching a weapon to a character's hand"</li>
            <li>"Grouping objects together (e.g. a building made of parts)"</li>
            <li>"Creating pivot points for rotation"</li>
        </ul>

        <h2>"Multi-selection"</h2>
        <ul>
            <li><code>"Ctrl+Click"</code>" — toggle an entity in the selection"</li>
            <li><code>"Shift+Click"</code>" — add to selection"</li>
            <li><code>"Escape"</code>" — deselect all"</li>
        </ul>

        <h2>"Scene operations"</h2>
        <ul>
            <li><code>"Ctrl+N"</code>" — new scene"</li>
            <li><code>"Ctrl+O"</code>" — open scene"</li>
            <li><code>"Ctrl+S"</code>" — save scene"</li>
            <li><code>"Ctrl+Shift+S"</code>" — save scene as"</li>
            <li><code>"Ctrl+D"</code>" — duplicate selected entities"</li>
            <li><code>"Delete"</code>" — delete selected entities"</li>
        </ul>
    }
}

fn doc_inspector() -> impl IntoView {
    view! {
        <p class="doc-breadcrumb"><a href="/docs">"Docs"</a>" / Editor"</p>
        <h1>"Inspector"</h1>
        <p class="doc-lead">"View and edit the components attached to the selected entity."</p>

        <h2>"Using the Inspector"</h2>
        <p>"Select an entity in the Hierarchy or viewport. The Inspector panel (right side) shows all its components with editable fields."</p>

        <h2>"Common components"</h2>

        <h3>"Transform"</h3>
        <p>"Every entity has a Transform with position (X, Y, Z), rotation (degrees), and scale. Type values directly or drag the number fields."</p>

        <h3>"Mesh"</h3>
        <p>"3D entities have a Mesh component defining their shape. Change the mesh type or assign a custom model file."</p>

        <h3>"Material"</h3>
        <p>"Controls the visual appearance. Click the material slot to open the Material Editor, or drag a "<code>".material"</code>" file from the Asset Browser."</p>

        <h3>"Script"</h3>
        <p>"Attach a script file to run game logic on this entity. Scripts can expose properties that appear as editable fields in the Inspector."</p>

        <h3>"Rigid Body & Collider"</h3>
        <p>"Add physics simulation. The rigid body controls mass and gravity, while the collider defines the shape used for collision detection."</p>

        <h2>"Adding components"</h2>
        <p>"Click \"Add Component\" at the bottom of the Inspector to see all available component types. You can search by name."</p>

        <h2>"Script properties"</h2>
        <p>"When a script defines a "<code>"props()"</code>" function, those properties appear in the Inspector as editable fields with sliders, checkboxes, or color pickers depending on the type."</p>
    }
}

fn doc_viewport() -> impl IntoView {
    view! {
        <p class="doc-breadcrumb"><a href="/docs">"Docs"</a>" / Editor"</p>
        <h1>"Viewport & Camera"</h1>
        <p class="doc-lead">"Navigate the 3D scene and control the editor camera."</p>

        <h2>"Navigation"</h2>
        <ul>
            <li><strong>"Fly mode:"</strong>" Hold right-click and use "<code>"W/A/S/D/E/Q"</code>" to fly. Hold "<code>"Shift"</code>" to move faster."</li>
            <li><strong>"Orbit:"</strong>" Alt + left-click to orbit around the focus point."</li>
            <li><strong>"Pan:"</strong>" Middle-click and drag to pan."</li>
            <li><strong>"Zoom:"</strong>" Scroll wheel to zoom in/out."</li>
            <li><strong>"Focus:"</strong>" Press "<code>"F"</code>" to focus the camera on the selected entity."</li>
        </ul>

        <h2>"View presets"</h2>
        <p>"Quick camera angles using the numpad:"</p>
        <ul>
            <li><code>"Numpad 1"</code>" — Front view"</li>
            <li><code>"Numpad 3"</code>" — Right view"</li>
            <li><code>"Numpad 7"</code>" — Top view"</li>
            <li><code>"Ctrl+Numpad 1/3/7"</code>" — Back / Left / Bottom view"</li>
            <li><code>"Numpad 5"</code>" — Toggle perspective / orthographic"</li>
        </ul>

        <h2>"Display options"</h2>
        <ul>
            <li><code>"Z"</code>" — Toggle wireframe mode"</li>
            <li><code>"Shift+Z"</code>" — Toggle lighting"</li>
            <li><code>"H"</code>" — Toggle grid"</li>
        </ul>

        <h2>"2D mode"</h2>
        <p>"The viewport automatically switches to 2D mode when you select a 2D entity (sprite, tilemap, UI canvas). In 2D mode, the camera is locked to an orthographic top-down view."</p>
    }
}

fn doc_keybindings() -> impl IntoView {
    view! {
        <p class="doc-breadcrumb"><a href="/docs">"Docs"</a>" / Editor"</p>
        <h1>"Keyboard Shortcuts"</h1>
        <p class="doc-lead">"Quick reference for all editor keyboard shortcuts."</p>

        <h2>"Camera"</h2>
        <pre><code>"W/A/S/D/E/Q    Fly camera (hold right-click)\nShift           Move faster\nF               Focus selected\nNumpad 1/3/7    Front / Right / Top view\nCtrl+Numpad     Opposite views\nNumpad 5        Toggle perspective/ortho"</code></pre>

        <h2>"Tools"</h2>
        <pre><code>"Q               Select mode\nW               Translate (move)\nE               Rotate\nR               Scale\nG               Grab (Blender-style)\nR               Rotate (Blender-style)\nS               Scale (Blender-style)"</code></pre>

        <h2>"Selection"</h2>
        <pre><code>"Left Click      Select\nCtrl+Click      Toggle selection\nShift+Click     Add to selection\nEscape          Deselect all\nCtrl+D          Duplicate\nDelete          Delete"</code></pre>

        <h2>"File"</h2>
        <pre><code>"Ctrl+N          New scene\nCtrl+O          Open scene\nCtrl+S          Save scene\nCtrl+Shift+S    Save as\nCtrl+Z          Undo\nCtrl+Y          Redo"</code></pre>

        <h2>"View"</h2>
        <pre><code>"Z               Wireframe\nShift+Z         Toggle lighting\nH               Toggle grid\nCtrl+`          Toggle bottom panel"</code></pre>

        <h2>"Play"</h2>
        <pre><code>"F5              Play / Stop\nShift+F5        Run scripts only"</code></pre>
    }
}

fn doc_scripting_overview() -> impl IntoView {
    view! {
        <p class="doc-breadcrumb"><a href="/docs">"Docs"</a>" / Scripting"</p>
        <h1>"Scripting Overview"</h1>
        <p class="doc-lead">"Add game logic to your entities with Rhai, Lua, or visual blueprints."</p>

        <h2>"Three ways to script"</h2>
        <p>"Renzora supports three scripting approaches. You can mix and match them in the same project:"</p>
        <ul>
            <li><strong><a href="/docs/scripting/rhai">"Rhai"</a></strong>" — a lightweight scripting language designed for Rust. Simple syntax, great for gameplay logic."</li>
            <li><strong><a href="/docs/scripting/lua">"Lua"</a></strong>" — the industry-standard game scripting language. Use if you're coming from other engines."</li>
            <li><strong><a href="/docs/scripting/blueprints">"Visual Blueprints"</a></strong>" — node-based visual scripting. No coding required."</li>
        </ul>

        <h2>"Script lifecycle"</h2>
        <p>"Every script has two key functions:"</p>
        <ul>
            <li><code>"on_ready()"</code>" — called once when the entity spawns"</li>
            <li><code>"on_update()"</code>" — called every frame"</li>
        </ul>

        <h2>"Attaching scripts"</h2>
        <ol>
            <li>"Create a script file in your project's "<code>"scripts/"</code>" folder"</li>
            <li>"Select an entity in the editor"</li>
            <li>"In the Inspector, click \"Add Component\" → Script"</li>
            <li>"Choose your script file"</li>
        </ol>

        <h2>"Built-in variables"</h2>
        <p>"Every frame, your script has access to these globals:"</p>

        <h3>"Time"</h3>
        <pre><code>"delta           Seconds since last frame (use for smooth movement)\nelapsed         Total seconds since game started"</code></pre>

        <h3>"Transform"</h3>
        <pre><code>"position_x      Entity X position (read/write)\nposition_y      Entity Y position\nposition_z      Entity Z position\nrotation_x/y/z  Entity rotation\nscale_x/y/z     Entity scale"</code></pre>

        <h3>"Input"</h3>
        <pre><code>"input_x         Horizontal axis (-1 to 1, from A/D keys)\ninput_y         Vertical axis (-1 to 1, from W/S keys)\nmouse_x/y       Mouse position\nmouse_delta_x/y Mouse movement since last frame\nmouse_button_left/right/middle  Mouse button state"</code></pre>

        <h3>"Gamepad"</h3>
        <pre><code>"gamepad_left_x/y     Left stick\ngamepad_right_x/y    Right stick\ngamepad_south/east/north/west  Face buttons\ngamepad_left_trigger/right_trigger  Triggers"</code></pre>

        <h2>"What's next?"</h2>
        <p>"Pick your language: "<a href="/docs/scripting/rhai">"Rhai"</a>", "<a href="/docs/scripting/lua">"Lua"</a>", or "<a href="/docs/scripting/blueprints">"Blueprints"</a>"."</p>
    }
}

fn doc_rhai() -> impl IntoView {
    view! {
        <p class="doc-breadcrumb"><a href="/docs">"Docs"</a>" / Scripting"</p>
        <h1>"Rhai Scripting"</h1>
        <p class="doc-lead">"Write game logic with Rhai — a lightweight, Rust-native scripting language."</p>

        <h2>"Your first script"</h2>
        <p>"Create a file called "<code>"player.rhai"</code>" in your project's "<code>"scripts/"</code>" folder:"</p>
        <pre><code>"fn on_ready() {\n    print(\"Player spawned!\");\n}\n\nfn on_update() {\n    // Move the entity based on input\n    let speed = 5.0;\n    position_x += input_x * speed * delta;\n    position_z += input_y * speed * delta;\n}"</code></pre>
        <p>"Attach it to an entity in the Inspector. When you hit play, the entity moves with WASD."</p>

        <h2>"Script properties"</h2>
        <p>"Expose variables to the Inspector so designers can tweak values without editing code:"</p>
        <pre><code>"fn props() {\n    #{\n        speed: #{ default: 5.0, min: 0.0, max: 100.0 },\n        jump_force: #{ default: 10.0, min: 0.0, max: 50.0 },\n        can_fly: #{ default: false }\n    }\n}\n\nfn on_update() {\n    position_x += input_x * speed * delta;\n}"</code></pre>
        <p>"The "<code>"speed"</code>", "<code>"jump_force"</code>", and "<code>"can_fly"</code>" properties appear as editable fields in the Inspector with appropriate widgets (sliders, checkboxes)."</p>

        <h2>"Working with collisions"</h2>
        <pre><code>"fn on_update() {\n    // Check if we collided with something this frame\n    if collisions_entered > 0 {\n        print(\"Hit something!\");\n    }\n\n    // Number of things currently overlapping\n    if active_collisions > 0 {\n        print(\"Still touching\");\n    }\n}"</code></pre>

        <h2>"Entity hierarchy"</h2>
        <pre><code>"fn on_ready() {\n    // Access hierarchy info\n    print(self_entity_name);\n    print(\"Children: \" + children_count);\n    print(\"Parent: \" + parent_entity_id);\n}"</code></pre>

        <h2>"Rhai syntax basics"</h2>
        <p>"Rhai is similar to Rust and JavaScript:"</p>
        <pre><code>"// Variables\nlet x = 42;\nlet name = \"hello\";\n\n// Functions\nfn add(a, b) {\n    a + b\n}\n\n// Control flow\nif x > 10 {\n    print(\"big\");\n} else {\n    print(\"small\");\n}\n\n// Loops\nfor i in 0..10 {\n    print(i);\n}\n\nwhile x > 0 {\n    x -= 1;\n}"</code></pre>
    }
}

fn doc_input() -> impl IntoView {
    view! {
        <p class="doc-breadcrumb"><a href="/docs">"Docs"</a>" / Scripting"</p>
        <h1>"Input Handling"</h1>
        <p class="doc-lead">"Read keyboard, mouse, and gamepad input in your scripts."</p>

        <h2>"Keyboard"</h2>
        <p>"The "<code>"input_x"</code>" and "<code>"input_y"</code>" globals give you axis values from WASD/arrow keys:"</p>
        <pre><code>"fn on_update() {\n    let speed = 5.0;\n    position_x += input_x * speed * delta;  // A/D or Left/Right\n    position_z += input_y * speed * delta;  // W/S or Up/Down\n}"</code></pre>

        <h2>"Mouse"</h2>
        <pre><code>"fn on_update() {\n    // Mouse position (screen coordinates)\n    let mx = mouse_x;\n    let my = mouse_y;\n\n    // Mouse movement since last frame (great for camera look)\n    let dx = mouse_delta_x;\n    let dy = mouse_delta_y;\n\n    // Mouse buttons (true while held)\n    if mouse_button_left {\n        print(\"Shooting!\");\n    }\n}"</code></pre>

        <h2>"Gamepad"</h2>
        <pre><code>"fn on_update() {\n    // Left stick — movement\n    position_x += gamepad_left_x * 5.0 * delta;\n    position_z += gamepad_left_y * 5.0 * delta;\n\n    // Right stick — camera\n    rotation_y += gamepad_right_x * 2.0 * delta;\n\n    // Face buttons\n    if gamepad_south {  // A button / X button\n        print(\"Jump!\");\n    }\n\n    // Triggers (0.0 to 1.0)\n    if gamepad_right_trigger > 0.5 {\n        print(\"Firing!\");\n    }\n}"</code></pre>
    }
}

fn doc_game_ui() -> impl IntoView {
    view! {
        <p class="doc-breadcrumb"><a href="/docs">"Docs"</a>" / Scripting"</p>
        <h1>"Game UI"</h1>
        <p class="doc-lead">"Control in-game UI elements from your scripts."</p>

        <h2>"Overview"</h2>
        <p>"Renzora has a built-in UI system with 19 widget types. You create UI in the editor's UI workspace, then control it from scripts."</p>

        <h2>"Script functions"</h2>
        <pre><code>"// Show, hide, or toggle a UI element by name\nui_show(\"health_bar\");\nui_hide(\"main_menu\");\nui_toggle(\"inventory\");\n\n// Set text content\nui_set_text(\"score_label\", \"Score: 1500\");\n\n// Progress bars (0.0 to 1.0)\nui_set_progress(\"loading_bar\", 0.75);\n\n// Health bars\nui_set_health(\"player_health\", 80, 100);  // current, max\n\n// Sliders\nui_set_slider(\"volume\", 0.5);\n\n// Checkboxes and toggles\nui_set_checkbox(\"vsync\", true);\nui_set_toggle(\"mute\", false);\n\n// Visibility\nui_set_visible(\"tooltip\", true);\n\n// Colors (RGBA, 0-255)\nui_set_color(\"damage_flash\", 255, 0, 0, 128);\n\n// Themes\nui_set_theme(\"dark\");    // dark, light, high_contrast"</code></pre>

        <h2>"Widget types"</h2>
        <p>"Available in the UI workspace palette:"</p>
        <ul>
            <li><strong>"Layout:"</strong>" Canvas, Panel, Scroll Area, Grid"</li>
            <li><strong>"Basic:"</strong>" Text, Image, Button"</li>
            <li><strong>"Input:"</strong>" Slider, Checkbox, Toggle, Radio Button, Dropdown, Text Input"</li>
            <li><strong>"Display:"</strong>" Progress Bar, Health Bar, Tab Bar, Spinner"</li>
            <li><strong>"Overlay:"</strong>" Tooltip, Modal, Draggable Window"</li>
        </ul>

        <h2>"Example: HUD"</h2>
        <pre><code>"fn on_update() {\n    // Update health bar based on player health\n    let health = 80;  // from your game logic\n    let max_health = 100;\n    ui_set_health(\"player_health\", health, max_health);\n    ui_set_text(\"health_text\", health + \" / \" + max_health);\n\n    // Show damage flash when hit\n    if collisions_entered > 0 {\n        ui_show(\"damage_flash\");\n    }\n}"</code></pre>
    }
}

fn doc_export_overview() -> impl IntoView {
    view! {
        <p class="doc-breadcrumb"><a href="/docs">"Docs"</a>" / Export & Deploy"</p>
        <h1>"Export Overview"</h1>
        <p class="doc-lead">"Build your game for 10 platforms from a single project."</p>

        <h2>"Supported platforms"</h2>
        <ul>
            <li><strong>"Windows"</strong>" (x64) → "<code>".exe"</code></li>
            <li><strong>"Linux"</strong>" (x64) → executable"</li>
            <li><strong>"macOS"</strong>" (Intel x64 and Apple Silicon ARM64) → "<code>".app"</code></li>
            <li><strong>"Android"</strong>" (ARM64 and x86_64) → "<code>".apk"</code></li>
            <li><strong>"Fire TV"</strong>" (ARM64) → "<code>".apk"</code></li>
            <li><strong>"iOS"</strong>" (ARM64) → "<code>".ipa"</code></li>
            <li><strong>"tvOS"</strong>" (ARM64) → "<code>".ipa"</code></li>
            <li><strong>"Web"</strong>" (WASM/WebGL) → "<code>".wasm"</code></li>
        </ul>

        <h2>"How to export"</h2>
        <ol>
            <li>"Open your project in the editor"</li>
            <li>"Go to "<strong>"File → Export"</strong>" (or use the export button in the title bar)"</li>
            <li>"Select your target platform"</li>
            <li>"Configure export settings (window size, icon, etc.)"</li>
            <li>"Click Export"</li>
        </ol>
        <p>"The engine packs your assets into a "<code>".rpak"</code>" archive and bundles them with a pre-built runtime template."</p>

        <h2>"Export templates"</h2>
        <p>"Each platform requires a pre-compiled runtime template. These are downloaded automatically on first export and cached for future builds."</p>

        <h2>"Multiplayer exports"</h2>
        <p>"If your project uses networking, the export includes an optional server binary alongside the game client. You can deploy the server to any Linux host."</p>
    }
}

fn doc_networking_overview() -> impl IntoView {
    view! {
        <p class="doc-breadcrumb"><a href="/docs">"Docs"</a>" / Networking"</p>
        <h1>"Networking Overview"</h1>
        <p class="doc-lead">"Build multiplayer games with dedicated server support."</p>

        <h2>"Architecture"</h2>
        <p>"Renzora uses a "<strong>"dedicated server"</strong>" model — the server runs the authoritative game simulation, and clients send input and receive state updates. This prevents cheating and ensures a consistent experience."</p>

        <h2>"Transport"</h2>
        <p>"Three transport layers are supported:"</p>
        <ul>
            <li><strong>"UDP"</strong>" — lowest latency, best for desktop games"</li>
            <li><strong>"WebTransport"</strong>" — modern web protocol, works in browsers"</li>
            <li><strong>"WebSocket"</strong>" — widest compatibility, works everywhere"</li>
        </ul>

        <h2>"Configuration"</h2>
        <p>"Network settings are defined in "<code>"project.toml"</code>":"</p>
        <pre><code>"[network]\nserver_addr = \"127.0.0.1\"\nport = 5000\ntransport = \"udp\"\ntick_rate = 60\nmax_clients = 16"</code></pre>

        <h2>"State replication"</h2>
        <p>"Mark components for replication in the editor, and the server automatically syncs their state to connected clients. The engine handles interpolation and prediction to keep gameplay smooth even with latency."</p>

        <h2>"What's next?"</h2>
        <p>"Follow the "<a href="/docs/networking/server-setup">"Server Setup"</a>" guide to create your first multiplayer project."</p>
    }
}
