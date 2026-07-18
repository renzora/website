use leptos::prelude::*;
use leptos_meta::MetaTags;

use crate::app::App;
use crate::pages::embed::EmbedPreviewPage;

/// Site-wide structured data (JSON-LD). `{SITE}` is replaced with the runtime
/// site URL. Describes Renzora as a SoftwareApplication (a Bevy editor / game
/// engine), the WebSite (with marketplace search), and the Organization — this
/// helps search engines understand the entity and enables rich results.
const JSON_LD: &str = r#"{"@context":"https://schema.org","@graph":[{"@type":"SoftwareApplication","name":"Renzora","alternateName":"Renzora Engine","applicationCategory":"DeveloperApplication","applicationSubCategory":"Game Engine","operatingSystem":"Windows, macOS, Linux, Android, iOS, Web","description":"Renzora is a free, open-source Bevy editor and game engine with a full visual editor, Lua and Rhai scripting, a plugin system, physics and real-time rendering, built in Rust on Bevy.","url":"{SITE}","downloadUrl":"{SITE}/download","softwareVersion":"r1-alpha6","offers":{"@type":"Offer","price":"0","priceCurrency":"USD"},"isAccessibleForFree":true,"license":"https://opensource.org/licenses/MIT","sameAs":["https://github.com/renzora/engine","https://bevy.org/assets/"]},{"@type":"WebSite","name":"Renzora","url":"{SITE}","potentialAction":{"@type":"SearchAction","target":{"@type":"EntryPoint","urlTemplate":"{SITE}/marketplace?q={search_term_string}"},"query-input":"required name=search_term_string"}},{"@type":"Organization","name":"Renzora","url":"{SITE}","logo":"{SITE}/assets/previews/logo.png","sameAs":["https://github.com/renzora/engine"]}]}"#;

/// The HTML shell that wraps the entire application for SSR.
#[component]
pub fn Shell() -> impl IntoView {
    // Runtime site URL drives absolute URLs in OG/JSON-LD. Set SITE_URL to the
    // production origin (e.g. https://renzora.com) in the deployment env.
    let site = std::env::var("SITE_URL")
        .unwrap_or_else(|_| "https://renzora.com".into())
        .trim_end_matches('/')
        .to_string();
    let og_image = format!("{site}/assets/previews/interface.png");
    let json_ld = JSON_LD.replace("{SITE}", &site);

    view! {
        <!DOCTYPE html>
        <html lang="en" class="dark">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="icon" type="image/x-icon" href="/assets/favicon.ico" />

                // ── SEO: discoverability + social + structured data ──
                <meta name="keywords" content="Bevy editor, Bevy game engine, Bevy editor download, 2D and 3D Bevy editor, Rust game engine, open source game engine, Renzora, Renzora Engine, Bevy tools, game editor" />
                <meta name="robots" content="index, follow, max-image-preview:large, max-snippet:-1" />
                <meta name="author" content="Renzora" />
                <meta name="theme-color" content="#0b0617" />
                <meta name="twitter:card" content="summary_large_image" />
                <meta name="twitter:title" content="Renzora — Open Source Bevy Editor & Game Engine" />
                <meta name="twitter:description" content="A free, open-source Bevy editor and game engine, built in Rust on Bevy 0.19." />
                <meta name="twitter:image" content=og_image />
                <script type="application/ld+json" inner_html=json_ld></script>

                // Phosphor icons — self-hosted subset (only the icons the app uses)
                <link rel="stylesheet" href="/assets/style/phosphor.css" />
                <link rel="stylesheet" href="/assets/style/main.css" />

                // Lightweight animation shim — implements only the anime.js subset the
                // pages use (targets, opacity/transform props, stagger, timeline, and
                // object/counter tweens) on the Web Animations API. Applies final
                // states immediately so content is never left hidden. Replaces the
                // external anime.js CDN (removed for performance).
                <script>
                    r#"
                    (function(){
                      if (window.anime) return;
                      var EASE={linear:'linear',easeOutQuad:'cubic-bezier(0.5,1,0.89,1)',easeOutCubic:'cubic-bezier(0.33,1,0.68,1)',easeOutExpo:'cubic-bezier(0.16,1,0.3,1)',easeOutBack:'cubic-bezier(0.34,1.56,0.64,1)',easeInOutSine:'cubic-bezier(0.37,0,0.63,1)'};
                      function ease(e){ if(!e) return 'ease'; if(EASE[e]) return EASE[e]; if(e.indexOf('easeOutElastic')===0||e.indexOf('easeOutBack')===0) return EASE.easeOutBack; if(e.indexOf('easeInOut')===0) return EASE.easeInOutSine; if(e.indexOf('easeOut')===0) return EASE.easeOutCubic; return 'ease'; }
                      function toEls(t){ if(t==null) return []; if(typeof t==='string') return Array.prototype.slice.call(document.querySelectorAll(t)); if(t instanceof Element) return [t]; if(t&&t.nodeType) return [t]; if(t&&typeof t.length==='number') return Array.prototype.slice.call(t); return [t]; }
                      var TF=['translateX','translateY','translateZ','scale','scaleX','scaleY','rotate'];
                      function unit(k,v){ if(k==='rotate') return (typeof v==='number'? v+'deg': v); if(k.indexOf('translate')===0) return (typeof v==='number'? v+'px': v); return v; }
                      function ft(v){ return Array.isArray(v)? {from:v[0],to:v[1]} : {from:null,to:v}; }
                      var SKIP=['targets','duration','easing','delay','loop','direction','round','update','begin','complete','autoplay'];
                      function animateEl(el,opts,delay){
                        var dur=opts.duration||0, tf={}, css={}, hasTf=false, hasTfFrom=false;
                        for(var k in opts){ if(SKIP.indexOf(k)>=0) continue; var o=ft(opts[k]); if(TF.indexOf(k)>=0){ hasTf=true; tf[k]=o; if(o.from!=null) hasTfFrom=true; } else css[k]=o; }
                        function tfStr(w){ var s=''; for(var k in tf){ var val=tf[k][w]; if(val==null) val=tf[k].to; s+=k+'('+unit(k,val)+') '; } return s.trim(); }
                        var kf0={}, kf1={};
                        for(var c in css){ if(css[c].from!=null) kf0[c]=css[c].from; kf1[c]=css[c].to; }
                        if(hasTf){ if(hasTfFrom) kf0.transform=tfStr('from'); kf1.transform=tfStr('to'); }
                        for(var c2 in css){ try{ el.style[c2]=css[c2].to; }catch(e){} }
                        if(hasTf){ try{ el.style.transform=tfStr('to'); }catch(e){} }
                        if(el.animate){ try{ el.animate([kf0,kf1],{duration:dur,delay:delay||0,easing:ease(opts.easing),fill:'both'}); }catch(e){} }
                      }
                      function tweenObj(obj,opts,delay){
                        var dur=opts.duration||0, upd=opts.update, round=opts.round, tg={};
                        for(var k in opts){ if(SKIP.indexOf(k)>=0) continue; var o=ft(opts[k]); tg[k]={from:(o.from!=null?o.from:(obj[k]||0)),to:o.to}; }
                        var start=null;
                        function frame(ts){ if(start==null) start=ts; var p=dur>0?Math.min(1,(ts-start)/dur):1; for(var k in tg){ var v=tg[k].from+(tg[k].to-tg[k].from)*p; if(round) v=Math.round(v*round)/round; obj[k]=v; } if(upd) upd(obj); if(p<1) requestAnimationFrame(frame); }
                        setTimeout(function(){ requestAnimationFrame(frame); }, delay||0);
                      }
                      function run(opts){ var els=toEls(opts.targets); var isObj=els.length===1 && !(els[0] instanceof Element) && !(els[0]&&els[0].nodeType); if(isObj){ tweenObj(els[0],opts,(typeof opts.delay==='number'?opts.delay:0)); return; } els.forEach(function(el,i){ var d=(typeof opts.delay==='function')?opts.delay(el,i):(opts.delay||0); animateEl(el,opts,d); }); }
                      function anime(opts){ run(opts); return {}; }
                      anime.stagger=function(v,o){ o=o||{}; return function(el,i){ return (o.start||0)+i*v; }; };
                      anime.timeline=function(t){ t=t||{}; var cursor=0; var tl={ add:function(params,offset){ var p={}; for(var a in t) p[a]=t[a]; for(var b in params) p[b]=params[b]; var start=cursor; if(typeof offset==='string'){ var n=parseFloat(offset.slice(2))||0; if(offset[0]==='-') start=cursor-n; else if(offset[0]==='+') start=cursor+n; } else if(typeof offset==='number'){ start=offset; } var els=toEls(p.targets); var last=0; els.forEach(function(el,i){ var dd=(typeof p.delay==='function')?p.delay(el,i):(p.delay||0); last=Math.max(last,dd); animateEl(el,p,start+dd); }); cursor=start+last+(p.duration||0); return tl; } }; return tl; };
                      window.anime=anime;
                    })();
                    "#
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

                    /* ── App shell layout: fixed sidebar + top header ── */
                    :root{--sidebar-w:248px;--header-h:60px}
                    .app-main{margin-left:var(--sidebar-w);padding-top:var(--header-h);min-height:100vh}
                    @media (max-width:1023px){.app-main{margin-left:0}}
                    #app-sidebar{position:fixed;top:0;left:0;bottom:0;width:var(--sidebar-w);z-index:60;display:flex;flex-direction:column;background:#0b0617;border-right:1px solid rgba(255,255,255,0.06)}
                    #app-header{position:fixed;top:0;left:var(--sidebar-w);right:0;height:var(--header-h);z-index:55;display:flex;align-items:center;gap:1rem;padding:0 1.25rem;background:rgba(11,6,23,0.78);backdrop-filter:blur(18px);-webkit-backdrop-filter:blur(18px);border-bottom:1px solid rgba(255,255,255,0.06)}
                    @media (max-width:1023px){#app-sidebar{transform:translateX(-100%);transition:transform .25s ease}#app-sidebar.open{transform:translateX(0)}#app-header{left:0}}
                    #sidebar-scrim{position:fixed;inset:0;z-index:59;background:rgba(0,0,0,0.5);backdrop-filter:blur(2px);display:none}
                    #sidebar-scrim.open{display:block}
                    @media (min-width:1024px){#sidebar-scrim{display:none!important}#sidebar-burger{display:none}}
                    /* Sidebar nav links */
                    .side-link{position:relative;display:flex;align-items:center;gap:.7rem;padding:.55rem .7rem;border-radius:.6rem;font-size:.9rem;color:#a1a1aa;transition:all .15s}
                    .side-link:hover{color:#f4f4f5;background:rgba(255,255,255,0.05)}
                    .side-link.active{color:#fff;background:linear-gradient(90deg,rgba(168,85,247,0.22),rgba(168,85,247,0.06))}
                    .side-link.active::before{content:'';position:absolute;left:-.7rem;top:50%;transform:translateY(-50%);width:3px;height:60%;border-radius:0 3px 3px 0;background:linear-gradient(180deg,#a855f7,#22d3ee)}"
                </style>
                <MetaTags />
            </head>
            <body class="text-zinc-50 antialiased">
                <App />
            </body>
        </html>
    }
}

/// Minimal shell for embed pages — no nav, no footer, no app wrapper.
#[component]
pub fn EmbedShell() -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="dark">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="stylesheet" href="/assets/style/phosphor.css" />
                <link rel="stylesheet" href="/assets/style/main.css" />
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
