use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="footer-inner">
                <div class="footer-brand">
                    <span class="footer-logo">"Renzora"</span>
                    <p class="footer-tagline">"A modern game engine built with Rust."</p>
                </div>
                <div class="footer-links">
                    <div class="footer-col">
                        <h4>"Product"</h4>
                        <a href="/download">"Download"</a>
                        <a href="/docs">"Documentation"</a>
                        <a href="/marketplace">"Marketplace"</a>
                    </div>
                    <div class="footer-col">
                        <h4>"Community"</h4>
                        <a href="https://github.com/renzora">"GitHub"</a>
                        <a href="https://discord.gg/renzora">"Discord"</a>
                    </div>
                </div>
                <div class="footer-bottom">
                    <p>"© 2026 Renzora. All rights reserved."</p>
                </div>
            </div>
        </footer>
    }
}
