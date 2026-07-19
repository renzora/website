use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::components::nav::Nav;
use crate::pages::{
    articles::{ArticlesPage, ArticleDetailPage, WriteArticlePage},
    asset_detail::AssetDetailPage,
    asset_edit::AssetEditPage,
    community::CommunityPage,
    community_post::PostDetailPage,
    dashboard::DashboardPage,
    developers::DevelopersPage,
    docs::{DocsPage, DocArticle},
    donate::DonatePage,
    courses::{CoursesPage, CourseDetailPage, ChapterViewPage, CreateCoursePage, EditCoursePage},
    download::DownloadPage,
    friends::FriendsPage,
    gifts::GiftsPage,
    home::HomePage,
    library::LibraryPage,
    login::{LoginPage, RegisterPage},
    marketplace::MarketplacePage,
    profile::ProfilePage,
    sell::SellOnboardingPage,
    shop::ShopPage,
    settings::SettingsPage,
    subscription::SubscriptionPage,
    teams::TeamsPage,
    upload::UploadPage,
    wallet::WalletPage,
    embed::EmbedPreviewPage,
    messages::MessagesPage,
    notifications::NotificationsPage,
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
        <Title text="Renzora, Open Source Bevy Editor & Game Engine" />
        <Meta name="description" content="Renzora is a free, open-source Bevy editor and game engine, a full 2D & 3D visual editor for Bevy with Lua & Rhai scripting, a plugin system, physics and real-time rendering, built in Rust. Download for Windows, macOS, Linux and the web." />
        <Meta property="og:type" content="website" />
        <Meta property="og:site_name" content="Renzora" />
        <Meta property="og:title" content="Renzora, Open Source Bevy Editor & Game Engine" />
        <Meta property="og:description" content="A free, open-source Bevy editor and game engine, full 2D & 3D scene tooling, scripting, plugins, physics and real-time rendering, built in Rust on Bevy 0.19." />
        <Meta property="og:image" content="https://renzora.com/assets/previews/og.jpg" />
        <Meta property="og:url" content="https://renzora.com/" />

        <Router>
            <Nav />
            <main class="app-main">
                <Routes fallback=|| view! { <p class="text-center text-zinc-500 py-20">"Page not found."</p> }>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/download") view=DownloadPage />
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
                    <Route path=path!("/courses") view=CoursesPage />
                    <Route path=path!("/courses/create") view=CreateCoursePage />
                    <Route path=path!("/courses/:slug") view=CourseDetailPage />
                    <Route path=path!("/courses/:slug/edit") view=EditCoursePage />
                    <Route path=path!("/courses/:slug/chapter/:chapter") view=ChapterViewPage />
                    <Route path=path!("/community") view=CommunityPage />
                    <Route path=path!("/community/channel/:slug") view=CommunityPage />
                    <Route path=path!("/community/post/:id") view=PostDetailPage />
                    <Route path=path!("/articles") view=ArticlesPage />
                    <Route path=path!("/articles/write") view=WriteArticlePage />
                    <Route path=path!("/articles/:slug") view=ArticleDetailPage />
                    <Route path=path!("/friends") view=FriendsPage />
                    <Route path=path!("/notifications") view=NotificationsPage />
                    <Route path=path!("/profile/:username") view=ProfilePage />
                    <Route path=path!("/shop/:username") view=ShopPage />
                    <Route path=path!("/dashboard") view=DashboardPage />
                    <Route path=path!("/developers") view=DevelopersPage />
                    <Route path=path!("/subscription") view=SubscriptionPage />
                    <Route path=path!("/teams") view=TeamsPage />
                    <Route path=path!("/messages") view=MessagesPage />
                    <Route path=path!("/feed") view=CommunityPage />
                    <Route path=path!("/donate") view=DonatePage />
                    <Route path=path!("/gifts") view=GiftsPage />
                    <Route path=path!("/terms") view=TermsPage />
                    <Route path=path!("/privacy") view=PrivacyPage />
                    <Route path=path!("/settings") view=SettingsPage />
                    <Route path=path!("/embed/preview/:slug") view=EmbedPreviewPage />
                </Routes>
            </main>
        </Router>
    }
}
