//! Data-transfer objects for server-rendered content pages.
//!
//! The server fetches these from the database and provides them via Leptos
//! context (`render_app_to_stream_with_context`); the page components read them
//! with `use_context` to emit crawlable HTML + per-page `<title>`/meta. App /
//! auth-gated pages don't use this — they stay client-rendered.

use serde::{Deserialize, Serialize};

/// JSON-escape a string for safe embedding in a JSON-LD literal (includes the
/// surrounding quotes).
pub fn json_escape(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    o.push('"');
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

/// A post summary for server-rendered channel/hub feed listings.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SsrPostItem {
    pub id: String,
    pub body: String,
    pub username: String,
    pub channel_slug: Option<String>,
    pub channel_name: Option<String>,
    pub like_count: i32,
    pub comment_count: i32,
    pub created_at: String,
}

/// Community hub (`/community`) and channel pages (`/community/channel/:slug`).
/// Server-rendered so channels are indexable with their own title + posts.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CommunitySsr {
    pub found: bool,
    pub is_channel: bool,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub posts: Vec<SsrPostItem>,
}

/// A comment on a community post (for the SSR permalink page).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SsrComment {
    pub username: String,
    pub body: String,
    pub created_at: String,
}

/// Community post permalink — `/community/post/:id`. Server-rendered so each
/// public discussion is its own indexable page. Only public, non-hidden posts.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PostSsr {
    pub found: bool,
    pub id: String,
    pub body: String,
    pub username: String,
    pub channel_slug: Option<String>,
    pub channel_name: Option<String>,
    pub like_count: i32,
    pub comment_count: i32,
    pub created_at: String,
    pub comments: Vec<SsrComment>,
}

/// Article detail — `/articles/:slug`.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ArticleSsr {
    pub found: bool,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub content_html: String,
    pub cover_image_url: Option<String>,
    pub author: String,
}

/// Course detail — `/courses/:slug`.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CourseSsr {
    pub found: bool,
    pub title: String,
    pub slug: String,
    pub description: String,
}

/// Public profile — `/profile/:username`.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ProfileSsr {
    pub found: bool,
    pub username: String,
    pub role: String,
}

/// Documentation page — `/docs/:version/*slug`.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DocSsr {
    pub found: bool,
    pub is_page: bool,
    pub version: String,
    pub slug: String,
    pub title: String,
    pub content_html: String,
}

/// Marketplace asset detail — `/marketplace/asset/:slug`.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AssetSsr {
    pub found: bool,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub category: String,
    pub price_credits: i64,
    pub thumbnail_url: Option<String>,
    pub downloads: i64,
    pub rating_count: i32,
    pub seller: String,
}
