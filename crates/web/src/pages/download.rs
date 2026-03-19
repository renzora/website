use leptos::prelude::*;

#[component]
pub fn DownloadPage() -> impl IntoView {
    view! {
        <section class="py-12 px-6">
            <div class="max-w-[1000px] mx-auto">
                <div class="text-center mb-10">
                    <h1 class="text-3xl font-bold">"Download Renzora Engine"</h1>
                    <p class="text-zinc-400 mt-2 text-sm">"Free and open source."</p>
                    <p id="release-version" class="mt-2 text-xs text-accent">"Loading latest release..."</p>
                </div>

                // Editor downloads
                <h2 class="text-lg font-semibold mb-4">"Editor"</h2>
                <div id="editor-downloads" class="grid grid-cols-1 md:grid-cols-3 gap-3 mb-12">
                    <DownloadSkeleton />
                    <DownloadSkeleton />
                    <DownloadSkeleton />
                </div>

                // Export templates
                <h2 class="text-lg font-semibold mb-2">"Export Templates"</h2>
                <p class="text-xs text-zinc-500 mb-4">"Required for exporting your game to each platform. Downloaded automatically on first export, or grab them here."</p>
                <div id="template-downloads" class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-3 mb-12">
                    <TemplateSkeleton />
                    <TemplateSkeleton />
                    <TemplateSkeleton />
                    <TemplateSkeleton />
                </div>

                // Other options
                <div class="mb-10">
                    <h2 class="text-lg font-semibold mb-3">"Other options"</h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                        <a href="https://github.com/renzora/engine" class="p-5 bg-surface-card border border-zinc-800 rounded-xl hover:border-accent transition-colors flex items-center gap-4">
                            <i class="ph ph-terminal text-2xl text-zinc-400"></i>
                            <div>
                                <h4 class="text-sm font-semibold mb-0.5">"Build from source"</h4>
                                <p class="text-xs text-zinc-400">"Clone the repo and compile with Cargo. Requires Rust 1.85+."</p>
                            </div>
                        </a>
                        <a href="https://github.com/renzora/engine/releases" class="p-5 bg-surface-card border border-zinc-800 rounded-xl hover:border-accent transition-colors flex items-center gap-4">
                            <i class="ph ph-git-branch text-2xl text-zinc-400"></i>
                            <div>
                                <h4 class="text-sm font-semibold mb-0.5">"All releases"</h4>
                                <p class="text-xs text-zinc-400">"Browse all versions and pre-release builds on GitHub."</p>
                            </div>
                        </a>
                    </div>
                </div>

                <p class="text-center text-sm text-zinc-400">
                    "After installing, follow the "
                    <a href="/docs/getting-started/installation" class="text-accent hover:text-accent-hover">"Getting Started guide"</a>
                    " to create your first project."
                </p>
            </div>
        </section>
        <script>
            r##"
            const EDITOR_PLATFORMS = [
                { key: 'windows', name: 'Windows', icon: 'ph-windows-logo', match: /windows.*x64|win64|\.exe$/i, req: 'Windows 10+, 64-bit' },
                { key: 'macos', name: 'macOS', icon: 'ph-apple-logo', match: /macos|darwin|\.dmg$/i, req: 'macOS 12 Monterey+' },
                { key: 'linux', name: 'Linux', icon: 'ph-linux-logo', match: /linux.*x64|\.appimage$/i, req: 'Ubuntu 22.04+, Fedora 38+' },
            ];

            const TEMPLATE_PLATFORMS = [
                { key: 'tpl-windows', name: 'Windows', icon: 'ph-windows-logo', match: /template.*windows|export.*windows/i },
                { key: 'tpl-macos', name: 'macOS', icon: 'ph-apple-logo', match: /template.*macos|export.*macos|template.*darwin/i },
                { key: 'tpl-linux', name: 'Linux', icon: 'ph-linux-logo', match: /template.*linux|export.*linux/i },
                { key: 'tpl-ios', name: 'iOS', icon: 'ph-device-mobile', match: /template.*ios|export.*ios/i },
                { key: 'tpl-ipad', name: 'iPadOS', icon: 'ph-device-tablet', match: /template.*ipad|export.*ipad/i },
                { key: 'tpl-tvos', name: 'Apple TV', icon: 'ph-television', match: /template.*tvos|export.*tvos|template.*appletv/i },
                { key: 'tpl-android-arm', name: 'Android ARM', icon: 'ph-android-logo', match: /template.*android.*arm|export.*android.*arm/i },
                { key: 'tpl-android-x86', name: 'Android x86', icon: 'ph-android-logo', match: /template.*android.*x86|export.*android.*x86/i },
                { key: 'tpl-web', name: 'Web', icon: 'ph-globe', match: /template.*web|template.*wasm|export.*web/i },
            ];

            (async function() {
                let release = null;
                let assets = [];
                try {
                    const res = await fetch('https://api.github.com/repos/renzora/engine/releases/latest');
                    if (res.ok) {
                        release = await res.json();
                        assets = release.assets || [];
                    }
                } catch(e) {}

                // Fallback: try listing all releases
                if (!release) {
                    try {
                        const res = await fetch('https://api.github.com/repos/renzora/engine/releases');
                        if (res.ok) {
                            const releases = await res.json();
                            if (releases.length) { release = releases[0]; assets = release.assets || []; }
                        }
                    } catch(e) {}
                }

                // Update version
                const versionEl = document.getElementById('release-version');
                if (release) {
                    versionEl.innerHTML = `<a href="${release.html_url}" class="text-accent hover:text-accent-hover">${release.tag_name}</a> &mdash; ${new Date(release.published_at).toLocaleDateString()}`;
                } else {
                    versionEl.textContent = 'Could not fetch release info';
                }

                // Match assets to platforms
                function findAsset(pattern) {
                    return assets.find(a => pattern.test(a.name));
                }

                // Render editor downloads
                const editorEl = document.getElementById('editor-downloads');
                editorEl.innerHTML = EDITOR_PLATFORMS.map(p => {
                    const asset = findAsset(p.match);
                    const available = !!asset;
                    const url = asset ? asset.browser_download_url : '#';
                    const size = asset ? (asset.size / 1024 / 1024).toFixed(1) + ' MB' : '';
                    return `
                        <div class="p-6 bg-surface-card border border-zinc-800 rounded-xl text-center flex flex-col items-center gap-2 ${available ? 'hover:border-accent' : 'opacity-40'} transition-colors">
                            <i class="ph ${p.icon} text-3xl ${available ? 'text-zinc-300' : 'text-zinc-600'}"></i>
                            <h3 class="text-lg font-semibold">${p.name}</h3>
                            <p class="text-xs text-zinc-500">${p.req}</p>
                            ${available ? `
                                <a href="${url}" class="w-full mt-2 inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                                    <i class="ph ph-download-simple"></i>Download
                                </a>
                                <span class="text-[10px] text-zinc-500">${size}</span>
                            ` : `
                                <span class="w-full mt-2 inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg text-sm font-medium bg-zinc-800 text-zinc-500 cursor-not-allowed">
                                    Not available yet
                                </span>
                            `}
                        </div>
                    `;
                }).join('');

                // Render export templates
                const tplEl = document.getElementById('template-downloads');
                tplEl.innerHTML = TEMPLATE_PLATFORMS.map(p => {
                    const asset = findAsset(p.match);
                    const available = !!asset;
                    const url = asset ? asset.browser_download_url : '#';
                    const size = asset ? (asset.size / 1024 / 1024).toFixed(1) + ' MB' : '';
                    return `
                        <div class="p-4 bg-surface-card border border-zinc-800 rounded-xl flex flex-col items-center gap-2 ${available ? 'hover:border-accent' : 'opacity-40'} transition-colors">
                            <i class="ph ${p.icon} text-xl ${available ? 'text-zinc-300' : 'text-zinc-600'}"></i>
                            <span class="text-sm font-medium">${p.name}</span>
                            ${available ? `
                                <a href="${url}" class="w-full inline-flex items-center justify-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                                    <i class="ph ph-download-simple"></i>Download
                                </a>
                                <span class="text-[10px] text-zinc-500">${size}</span>
                            ` : `
                                <span class="text-[10px] text-zinc-500">Coming soon</span>
                            `}
                        </div>
                    `;
                }).join('');
            })();
            "##
        </script>
    }
}

#[component]
fn DownloadSkeleton() -> impl IntoView {
    view! {
        <div class="p-6 bg-surface-card border border-zinc-800 rounded-xl animate-pulse">
            <div class="h-8 w-8 bg-zinc-800 rounded mx-auto mb-3"></div>
            <div class="h-4 w-20 bg-zinc-800 rounded mx-auto mb-2"></div>
            <div class="h-3 w-32 bg-zinc-800 rounded mx-auto mb-4"></div>
            <div class="h-10 bg-zinc-800 rounded"></div>
        </div>
    }
}

#[component]
fn TemplateSkeleton() -> impl IntoView {
    view! {
        <div class="p-4 bg-surface-card border border-zinc-800 rounded-xl animate-pulse">
            <div class="h-5 w-5 bg-zinc-800 rounded mx-auto mb-2"></div>
            <div class="h-3 w-16 bg-zinc-800 rounded mx-auto mb-2"></div>
            <div class="h-7 bg-zinc-800 rounded"></div>
        </div>
    }
}
