use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="site-nav">
            <div class="nav-inner">
                <a href="/" class="nav-logo">"Renzora"</a>
                <div class="nav-links">
                    <a href="/docs">"Docs"</a>
                    <a href="/marketplace">"Marketplace"</a>
                    <a href="/community">"Community"</a>
                </div>
                <div class="nav-actions">
                    <a href="/dashboard" class="btn btn-ghost">"Dashboard"</a>
                    <a href="/login" class="btn btn-ghost">"Sign In"</a>
                </div>
            </div>
        </nav>
    }
}
