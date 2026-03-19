use leptos::prelude::*;
use leptos_meta::MetaTags;

use crate::app::App;

/// The HTML shell that wraps the entire application for SSR.
#[component]
pub fn Shell() -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="stylesheet" href="/assets/style/main.css" />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}
