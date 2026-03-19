use leptos::prelude::*;

#[component]
pub fn DownloadPage() -> impl IntoView {
    view! {
        <section class="py-20 px-6">
            <div class="max-w-[1200px] mx-auto">
                <div class="text-center mb-12">
                    <h1 class="text-4xl font-bold">"Download Renzora Engine"</h1>
                    <p class="text-zinc-400 mt-2">"Free and open source. Get started in minutes."</p>
                    <p class="mt-3 text-sm text-accent">"Current release: r1-alpha4 — Early Access"</p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-12">
                    <DownloadCard
                        platform="Windows"
                        icon="\u{1F5B5}"
                        requirements="Windows 10 or later, 64-bit"
                        primary_label="Download .exe"
                        primary_href="https://github.com/renzora/engine/releases/latest"
                        alt_label="Portable .zip"
                        alt_href="https://github.com/renzora/engine/releases/latest"
                    />
                    <DownloadCard
                        platform="macOS"
                        icon="\u{F8FF}"
                        requirements="macOS 12 Monterey or later"
                        primary_label="Download .dmg"
                        primary_href="https://github.com/renzora/engine/releases/latest"
                        alt_label="Homebrew (coming soon)"
                        alt_href=""
                    />
                    <DownloadCard
                        platform="Linux"
                        icon="\u{1F427}"
                        requirements="Ubuntu 22.04+, Fedora 38+, or Arch"
                        primary_label="Download .AppImage"
                        primary_href="https://github.com/renzora/engine/releases/latest"
                        alt_label=".deb / .rpm"
                        alt_href="https://github.com/renzora/engine/releases/latest"
                    />
                </div>

                <div class="mb-10">
                    <h3 class="text-base font-semibold mb-3">"Other options"</h3>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                        <a href="https://github.com/renzora/engine" class="p-5 bg-surface-card border border-zinc-800 rounded-lg hover:border-accent transition-colors">
                            <h4 class="text-sm font-semibold mb-1">"Build from source"</h4>
                            <p class="text-xs text-zinc-400">"Clone the repo and compile with Cargo. Requires Rust 1.85+."</p>
                        </a>
                        <a href="https://github.com/renzora/engine/releases" class="p-5 bg-surface-card border border-zinc-800 rounded-lg hover:border-accent transition-colors">
                            <h4 class="text-sm font-semibold mb-1">"All releases"</h4>
                            <p class="text-xs text-zinc-400">"Browse previous versions and pre-release builds on GitHub."</p>
                        </a>
                    </div>
                </div>

                <p class="text-center text-sm text-zinc-400">
                    "After installing, follow the "
                    <a href="/docs/getting-started/first-project" class="text-accent hover:text-accent-hover">"Getting Started guide"</a>
                    " to create your first project."
                </p>
            </div>
        </section>
    }
}

#[component]
fn DownloadCard(
    platform: &'static str,
    icon: &'static str,
    requirements: &'static str,
    primary_label: &'static str,
    primary_href: &'static str,
    alt_label: &'static str,
    alt_href: &'static str,
) -> impl IntoView {
    view! {
        <div class="p-8 bg-surface-card border border-zinc-800 rounded-lg text-center flex flex-col items-center gap-2 hover:border-accent transition-colors">
            <div class="text-4xl mb-1">{icon}</div>
            <h2 class="text-xl font-semibold">{platform}</h2>
            <p class="text-xs text-zinc-400 mb-3">{requirements}</p>
            <a href=primary_href class="w-full inline-flex items-center justify-center px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">{primary_label}</a>
            {if !alt_href.is_empty() {
                view! { <a href=alt_href class="text-xs text-zinc-400 hover:text-zinc-50 mt-1">{alt_label}</a> }.into_any()
            } else {
                view! { <span class="text-xs text-zinc-500 opacity-50 mt-1">{alt_label}</span> }.into_any()
            }}
        </div>
    }
}
