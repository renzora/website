use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Auth requests ──

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub referral_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

// ── Auth responses ──

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserProfile,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub credit_balance: i64,
    pub discord_username: Option<String>,
    pub discord_avatar: Option<String>,
    pub totp_enabled: bool,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

// ── Marketplace types ──

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetSummary {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub category: String,
    pub price_credits: i64,
    pub thumbnail_url: Option<String>,
    pub version: String,
    pub downloads: i64,
    pub views: i64,
    pub creator_name: String,
    pub creator_avatar_url: Option<String>,
    pub rating_avg: f64,
    pub rating_count: i32,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetDetail {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub category: String,
    pub price_credits: i64,
    pub file_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub version: String,
    pub downloads: i64,
    pub views: i64,
    pub published: bool,
    pub rating_sum: i64,
    pub rating_count: i32,
    pub tags: Vec<String>,
    pub licence: String,
    pub ai_generated: bool,
    pub metadata: serde_json::Value,
    pub download_filename: String,
    pub subcategory: String,
    pub credit_name: String,
    pub credit_url: String,
    pub creator: UserProfile,
    pub created_at: String,
    pub updated_at: String,
    /// Whether the current user owns this asset (only set when authenticated).
    pub owned: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct MarketplaceQuery {
    pub q: Option<String>,
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub tag: Option<String>,
    pub sort: Option<String>,
    pub page: Option<i64>,
    pub free: Option<bool>,
    pub min_rating: Option<i32>,
    pub max_price: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct MarketplaceListResponse {
    pub assets: Vec<AssetSummary>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Deserialize)]
pub struct UploadAssetRequest {
    pub name: String,
    pub description: String,
    pub category: String,
    pub price_credits: i64,
    pub version: String,
    /// Up to 5 tags for discoverability.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Licence: "standard", "extended", "cc0", "mit", "apache2", "gpl3"
    #[serde(default = "default_licence")]
    pub licence: String,
    /// Whether this asset contains AI-generated content.
    #[serde(default)]
    pub ai_generated: bool,
    /// Flexible metadata (render pipeline, texture resolution, poly count, etc.)
    /// Example: {"render_pipeline":"pbr","texture_resolution":"2048x2048","poly_count":1500}
    #[serde(default)]
    pub metadata: serde_json::Value,
    /// Human-readable filename for downloads (auto-populated from uploaded file).
    #[serde(default)]
    pub download_filename: String,
    /// Subcategory slug (optional).
    #[serde(default)]
    pub subcategory: String,
    /// Original creator name (attribution). Forces price to free when set.
    #[serde(default)]
    pub credit_name: String,
    /// Link to original creator or source.
    #[serde(default)]
    pub credit_url: String,
}

fn default_licence() -> String { "standard".to_string() }

/// Valid licence identifiers.
pub const VALID_LICENCES: &[&str] = &["standard", "extended", "cc0", "mit", "apache2", "gpl3"];

#[derive(Debug, Deserialize)]
pub struct UpdateAssetRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price_credits: Option<i64>,
    pub version: Option<String>,
    pub published: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub licence: Option<String>,
    pub ai_generated: Option<bool>,
    pub metadata: Option<serde_json::Value>,
    pub download_filename: Option<String>,
    pub subcategory: Option<String>,
    pub credit_name: Option<String>,
    pub credit_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DownloadResponse {
    pub download_url: String,
    pub download_filename: String,
}

#[derive(Debug, Serialize)]
pub struct CreatorAssetsResponse {
    pub assets: Vec<AssetDetail>,
}

// ── Game Store types ──

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameSummary {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub category: String,
    pub price_credits: i64,
    pub thumbnail_url: Option<String>,
    pub version: String,
    pub downloads: i64,
    pub views: i64,
    pub creator_name: String,
    pub rating_avg: f64,
    pub rating_count: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct GameDetail {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub category: String,
    pub price_credits: i64,
    pub file_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub version: String,
    pub downloads: i64,
    pub views: i64,
    pub published: bool,
    pub creator: UserProfile,
    pub created_at: String,
    pub updated_at: String,
    pub owned: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct GameStoreListResponse {
    pub games: Vec<GameSummary>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Deserialize)]
pub struct GameStoreQuery {
    pub q: Option<String>,
    pub category: Option<String>,
    pub sort: Option<String>,
    pub page: Option<i64>,
    pub free: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct CreatorGamesResponse {
    pub games: Vec<GameDetail>,
}

#[derive(Debug, Deserialize)]
pub struct PurchaseGameRequest {
    pub game_id: Uuid,
    pub promo_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GameMediaResponse {
    pub id: Uuid,
    pub media_type: String,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Serialize)]
pub struct GameCategoryResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub icon: String,
    pub sort_order: i32,
}

// ── Credits types ──

#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub credit_balance: i64,
}

#[derive(Debug, Deserialize)]
pub struct TopUpRequest {
    /// Amount in credits to purchase.
    pub amount: i64,
}

#[derive(Debug, Serialize)]
pub struct TopUpResponse {
    /// Stripe Checkout session URL to redirect the user to.
    pub checkout_url: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct TransactionEntry {
    pub id: Uuid,
    pub r#type: String,
    pub amount: i64,
    pub asset_id: Option<Uuid>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct TransactionHistoryResponse {
    pub transactions: Vec<TransactionEntry>,
    pub total: i64,
    pub page: i64,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PurchaseRequest {
    pub asset_id: Uuid,
    pub promo_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PurchaseResponse {
    pub message: String,
    pub new_balance: i64,
}

// ── Documentation types ──

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocEntry {
    pub slug: String,
    pub title: String,
    pub category: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct DocPageResponse {
    pub slug: String,
    pub title: String,
    pub content: String,
    pub category: String,
}

#[derive(Debug, Serialize)]
pub struct DocListResponse {
    pub categories: Vec<DocCategoryGroup>,
}

#[derive(Debug, Serialize, Clone)]
pub struct DocCategoryGroup {
    pub category: String,
    pub pages: Vec<DocEntry>,
}

#[derive(Debug, Deserialize)]
pub struct DocSearchQuery {
    pub q: Option<String>,
}

// ── Article types ──

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticleSummary {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub cover_image_url: Option<String>,
    pub likes: i32,
    pub views: i32,
    pub author_name: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ArticleDetailResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content: String,
    pub tags: Vec<String>,
    pub cover_image_url: Option<String>,
    pub likes: i32,
    pub views: i32,
    pub author: UserProfile,
    pub created_at: String,
    pub updated_at: String,
    pub user_has_liked: Option<bool>,
    pub comments: Vec<CommentResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentResponse {
    pub id: Uuid,
    pub content: String,
    pub author_name: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct ArticleListResponse {
    pub articles: Vec<ArticleSummary>,
    pub total: i64,
    pub page: i64,
}

#[derive(Debug, Deserialize)]
pub struct ArticleListQuery {
    pub tag: Option<String>,
    pub sort: Option<String>,
    pub page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateArticleRequest {
    pub title: String,
    pub summary: String,
    pub content: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
    pub published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct LikeResponse {
    pub liked: bool,
    pub total_likes: i32,
}

// ── Creator dashboard types ──

#[derive(Debug, Serialize)]
pub struct CreatorStatsResponse {
    pub total_assets: i64,
    pub total_downloads: i64,
    pub total_earnings: i64,
    pub credit_balance: i64,
    pub top_assets: Vec<AssetSummary>,
}

#[derive(Debug, Serialize)]
pub struct CreatorEarningsResponse {
    pub earnings: Vec<EarningEntry>,
    pub total: i64,
    pub page: i64,
}

#[derive(Debug, Serialize, Clone)]
pub struct EarningEntry {
    pub id: Uuid,
    pub amount: i64,
    pub asset_name: String,
    pub created_at: String,
}
