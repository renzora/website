use leptos::prelude::*;

#[component]
pub fn DownloadPage() -> impl IntoView {
    view! {
        // Hero
        <section class="relative pt-20 pb-12 px-6 overflow-hidden">
            <div class="absolute inset-0 pointer-events-none">
                <div class="absolute top-0 left-1/2 -translate-x-1/2 w-96 h-96 bg-accent/10 rounded-full blur-[120px]"></div>
            </div>
            <div class="relative z-10 max-w-[1000px] mx-auto text-center">
                <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-accent/10 border border-accent/20 text-accent text-xs font-medium mb-5">
                    <i class="ph ph-download-simple"></i>
                    <span id="release-version">"Loading latest release..."</span>
                </div>
                <h1 class="text-4xl md:text-5xl font-extrabold tracking-tight">"Download Renzora Launcher"</h1>
                <p class="text-zinc-400 mt-3 text-base">"Your hub for the engine, game store, marketplace, and more."</p>
            </div>
        </section>

        <section class="pb-20 px-6">
            <div class="max-w-[1000px] mx-auto">
                // Launcher downloads
                <h2 class="text-lg font-semibold mb-5 flex items-center gap-2">
                    <div class="w-7 h-7 rounded-lg bg-accent/10 flex items-center justify-center">
                        <i class="ph ph-rocket-launch text-sm text-accent"></i>
                    </div>
                    "Launcher"
                </h2>
                <div id="launcher-downloads" class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-14">
                    <DownloadSkeleton />
                    <DownloadSkeleton />
                    <DownloadSkeleton />
                </div>

                // What's included
                <h2 class="text-lg font-semibold mb-5 flex items-center gap-2">
                    <div class="w-7 h-7 rounded-lg bg-purple-500/10 flex items-center justify-center">
                        <i class="ph ph-package text-sm text-purple-400"></i>
                    </div>
                    "What's Included"
                </h2>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-14">
                    <div class="p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-center flex flex-col items-center gap-2">
                        <i class="ph ph-engine text-xl text-accent"></i>
                        <span class="text-sm font-medium">"Engine Manager"</span>
                        <span class="text-[11px] text-zinc-500">"Install and manage engine versions"</span>
                    </div>
                    <div class="p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-center flex flex-col items-center gap-2">
                        <i class="ph ph-game-controller text-xl text-purple-400"></i>
                        <span class="text-sm font-medium">"Game Store"</span>
                        <span class="text-[11px] text-zinc-500">"Discover and play games"</span>
                    </div>
                    <div class="p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-center flex flex-col items-center gap-2">
                        <i class="ph ph-storefront text-xl text-emerald-400"></i>
                        <span class="text-sm font-medium">"Marketplace"</span>
                        <span class="text-[11px] text-zinc-500">"Assets, plugins, and templates"</span>
                    </div>
                    <div class="p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-center flex flex-col items-center gap-2">
                        <i class="ph ph-arrows-clockwise text-xl text-amber-400"></i>
                        <span class="text-sm font-medium">"Auto Updates"</span>
                        <span class="text-[11px] text-zinc-500">"Always up to date automatically"</span>
                    </div>
                </div>

                // Other options
                <h2 class="text-lg font-semibold mb-4 flex items-center gap-2">
                    <div class="w-7 h-7 rounded-lg bg-emerald-500/10 flex items-center justify-center">
                        <i class="ph ph-terminal text-sm text-emerald-400"></i>
                    </div>
                    "Other Options"
                </h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-12">
                    <a href="https://github.com/renzora/engine" target="_blank" rel="noopener noreferrer" class="group p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl hover:border-zinc-600 hover:bg-white/[0.04] transition-all flex items-center gap-4">
                        <div class="w-10 h-10 rounded-xl bg-zinc-800/80 flex items-center justify-center shrink-0 group-hover:scale-110 transition-transform">
                            <i class="ph ph-terminal text-xl text-zinc-400"></i>
                        </div>
                        <div>
                            <h4 class="text-sm font-semibold mb-0.5 group-hover:text-accent transition-colors">"Build engine from source"</h4>
                            <p class="text-xs text-zinc-500">"Clone the repo and compile with Cargo. Requires Rust 1.85+."</p>
                        </div>
                        <i class="ph ph-arrow-up-right text-zinc-600 ml-auto group-hover:text-accent transition-colors"></i>
                    </a>
                    <a href="https://github.com/renzora/engine/releases" target="_blank" rel="noopener noreferrer" class="group p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl hover:border-zinc-600 hover:bg-white/[0.04] transition-all flex items-center gap-4">
                        <div class="w-10 h-10 rounded-xl bg-zinc-800/80 flex items-center justify-center shrink-0 group-hover:scale-110 transition-transform">
                            <i class="ph ph-git-branch text-xl text-zinc-400"></i>
                        </div>
                        <div>
                            <h4 class="text-sm font-semibold mb-0.5 group-hover:text-accent transition-colors">"All engine releases"</h4>
                            <p class="text-xs text-zinc-500">"Browse all versions and pre-release builds on GitHub."</p>
                        </div>
                        <i class="ph ph-arrow-up-right text-zinc-600 ml-auto group-hover:text-accent transition-colors"></i>
                    </a>
                </div>

                <p class="text-center text-sm text-zinc-500">
                    "The launcher manages engine versions, export templates, and keeps everything up to date."
                </p>
            </div>
        </section>
        <script>
            r##"
            const PLATFORMS = [
                { key: 'windows', name: 'Windows', icon: 'ph-windows-logo', match: /windows.*x64|\.exe$/i, req: 'Windows 10+, 64-bit', color: 'cyan' },
                { key: 'macos', name: 'macOS', icon: 'ph-apple-logo', match: /macos.*arm64|\.dmg$/i, req: 'macOS 12 Monterey+', color: 'zinc' },
                { key: 'linux', name: 'Linux', icon: 'ph-linux-logo', match: /linux.*x64|\.appimage$/i, req: 'Ubuntu 22.04+, Fedora 38+', color: 'amber' },
            ];

            (async function() {
                let release = null;
                let assets = [];
                try {
                    const res = await fetch('https://api.github.com/repos/renzora/launcher/releases/latest');
                    if (res.ok) { release = await res.json(); assets = release.assets || []; }
                } catch(e) {}

                if (!release) {
                    try {
                        const res = await fetch('https://api.github.com/repos/renzora/launcher/releases');
                        if (res.ok) { const releases = await res.json(); if (releases.length) { release = releases[0]; assets = release.assets || []; } }
                    } catch(e) {}
                }

                const versionEl = document.getElementById('release-version');
                if (release) {
                    versionEl.innerHTML = `<a href="${release.html_url}" target="_blank" rel="noopener noreferrer" class="text-accent hover:text-accent-hover">${release.tag_name}</a> — ${new Date(release.published_at).toLocaleDateString()}`;
                } else {
                    versionEl.textContent = 'Could not fetch release info';
                }

                function findAsset(pattern) { return assets.find(a => pattern.test(a.name)); }

                const el = document.getElementById('launcher-downloads');
                el.innerHTML = PLATFORMS.map((p, i) => {
                    const asset = findAsset(p.match);
                    const available = !!asset;
                    const url = asset ? asset.browser_download_url : '#';
                    const size = asset ? formatSize(asset.size) : '';
                    return `
                        <div class="relative p-6 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-center flex flex-col items-center gap-3 ${available ? 'hover:border-accent/40 hover:bg-white/[0.04] hover:shadow-lg hover:shadow-accent/5' : 'opacity-40'} transition-all" style="animation: fadeSlideUp 0.5s ease both; animation-delay: ${i * 100}ms">
                            <div class="w-14 h-14 rounded-2xl bg-white/[0.03] border border-zinc-800/30 flex items-center justify-center">
                                <i class="ph ${p.icon} text-2xl ${available ? 'text-zinc-200' : 'text-zinc-600'}"></i>
                            </div>
                            <h3 class="text-lg font-semibold">${p.name}</h3>
                            <p class="text-[11px] text-zinc-500">${p.req}</p>
                            ${available ? `
                                <a href="${url}" class="w-full mt-1 inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)]">
                                    <i class="ph ph-download-simple"></i>Download
                                </a>
                                <span class="text-[10px] text-zinc-600">${size}</span>
                            ` : `
                                <span class="w-full mt-1 inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl text-sm font-medium bg-zinc-800/50 text-zinc-600 cursor-not-allowed">
                                    Coming soon
                                </span>
                            `}
                        </div>
                    `;
                }).join('');
            })();

            function formatSize(bytes) {
                if (bytes > 1e9) return (bytes / 1e9).toFixed(1) + ' GB';
                if (bytes > 1e6) return (bytes / 1e6).toFixed(1) + ' MB';
                return (bytes / 1e3).toFixed(0) + ' KB';
            }
            "##
        </script>

        <style>
            r#"
            @keyframes fadeSlideUp {
                from { opacity: 0; transform: translateY(16px); }
                to { opacity: 1; transform: translateY(0); }
            }
            "#
        </style>
    }
}

#[component]
fn DownloadSkeleton() -> impl IntoView {
    view! {
        <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-xl animate-pulse">
            <div class="w-14 h-14 bg-zinc-800/50 rounded-2xl mx-auto mb-3"></div>
            <div class="h-4 w-20 bg-zinc-800/50 rounded mx-auto mb-2"></div>
            <div class="h-3 w-32 bg-zinc-800/50 rounded mx-auto mb-4"></div>
            <div class="h-10 bg-zinc-800/50 rounded-xl"></div>
        </div>
    }
}
