//! Markdown rendering for **untrusted** content — the README and docs files
//! that come out of a creator's uploaded archive.
//!
//! The site's own docs (`crates/server/src/docs_files.rs`) render repo files we
//! wrote, so they can pass raw HTML straight through. Nothing here can: a
//! marketplace archive is arbitrary user upload, and its markdown is rendered
//! on a page that carries the viewer's session. So this module
//!
//!   * turns raw HTML blocks and inline HTML into **escaped text** rather than
//!     markup, which kills `<script>`, `<iframe>`, `onerror=` and friends in
//!     one move instead of trying to enumerate them;
//!   * allows only `http`, `https` and `mailto` in link and image URLs, so
//!     `javascript:` and `data:` payloads can't ride in on a link; and
//!   * rewrites *relative* links so a README that points at `docs/install.md`
//!     or `./screenshot.png` resolves inside the asset's own file tree.

use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use uuid::Uuid;

/// Where a markdown file sits, so relative links in it can be resolved.
pub struct MdContext {
    pub asset_id: Uuid,
    pub asset_slug: String,
    /// The release the file was read from, as a link query (`?release=1.2.0`),
    /// empty when it's the current release.
    pub release_query: String,
    /// Directory of the file inside the archive: `""` at the root,
    /// `"docs"` for `docs/install.md`.
    pub dir: String,
}

impl MdContext {
    pub fn new(asset_id: Uuid, asset_slug: &str, path: &str, release_query: &str) -> Self {
        Self {
            asset_id,
            asset_slug: asset_slug.to_string(),
            release_query: release_query.to_string(),
            dir: parent_dir(path).to_string(),
        }
    }
}

/// Directory portion of an archive path (`docs/api/events.md` -> `docs/api`).
pub fn parent_dir(path: &str) -> &str {
    match path.rfind('/') {
        Some(i) => &path[..i],
        None => "",
    }
}

/// Resolve a relative archive reference against a directory, collapsing `.`
/// and `..`. Returns `None` if it climbs above the archive root — a README
/// linking to `../../etc/passwd` resolves to nothing rather than to something.
pub fn resolve_path(dir: &str, link: &str) -> Option<String> {
    let mut parts: Vec<&str> = if dir.is_empty() {
        Vec::new()
    } else {
        dir.split('/').collect()
    };
    for seg in link.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            s => parts.push(s),
        }
    }
    let joined = parts.join("/");
    (!joined.is_empty()).then_some(joined)
}

fn has_scheme(url: &str) -> bool {
    // `scheme:` before any `/`, `?` or `#` — so `docs/a:b.md` isn't mistaken
    // for a scheme, but `javascript:alert(1)` is.
    match url.find(':') {
        Some(i) => !url[..i].contains(['/', '?', '#']),
        None => false,
    }
}

fn scheme_allowed(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://") || lower.starts_with("mailto:")
}

fn is_markdown(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    lower.ends_with(".md") || lower.ends_with(".markdown")
}

/// Rewrite a link destination found in an asset's markdown.
fn rewrite_link(ctx: &MdContext, dest: &str, is_image: bool) -> String {
    if dest.starts_with('#') {
        return dest.to_string(); // in-page anchor
    }
    if has_scheme(dest) {
        // Absolute URL: keep it if the scheme is one we allow, otherwise
        // neutralise it. Images additionally can't be `mailto:`.
        return if scheme_allowed(dest) && !(is_image && dest.to_ascii_lowercase().starts_with("mailto:")) {
            dest.to_string()
        } else {
            "#".to_string()
        };
    }
    if dest.starts_with("//") {
        return "#".to_string(); // protocol-relative, scheme not ours to pick
    }
    if dest.starts_with('/') {
        return dest.to_string(); // site-relative, e.g. /docs/scripting/lua
    }

    // Relative to the file's own directory inside the archive.
    let (path_part, fragment) = match dest.split_once('#') {
        Some((p, f)) => (p, Some(f)),
        None => (dest, None),
    };
    let Some(resolved) = resolve_path(&ctx.dir, path_part) else {
        return "#".to_string();
    };

    if is_image {
        // Images need bytes, so they point at the raw endpoint. For a paid
        // asset that 403s for non-owners, and the viewer shows a locked
        // placeholder in its place.
        return format!(
            "/api/marketplace/{}/raw?path={}{}",
            ctx.asset_id,
            urlencode(&resolved),
            if ctx.release_query.is_empty() {
                String::new()
            } else {
                format!("&{}", ctx.release_query.trim_start_matches('?'))
            }
        );
    }

    // Everything else goes to the in-site file browser, so `docs/install.md`
    // reads as a rendered page rather than downloading a file.
    let mut url = format!(
        "/marketplace/asset/{}/files/{}{}",
        ctx.asset_slug, resolved, ctx.release_query
    );
    if let Some(f) = fragment {
        url.push('#');
        url.push_str(f);
    }
    let _ = is_markdown(&resolved); // both cases route the same way; the viewer decides how to show it
    url
}

fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b'/' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// GitHub-style heading anchor: lowercase, spaces to dashes, punctuation gone.
fn heading_slug(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut last_dash = true;
    for c in text.chars() {
        if c.is_alphanumeric() {
            for l in c.to_lowercase() {
                out.push(l);
            }
            last_dash = false;
        } else if (c == ' ' || c == '-' || c == '_') && !last_dash {
            out.push('-');
            last_dash = true;
        }
    }
    out.trim_matches('-').to_string()
}

fn options() -> Options {
    Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_SMART_PUNCTUATION
}

/// Headings in document order, as `(level, text, anchor)`. Drives the
/// "On this page" nav on a rendered doc.
pub fn outline(md: &str) -> Vec<(u8, String, String)> {
    let mut out = Vec::new();
    let mut depth: Option<u8> = None;
    let mut text = String::new();
    for ev in Parser::new_ext(md, options()) {
        match ev {
            Event::Start(Tag::Heading { level, .. }) => {
                depth = Some(level as u8);
                text.clear();
            }
            Event::Text(t) | Event::Code(t) if depth.is_some() => text.push_str(&t),
            Event::End(TagEnd::Heading(_)) => {
                if let Some(l) = depth.take() {
                    let slug = heading_slug(&text);
                    if !slug.is_empty() {
                        out.push((l, text.trim().to_string(), slug));
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// Render untrusted markdown to HTML. See the module docs for what is stripped.
pub fn render(md: &str, ctx: &MdContext) -> String {
    // Heading anchors are assigned in a first pass, because a heading's text
    // only arrives after its opening event.
    let mut anchors: std::collections::VecDeque<String> =
        outline(md).into_iter().map(|(_, _, a)| a).collect();

    let mut events: Vec<Event> = Vec::new();
    for ev in Parser::new_ext(md, options()) {
        match ev {
            // Raw HTML becomes visible text instead of live markup.
            Event::Html(t) => events.push(Event::Text(t)),
            Event::InlineHtml(t) => events.push(Event::Text(t)),

            Event::Start(Tag::Heading {
                level,
                classes,
                attrs,
                ..
            }) => {
                let id = anchors.pop_front().filter(|a| !a.is_empty());
                events.push(Event::Start(Tag::Heading {
                    level,
                    id: id.map(Into::into),
                    classes,
                    attrs,
                }));
            }

            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }) => {
                events.push(Event::Start(Tag::Link {
                    link_type,
                    dest_url: rewrite_link(ctx, &dest_url, false).into(),
                    title,
                    id,
                }));
            }
            Event::Start(Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            }) => {
                events.push(Event::Start(Tag::Image {
                    link_type,
                    dest_url: rewrite_link(ctx, &dest_url, true).into(),
                    title,
                    id,
                }));
            }

            other => events.push(other),
        }
    }

    let mut html = String::with_capacity(md.len() * 3 / 2);
    pulldown_cmark::html::push_html(&mut html, events.into_iter());
    html
}

/// The language tag of the first fenced code block, if any — used to pick a
/// highlight hint when previewing a source file.
pub fn first_code_language(md: &str) -> Option<String> {
    Parser::new_ext(md, options()).find_map(|ev| match ev {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) if !lang.is_empty() => {
            Some(lang.to_string())
        }
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(dir: &str) -> MdContext {
        MdContext {
            asset_id: Uuid::nil(),
            asset_slug: "my-plugin".into(),
            release_query: String::new(),
            dir: dir.into(),
        }
    }

    #[test]
    fn raw_html_is_escaped_not_emitted() {
        let html = render("<script>alert(1)</script>\n\nhi", &ctx(""));
        assert!(!html.contains("<script>"), "script tag survived: {html}");
        assert!(html.contains("&lt;script&gt;"));
    }

    #[test]
    fn inline_html_is_escaped() {
        let html = render("text <img src=x onerror=alert(1)> more", &ctx(""));
        // The text may still *read* `onerror=…`, but only as escaped text —
        // what matters is that no live tag was emitted.
        assert!(!html.contains("<img"), "live tag emitted: {html}");
        assert!(html.contains("&lt;img"));
    }

    #[test]
    fn javascript_urls_are_dropped() {
        let html = render("[click](javascript:alert(1))", &ctx(""));
        assert!(!html.to_ascii_lowercase().contains("javascript:"));
        assert!(html.contains("href=\"#\""));
    }

    #[test]
    fn data_uri_images_are_dropped() {
        let html = render("![x](data:text/html;base64,PHNjcmlwdD4=)", &ctx(""));
        assert!(!html.contains("data:text/html"));
    }

    #[test]
    fn external_links_survive() {
        let html = render("[docs](https://example.com/a)", &ctx(""));
        assert!(html.contains("href=\"https://example.com/a\""));
    }

    #[test]
    fn relative_doc_links_point_into_the_file_browser() {
        let html = render("[install](docs/install.md)", &ctx(""));
        assert!(
            html.contains("href=\"/marketplace/asset/my-plugin/files/docs/install.md\""),
            "{html}"
        );
    }

    #[test]
    fn relative_links_resolve_against_the_files_own_directory() {
        let html = render("[api](./api.md) and [up](../README.md)", &ctx("docs/guide"));
        assert!(html.contains("/files/docs/guide/api.md"), "{html}");
        assert!(html.contains("/files/docs/README.md"), "{html}");
    }

    #[test]
    fn links_cannot_escape_the_archive_root() {
        let html = render("[x](../../../etc/passwd)", &ctx("docs"));
        assert!(!html.contains("etc/passwd"), "{html}");
        assert!(html.contains("href=\"#\""));
    }

    #[test]
    fn relative_images_point_at_the_raw_endpoint() {
        let html = render("![shot](images/shot.png)", &ctx("docs"));
        assert!(
            html.contains("/api/marketplace/00000000-0000-0000-0000-000000000000/raw?path=docs/images/shot.png"),
            "{html}"
        );
    }

    #[test]
    fn release_is_carried_through_rewritten_links() {
        let c = MdContext {
            release_query: "?release=1.1.0".into(),
            ..ctx("")
        };
        let html = render("[a](docs/a.md)\n\n![b](b.png)", &c);
        assert!(html.contains("/files/docs/a.md?release=1.1.0"), "{html}");
        // `&` is escaped inside the attribute, as it must be in valid HTML.
        assert!(html.contains("raw?path=b.png&amp;release=1.1.0"), "{html}");
    }

    #[test]
    fn headings_get_stable_anchors() {
        let html = render("## Getting Started!\n", &ctx(""));
        assert!(html.contains("id=\"getting-started\""), "{html}");
    }

    #[test]
    fn outline_reports_levels_and_anchors() {
        let o = outline("# Title\n\n## One\n\n### Two words\n");
        assert_eq!(
            o,
            vec![
                (1, "Title".to_string(), "title".to_string()),
                (2, "One".to_string(), "one".to_string()),
                (3, "Two words".to_string(), "two-words".to_string()),
            ]
        );
    }

    #[test]
    fn tables_and_task_lists_render() {
        let html = render("| a | b |\n|---|---|\n| 1 | 2 |\n\n- [x] done\n", &ctx(""));
        assert!(html.contains("<table>"), "{html}");
        assert!(html.contains("type=\"checkbox\""), "{html}");
    }

    #[test]
    fn anchors_only_links_are_left_alone() {
        let html = render("[top](#introduction)", &ctx("docs"));
        assert!(html.contains("href=\"#introduction\""), "{html}");
    }
}
