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
    /// Individual files within this asset (populated for multi-file assets).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<AssetFileInfo>,
}

/// Information about an individual file within a multi-file asset.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetFileInfo {
    pub id: Uuid,
    pub original_filename: String,
    pub file_size: i64,
    pub mime_type: String,
    pub sort_order: i32,
    /// For paid unowned assets: URL to the watermarked/clipped preview. None if unavailable.
    pub preview_url: Option<String>,
    /// For owned/free assets: presigned download URL. None if not authorized.
    pub download_url: Option<String>,
}

// ── Releases, file tree and README/docs ──

/// One published version of an asset.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReleaseInfo {
    pub id: Uuid,
    pub version: String,
    /// Release notes as the creator wrote them (markdown).
    pub notes: String,
    /// The same notes rendered to sanitised HTML.
    pub notes_html: String,
    pub is_current: bool,
    pub downloads: i64,
    pub file_count: i64,
    pub total_size: i64,
    pub created_at: String,
}

/// A node in an asset's file tree. Directories are synthesised from the file
/// paths, so they have no id and their size is the sum of their contents.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetTreeEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    /// Full path inside the archive, e.g. `docs/install.md`.
    pub path: String,
    /// Last path segment, e.g. `install.md`.
    pub name: String,
    /// `"file"` or `"dir"`.
    pub kind: String,
    pub size: i64,
    pub mime_type: String,
    /// Readable without owning the asset — markdown docs and licence files.
    pub is_doc: bool,
}

/// A heading in a rendered markdown document, for the "on this page" nav.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocHeading {
    pub level: u8,
    pub text: String,
    pub anchor: String,
}

/// A rendered markdown document from an asset's archive.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RenderedDoc {
    pub path: String,
    pub html: String,
    pub outline: Vec<DocHeading>,
}

/// The whole file tree of one release, plus its README — everything the asset
/// page needs to draw the repo view in a single request.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetTreeResponse {
    pub release: ReleaseInfo,
    pub entries: Vec<AssetTreeEntry>,
    /// The README nearest the archive root, already rendered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<RenderedDoc>,
    /// Whether the caller may read file *contents*. The tree itself is public.
    pub has_access: bool,
}

/// One file, opened in the browser's viewer.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetFileView {
    pub path: String,
    pub name: String,
    /// `markdown` | `text` | `image` | `binary` | `locked`
    pub kind: String,
    pub mime_type: String,
    pub size: i64,
    /// Rendered HTML, for `markdown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// Source text, for `text`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// highlight.js language hint, derived from the extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub outline: Vec<DocHeading>,
    /// Presigned URL, for `binary`/`image` when the caller has access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// Set when a large text file was cut short.
    pub truncated: bool,
}

/// Create a release. Sent as multipart alongside the files, so this is the
/// JSON `metadata` part rather than the whole body.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReleaseRequest {
    pub version: String,
    #[serde(default)]
    pub notes: String,
    /// `"keep"` (store the zip as-is) or `"extract"` (unpack into a tree).
    #[serde(default = "default_release_zip_action")]
    pub zip_action: String,
}

/// Releases default to unpacking the archive, because the file tree and the
/// README docs only exist once it has been extracted.
fn default_release_zip_action() -> String {
    "extract".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateReleaseRequest {
    pub notes: Option<String>,
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
    /// For zip uploads: "keep" (store as-is) or "extract" (unpack into individual files).
    #[serde(default = "default_zip_action")]
    pub zip_action: String,
}

fn default_zip_action() -> String { "keep".to_string() }

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
    /// Individual file download URLs for multi-file assets.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<AssetFileInfo>,
}

#[derive(Debug, Serialize)]
pub struct CreatorAssetsResponse {
    pub assets: Vec<AssetDetail>,
}

// ── Credits types ──

#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub credit_balance: i64,
    pub earnings_balance: i64,
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

#[derive(Debug, Serialize)]
pub struct PurchasePartResponse {
    pub message: String,
    pub new_balance: i64,
}

// ── XP / Level types ──

#[derive(Debug, Serialize)]
pub struct UserLevelResponse {
    pub total_xp: i64,
    pub level: i32,
    pub xp_for_current_level: i64,
    pub xp_for_next_level: i64,
    pub progress_percent: f64,
    pub seller_level: i32,
    pub seller_xp: i64,
    pub seller_level_name: String,
    pub seller_level_color: String,
    pub next_seller_level_xp: i64,
    pub seller_progress_percent: f64,
}

#[derive(Debug, Serialize)]
pub struct SellerProgressResponse {
    pub current_level: serde_json::Value,
    pub next_level: Option<serde_json::Value>,
    pub tasks: Vec<SellerTaskProgress>,
    pub seller_xp: i64,
    pub xp_to_next: i64,
    pub progress_percent: f64,
}

#[derive(Debug, Serialize)]
pub struct SellerTaskProgress {
    pub description: String,
    pub task_type: String,
    pub target_value: i64,
    pub current_value: i64,
    pub completed: bool,
    pub xp_reward: i64,
}

#[derive(Debug, Serialize)]
pub struct XpEventResponse {
    pub amount: i64,
    pub reason: String,
    pub created_at: String,
}
