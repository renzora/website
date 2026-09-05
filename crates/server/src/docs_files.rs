use axum::{extract::Path, http::StatusCode, routing::get, Json, Router};
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ---- Single-portal, versioned sidebar schema (docs/<version>/_sidebar.json) ----

#[derive(Serialize, Deserialize, Clone)]
struct Sidebar {
    version: String,
    label: String,
    description: String,
    groups: Vec<SidebarGroup>,
}

#[derive(Serialize, Deserialize, Clone)]
struct SidebarGroup {
    group: String,
    /// "basic" (game-making, shown by default) or "advanced" (engine internals,
    /// revealed by the docs Basic/Advanced toggle). Missing/empty = treated as basic.
    #[serde(default)]
    level: String,
    categories: Vec<SidebarCategory>,
}

#[derive(Serialize, Deserialize, Clone)]
struct SidebarCategory {
    category: String,
    pages: Vec<SidebarPage>,
}

#[derive(Serialize, Deserialize, Clone)]
struct SidebarPage {
    slug: String,
    title: String,
}

#[derive(Serialize)]
struct DocPage {
    version: String,
    slug: String,
    title: String,
    group: String,
    category: String,
    content: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/versions", get(get_versions))
        .route("/sidebar/:version", get(get_sidebar))
        .route("/search/:version", get(search_docs))
        // Back-compat: version-less search hits the default version (used by the global nav search).
        .route("/search", get(search_docs_default))
        .route("/page/:version/*slug", get(get_page))
}

fn docs_dir() -> PathBuf {
    PathBuf::from("docs")
}

/// The default version id from docs/_versions.json (falls back to "r1-alpha7").
async fn default_version() -> String {
    let path = docs_dir().join("_versions.json");
    if let Ok(content) = tokio::fs::read_to_string(&path).await {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(def) = value.get("default").and_then(|d| d.as_str()) {
                return def.to_string();
            }
        }
    }
    "r1-alpha7".to_string()
}

/// A version id is a single path segment with no separators or traversal.
fn safe_version(version: &str) -> bool {
    !version.is_empty()
        && !version.contains("..")
        && !version.contains('/')
        && !version.contains('\\')
}

/// A slug may contain `/` (nested pages) but never `..`, backslashes, or a leading slash.
fn safe_slug(slug: &str) -> bool {
    !slug.is_empty()
        && !slug.contains("..")
        && !slug.contains('\\')
        && !slug.starts_with('/')
}

async fn load_sidebar(version: &str) -> Result<Sidebar, StatusCode> {
    if !safe_version(version) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let path = docs_dir().join(version).join("_sidebar.json");
    let content = tokio::fs::read_to_string(&path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    serde_json::from_str(&content).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

/// Available doc versions (docs/_versions.json) — drives the version switcher.
async fn get_versions() -> Result<Json<serde_json::Value>, StatusCode> {
    let path = docs_dir().join("_versions.json");
    let content = tokio::fs::read_to_string(&path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    let value: serde_json::Value =
        serde_json::from_str(&content).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(value))
}

async fn get_sidebar(Path(version): Path<String>) -> Result<Json<Sidebar>, StatusCode> {
    Ok(Json(load_sidebar(&version).await?))
}

#[derive(Deserialize)]
struct PageQuery {
    /// `md` returns the raw markdown instead of rendered HTML (used by the
    /// engine's in-editor docs viewer, which renders markdown natively).
    format: Option<String>,
}

async fn get_page(
    Path((version, slug)): Path<(String, String)>,
    axum::extract::Query(params): axum::extract::Query<PageQuery>,
) -> Result<Json<DocPage>, StatusCode> {
    if !safe_version(&version) || !safe_slug(&slug) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let md_path = docs_dir().join(&version).join(format!("{slug}.md"));
    let content = tokio::fs::read_to_string(&md_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let title = content
        .lines()
        .find(|l| l.starts_with("# "))
        .map(|l| l.trim_start_matches("# ").to_string())
        .unwrap_or_else(|| slug.clone());

    let (group, category) = find_location(&version, &slug).await.unwrap_or_default();

    let body = if params.format.as_deref() == Some("md") {
        content
    } else {
        let options = Options::all();
        let parser = Parser::new_ext(&content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        optimize_doc_images(&html_output)
    };

    Ok(Json(DocPage {
        version,
        slug,
        title,
        group,
        category,
        content: body,
    }))
}

/// Rewrite `<img src="/assets/previews/NAME.png">` (produced from the docs
/// markdown) into a <picture> serving AVIF with a WebP fallback instead of the
/// multi-hundred-KB uncompressed PNG. Every preview has full-size .avif/.webp
/// generated by scripts/optimize-images.mjs, so no referenced file is missing.
/// Also adds loading="lazy" so offscreen doc images defer.
pub(crate) fn optimize_doc_images(html: &str) -> String {
    const PREFIX: &str = "/assets/previews/";
    let mut out = String::with_capacity(html.len() + html.len() / 4);
    let mut rest = html;
    while let Some(rel) = rest.find("<img ") {
        out.push_str(&rest[..rel]);
        let tail = &rest[rel..];
        let end = match tail.find('>') {
            Some(e) => e + 1,
            None => {
                out.push_str(tail);
                return out;
            }
        };
        let tag = &tail[..end];
        rest = &tail[end..];

        // Extract the preview base name only if this <img> is a previews PNG.
        let base = tag.find(PREFIX).and_then(|p| {
            let after = &tag[p + PREFIX.len()..];
            after.find(".png").map(|d| &after[..d])
        });
        match base {
            Some(base)
                if !base.is_empty()
                    && base.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') =>
            {
                let png = format!("{PREFIX}{base}.png");
                let webp = format!("{PREFIX}{base}.webp");
                let webp_tag = {
                    let swapped = tag.replacen(&png, &webp, 1);
                    if swapped.contains("loading=") {
                        swapped
                    } else {
                        swapped.replacen("<img ", "<img loading=\"lazy\" ", 1)
                    }
                };
                out.push_str(&format!(
                    "<picture><source type=\"image/avif\" srcset=\"{PREFIX}{base}.avif\">{webp_tag}</picture>"
                ));
            }
            _ => out.push_str(tag),
        }
    }
    out.push_str(rest);
    out
}

#[derive(Deserialize)]
struct SearchQuery {
    q: Option<String>,
}

async fn search_docs(
    Path(version): Path<String>,
    axum::extract::Query(params): axum::extract::Query<SearchQuery>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    Ok(Json(do_search(&version, &params.q.unwrap_or_default()).await?))
}

async fn search_docs_default(
    axum::extract::Query(params): axum::extract::Query<SearchQuery>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let version = default_version().await;
    Ok(Json(do_search(&version, &params.q.unwrap_or_default()).await?))
}

async fn do_search(version: &str, raw_query: &str) -> Result<Vec<serde_json::Value>, StatusCode> {
    let query = raw_query.to_lowercase();
    if query.is_empty() {
        return Ok(vec![]);
    }
    let sidebar = load_sidebar(version).await?;
    let mut results = vec![];
    for group in &sidebar.groups {
        for cat in &group.categories {
            for page in &cat.pages {
                let title_hit = page.title.to_lowercase().contains(&query);
                let body_hit = if title_hit {
                    false
                } else {
                    let md_path = docs_dir().join(version).join(format!("{}.md", page.slug));
                    tokio::fs::read_to_string(&md_path)
                        .await
                        .map(|md| md.to_lowercase().contains(&query))
                        .unwrap_or(false)
                };
                if title_hit || body_hit {
                    results.push(serde_json::json!({
                        "slug": page.slug,
                        "title": page.title,
                        "group": group.group,
                        "category": cat.category,
                        "version": version,
                    }));
                }
            }
        }
    }
    Ok(results)
}

/// Resolve which (group, category) a slug belongs to, for breadcrumbs.
async fn find_location(version: &str, slug: &str) -> Option<(String, String)> {
    let sidebar = load_sidebar(version).await.ok()?;
    for group in &sidebar.groups {
        for cat in &group.categories {
            for page in &cat.pages {
                if page.slug == slug {
                    return Some((group.group.clone(), cat.category.clone()));
                }
            }
        }
    }
    None
}
