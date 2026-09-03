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
