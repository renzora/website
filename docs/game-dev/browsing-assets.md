# Browsing & Installing Assets

Find and install community-created assets from the Renzora Marketplace.

## Accessing the Marketplace

- **From the editor**: Window → Marketplace
- **From the website**: [renzora.com/marketplace](/marketplace)

Both show the same catalog. Purchases sync to your account.

## Searching and filtering

Use the search bar to find assets by name, author, or keyword.

### Filters

| Filter | Options |
|--------|---------|
| **Category** | Scripts, Plugins, Models, Textures, Materials, Audio, Scenes, Tools, Templates |
| **Price** | Free, Under 100 credits, Under 500, Any price |
| **Rating** | 4+ stars, 3+ stars, Any |
| **Sort** | Newest, Most popular, Highest rated, Price low/high |
| **Tags** | Community-applied tags (e.g., "FPS", "RPG", "pixel art", "sci-fi") |

## Asset types

| Type | Description | File types |
|------|-------------|------------|
| **Scripts** | Rhai/Lua scripts for gameplay logic | `.rhai`, `.lua` |
| **Plugins** | Engine extensions (Rust crates) | `.dll`, `.so`, `.dylib` |
| **Models** | 3D meshes with materials | `.glb`, `.gltf`, `.fbx` |
| **Textures** | Images for materials | `.png`, `.jpg`, `.hdr` |
| **Materials** | Ready-to-use PBR materials | `.mat` |
| **Audio** | Sound effects and music | `.ogg`, `.wav` |
| **Scenes** | Pre-built scenes or prefabs | `.ron` |
| **Tools** | Editor extensions and utilities | Plugin + panel |
| **Templates** | Complete starter projects | Full project |

## Previewing assets

Click an asset to see its detail page:

- **Screenshots** — preview images/videos
- **Description** — what it does and how to use it
- **Reviews** — community ratings and comments
- **Version history** — changelog for each update
- **Compatibility** — supported engine versions
- **File size** — download size

## Purchasing

1. Click **Get** (free assets) or **Buy for X credits**
2. Confirm the purchase
3. The asset is added to your [Library](/library)

You need sufficient credits in your [Wallet](/wallet). See [Credits System](/docs/game-dev/credits).

## Installing into your project

### From the editor

1. Open **Window → Library** (or Marketplace → My Library)
2. Find the purchased asset
3. Click **Install** — files are copied to your project's `assets/` directory

### Manual installation

1. Download the asset from the website
2. Extract into your project's `assets/` folder
3. Restart the editor to detect new files

## Managing installed assets

In **Window → Library → Installed**:

- **Update** — install the latest version (if the creator published an update)
- **Remove** — delete the asset files from your project
- **Disable** — keep files but exclude from builds

## Leaving reviews

After purchasing, leave a review:

1. Go to the asset's detail page
2. Click **Write a Review**
3. Rate 1–5 stars and add a comment
4. Reviews help other users and help creators improve their assets
