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
    asset_detail::AssetDetailPage,
    community::CommunityPage,
    dashboard::DashboardPage,
    docs::{DocsPage, DocArticle},
    courses::{CoursesPage, CourseDetailPage, ChapterViewPage, CreateCoursePage, EditCoursePage},
    download::DownloadPage,
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
        <Title text="Renzora Engine — Open Source Game Engine" />
        <Meta name="description" content="An open-source game engine built with Rust and Bevy. Visual editor, scripting, marketplace, and cross-platform export." />

        <Router>
            <Nav />
            <main>
                <Routes fallback=|| view! { <p class="text-center text-zinc-500 py-20">"Page not found."</p> }>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/download") view=DownloadPage />
                    <Route path=path!("/login") view=LoginPage />
                    <Route path=path!("/register") view=RegisterPage />
                    <Route path=path!("/docs") view=DocsPage />
                    <Route path=path!("/docs/*slug") view=DocArticle />
                    <Route path=path!("/marketplace") view=MarketplacePage />
                    <Route path=path!("/marketplace/upload") view=UploadPage />
                    <Route path=path!("/marketplace/asset/:slug") view=AssetDetailPage />
                    <Route path=path!("/wallet") view=WalletPage />
                    <Route path=path!("/courses") view=CoursesPage />
                    <Route path=path!("/courses/create") view=CreateCoursePage />
                    <Route path=path!("/courses/:slug") view=CourseDetailPage />
                    <Route path=path!("/courses/:slug/edit") view=EditCoursePage />
                    <Route path=path!("/courses/:slug/chapter/:chapter") view=ChapterViewPage />
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
