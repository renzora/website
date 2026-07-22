use leptos::prelude::*;
use leptos_meta::MetaTags;

use crate::app::App;
use crate::pages::embed::EmbedPreviewPage;

/// Site-wide structured data (JSON-LD). `{SITE}` is replaced with the runtime
/// site URL. Describes Renzora as a SoftwareApplication (a Bevy editor / game
/// engine), the WebSite (with marketplace search), and the Organization, this
/// helps search engines understand the entity and enables rich results.
const JSON_LD: &str = r#"{"@context":"https://schema.org","@graph":[{"@type":"SoftwareApplication","name":"Renzora","alternateName":"Renzora Engine","applicationCategory":"DeveloperApplication","applicationSubCategory":"Game Engine","operatingSystem":"Windows, macOS, Linux, Android, iOS, Web","description":"Renzora is a free, open-source Bevy editor and game engine with a full visual editor, Lua and Rhai scripting, a plugin system, physics and real-time rendering, built in Rust on Bevy.","url":"{SITE}","downloadUrl":"{SITE}/download","softwareVersion":"r1-alpha6","offers":{"@type":"Offer","price":"0","priceCurrency":"USD"},"isAccessibleForFree":true,"license":"https://opensource.org/licenses/MIT","sameAs":["https://github.com/renzora/engine","https://bevy.org/assets/"]},{"@type":"WebSite","name":"Renzora","url":"{SITE}","potentialAction":{"@type":"SearchAction","target":{"@type":"EntryPoint","urlTemplate":"{SITE}/marketplace?q={search_term_string}"},"query-input":"required name=search_term_string"}},{"@type":"Organization","name":"Renzora","url":"{SITE}","logo":"{SITE}/assets/previews/logo.png","sameAs":["https://github.com/renzora/engine"]}]}"#;

/// Speculation Rules (Chromium): make internal navigation feel instant by
/// resolving the destination *before* the click.
///   • `prerender` (moderate = hover/pointerdown intent) fully renders the next
///     page in a hidden context; the click becomes a compositor swap, ~0ms.
///     Scoped to read-only routes only.
///   • `prefetch` is the universal fallback; where both rules match a URL the
///     browser upgrades to prerender. Detail routes that count a view via their
///     client JS (`/articles/:slug`, `/marketplace/asset/:slug`) are excluded
///     from prerender and only prefetched, prefetch fetches the HTML but does
///     NOT execute page JS, so a hover can't inflate view counts, while a real
///     click still loads from cache (zero network) and counts once.
/// Links can opt out with `data-no-prefetch`; `target="_blank"` and cross-origin
/// links are never speculated. Non-Chromium browsers ignore this block and use
/// the hover-prefetch fallback script instead.
const SPEC_RULES: &str = r#"{"prerender":[{"where":{"and":[{"href_matches":"/*"},{"not":{"href_matches":"/articles/*"}},{"not":{"href_matches":"/marketplace/asset/*"}},{"not":{"href_matches":"/login"}},{"not":{"href_matches":"/register"}},{"not":{"selector_matches":"[data-no-prefetch]"}},{"not":{"selector_matches":"[target=\"_blank\"]"}}]},"eagerness":"moderate"}],"prefetch":[{"where":{"and":[{"href_matches":"/*"},{"not":{"selector_matches":"[data-no-prefetch]"}},{"not":{"selector_matches":"[target=\"_blank\"]"}}]},"eagerness":"moderate"}]}"#;

/// Extract the `scheme://host[:port]` origin from a URL, or `None` if it has no
/// scheme (used to turn `S3_PUBLIC_URL` into a `preconnect` target).
fn origin_of(url: &str) -> Option<String> {
    let u = url.trim();
    let after = u.find("://")? + 3;
    let end = u[after..].find('/').map(|i| after + i).unwrap_or(u.len());
    let origin = &u[..end];
    (end > after).then(|| origin.to_string())
}

/// The compiled CSS bundles, read once per process and inlined into every SSR
/// `<head>` so the first paint isn't render-blocked on two extra round trips
/// (PSI measured ~420ms on mobile for the external sheets). The CSS is baked
/// into the image at build time and never changes while the server runs; the
/// inlined bytes ride along in the (edge-cached) HTML and are always in sync.
fn read_css(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}
static MAIN_CSS: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| read_css("assets/style/main.css"));
static PHOSPHOR_CSS: std::sync::LazyLock<String> =
    std::sync::LazyLock::new(|| read_css("assets/style/phosphor.css"));

/// The HTML shell that wraps the entire application for SSR.
#[component]
pub fn Shell() -> impl IntoView {
    // Runtime site URL drives absolute URLs in OG/JSON-LD. Set SITE_URL to the
    // production origin (e.g. https://renzora.com) in the deployment env.
    let site = std::env::var("SITE_URL")
        .unwrap_or_else(|_| "https://renzora.com".into())
        .trim_end_matches('/')
        .to_string();
    let og_image = format!("{site}/assets/previews/og.jpg");
    let json_ld = JSON_LD.replace("{SITE}", &site);

    // Marketplace/asset thumbnails come from S3_PUBLIC_URL. When that's a distinct
    // origin, resolve its DNS early. Only a dns-prefetch (not a preconnect): pages
    // without those images — e.g. the home page — otherwise open an unused TLS
    // connection, which PSI flags. Emitted only when cross-origin.
    let asset_hint = origin_of(&std::env::var("S3_PUBLIC_URL").unwrap_or_default())
        .filter(|o| *o != site)
        .map(|o| view! { <link rel="dns-prefetch" href=o /> });

    view! {
        <!DOCTYPE html>
        <html lang="en" class="dark">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="icon" type="image/x-icon" href="/assets/favicon.ico" />

                // Warm the cross-origin asset/CDN connection as early as possible.
                {asset_hint}

                // ── SEO: discoverability + social + structured data ──
                <meta name="keywords" content="Bevy editor, Bevy game engine, Bevy editor download, 2D and 3D Bevy editor, Rust game engine, open source game engine, Renzora, Renzora Engine, Bevy tools, game editor" />
                <meta name="robots" content="index, follow, max-image-preview:large, max-snippet:-1" />
                <meta name="author" content="Renzora" />
                <meta name="theme-color" content="#0b0617" />
                <meta name="twitter:card" content="summary_large_image" />
                <meta name="twitter:title" content="Renzora, Open Source Bevy Editor & Game Engine" />
                <meta name="twitter:description" content="A free, open-source Bevy editor and game engine, built in Rust on Bevy 0.19." />
                <meta name="twitter:image" content=og_image />
                <script type="application/ld+json" inner_html=json_ld></script>

                // CSS inlined into <head> so the first paint isn't render-blocked
                // on external sheets (self-hosted Phosphor icon subset + Tailwind).
                <style inner_html=PHOSPHOR_CSS.as_str()></style>
                <style inner_html=MAIN_CSS.as_str()></style>

                // Instant navigation: prerender/prefetch internal links on hover
                // intent (Chromium via Speculation Rules; others via the fallback
                // below). A click then resolves from local memory, no network.
                <script type="speculationrules" inner_html=SPEC_RULES></script>
                <script>
                    r#"
                    (function(){
                      // Chromium handles hover prefetch/prerender via the
                      // speculationrules block above, don't double up there.
                      if (window.HTMLScriptElement && HTMLScriptElement.supports && HTMLScriptElement.supports('speculationrules')) return;
                      var seen = new Set();
                      function prefetch(href){
                        if (seen.has(href)) return; seen.add(href);
                        var l = document.createElement('link');
                        l.rel = 'prefetch'; l.href = href; document.head.appendChild(l);
                      }
                      function onIntent(e){
                        var a = e.target && e.target.closest && e.target.closest('a[href]');
                        if (!a || a.origin !== location.origin) return;
                        if (a.target === '_blank' || a.hasAttribute('data-no-prefetch')) return;
                        if (a.hash && a.pathname === location.pathname) return; // same-page anchor
                        prefetch(a.href);
                      }
                      ['pointerover','focusin','touchstart'].forEach(function(ev){
                        document.addEventListener(ev, onIntent, { passive: true });
                      });
                    })();
                    "#
                </script>

                // Register the service worker (offline fallback; network-first so
                // it never serves a stale page while online).
                <script>
                    r#"if ('serviceWorker' in navigator) { addEventListener('load', function(){ navigator.serviceWorker.register('/sw.js').catch(function(){}); }); }"#
                </script>

                <style>
                    "body{background:radial-gradient(1200px 620px at 15% -8%,rgba(168,85,247,0.12),transparent 60%),radial-gradient(1000px 520px at 100% 0%,rgba(34,211,238,0.07),transparent 55%),#09040f;background-attachment:fixed;min-height:100vh}
                    html,body{scrollbar-width:thin;scrollbar-color:#241633 #09040f}
                    *{scrollbar-width:thin;scrollbar-color:#241633 #09040f}
                    ::-webkit-scrollbar{width:8px!important;height:8px!important}
                    ::-webkit-scrollbar-track{background:#09040f!important}
                    ::-webkit-scrollbar-thumb{background:#241633!important;border-radius:4px!important}
                    ::-webkit-scrollbar-thumb:hover{background:#33234a!important}
                    ::-webkit-scrollbar-corner{background:#09040f!important}
                    select,select option{background-color:#160d26!important;color:#fafafa!important}
                    select option:checked{background-color:#241633!important}
                    select{-webkit-appearance:none;-moz-appearance:none;appearance:none;background-image:url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%2371717a' stroke-width='2'%3E%3Cpath d='M6 9l6 6 6-6'/%3E%3C/svg%3E\");background-repeat:no-repeat;background-position:right 8px center;padding-right:28px}

                    /* Site-wide base font size (bumped from the 16px browser default;
                       rem-based Tailwind sizes scale with it). */
                    html{font-size:17px}
                    /* ── App shell layout: fixed sidebar + top header ── */
                    :root{--sidebar-w:248px;--header-h:60px}
                    .app-main{margin-right:var(--sidebar-w);padding-top:var(--header-h);min-height:100vh}
                    @media (max-width:1023px){.app-main{margin-right:0}}
                    #app-sidebar{position:fixed;top:var(--header-h);right:0;bottom:0;width:var(--sidebar-w);z-index:60;display:flex;flex-direction:column;background:#0b0617;border-left:1px solid rgba(255,255,255,0.06)}
                    #app-header{position:fixed;top:0;left:0;right:0;height:var(--header-h);z-index:62;display:flex;align-items:center;gap:1rem;padding:0 1.25rem;background:rgba(11,6,23,0.78);backdrop-filter:blur(18px);-webkit-backdrop-filter:blur(18px);border-bottom:1px solid rgba(255,255,255,0.06)}
                    @media (max-width:1023px){#app-sidebar{transform:translateX(100%);transition:transform .25s ease}#app-sidebar.open{transform:translateX(0)}#app-header{right:0}}
                    #sidebar-scrim{position:fixed;inset:0;z-index:59;background:rgba(0,0,0,0.5);backdrop-filter:blur(2px);display:none}
                    #sidebar-scrim.open{display:block}
                    @media (min-width:1024px){#sidebar-scrim{display:none!important}#sidebar-burger{display:none}}
                    /* Sidebar nav links */
                    .side-link{position:relative;display:flex;align-items:center;gap:.7rem;padding:.55rem .7rem;border-radius:.6rem;font-size:.9rem;color:#a1a1aa;transition:all .15s}
                    .side-link:hover{color:#f4f4f5;background:rgba(255,255,255,0.05)}
                    .side-link.active{color:#fff;background:linear-gradient(90deg,rgba(168,85,247,0.22),rgba(168,85,247,0.06))}
                    .side-link.active::before{content:'';position:absolute;left:-.7rem;top:50%;transform:translateY(-50%);width:3px;height:60%;border-radius:0 3px 3px 0;background:linear-gradient(180deg,#a855f7,#22d3ee)}
                    .side-link.donate{color:#f87171}
                    .side-link.donate:hover{color:#fca5a5;background:rgba(248,113,113,0.08)}
                    .side-link.donate.active{color:#fff;background:linear-gradient(90deg,rgba(248,113,113,0.22),rgba(248,113,113,0.06))}
                    .side-link.donate.active::before{background:linear-gradient(180deg,#f87171,#fb7185)}
                    .top-link{position:relative;display:inline-flex;align-items:center;gap:.4rem;padding:.4rem .7rem;border-radius:.5rem;font-size:.85rem;font-weight:500;color:#a1a1aa;transition:all .15s;white-space:nowrap}
                    .top-link:hover{color:#f4f4f5;background:rgba(255,255,255,0.05)}
                    .top-link.active{color:#fff;background:linear-gradient(180deg,rgba(168,85,247,0.22),rgba(168,85,247,0.06))}
                    .top-link.donate{color:#f87171}
                    .top-link.donate:hover{color:#fca5a5;background:rgba(248,113,113,0.08)}
                    .top-link.donate.active{color:#fff;background:linear-gradient(180deg,rgba(248,113,113,0.22),rgba(248,113,113,0.06))}
                    .menu-item{display:flex;align-items:center;gap:.625rem;width:100%;padding:.375rem .5rem;border-radius:.5rem;font-size:.875rem;line-height:1.25rem;color:#d4d4d8;text-align:left;transition:background-color .15s,color .15s}
                    .menu-item:hover{background:rgba(255,255,255,0.04)}
                    .menu-item.active{background:rgba(168,85,247,0.15);color:#a855f7}
                    .menu-item i{color:#71717a;transition:color .15s}
                    .menu-item.active i{color:#a855f7}
                    .menu-label{margin-bottom:.5rem;padding:0 .5rem;font-size:.6875rem;font-weight:600;text-transform:uppercase;letter-spacing:.08em;color:#71717a}"
                </style>
                <MetaTags />
            </head>
            <body class="text-zinc-50 antialiased">
                <App />
            </body>
        </html>
    }
}

/// Minimal shell for embed pages, no nav, no footer, no app wrapper.
#[component]
pub fn EmbedShell() -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="dark">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <style inner_html=PHOSPHOR_CSS.as_str()></style>
                <style inner_html=MAIN_CSS.as_str()></style>
                <style>
                    "body { margin: 0; padding: 0; background: #060608; overflow: hidden; }
                    * { box-sizing: border-box; }
                    .spinner { width: 24px; height: 24px; border: 2px solid #27272a; border-top-color: #6366f1; border-radius: 50%; animation: spin .6s linear infinite; }
                    @keyframes spin { to { transform: rotate(360deg); } }"
                </style>
            </head>
            <body class="text-zinc-50">
                <EmbedPreviewPage />
            </body>
        </html>
    }
}
