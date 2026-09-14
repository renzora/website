//! Cache-busted URLs for the static scripts the pages load.
//!
//! The stylesheet is inlined into every response (see `shell`), so it is always
//! exactly as new as the HTML around it. A `<script src>` is not: the browser
//! caches it, and nothing in the URL says which build it came from.
//!
//! That gap is not theoretical. The listing renderer lives in a script file and
//! the classes it names are generated into the inlined stylesheet, so a stale
//! copy of one against a fresh copy of the other means the markup asks for
//! classes the CSS does not define. The failure is not subtle: a width class
//! that goes missing leaves a `shrink-0` column sized by its contents, which
//! blows the layout apart rather than degrading.
//!
//! So the URL carries a fingerprint of the file. A new build is a new URL and
//! cannot be served from a cache keyed on the old one.

use std::sync::LazyLock;

/// Fingerprint a file's bytes, short enough to read in a URL.
///
/// FNV-1a rather than a cryptographic hash: this is a cache key, not a
/// signature, and nothing here defends against an attacker who can already
/// write to the assets directory.
fn fingerprint(path: &str) -> String {
    let Ok(bytes) = std::fs::read(path) else {
        // An unreadable asset is a deploy problem, not a reason to refuse to
        // render. Serving it unversioned is exactly the old behaviour.
        return String::new();
    };
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:x}")
}

fn versioned(path: &str) -> String {
    let fp = fingerprint(path);
    if fp.is_empty() {
        format!("/{path}")
    } else {
        format!("/{path}?v={fp}")
    }
}

/// The shared listing renderer, used by the asset page and the marketplace
/// overlay.
pub static ASSET_DETAIL_JS: LazyLock<String> =
    LazyLock::new(|| versioned("assets/js/asset-detail.js"));

/// The activity chart both of those mount.
pub static STATS_CHART_JS: LazyLock<String> =
    LazyLock::new(|| versioned("assets/js/stats-chart.js"));
