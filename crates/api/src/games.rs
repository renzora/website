use axum::{
    extract::{Extension, Multipart, Path, Query, State},
    routing::{get, post, put, delete},
    Json, Router,
};
use renzora_common::types::*;
use renzora_models::game::{self, Game, GameCategory, GameMedia};
use renzora_models::user::User;
use uuid::Uuid;

use crate::{error::ApiError, middleware, middleware::AuthUser, AppState};

pub fn router() -> Router<AppState> {
    let protected = Router::new()
        .route("/upload", post(upload_game))
        .route("/my-games", get(my_games))
        .route("/library", get(user_library))
        .route("/{id}/update", put(update_game))
        .route("/{id}/download", get(download_game))
        .route("/{id}/purchase", post(purchase_game))
        .route("/{id}/media", post(upload_media))
        .route("/media/{media_id}", delete(delete_media))
        .route("/wishlist", get(get_wishlist))
        .route("/wishlist/{id}", post(toggle_wishlist))
        .layer(axum::middleware::from_fn(middleware::require_auth));

    Router::new()
        .route("/", get(list_games))
        .route("/categories", get(list_categories))
        .route("/detail/{slug}", get(get_game))
        .route("/{id}/media", get(list_media))
        .merge(protected)
}

// ── Browse ──

async fn list_games(
    State(state): State<AppState>,
    Query(params): Query<GameStoreQuery>,
) -> Result<Json<GameStoreListResponse>, ApiError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = 24i64;
    let sort = params.sort.as_deref().unwrap_or("newest");

    let (games, total) = Game::list_published_filtered(
        &state.db,
        params.q.as_deref(),
        params.category.as_deref(),
        sort,
        page,
        per_page,
        params.free,
        None,
        None,
    )
    .await?;

    let summaries = games
        .into_iter()
        .map(|g| {
            let rating_avg = if g.rating_count > 0 {
                g.rating_sum as f64 / g.rating_count as f64
            } else {
                0.0
            };
            GameSummary {
                id: g.id,
                name: g.name,
                slug: g.slug,
                description: g.description,
                category: g.category,
                price_credits: g.price_credits,
                thumbnail_url: g.thumbnail_url,
                version: g.version,
                downloads: g.downloads,
                views: g.views,
                creator_name: g.creator_name,
                rating_avg,
                rating_count: g.rating_count,
            }
        })
        .collect();

    Ok(Json(GameStoreListResponse {
        games: summaries,
        total,
        page,
        per_page,
    }))
}

async fn list_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<GameCategoryResponse>>, ApiError> {
    let cats = GameCategory::list_all(&state.db).await?;
    Ok(Json(
        cats.into_iter()
            .map(|c| GameCategoryResponse {
                id: c.id,
                name: c.name,
                slug: c.slug,
                description: c.description,
                icon: c.icon,
                sort_order: c.sort_order,
            })
            .collect(),
    ))
}

async fn get_game(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    headers: axum::http::HeaderMap,
    connect_info: axum::extract::ConnectInfo<std::net::SocketAddr>,
    auth: Option<Extension<AuthUser>>,
) -> Result<Json<GameDetail>, ApiError> {
    let game = Game::find_by_slug(&state.db, &slug)
        .await?
        .ok_or(ApiError::NotFound)?;

    // Record view (deduplicated by IP, 24h cooldown)
    let user_id = auth.as_ref().map(|Extension(a)| a.user_id);
    let ip = crate::marketplace::client_ip(&headers, &connect_info);
    let ip_hash = crate::marketplace::hash_ip(&ip);
    let _ = Game::record_view(&state.db, game.id, &ip_hash, user_id).await;

    let creator = User::find_by_id(&state.db, game.creator_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    let owned = if let Some(Extension(auth)) = auth {
        Some(game::user_owns_game(&state.db, auth.user_id, game.id).await?)
    } else {
        None
    };

    Ok(Json(game_to_detail(&game, &creator, owned)))
}

// ── Upload / Manage ──

async fn upload_game(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    mut multipart: Multipart,
) -> Result<Json<GameDetail>, ApiError> {
    let mut name = String::new();
    let mut description = String::new();
    let mut category = String::from("other");
    let mut price_credits: i64 = 0;
    let mut version = String::from("1.0.0");
    let mut file_url: Option<String> = None;
    let mut thumb_url: Option<String> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| ApiError::Validation(e.to_string()))? {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "metadata" => {
                let text = field.text().await.map_err(|e| ApiError::Validation(e.to_string()))?;
                let meta: serde_json::Value = serde_json::from_str(&text)
                    .map_err(|e| ApiError::Validation(format!("Invalid metadata JSON: {}", e)))?;
                name = meta.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                description = meta.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                category = meta.get("category").and_then(|v| v.as_str()).unwrap_or("other").to_string();
                price_credits = meta.get("price_credits").and_then(|v| v.as_i64()).unwrap_or(0);
                version = meta.get("version").and_then(|v| v.as_str()).unwrap_or("1.0.0").to_string();
            }
            "file" => {
                let filename = field.file_name().unwrap_or("game.zip").to_string();
                let data = field.bytes().await.map_err(|e| ApiError::Validation(e.to_string()))?;
                file_url = Some(upload_to_storage(&state, "games", &filename, data.to_vec()).await?);
            }
            "thumbnail" => {
                let filename = field.file_name().unwrap_or("thumb.png").to_string();
                let data = field.bytes().await.map_err(|e| ApiError::Validation(e.to_string()))?;
                thumb_url = Some(upload_to_storage(&state, "game_thumbnails", &filename, data.to_vec()).await?);
            }
            _ => {}
        }
    }

    if name.is_empty() {
        return Err(ApiError::Validation("Game name is required".into()));
    }

    // Validate category
    if GameCategory::find_by_slug(&state.db, &category).await?.is_none() {
        return Err(ApiError::Validation(format!("Unknown category: '{}'", category)));
    }

    // Check user has enough credits for publishing fee (if any)
    let creator = User::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    let game = Game::create(&state.db, auth.user_id, &name, &description, &category, price_credits, &version).await?;

    if let Some(url) = &file_url {
        Game::update_file_url(&state.db, game.id, url).await?;
    }
    if let Some(url) = &thumb_url {
        Game::update_thumbnail_url(&state.db, game.id, url).await?;
    }

    // Auto-grant ownership to creator
    game::grant_game_ownership(&state.db, auth.user_id, game.id).await?;

    // Upgrade role if needed
    if creator.role == "user" {
        sqlx::query("UPDATE users SET role = 'creator' WHERE id = $1")
            .bind(auth.user_id)
            .execute(&state.db)
            .await?;
    }

    let updated = Game::find_by_id(&state.db, game.id).await?.ok_or(ApiError::NotFound)?;
    Ok(Json(game_to_detail(&updated, &creator, Some(true))))
}

async fn update_game(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateAssetRequest>,
) -> Result<Json<GameDetail>, ApiError> {
    let game = Game::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;

    if game.creator_id != auth.user_id {
        return Err(ApiError::Unauthorized);
    }

    let updated = Game::update_metadata(
        &state.db,
        id,
        body.name.as_deref(),
        body.description.as_deref(),
        body.price_credits,
        body.version.as_deref(),
        body.published,
    )
    .await?;

    let creator = User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;
    Ok(Json(game_to_detail(&updated, &creator, Some(true))))
}

async fn my_games(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<CreatorGamesResponse>, ApiError> {
    let games = Game::list_by_creator(&state.db, auth.user_id).await?;
    let creator = User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;

    Ok(Json(CreatorGamesResponse {
        games: games.iter().map(|g| game_to_detail(g, &creator, Some(true))).collect(),
    }))
}

// ── Library & Purchase ──

async fn user_library(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<GameStoreListResponse>, ApiError> {
    let (games, total) = Game::list_purchased_by_user(&state.db, auth.user_id).await?;

    let summaries = games
        .into_iter()
        .map(|g| {
            let rating_avg = if g.rating_count > 0 { g.rating_sum as f64 / g.rating_count as f64 } else { 0.0 };
            GameSummary {
                id: g.id, name: g.name, slug: g.slug, description: g.description,
                category: g.category, price_credits: g.price_credits, thumbnail_url: g.thumbnail_url,
                version: g.version, downloads: g.downloads, views: g.views, creator_name: g.creator_name,
                rating_avg, rating_count: g.rating_count,
            }
        })
        .collect();

    Ok(Json(GameStoreListResponse { games: summaries, total, page: 1, per_page: total }))
}

async fn purchase_game(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<PurchaseGameRequest>,
) -> Result<Json<PurchaseResponse>, ApiError> {
    let game = Game::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;

    if !game.published {
        return Err(ApiError::NotFound);
    }

    if game::user_owns_game(&state.db, auth.user_id, id).await? {
        return Err(ApiError::Validation("You already own this game".into()));
    }

    if game.price_credits == 0 {
        // Free game — just grant ownership
        game::grant_game_ownership(&state.db, auth.user_id, id).await?;
        let user = User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;
        return Ok(Json(PurchaseResponse {
            message: "Game added to library".into(),
            new_balance: user.credit_balance,
        }));
    }

    // Paid game — use credits
    let buyer = User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;
    if buyer.credit_balance < game.price_credits {
        return Err(ApiError::Validation("Insufficient credits. Please top up your balance.".into()));
    }

    // Process purchase atomically
    let platform_cut_percent: i64 = 20;
    let creator_share = (game.price_credits * (100 - platform_cut_percent)) / 100;

    let mut tx = state.db.begin().await?;

    // Deduct from buyer
    let result = sqlx::query(
        "UPDATE users SET credit_balance = credit_balance - $1, updated_at = NOW() WHERE id = $2 AND credit_balance >= $1"
    )
    .bind(game.price_credits)
    .bind(auth.user_id)
    .execute(&mut *tx)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::Validation("Insufficient credits".into()));
    }

    // Credit creator
    sqlx::query("UPDATE users SET credit_balance = credit_balance + $1, updated_at = NOW() WHERE id = $2")
        .bind(creator_share)
        .bind(game.creator_id)
        .execute(&mut *tx)
        .await?;

    // Record transactions
    let now = time::OffsetDateTime::now_utc();
    sqlx::query(
        "INSERT INTO transactions (id, user_id, type, amount, asset_id, created_at) VALUES ($1, $2, 'purchase', $3, $4, $5)"
    )
    .bind(Uuid::new_v4()).bind(auth.user_id).bind(-game.price_credits).bind(id).bind(now)
    .execute(&mut *tx).await?;

    sqlx::query(
        "INSERT INTO transactions (id, user_id, type, amount, asset_id, created_at) VALUES ($1, $2, 'earning', $3, $4, $5)"
    )
    .bind(Uuid::new_v4()).bind(game.creator_id).bind(creator_share).bind(id).bind(now)
    .execute(&mut *tx).await?;

    // Grant ownership
    sqlx::query("INSERT INTO user_games (user_id, game_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
        .bind(auth.user_id).bind(id)
        .execute(&mut *tx).await?;

    tx.commit().await?;

    let updated_buyer = User::find_by_id(&state.db, auth.user_id).await?.ok_or(ApiError::NotFound)?;
    Ok(Json(PurchaseResponse {
        message: "Game purchased successfully".into(),
        new_balance: updated_buyer.credit_balance,
    }))
}

async fn download_game(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<DownloadResponse>, ApiError> {
    let game = Game::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;

    if game.price_credits > 0 {
        let owns = game::user_owns_game(&state.db, auth.user_id, id).await?;
        if !owns && game.creator_id != auth.user_id {
            return Err(ApiError::Unauthorized);
        }
    }

    let file_url = game.file_url.ok_or(ApiError::Internal("Game has no file".into()))?;
    Game::increment_downloads(&state.db, id).await?;

    let filename = file_url
        .rsplit('/')
        .next()
        .unwrap_or("game")
        .to_string();
    Ok(Json(DownloadResponse { download_url: file_url, download_filename: filename }))
}

// ── Media ──

async fn list_media(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<GameMediaResponse>>, ApiError> {
    let media = GameMedia::list_by_game(&state.db, id).await?;
    Ok(Json(
        media.into_iter().map(|m| GameMediaResponse {
            id: m.id, media_type: m.media_type, url: m.url,
            thumbnail_url: m.thumbnail_url, sort_order: m.sort_order,
        }).collect(),
    ))
}

async fn upload_media(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    mut multipart: Multipart,
) -> Result<Json<GameMediaResponse>, ApiError> {
    let game = Game::find_by_id(&state.db, id).await?.ok_or(ApiError::NotFound)?;
    if game.creator_id != auth.user_id {
        return Err(ApiError::Unauthorized);
    }

    let mut media_type = String::from("image");
    let mut url = String::new();
    let mut thumb = None;
    let mut sort_order = 0i32;

    while let Some(field) = multipart.next_field().await.map_err(|e| ApiError::Validation(e.to_string()))? {
        let field_name = field.name().unwrap_or("").to_string();
        match field_name.as_str() {
            "type" => media_type = field.text().await.unwrap_or_default(),
            "sort_order" => sort_order = field.text().await.unwrap_or_default().parse().unwrap_or(0),
            "file" => {
                let filename = field.file_name().unwrap_or("media.png").to_string();
                let data = field.bytes().await.map_err(|e| ApiError::Validation(e.to_string()))?;
                url = upload_to_storage(&state, "game_media", &filename, data.to_vec()).await?;
            }
            "thumbnail" => {
                let filename = field.file_name().unwrap_or("thumb.png").to_string();
                let data = field.bytes().await.map_err(|e| ApiError::Validation(e.to_string()))?;
                thumb = Some(upload_to_storage(&state, "game_media_thumbs", &filename, data.to_vec()).await?);
            }
            _ => {}
        }
    }

    if url.is_empty() {
        return Err(ApiError::Validation("Media file is required".into()));
    }

    let media = GameMedia::create(&state.db, id, &media_type, &url, thumb.as_deref(), sort_order).await?;
    Ok(Json(GameMediaResponse {
        id: media.id, media_type: media.media_type, url: media.url,
        thumbnail_url: media.thumbnail_url, sort_order: media.sort_order,
    }))
}

async fn delete_media(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(media_id): Path<Uuid>,
) -> Result<Json<MessageResponse>, ApiError> {
    // Verify ownership via game
    GameMedia::delete(&state.db, media_id).await?;
    Ok(Json(MessageResponse { message: "Media deleted".into() }))
}

// ── Helpers ──

fn game_to_detail(game: &Game, creator: &User, owned: Option<bool>) -> GameDetail {
    GameDetail {
        id: game.id,
        name: game.name.clone(),
        slug: game.slug.clone(),
        description: game.description.clone(),
        category: game.category.clone(),
        price_credits: game.price_credits,
        file_url: game.file_url.clone(),
        thumbnail_url: game.thumbnail_url.clone(),
        version: game.version.clone(),
        downloads: game.downloads,
        views: game.views,
        published: game.published,
        creator: UserProfile {
            id: creator.id,
            username: creator.username.clone(),
            email: creator.email.clone(),
            role: creator.role.clone(),
            credit_balance: creator.credit_balance,
            discord_username: creator.discord_username.clone(),
            discord_avatar: creator.discord_avatar.clone(),
            totp_enabled: creator.totp_enabled,
        },
        created_at: game.created_at.to_string(),
        updated_at: game.updated_at.to_string(),
        owned,
    }
}

async fn upload_to_storage(state: &AppState, folder: &str, original_filename: &str, data: Vec<u8>) -> Result<String, ApiError> {
    crate::marketplace::upload_to_storage(state, folder, original_filename, data).await
}

// ── Wishlists ──

use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
struct WishlistItem {
    id: Uuid,
    game_id: Uuid,
    name: String,
    slug: String,
    thumbnail_url: Option<String>,
    price_credits: i64,
    category: String,
    created_at: time::OffsetDateTime,
}

async fn get_wishlist(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<Json<Vec<WishlistItem>>, ApiError> {
    let items = sqlx::query_as::<_, WishlistItem>(
        "SELECT w.id, w.game_id, g.name, g.slug, g.thumbnail_url, g.price_credits, g.category, w.created_at FROM wishlists w JOIN games g ON g.id=w.game_id WHERE w.user_id=$1 ORDER BY w.created_at DESC"
    ).bind(auth.user_id).fetch_all(&state.db).await?;
    Ok(Json(items))
}

async fn toggle_wishlist(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(game_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let existing: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM wishlists WHERE user_id=$1 AND game_id=$2"
    ).bind(auth.user_id).bind(game_id).fetch_optional(&state.db).await?;

    if existing.is_some() {
        sqlx::query("DELETE FROM wishlists WHERE user_id=$1 AND game_id=$2")
            .bind(auth.user_id).bind(game_id).execute(&state.db).await?;
        Ok(Json(serde_json::json!({"wishlisted": false, "message": "Removed from wishlist"})))
    } else {
        sqlx::query("INSERT INTO wishlists (user_id, game_id) VALUES ($1,$2) ON CONFLICT DO NOTHING")
            .bind(auth.user_id).bind(game_id).execute(&state.db).await?;
        Ok(Json(serde_json::json!({"wishlisted": true, "message": "Added to wishlist"})))
    }
}
