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
                <link rel="stylesheet" href="https://unpkg.com/@phosphor-icons/web@2.1.1/src/fill/style.css" />
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
                    "html,body{scrollbar-width:thin;scrollbar-color:#1a1a1e #060608}
                    *{scrollbar-width:thin;scrollbar-color:#1a1a1e #060608}
                    ::-webkit-scrollbar{width:8px!important;height:8px!important}
                    ::-webkit-scrollbar-track{background:#060608!important}
                    ::-webkit-scrollbar-thumb{background:#1a1a1e!important;border-radius:4px!important}
                    ::-webkit-scrollbar-thumb:hover{background:#28282e!important}
                    ::-webkit-scrollbar-corner{background:#060608!important}
                    select,select option{background-color:#18181b!important;color:#fafafa!important}
                    select option:checked{background-color:#27272a!important}
                    select{-webkit-appearance:none;-moz-appearance:none;appearance:none;background-image:url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%2371717a' stroke-width='2'%3E%3Cpath d='M6 9l6 6 6-6'/%3E%3C/svg%3E\");background-repeat:no-repeat;background-position:right 8px center;padding-right:28px}"
                </style>
                <MetaTags />
            </head>
            <body class="bg-[#060608] text-zinc-50 antialiased">
                <App />
            </body>
        </html>
    }
}
