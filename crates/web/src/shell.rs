use leptos::prelude::*;
use leptos_meta::MetaTags;

use crate::app::App;

/// The HTML shell that wraps the entire application for SSR.
#[component]
pub fn Shell() -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="dark">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <script src="https://cdn.tailwindcss.com"></script>
                <link rel="stylesheet" href="https://unpkg.com/@phosphor-icons/web@2.1.1/src/regular/style.css" />
                <script>
                    "tailwind.config = {
                        darkMode: 'class',
                        theme: {
                            extend: {
                                colors: {
                                    accent: { DEFAULT: '#6366f1', hover: '#818cf8', subtle: 'rgba(99,102,241,0.1)' },
                                    surface: { DEFAULT: '#111113', card: '#18181b', panel: '#0a0a0b' },
                                },
                                fontFamily: {
                                    sans: ['-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Roboto', 'sans-serif'],
                                    mono: ['Cascadia Code', 'Fira Code', 'monospace'],
                                },
                            },
                        },
                    }"
                </script>
                <link rel="stylesheet" href="/assets/style/main.css" />
                <style>
                    "html{scrollbar-width:thin;scrollbar-color:#1a1a1e #060608}
                    body{scrollbar-width:thin;scrollbar-color:#1a1a1e #060608}
                    ::-webkit-scrollbar{width:8px;height:8px}
                    ::-webkit-scrollbar-track{background:#060608}
                    ::-webkit-scrollbar-thumb{background:#1a1a1e;border-radius:4px}
                    ::-webkit-scrollbar-thumb:hover{background:#28282e}
                    ::-webkit-scrollbar-corner{background:#060608}"
                </style>
                <MetaTags />
            </head>
            <body class="bg-[#060608] text-zinc-50 antialiased">
                <App />
            </body>
        </html>
    }
}
