use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use renzora_common::types::*;
use renzora_models::doc::Doc;

use crate::{error::ApiError, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_docs))
        .route("/search", get(search_docs))
        .route("/:slug", get(get_doc))
}

/// List all published docs grouped by category.
async fn list_docs(
    State(state): State<AppState>,
) -> Result<Json<DocListResponse>, ApiError> {
    let docs = Doc::list_published(&state.db).await?;

    let mut groups: Vec<DocCategoryGroup> = Vec::new();
    for doc in docs {
        let entry = DocEntry {
            slug: doc.slug,
            title: doc.title,
            category: doc.category.clone(),
        };
        if let Some(group) = groups.iter_mut().find(|g| g.category == doc.category) {
            group.pages.push(entry);
        } else {
            groups.push(DocCategoryGroup {
                category: doc.category,
                pages: vec![entry],
            });
        }
    }

    Ok(Json(DocListResponse { categories: groups }))
}

/// Search docs by title or content.
async fn search_docs(
    State(state): State<AppState>,
    Query(params): Query<DocSearchQuery>,
) -> Result<Json<Vec<DocEntry>>, ApiError> {
    let query = params.q.unwrap_or_default();
    if query.is_empty() {
        return Ok(Json(vec![]));
    }

    let docs = Doc::search(&state.db, &query).await?;
    let entries: Vec<DocEntry> = docs
        .into_iter()
        .map(|d| DocEntry {
            slug: d.slug,
            title: d.title,
            category: d.category,
        })
        .collect();

    Ok(Json(entries))
}

/// Get a single doc page by slug.
async fn get_doc(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<DocPageResponse>, ApiError> {
    let doc = Doc::find_by_slug(&state.db, &slug)
        .await?
        .ok_or(ApiError::NotFound)?;

    Ok(Json(DocPageResponse {
        slug: doc.slug,
        title: doc.title,
        content: doc.content,
        category: doc.category,
    }))
}
