use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        // Hero section with animated particle canvas
        <section class="relative min-h-[80vh] flex items-start justify-center overflow-hidden -mt-14 pt-36">
            // Animated background
            <canvas id="hero-canvas" class="absolute inset-0 w-full h-full"></canvas>

            // Glow orbs
            <div class="absolute top-1/4 left-1/4 w-96 h-96 bg-accent/20 rounded-full blur-[128px] animate-pulse pointer-events-none"></div>
            <div class="absolute bottom-1/4 right-1/4 w-80 h-80 bg-purple-600/15 rounded-full blur-[100px] pointer-events-none" style="animation: pulse 4s ease-in-out infinite 1s"></div>

            <div class="relative z-10 text-center px-6 max-w-3xl mx-auto">
                // Badge
                <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-accent/10 border border-accent/20 text-accent text-xs font-medium mb-3 backdrop-blur-sm">
                    <span class="w-1.5 h-1.5 rounded-full bg-accent animate-pulse"></span>
                    "r1-alpha4 — Early Access"
                </div>

                <h1 class="text-6xl md:text-7xl lg:text-8xl font-extrabold tracking-tight leading-[1.05]">
                    <span class="hero-title">"Renzora Engine"</span>
                </h1>
                <p class="mt-4 text-sm text-zinc-500 uppercase tracking-widest font-medium">"Powered by Rust & Bevy 0.18"</p>
                <p class="mt-4 text-lg md:text-xl text-zinc-400 leading-relaxed max-w-xl mx-auto">
                    "An open-source game engine with a visual editor, "
                    "scripting, and a community marketplace."
                </p>

                <div class="mt-10 flex gap-3 justify-center flex-wrap">
                    <a href="/download" class="group relative inline-flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_30px_rgba(99,102,241,0.3)] hover:scale-[1.02]">
                        <i class="ph ph-download-simple text-lg"></i>"Download Engine"
                        <span class="absolute inset-0 rounded-xl bg-white/10 opacity-0 group-hover:opacity-100 transition-opacity"></span>
                    </a>
                    <a href="/docs" class="inline-flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-semibold bg-white/5 text-zinc-50 border border-zinc-700/50 hover:border-zinc-500 hover:bg-white/10 transition-all backdrop-blur-sm">
                        <i class="ph ph-book-open text-lg"></i>"Documentation"
                    </a>
                    <a href="https://github.com/renzora/engine" target="_blank" rel="noopener noreferrer" class="inline-flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-semibold bg-white/5 text-zinc-50 border border-zinc-700/50 hover:border-zinc-500 hover:bg-white/10 transition-all backdrop-blur-sm">
                        <i class="ph ph-github-logo text-lg"></i>"Source"
                    </a>
                </div>
            </div>

            // Scroll indicator
            <div class="absolute bottom-8 left-1/2 -translate-x-1/2 z-10">
                <div class="w-5 h-8 rounded-full border-2 border-zinc-600 flex justify-center pt-1.5">
                    <div class="w-1 h-2 rounded-full bg-zinc-500 scroll-dot"></div>
                </div>
            </div>
        </section>

        // Editor screenshot with parallax reveal
        <section class="relative -mt-20 pb-20 w-full overflow-hidden">
            <div class="max-w-[1100px] mx-auto px-6">
                <div class="relative rounded-xl overflow-hidden border border-zinc-800/50 shadow-2xl shadow-black/50 editor-reveal">
                    <div class="absolute inset-0 bg-gradient-to-t from-surface-panel via-transparent to-transparent z-10 pointer-events-none"></div>
                    <img src="/assets/images/interface.png" alt="Renzora Engine editor" class="w-full h-auto block" loading="lazy" />
                </div>
            </div>
        </section>

        // Features grid with stagger animation
        <section class="pb-24">
            <div class="max-w-[1200px] mx-auto px-6">
                <div class="text-center mb-14">
                    <h2 class="text-3xl md:text-4xl font-bold">"Everything you need"</h2>
                    <p class="text-zinc-500 mt-3 text-base">"A complete toolkit for building games of any scale."</p>
                </div>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 feature-grid">
                    <FeatureCard icon="ph-cube" title="Visual Editor" description="Scene hierarchy, inspector, material graphs, terrain tools, and a fully dockable panel system." color="indigo" />
                    <FeatureCard icon="ph-tree-structure" title="Visual Scripting" description="Blueprint-style node graphs alongside Lua and Rhai scripting for rapid gameplay iteration." color="violet" />
                    <FeatureCard icon="ph-devices" title="Cross-Platform" description="Export to Windows, macOS, Linux, Android, iOS, tvOS, and Web from a single project." color="cyan" />
                    <FeatureCard icon="ph-wifi-high" title="Multiplayer" description="Dedicated server networking with state replication and client-side prediction." color="emerald" />
                    <FeatureCard icon="ph-storefront" title="Marketplace" description="Browse and sell plugins, asset packs, themes, and scripts. Earn real money." color="amber" />
                    <FeatureCard icon="ph-mountains" title="Terrain" description="GPU-powered sculpting and painting with real-time LOD and physics integration." color="rose" />
                    <FeatureCard icon="ph-drop" title="Materials" description="Node-based material editor with PBR workflows and custom WGSL shader support." color="sky" />
                    <FeatureCard icon="ph-code" title="Open Source" description="Built on Rust and Bevy. Inspect every line, extend anything, contribute freely." color="orange" />
                </div>
            </div>
        </section>

        // Stats bar
        <section class="pb-24">
            <div class="max-w-[1200px] mx-auto px-6">
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                    <div class="text-center p-6 rounded-xl bg-white/[0.02] border border-zinc-800/50">
                        <div class="text-3xl font-bold text-accent counter" data-target="7">"0"</div>
                        <div class="text-xs text-zinc-500 mt-1 uppercase tracking-wider">"Export Platforms"</div>
                    </div>
                    <div class="text-center p-6 rounded-xl bg-white/[0.02] border border-zinc-800/50">
                        <div class="text-3xl font-bold text-accent counter" data-target="100">"0"</div>
                        <div class="text-xs text-zinc-500 mt-1 uppercase tracking-wider">"Open Source"</div>
                    </div>
                    <div class="text-center p-6 rounded-xl bg-white/[0.02] border border-zinc-800/50">
                        <div class="text-3xl font-bold text-accent counter" data-target="3">"0"</div>
                        <div class="text-xs text-zinc-500 mt-1 uppercase tracking-wider">"Script Languages"</div>
                    </div>
                    <div class="text-center p-6 rounded-xl bg-white/[0.02] border border-zinc-800/50">
                        <div class="text-3xl font-bold text-accent counter" data-target="60">"0"</div>
                        <div class="text-xs text-zinc-500 mt-1 uppercase tracking-wider">"FPS Target"</div>
                    </div>
                </div>
            </div>
        </section>

        // Explore section
        <section class="pb-24">
            <div class="max-w-[1200px] mx-auto px-6">
                <h2 class="text-lg font-semibold mb-5">"Explore"</h2>
                <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
                    <ExploreCard icon="ph-storefront" name="Marketplace" desc="Plugins, assets, and themes" href="/marketplace" />
                    <ExploreCard icon="ph-book-open" name="Documentation" desc="Guides and API reference" href="/docs" />
                    <ExploreCard icon="ph-download-simple" name="Download" desc="Get the engine" href="/download" />
                </div>
            </div>
        </section>

        // CTA section
        <section class="pb-24">
            <div class="max-w-[1200px] mx-auto px-6">
                <div class="relative overflow-hidden text-center p-16 rounded-2xl border border-zinc-800/50">
                    <div class="absolute inset-0 bg-gradient-to-br from-accent/5 via-transparent to-purple-600/5"></div>
                    <div class="absolute top-0 left-1/2 -translate-x-1/2 w-64 h-px bg-gradient-to-r from-transparent via-accent/50 to-transparent"></div>
                    <div class="relative z-10">
                        <h2 class="text-3xl md:text-4xl font-bold">"Ready to build?"</h2>
                        <p class="text-zinc-400 mt-3 mb-8 text-base max-w-md mx-auto">"Download the engine and create your first project in minutes."</p>
                        <div class="flex gap-3 justify-center flex-wrap">
                            <a href="/download" class="group relative inline-flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_30px_rgba(99,102,241,0.3)]">
                                <i class="ph ph-download-simple text-lg"></i>"Download Engine"
                            </a>
                            <a href="/docs/getting-started/installation" class="inline-flex items-center gap-2 px-6 py-3 rounded-xl text-sm font-semibold bg-white/5 text-zinc-50 border border-zinc-700/50 hover:border-zinc-500 transition-all">
                                <i class="ph ph-rocket-launch text-lg"></i>"Getting Started"
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <script>
            r#"
            // ── Particle canvas ──
            (function() {
                const canvas = document.getElementById('hero-canvas');
                if (!canvas) return;
                const ctx = canvas.getContext('2d');
                let w, h, particles = [], mouse = { x: -1000, y: -1000 };

                function resize() {
                    w = canvas.width = canvas.offsetWidth;
                    h = canvas.height = canvas.offsetHeight;
                }
                resize();
                window.addEventListener('resize', resize);

                canvas.addEventListener('mousemove', e => {
                    const rect = canvas.getBoundingClientRect();
                    mouse.x = e.clientX - rect.left;
                    mouse.y = e.clientY - rect.top;
                });
                canvas.addEventListener('mouseleave', () => { mouse.x = -1000; mouse.y = -1000; });

                const count = Math.min(80, Math.floor(w * h / 15000));
                for (let i = 0; i < count; i++) {
                    particles.push({
                        x: Math.random() * w,
                        y: Math.random() * h,
                        vx: (Math.random() - 0.5) * 0.3,
                        vy: (Math.random() - 0.5) * 0.3,
                        r: Math.random() * 1.5 + 0.5,
                    });
                }

                function draw() {
                    ctx.clearRect(0, 0, w, h);

                    for (let i = 0; i < particles.length; i++) {
                        const p = particles[i];
                        p.x += p.vx;
                        p.y += p.vy;
                        if (p.x < 0) p.x = w;
                        if (p.x > w) p.x = 0;
                        if (p.y < 0) p.y = h;
                        if (p.y > h) p.y = 0;

                        // Mouse repulsion
                        const dx = p.x - mouse.x;
                        const dy = p.y - mouse.y;
                        const dist = Math.sqrt(dx * dx + dy * dy);
                        if (dist < 120) {
                            const force = (120 - dist) / 120 * 0.8;
                            p.x += dx / dist * force;
                            p.y += dy / dist * force;
                        }

                        ctx.beginPath();
                        ctx.arc(p.x, p.y, p.r, 0, Math.PI * 2);
                        ctx.fillStyle = 'rgba(99, 102, 241, 0.4)';
                        ctx.fill();

                        // Draw connections
                        for (let j = i + 1; j < particles.length; j++) {
                            const p2 = particles[j];
                            const ddx = p.x - p2.x;
                            const ddy = p.y - p2.y;
                            const d = ddx * ddx + ddy * ddy;
                            if (d < 18000) {
                                ctx.beginPath();
                                ctx.moveTo(p.x, p.y);
                                ctx.lineTo(p2.x, p2.y);
                                const alpha = (1 - d / 18000) * 0.15;
                                ctx.strokeStyle = `rgba(99, 102, 241, ${alpha})`;
                                ctx.lineWidth = 0.5;
                                ctx.stroke();
                            }
                        }
                    }
                    requestAnimationFrame(draw);
                }
                draw();
            })();

            // ── Scroll-triggered animations (IntersectionObserver) ──
            (function() {
                const observer = new IntersectionObserver((entries) => {
                    entries.forEach(entry => {
                        if (entry.isIntersecting) {
                            entry.target.classList.add('animate-in');
                            observer.unobserve(entry.target);
                        }
                    });
                }, { threshold: 0.1, rootMargin: '0px 0px -50px 0px' });

                // Feature cards stagger
                document.querySelectorAll('.feature-card').forEach((card, i) => {
                    card.style.transitionDelay = `${i * 60}ms`;
                    observer.observe(card);
                });

                // Editor screenshot
                document.querySelectorAll('.editor-reveal').forEach(el => observer.observe(el));

                // Counters
                document.querySelectorAll('.counter').forEach(el => observer.observe(el));
            })();

            // ── Counter animation ──
            (function() {
                const observer = new IntersectionObserver((entries) => {
                    entries.forEach(entry => {
                        if (entry.isIntersecting) {
                            const el = entry.target;
                            const target = parseInt(el.dataset.target);
                            if (!target) return;
                            const suffix = target === 100 ? '%' : '+';
                            let current = 0;
                            const step = Math.max(1, Math.floor(target / 40));
                            const timer = setInterval(() => {
                                current += step;
                                if (current >= target) { current = target; clearInterval(timer); }
                                el.textContent = current + suffix;
                            }, 30);
                            observer.unobserve(el);
                        }
                    });
                }, { threshold: 0.5 });
                document.querySelectorAll('.counter').forEach(el => observer.observe(el));
            })();
            "#
        </script>

        <style>
            r#"
            /* Hero title shimmer */
            .hero-title {
                background: linear-gradient(
                    135deg,
                    #fafafa 0%,
                    #6366f1 40%,
                    #a78bfa 60%,
                    #fafafa 100%
                );
                background-size: 300% 300%;
                -webkit-background-clip: text;
                -webkit-text-fill-color: transparent;
                background-clip: text;
                animation: shimmer 6s ease-in-out infinite;
            }
            @keyframes shimmer {
                0%, 100% { background-position: 0% 50%; }
                50% { background-position: 100% 50%; }
            }

            /* Scroll indicator bounce */
            .scroll-dot {
                animation: scrollBounce 2s ease-in-out infinite;
            }
            @keyframes scrollBounce {
                0%, 100% { transform: translateY(0); opacity: 1; }
                50% { transform: translateY(6px); opacity: 0.3; }
            }

            /* Feature card animation */
            .feature-card {
                opacity: 0;
                transform: translateY(24px);
                transition: opacity 0.5s ease, transform 0.5s ease;
            }
            .feature-card.animate-in {
                opacity: 1;
                transform: translateY(0);
            }

            /* Editor reveal */
            .editor-reveal {
                opacity: 0;
                transform: translateY(40px) scale(0.97);
                transition: opacity 0.8s ease, transform 0.8s ease;
            }
            .editor-reveal.animate-in {
                opacity: 1;
                transform: translateY(0) scale(1);
            }

            /* Feature card glow on hover */
            .feature-card::before {
                content: '';
                position: absolute;
                inset: 0;
                border-radius: 0.75rem;
                opacity: 0;
                transition: opacity 0.3s;
                pointer-events: none;
            }
            .feature-card:hover::before {
                opacity: 1;
            }
            .feature-card.glow-indigo::before { background: radial-gradient(circle at 50% 0%, rgba(99,102,241,0.08), transparent 70%); }
            .feature-card.glow-violet::before { background: radial-gradient(circle at 50% 0%, rgba(139,92,246,0.08), transparent 70%); }
            .feature-card.glow-cyan::before { background: radial-gradient(circle at 50% 0%, rgba(6,182,212,0.08), transparent 70%); }
            .feature-card.glow-emerald::before { background: radial-gradient(circle at 50% 0%, rgba(16,185,129,0.08), transparent 70%); }
            .feature-card.glow-amber::before { background: radial-gradient(circle at 50% 0%, rgba(245,158,11,0.08), transparent 70%); }
            .feature-card.glow-rose::before { background: radial-gradient(circle at 50% 0%, rgba(244,63,94,0.08), transparent 70%); }
            .feature-card.glow-sky::before { background: radial-gradient(circle at 50% 0%, rgba(14,165,233,0.08), transparent 70%); }
            .feature-card.glow-orange::before { background: radial-gradient(circle at 50% 0%, rgba(249,115,22,0.08), transparent 70%); }

            .icon-indigo { color: #6366f1; background: rgba(99,102,241,0.1); }
            .icon-violet { color: #8b5cf6; background: rgba(139,92,246,0.1); }
            .icon-cyan { color: #06b6d4; background: rgba(6,182,212,0.1); }
            .icon-emerald { color: #10b981; background: rgba(16,185,129,0.1); }
            .icon-amber { color: #f59e0b; background: rgba(245,158,11,0.1); }
            .icon-rose { color: #f43f5e; background: rgba(244,63,94,0.1); }
            .icon-sky { color: #0ea5e9; background: rgba(14,165,233,0.1); }
            .icon-orange { color: #f97316; background: rgba(249,115,22,0.1); }
            "#
        </style>
    }
}

#[component]
fn FeatureCard(icon: &'static str, title: &'static str, description: &'static str, color: &'static str) -> impl IntoView {
    let icon_class = format!("ph {} text-2xl", icon);
    let icon_wrap_class = format!("w-10 h-10 rounded-xl flex items-center justify-center icon-{}", color);
    let card_class = format!("feature-card glow-{} relative p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl hover:border-zinc-700 transition-all group", color);
    view! {
        <div class=card_class>
            <div class=icon_wrap_class>
                <i class=icon_class></i>
            </div>
            <h3 class="text-sm font-semibold mt-3 mb-1">{title}</h3>
            <p class="text-xs text-zinc-500 leading-relaxed">{description}</p>
        </div>
    }
}

#[component]
fn ExploreCard(icon: &'static str, name: &'static str, desc: &'static str, href: &'static str) -> impl IntoView {
    let icon_class = format!("ph {} text-2xl text-accent", icon);
    view! {
        <a href=href class="flex items-center gap-4 p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl hover:border-zinc-600 hover:bg-white/[0.04] transition-all group">
            <div class="w-11 h-11 rounded-xl bg-accent/10 flex items-center justify-center shrink-0 group-hover:scale-110 transition-transform">
                <i class=icon_class></i>
            </div>
            <div>
                <h3 class="text-sm font-semibold group-hover:text-accent transition-colors">{name}</h3>
                <p class="text-[11px] text-zinc-500">{desc}</p>
            </div>
        </a>
    }
}
