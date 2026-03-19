use axum::{extract::Path, http::StatusCode, routing::get, Json, Router};
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct SidebarCategory {
    category: String,
    pages: Vec<SidebarPage>,
}

#[derive(Serialize, Deserialize)]
struct SidebarPage {
    slug: String,
    title: String,
}

#[derive(Serialize)]
struct DocPage {
    slug: String,
    title: String,
    category: String,
    content: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_sidebar))
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
    let categories: Vec<SidebarCategory> =
        serde_json::from_str(&content).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({ "categories": categories })))
}

async fn get_page(Path(slug): Path<String>) -> Result<Json<DocPage>, StatusCode> {
    let md_path = docs_dir().join(format!("{slug}.md"));
    let content = tokio::fs::read_to_string(&md_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // Extract title from first # heading
    let title = content
        .lines()
        .find(|l| l.starts_with("# "))
        .map(|l| l.trim_start_matches("# ").to_string())
        .unwrap_or_else(|| slug.clone());

    // Find category from sidebar
    let category = find_category_for_slug(&slug).await.unwrap_or_default();

    // Render markdown to HTML
    let options = Options::all();
    let parser = Parser::new_ext(&content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(Json(DocPage {
        slug,
        title,
        category,
        content: html_output,
    }))
}

async fn find_category_for_slug(slug: &str) -> Option<String> {
    let sidebar_path = docs_dir().join("_sidebar.json");
    let content = tokio::fs::read_to_string(&sidebar_path).await.ok()?;
    let categories: Vec<SidebarCategory> = serde_json::from_str(&content).ok()?;
    for cat in &categories {
        for page in &cat.pages {
            if page.slug == slug {
                return Some(cat.category.clone());
            }
        }
    }
    None
}
