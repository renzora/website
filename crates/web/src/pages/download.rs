use leptos::prelude::*;

#[component]
pub fn DownloadPage() -> impl IntoView {
    view! {
        <section class="download-page">
            <div class="container">
                <div class="download-hero">
                    <h1>"Download Renzora Engine"</h1>
                    <p class="download-sub">"Free and open source. Get started in minutes."</p>
                    <p class="download-version">"Current release: r1-alpha4 — Early Access"</p>
                </div>

                <div class="download-grid">
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
                        requirements="macOS 12 Monterey or later, Apple Silicon & Intel"
                        primary_label="Download .dmg"
                        primary_href="https://github.com/renzora/engine/releases/latest"
                        alt_label="Homebrew (coming soon)"
                        alt_href=""
                    />
                    <DownloadCard
                        platform="Linux"
                        icon="\u{1F427}"
                        requirements="Ubuntu 22.04+, Fedora 38+, or Arch. X11/Wayland."
                        primary_label="Download .AppImage"
                        primary_href="https://github.com/renzora/engine/releases/latest"
                        alt_label=".deb / .rpm"
                        alt_href="https://github.com/renzora/engine/releases/latest"
                    />
                </div>

                <div class="download-alt">
                    <h3>"Other options"</h3>
                    <div class="download-alt-grid">
                        <a href="https://github.com/renzora/engine" class="download-alt-card">
                            <h4>"Build from source"</h4>
                            <p>"Clone the repo and compile with Cargo. Requires Rust 1.85+."</p>
                        </a>
                        <a href="https://github.com/renzora/engine/releases" class="download-alt-card">
                            <h4>"All releases"</h4>
                            <p>"Browse previous versions and pre-release builds on GitHub."</p>
                        </a>
                    </div>
                </div>

                <div class="download-next">
                    <p>"After installing, follow the " <a href="/docs/getting-started/first-project">"Getting Started guide"</a> " to create your first project."</p>
                </div>
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
        <div class="download-card">
            <div class="download-card-icon">{icon}</div>
            <h2>{platform}</h2>
            <p class="download-req">{requirements}</p>
            <a href=primary_href class="btn btn-primary download-btn">{primary_label}</a>
            {if !alt_href.is_empty() {
                view! { <a href=alt_href class="btn btn-ghost download-btn-alt">{alt_label}</a> }.into_any()
            } else {
                view! { <span class="download-btn-alt disabled">{alt_label}</span> }.into_any()
            }}
        </div>
    }
}
