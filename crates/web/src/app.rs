use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::components::nav::Nav;
use crate::components::footer::Footer;
use crate::pages::{
    admin::AdminPage,
    community::CommunityPage,
    dashboard::DashboardPage,
    docs::{DocsPage, DocArticle},
    download::DownloadPage,
    engine::EnginePage,
    forum::{ForumPage, ForumCategoryPage, ForumThreadPage, NewThreadPage},
    home::HomePage,
    login::{LoginPage, RegisterPage},
    marketplace::MarketplacePage,
    profile::ProfilePage,
    settings::SettingsPage,
    upload::UploadPage,
    wallet::WalletPage,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet href="/assets/style/main.css" />
        <Title text="Renzora — The Game Developer Hub" />
        <Meta name="description" content="Browse and sell game assets for any engine. Community forum, marketplace, and the Renzora open-source game engine." />

        <Router>
            <Nav />
            <main>
                <Routes fallback=|| view! { <p class="text-center text-zinc-500 py-20">"Page not found."</p> }>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/engine") view=EnginePage />
                    <Route path=path!("/download") view=DownloadPage />
                    <Route path=path!("/login") view=LoginPage />
                    <Route path=path!("/register") view=RegisterPage />
                    <Route path=path!("/docs") view=DocsPage />
                    <Route path=path!("/docs/:category/:slug") view=DocArticle />
                    <Route path=path!("/marketplace") view=MarketplacePage />
                    <Route path=path!("/marketplace/upload") view=UploadPage />
                    <Route path=path!("/wallet") view=WalletPage />
                    <Route path=path!("/community") view=CommunityPage />
                    <Route path=path!("/forum") view=ForumPage />
                    <Route path=path!("/forum/new") view=NewThreadPage />
                    <Route path=path!("/forum/thread/:slug") view=ForumThreadPage />
                    <Route path=path!("/forum/:slug") view=ForumCategoryPage />
                    <Route path=path!("/profile/:username") view=ProfilePage />
                    <Route path=path!("/dashboard") view=DashboardPage />
                    <Route path=path!("/settings") view=SettingsPage />
                    <Route path=path!("/admin") view=AdminPage />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}
