use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::components::nav::Nav;
use crate::components::footer::Footer;
use crate::pages::{
    community::CommunityPage,
    dashboard::DashboardPage,
    docs::{DocsPage, DocArticle},
    download::DownloadPage,
    home::HomePage,
    marketplace::MarketplacePage,
    wallet::WalletPage,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet href="/assets/style/main.css" />
        <Title text="Renzora — Game Engine" />
        <Meta name="description" content="Renzora is a modern game engine built with Rust and Bevy." />

        <Router>
            <Nav />
            <main>
                <Routes fallback=|| view! { <p>"Page not found."</p> }>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/download") view=DownloadPage />
                    <Route path=path!("/docs") view=DocsPage />
                    <Route path=path!("/docs/:category/:slug") view=DocArticle />
                    <Route path=path!("/marketplace") view=MarketplacePage />
                    <Route path=path!("/wallet") view=WalletPage />
                    <Route path=path!("/community") view=CommunityPage />
                    <Route path=path!("/dashboard") view=DashboardPage />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}
