# Marketplace API

The Marketplace API allows you to browse, upload, and manage assets on the Renzora Marketplace.

## Base URL

```
https://renzora.com/api/marketplace
```

## Browse Assets

### List Assets

```bash
curl "https://renzora.com/api/marketplace/assets?page=1&limit=20&category=3d-models"
```

**Query Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | integer | Page number (default: 1) |
| `limit` | integer | Items per page (default: 20, max: 100) |
| `category` | string | Filter by category slug |
| `search` | string | Search query |
| `sort` | string | `newest`, `popular`, `price_asc`, `price_desc` |
| `price` | string | `free`, `paid`, or omit for all |

**Response:**

```json
{
  "assets": [
    {
      "id": "uuid",
      "slug": "low-poly-trees",
      "title": "Low Poly Trees Pack",
      "description": "50 low-poly tree models",
      "price": 500,
      "thumbnail_url": "/uploads/thumbnails/abc123.webp",
      "author": { "username": "artist42", "avatar_url": "..." },
      "category": "3d-models",
      "download_count": 1234,
      "rating": 4.8,
      "created_at": "2026-03-15T10:30:00Z"
    }
  ],
  "total": 156,
  "page": 1,
  "pages": 8
}
```

### Get Asset Details

```bash
curl "https://renzora.com/api/marketplace/assets/{slug}"
```

### Get Categories

```bash
curl "https://renzora.com/api/marketplace/categories"
```

## Upload Assets

Upload a new asset to the marketplace. Requires authentication.

### Step 1: Create Asset Listing

```bash
curl -X POST https://renzora.com/api/marketplace/assets \
  -H "Authorization: Bearer rz_..." \
  -H "Content-Type: multipart/form-data" \
  -F "title=My Asset Pack" \
  -F "description=A great asset pack" \
  -F "category=3d-models" \
  -F "price=500" \
  -F "tags=lowpoly,trees,nature" \
  -F "file=@asset-pack.zip" \
  -F "thumbnail=@preview.png"
```

> **Note:** Prices are in credits. Set `price=0` for free assets.

### Step 2: Asset Review

After upload, assets enter a review queue. You'll receive a notification when your asset is approved or if changes are requested.

## Download Assets

### Purchase and Download

```bash
curl -X POST https://renzora.com/api/marketplace/assets/{slug}/download \
  -H "Authorization: Bearer rz_..."
```

For paid assets, credits are deducted from your balance. Free assets download immediately.

## Manage Your Assets

### Update Asset

```bash
curl -X PATCH https://renzora.com/api/marketplace/assets/{slug} \
  -H "Authorization: Bearer rz_..." \
  -H "Content-Type: application/json" \
  -d '{"description": "Updated description", "price": 600}'
```

### Delete Asset

```bash
curl -X DELETE https://renzora.com/api/marketplace/assets/{slug} \
  -H "Authorization: Bearer rz_..."
```

### List Your Assets

```bash
curl "https://renzora.com/api/marketplace/my-assets" \
  -H "Authorization: Bearer rz_..."
```

## Files, Documentation & Releases

An asset's uploaded archive is browsable like a repository. The **tree is public** — anyone can list an asset's files, their sizes and structure. **Contents need ownership**, except documentation: markdown files and licence files (`LICENSE`, `LICENCE`, `COPYING`, `NOTICE`, `AUTHORS`) are readable by anyone.

A zip that is *kept whole* rather than unpacked — which is always the case for a plugin, since the editor builds from those exact bytes — still gets a tree: its entries are **indexed** rather than stored separately, and reads pull the entry back out of the stored archive. The download is unaffected and remains the original zip.

Endpoints that take a `release` parameter accept either a version string (`1.1.0`) or a release id. Omit it for the current release.

### Get the File Tree

```bash
curl "https://renzora.com/api/marketplace/{asset_id}/tree?release=1.1.0"
```

Returns the whole tree for one release plus its rendered README, so a page needs a single request. Directories are synthesised from the file paths, and their `size` is the total of everything beneath them.

```json
{
  "release": {
    "id": "uuid",
    "version": "1.1.0",
    "notes": "## Added\n- Per-axis snap increments",
    "notes_html": "<h2 id=\"added\">Added</h2>…",
    "is_current": true,
    "downloads": 42,
    "file_count": 5,
    "total_size": 2750,
    "created_at": "2026-09-11T09:02:50Z"
  },
  "entries": [
    { "path": "docs", "name": "docs", "kind": "dir", "size": 330, "mime_type": "inode/directory", "is_doc": false },
    { "id": "uuid", "path": "README.md", "name": "README.md", "kind": "file", "size": 420, "mime_type": "text/markdown", "is_doc": true }
  ],
  "readme": {
    "path": "README.md",
    "html": "<h1 id=\"grid-snap\">Grid Snap</h1>…",
    "outline": [ { "level": 1, "text": "Grid Snap", "anchor": "grid-snap" } ]
  },
  "has_access": false
}
```

`is_doc` marks entries readable without owning the asset. `has_access` says whether *this caller* may read file contents.

### Read a File

```bash
curl "https://renzora.com/api/marketplace/{asset_id}/file?path=docs/install.md"
```

The `kind` field says how to display it:

| `kind` | Meaning | Fields |
|---|---|---|
| `markdown` | A doc file, rendered | `html`, `content`, `outline` |
| `text` | Source code | `content`, `language`, `truncated` |
| `image` | An image | `download_url` |
| `binary` | Anything else | `download_url` |
| `locked` | Paid content, caller doesn't own it | *(no content)* |

Text files over 512 KB come back with `truncated: true`.

Markdown is rendered server-side with raw HTML escaped and `javascript:`/`data:` URLs stripped — user-uploaded documentation can never run script. Relative links are rewritten: `.md` links point at the file browser, and relative images at the raw endpoint below.

### Read Raw Bytes

```bash
curl "https://renzora.com/api/marketplace/{asset_id}/raw?path=images/logo.png"
```

Serves the file's bytes — this is what images in a rendered README resolve to. Same access rule as above, so a non-doc file in a paid asset returns `401` for a caller who doesn't own it. Only images are served with their own content type; everything else is `application/octet-stream` under a `sandbox` CSP.

### List Releases

```bash
curl "https://renzora.com/api/marketplace/{asset_id}/releases"
```

Returns every release, newest first, each with `version`, `notes`, `notes_html`, `is_current`, `downloads`, `file_count`, `total_size` and `created_at`.

### Publish a Release

```bash
curl -X POST https://renzora.com/api/marketplace/{asset_id}/releases \
  -H "Authorization: Bearer rz_..." \
  -F 'metadata={"version":"1.2.0","notes":"## Fixed\n- Crash on startup","zip_action":"extract"}' \
  -F "file=@my-plugin.zip"
```

Creator only. The `metadata` part takes:

| Field | Type | Description |
|-------|------|-------------|
| `version` | string | Required, 1–32 chars, unique for the asset |
| `notes` | string | Markdown. Empty falls back to a `CHANGELOG.md` in the archive |
| `zip_action` | string | `extract` (default) or `keep`. Ignored for plugins, which are always kept whole |

The new release becomes current and the asset's version follows it. **The previous release keeps its files** — it stays browsable and downloadable. A duplicate version returns `400`.

### Update Release Notes

```bash
curl -X PUT https://renzora.com/api/marketplace/{asset_id}/releases/{release_id} \
  -H "Authorization: Bearer rz_..." \
  -H "Content-Type: application/json" \
  -d '{"notes": "## Fixed\n- Corrected the changelog"}'
```

### Delete a Release

```bash
curl -X DELETE https://renzora.com/api/marketplace/{asset_id}/releases/{release_id} \
  -H "Authorization: Bearer rz_..."
```

Permanently removes that release's files from storage. The **current** release can't be deleted (`400`) — publish a newer one first.

### Download a Specific Version

```bash
curl "https://renzora.com/api/marketplace/{asset_id}/download?release=1.1.0" \
  -H "Authorization: Bearer rz_..."

curl -OJ "https://renzora.com/api/marketplace/{asset_id}/download-zip?release=1.1.0" \
  -H "Authorization: Bearer rz_..."
```

Buyers keep access to every release, not just the newest. A zip of an older version is named after it (`my-plugin-1.1.0.zip`) and unpacks with the same folder structure the creator uploaded.
