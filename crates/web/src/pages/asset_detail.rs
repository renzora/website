use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};
use renzora_common::ssr::AssetSsr;

/// JSON-escape a string for safe embedding in a JSON-LD literal.
fn json_escape(s: &str) -> String {
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

#[component]
pub fn AssetDetailPage() -> impl IntoView {
    // SEO: on a full page load the server provides the asset via context, so we
    // render crawlable content + per-page meta into the HTML. The inline JS
    // below still builds the full interactive UI on load (replacing #asset-detail).
    let ssr = use_context::<AssetSsr>().filter(|a| a.found);
    let has_ssr = ssr.is_some();
    let head = ssr.clone().map(|a| {
        let title = format!("{}, Renzora Marketplace", a.name);
        let full = a.description.chars().count();
        let d: String = a.description.chars().take(155).collect();
        let desc = if full > 155 { format!("{d}…") } else { d };
        let canonical = format!("https://renzora.com/marketplace/asset/{}", a.slug);
        let price = if a.price_credits == 0 { "0".to_string() } else { a.price_credits.to_string() };
        let img = a.thumbnail_url.clone().unwrap_or_default();
        let ld = format!(
            "{{\"@context\":\"https://schema.org\",\"@type\":\"Product\",\"name\":{},\"description\":{},\"category\":{},\"image\":{},\"brand\":{{\"@type\":\"Brand\",\"name\":\"Renzora\"}},\"offers\":{{\"@type\":\"Offer\",\"price\":\"{}\",\"priceCurrency\":\"USD\",\"availability\":\"https://schema.org/InStock\",\"url\":{}}}}}",
            json_escape(&a.name), json_escape(&a.description), json_escape(&a.category), json_escape(&img), price, json_escape(&canonical)
        );
        view! {
            <Title text=title />
            <Meta name="description" content=desc />
            <Link rel="canonical" href=canonical />
            <script type="application/ld+json" inner_html=ld></script>
        }
    });
    let content = ssr.map(|a| {
        let price_label = if a.price_credits == 0 { "Free".to_string() } else { format!("{} credits", a.price_credits) };
        view! {
            <div class="max-w-[1440px] mx-auto pt-2">
                <nav class="text-xs text-zinc-500 mb-3">
                    <a href="/marketplace" class="hover:text-accent">"Marketplace"</a>" / "{a.category.clone()}
                </nav>
                <h1 class="text-2xl font-bold text-white">{a.name.clone()}</h1>
                <p class="text-zinc-400 mt-3 leading-relaxed">{a.description.clone()}</p>
                {a.thumbnail_url.clone().map(|t| view! {
                    <img src=t alt=a.name.clone() class="mt-4 rounded-xl border border-white/10 max-w-full" loading="lazy" />
                })}
                <p class="mt-4 text-sm text-zinc-300">
                    {price_label}" · by "{a.seller.clone()}" · "{a.downloads}" downloads"
                </p>
            </div>
        }
    });
    view! {
        {head}
        <section class="py-8 px-6 relative min-h-screen">
            // Background layer, inside the section so it's part of the document flow
            <div class="fixed inset-0 pointer-events-none overflow-hidden" style="z-index:0" id="asset-bg-layer">
                <canvas id="asset-canvas" class="absolute inset-0 w-full h-full" style="z-index:1"></canvas>
                <div class="absolute top-[10%] left-[15%] w-96 h-96 bg-accent/20 rounded-full blur-[80px] animate-pulse" style="z-index:0"></div>
                <div class="absolute bottom-[20%] right-[10%] w-80 h-80 bg-purple-600/15 rounded-full blur-[60px]" style="z-index:0;animation:pulse 4s ease-in-out infinite 1s"></div>
                <div id="asset-thumb-bg" class="absolute inset-0" style="z-index:0"></div>
                <div class="absolute inset-0 bg-[#060608]/15" style="z-index:2"></div>
            </div>
            <div class="max-w-[1440px] mx-auto relative" style="z-index:10" id="asset-detail">
                {content}
                {(!has_ssr).then(|| view! {
                    <div class="text-center py-20">
                        <div class="inline-block animate-spin w-6 h-6 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                    </div>
                })}
            </div>
        </section>
        // Loaded ahead of the page script, which mounts the chart as soon as the
        // asset detail has rendered.
        <script src=crate::assets::STATS_CHART_JS.as_str()></script>
        // The listing renderer itself, shared with the marketplace overlay.
        // Fingerprinted: the stylesheet it depends on is inlined in this
        // response, so a cached copy of this file could otherwise be a build
        // behind the classes it names.
        <script src=crate::assets::ASSET_DETAIL_JS.as_str()></script>
        <script>
            r##"
            // The renderer lives in /assets/js/asset-detail.js, shared with the
            // marketplace grid's quick-look overlay. It used to be inline here and
            // read the slug off the URL itself, which is what made a second surface
            // impossible: there was no way to ask for a listing other than the one
            // the address bar named.
            (function () {
                const slug = window.location.pathname.split('/').pop();
                const root = document.getElementById('asset-detail');
                if (root) window.renderAssetDetail(slug, root);
            })();
            "##
        </script>

        <style>
            r#"
            @keyframes confettiFall {
                0% { transform: translateY(0) translateX(0) rotate(0deg); opacity: 1; }
                100% { transform: translateY(100vh) translateX(var(--drift, 0px)) rotate(720deg); opacity: 0; }
            }
            .gallery-thumb { transition: border-color 0.2s, transform 0.2s; }
            .gallery-thumb:hover { transform: scale(1.05); }
            "#
        </style>

        // Particle canvas script
        <script>
            r#"
            (function() {
                const canvas = document.getElementById('asset-canvas');
                if (!canvas) return;
                const ctx = canvas.getContext('2d');
                let w, h, particles = [], mouse = { x: -1000, y: -1000 };

                function resize() {
                    w = canvas.width = window.innerWidth;
                    h = canvas.height = window.innerHeight;
                }
                resize();
                window.addEventListener('resize', resize);

                document.addEventListener('mousemove', e => { mouse.x = e.clientX; mouse.y = e.clientY; });
                document.addEventListener('mouseleave', () => { mouse.x = -1000; mouse.y = -1000; });

                const count = Math.min(100, Math.floor(w * h / 12000));
                for (let i = 0; i < count; i++) {
                    particles.push({
                        x: Math.random() * w,
                        y: Math.random() * h,
                        vx: (Math.random() - 0.5) * 0.3,
                        vy: (Math.random() - 0.5) * 0.3,
                        r: Math.random() * 1.8 + 0.5,
                    });
                }

                function draw() {
                    ctx.clearRect(0, 0, w, h);
                    for (let i = 0; i < particles.length; i++) {
                        const p = particles[i];
                        p.x += p.vx; p.y += p.vy;
                        if (p.x < 0) p.x = w; if (p.x > w) p.x = 0;
                        if (p.y < 0) p.y = h; if (p.y > h) p.y = 0;

                        const dx = p.x - mouse.x, dy = p.y - mouse.y;
                        const dist = Math.sqrt(dx * dx + dy * dy);
                        if (dist < 140) {
                            const force = (140 - dist) / 140 * 0.8;
                            p.x += dx / dist * force;
                            p.y += dy / dist * force;
                        }

                        ctx.beginPath();
                        ctx.arc(p.x, p.y, p.r, 0, Math.PI * 2);
                        ctx.fillStyle = 'rgba(99, 102, 241, 0.5)';
                        ctx.fill();

                        for (let j = i + 1; j < particles.length; j++) {
                            const p2 = particles[j];
                            const ddx = p.x - p2.x, ddy = p.y - p2.y;
                            const d = ddx * ddx + ddy * ddy;
                            if (d < 22000) {
                                ctx.beginPath();
                                ctx.moveTo(p.x, p.y);
                                ctx.lineTo(p2.x, p2.y);
                                ctx.strokeStyle = `rgba(99, 102, 241, ${(1 - d / 22000) * 0.18})`;
                                ctx.lineWidth = 0.6;
                                ctx.stroke();
                            }
                        }
                    }
                    requestAnimationFrame(draw);
                }
                draw();

                // Cleanup on navigation
                window.addEventListener('beforeunload', () => {
                    const bg = document.getElementById('asset-bg-layer');
                    if (bg) bg.remove();
                });
            })();
            "#
        </script>
    }
}
