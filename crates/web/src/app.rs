use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::components::nav::Nav;
use crate::pages::{
    admin::AdminPage,
    asset_detail::AssetDetailPage,
    asset_edit::AssetEditPage,
    dashboard::DashboardPage,
    developers::DevelopersPage,
    docs::{DocsPage, DocArticle},
    donate::DonatePage,
    download::DownloadPage,
    game::GamePage,
    gifts::GiftsPage,
    library::LibraryPage,
    login::{LoginPage, RegisterPage},
    marketplace::MarketplacePage,
    sell::SellOnboardingPage,
    shop::ShopPage,
    settings::SettingsPage,
    upload::UploadPage,
    wallet::WalletPage,
    embed::EmbedPreviewPage,
    terms::TermsPage,
    privacy::PrivacyPage,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // CSS is inlined in the shell <head> (no external stylesheet). Still preload
        // the icon-font subset so the fetch starts in parallel with HTML parsing.
        <Link rel="preload" href="/assets/fonts/phosphor-regular.woff2" as_="font" crossorigin="anonymous" />
        <Title text="Download Renzora, Open Source Bevy Editor & Game Engine" />
        <Meta name="description" content="Download Renzora, a free, open-source Bevy editor and game engine, a full 2D & 3D visual editor for Bevy with Lua & Rhai scripting, a plugin system, physics and real-time rendering, built in Rust. Available for Windows, macOS, Linux and the web." />
        <Meta property="og:type" content="website" />
        <Meta property="og:site_name" content="Renzora" />
        <Meta property="og:title" content="Download Renzora, Open Source Bevy Editor & Game Engine" />
        <Meta property="og:description" content="A free, open-source Bevy editor and game engine, full 2D & 3D scene tooling, scripting, plugins, physics and real-time rendering, built in Rust on Bevy 0.19." />
        <Meta property="og:image" content="https://renzora.com/assets/previews/og.jpg" />
        <Meta property="og:url" content="https://renzora.com/" />

        <Router>
            <Nav />
            <main class="app-main">
                <Routes fallback=|| view! { <p class="text-center text-zinc-500 py-20">"Page not found."</p> }>
                    // The download page is the site root.
                    <Route path=path!("/") view=DownloadPage />
                    <Route path=path!("/game") view=GamePage />
                    <Route path=path!("/login") view=LoginPage />
                    <Route path=path!("/register") view=RegisterPage />
                    <Route path=path!("/docs") view=DocsPage />
                    <Route path=path!("/docs/*slug") view=DocArticle />
                    <Route path=path!("/marketplace") view=MarketplacePage />
                    <Route path=path!("/marketplace/sell") view=SellOnboardingPage />
                    <Route path=path!("/marketplace/upload") view=UploadPage />
                    <Route path=path!("/marketplace/asset/:slug/edit") view=AssetEditPage />
                    <Route path=path!("/marketplace/asset/:slug") view=AssetDetailPage />
                    <Route path=path!("/library") view=LibraryPage />
                    <Route path=path!("/wallet") view=WalletPage />
                    <Route path=path!("/gifts") view=GiftsPage />
                    <Route path=path!("/shop/:username") view=ShopPage />
                    <Route path=path!("/dashboard") view=DashboardPage />
                    <Route path=path!("/admin") view=AdminPage />
                    <Route path=path!("/developers") view=DevelopersPage />
                    <Route path=path!("/donate") view=DonatePage />
                    <Route path=path!("/terms") view=TermsPage />
                    <Route path=path!("/privacy") view=PrivacyPage />
                    <Route path=path!("/settings") view=SettingsPage />
                    <Route path=path!("/embed/preview/:slug") view=EmbedPreviewPage />
                </Routes>
            </main>
        </Router>
    }
}
