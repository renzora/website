use axum::{extract::Path, http::StatusCode, routing::get, Json, Router};
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
struct SidebarSection {
    label: String,
    description: String,
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
    slug: String,
    title: String,
    category: String,
    section: String,
    content: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_sidebar))
        .route("/search", get(search_docs))
        .route("/*slug", get(get_page))
}

fn docs_dir() -> PathBuf {
    PathBuf::from("docs")
}

async fn get_sidebar() -> Result<Json<serde_json::Value>, StatusCode> {
    let sidebar_path = docs_dir().join("_sidebar.json");
    let content = tokio::fs::read_to_string(&sidebar_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    let sections: std::collections::HashMap<String, SidebarSection> =
        serde_json::from_str(&content).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!(sections)))
}

async fn get_page(Path(slug): Path<String>) -> Result<Json<DocPage>, StatusCode> {
    let md_path = docs_dir().join(format!("{slug}.md"));
    let content = tokio::fs::read_to_string(&md_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let title = content
        .lines()
        .find(|l| l.starts_with("# "))
        .map(|l| l.trim_start_matches("# ").to_string())
        .unwrap_or_else(|| slug.clone());

    let (section, category) = find_section_for_slug(&slug).await.unwrap_or_default();

    let options = Options::all();
    let parser = Parser::new_ext(&content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(Json(DocPage {
        slug,
        title,
        category,
        section,
        content: html_output,
    }))
}

#[derive(Deserialize)]
struct SearchQuery {
    q: Option<String>,
}

async fn search_docs(
    axum::extract::Query(params): axum::extract::Query<SearchQuery>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let query = params.q.unwrap_or_default().to_lowercase();
    if query.is_empty() {
        return Ok(Json(vec![]));
    }

    let sidebar_path = docs_dir().join("_sidebar.json");
    let content = tokio::fs::read_to_string(&sidebar_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;
    let sections: std::collections::HashMap<String, SidebarSection> =
        serde_json::from_str(&content).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut results = vec![];
    for (key, section) in &sections {
        for cat in &section.categories {
            for page in &cat.pages {
                // Search title
                if page.title.to_lowercase().contains(&query) {
                    results.push(serde_json::json!({
                        "slug": page.slug, "title": page.title,
                        "section": key, "category": cat.category,
                    }));
                    continue;
                }
                // Search file content
                let md_path = docs_dir().join(format!("{}.md", page.slug));
                if let Ok(md) = tokio::fs::read_to_string(&md_path).await {
                    if md.to_lowercase().contains(&query) {
                        results.push(serde_json::json!({
                            "slug": page.slug, "title": page.title,
                            "section": key, "category": cat.category,
                        }));
                    }
                }
            }
        }
    }
    Ok(Json(results))
}

async fn find_section_for_slug(slug: &str) -> Option<(String, String)> {
    let sidebar_path = docs_dir().join("_sidebar.json");
    let content = tokio::fs::read_to_string(&sidebar_path).await.ok()?;
    let sections: std::collections::HashMap<String, SidebarSection> =
        serde_json::from_str(&content).ok()?;
    for (key, section) in &sections {
        for cat in &section.categories {
            for page in &cat.pages {
                if page.slug == slug {
                    return Some((key.clone(), cat.category.clone()));
                }
            }
        }
    }
    None
}
