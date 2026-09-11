# Releases & Documentation

Every marketplace asset is browsable like a code repository. Upload a `.zip` and Renzora unpacks it, keeps the folder structure, renders your `README.md` as the asset's documentation, and tracks each version you publish as its own **release**.

Two things follow from that, and this page covers both:

- **Your README is your docs.** You don't write documentation in a separate editor — it comes out of the archive you already ship.
- **New versions don't destroy old ones.** Publishing v1.2.0 leaves v1.1.0 downloadable for everyone who was already using it.

## The file tree

Upload a `.zip` and its contents become browsable at `/marketplace/asset/<slug>/files`:

```
my-plugin.zip
├─ README.md          → renders on the asset page
├─ CHANGELOG.md       → seeds your release notes
├─ LICENSE            → readable by anyone, before buying
├─ docs/
│   ├─ install.md     → /marketplace/asset/<slug>/files/docs/install.md
│   └─ api/
│       └─ events.md
└─ src/
    └─ main.lua
```

A few things happen automatically:

- **A single wrapping folder is stripped.** If everything sits under `my-plugin-1.2.0/`, that level is removed, so buyers don't click through an empty directory on every visit.
- **Paths are preserved.** `docs/api/events.md` stays where you put it, and the folders you see in the browser are built from those paths.
- **Junk is skipped.** `__MACOSX`, `.DS_Store`, dotfiles and dot-directories (`.git/`, `.env`) are dropped, as are nested `.zip` files.
- **Limits.** Up to 500 entries per archive, 200 MB per file, 500 MB uncompressed in total.

### Unpacked vs. kept whole

**Unpack it** stores each file separately. **Keep it as a single .zip** stores the archive as one object — and **a plugin is always kept whole**, because the editor extracts your zip into `plugins/<crate>/` and the SDK compiles it, so those bytes have to survive the round trip exactly as you uploaded them.

Either way you get the file tree and the rendered docs. A kept archive has its contents *indexed* rather than unpacked: browsing reads entries back out of the stored zip, and the download is still the original archive, byte for byte.

## What's public, and what isn't

This is the rule worth understanding before you package a paid asset:

| | Visible to everyone | Needs a purchase |
|---|---|---|
| File and folder **names**, sizes, structure | ✅ | |
| `README.md` and every other `.md` file | ✅ | |
| `LICENSE`, `LICENCE`, `COPYING`, `NOTICE`, `AUTHORS` | ✅ | |
| Source code, models, textures, audio, binaries | | 🔒 |
| Images (including ones your README links to) | | 🔒 |

The tree is public so buyers can see what they're getting. Documentation is public because that's what sells the asset — a plugin nobody can read the docs for is a plugin nobody buys. Everything else needs ownership, and the file viewer shows a purchase prompt in its place.

**Practical consequence:** don't put anything you're selling inside a `.md` file. Markdown is readable by the whole internet, whatever you charge for the asset. Equally, an image your README links to *won't* render for people who haven't bought a paid asset — for cover art and screenshots, use the gallery on the upload form instead, which is always public.

## Writing the README

The README nearest the archive root becomes the asset's documentation. `README.md`, `README.markdown`, `README.txt` and a bare `README` are all recognised.

It's rendered as GitHub-flavoured Markdown: headings (with anchor links), lists, tables, task lists, code blocks with syntax highlighting, blockquotes, footnotes and strikethrough.

**Links are rewritten for you.** A relative link resolves inside your own archive:

```markdown
See [the install guide](docs/install.md).
![The widget panel](images/panel.png)
```

- `docs/install.md` becomes a link to that file in the browser, rendered as a page.
- `images/panel.png` is served from your archive.
- Links inside `docs/` resolve relative to that folder, so `../README.md` gets you back to the root.
- Absolute `http://` and `https://` links are left alone.

**What isn't allowed.** Raw HTML in your markdown is shown as text rather than rendered, and `javascript:` or `data:` URLs are stripped. That applies to every asset from every creator, so a README can never run code in someone else's browser. Write plain Markdown and you'll never notice.

### A docs folder

Any `.md` file anywhere in the archive is a documentation page, so a `docs/` folder gives your asset a real manual:

```
docs/
├─ install.md
├─ configuration.md
└─ api/
    ├─ events.md
    └─ widgets.md
```

The file browser builds a sidebar from those files, grouped by folder, and each page gets an "on this page" outline from its headings. A `README.md` inside a folder renders below that folder's listing, the same way it does at the root.

## Publishing a release

**Marketplace → your asset → New Release**, or the **Releases** card on the edit page.

| Field | What it does |
|---|---|
| **New version** | The version string, 1–32 characters. Pre-filled with a suggested patch bump. Must be unique for this asset. |
| **Release notes** | Markdown. Leave it empty and a `CHANGELOG.md` from the archive is used instead. |
| **Files** | The new version's files. Uploading a `.zip` and unpacking it is the usual choice. |

On publish:

1. The new release becomes **current** — it's what the asset page shows and what a plain download returns.
2. The asset's version number follows it.
3. The previous release keeps its files. Nothing is overwritten.

Installed copies find out through the editor's plugin update check, which reports the latest published version for the plugins someone has installed.

### Edit vs. release

Both live on your asset, and they do different jobs:

- **Edit** changes the *current* version in place — the description, price, thumbnail, gallery, or a botched file you need to swap. There's no new version.
- **New Release** ships a *new* version alongside the old ones.

Use Edit to fix a typo in your description. Use a release to ship a bug fix.

## Older versions

Every release stays browsable and downloadable at its own URL:

```
/marketplace/asset/<slug>/files?release=1.1.0
```

The file browser has a version switcher, and the **Releases** section on the asset page has a download button per version. Buyers keep access to every release — if your new version breaks their project, they can drop back to the one that worked. A zip download of an older version is named after it (`my-plugin-1.1.0.zip`) so two versions don't collide in a downloads folder.

### Deleting a release

You can delete an old release from the **Releases** card on the edit page. Its files are removed from storage permanently, including for people who already downloaded it — so delete one only when it's genuinely broken or shouldn't have been published.

The **current** release can't be deleted. Publish a newer one first; that way every asset always has something to download.

## See also

- [Publishing Assets](./publishing) — the upload wizard, categories, pricing and limits
- [Browsing & Installing](./browsing) — what buyers see
- [Marketplace API](../platform-api/marketplace) — the endpoints behind all of this
