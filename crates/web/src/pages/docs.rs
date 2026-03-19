use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

/// Documentation landing page — fetches categories from DB.
#[component]
pub fn DocsPage() -> impl IntoView {
    view! {
        <div class="flex min-h-[calc(100vh-56px)] max-w-[1200px] mx-auto">
            <DocsSidebar />
            <div class="flex-1 min-w-0 px-8 py-10 lg:px-12">
                <div class="mb-10">
                    <h1 class="text-4xl font-bold">"Documentation"</h1>
                    <p class="text-zinc-400 mt-2">"Everything you need to build games with Renzora Engine."</p>
                </div>
                <div id="docs-landing">"Loading..."</div>
            </div>
        </div>
        <script>
            r##"
            (async function() {
                const res = await fetch('/api/docs');
                if (!res.ok) return;
                const data = await res.json();
                const el = document.getElementById('docs-landing');
                if (!data.categories?.length) {
                    el.innerHTML = '<p class="text-zinc-500 text-sm">No documentation yet.</p>';
                    return;
                }
                el.innerHTML = data.categories.map((cat, i) => `
                    <div class="mb-6">
                        <h2 class="text-sm font-semibold text-zinc-400 uppercase tracking-wider mb-3">${cat.category}</h2>
                        <div class="space-y-2">
                            ${cat.pages.map(p => `
                                <a href="/docs/${p.slug}" class="flex items-center gap-4 p-4 bg-surface-card border border-zinc-800 rounded-xl hover:border-zinc-700 transition-all group">
                                    <div class="flex-1">
                                        <h3 class="text-sm font-medium group-hover:text-accent transition-colors">${p.title}</h3>
                                    </div>
                                    <i class="ph ph-caret-right text-zinc-600 group-hover:text-zinc-400 transition-colors"></i>
                                </a>
                            `).join('')}
                        </div>
                    </div>
                `).join('');
            })();
            "##
        </script>
    }
}

/// Individual doc page — fetches from DB by slug.
#[component]
pub fn DocArticle() -> impl IntoView {
    view! {
        <div class="flex min-h-[calc(100vh-56px)] max-w-[1200px] mx-auto">
            <DocsSidebar />
            <div class="flex-1 min-w-0 px-8 py-10 lg:px-12">
                <article id="doc-content">"Loading..."</article>
            </div>
        </div>
        // Highlight.js for code blocks
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github-dark.min.css" />
        <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
        <script>
            r##"
            (async function() {
                const slug = window.location.pathname.replace('/docs/', '');
                const res = await fetch('/api/docs/' + slug);
                const el = document.getElementById('doc-content');
                if (!res.ok) {
                    el.innerHTML = '<h1 class="text-3xl font-bold mb-4">Page not found</h1><p class="text-zinc-400">This documentation page does not exist yet.</p><a href="/docs" class="text-accent hover:text-accent-hover text-sm mt-4 inline-block">Back to docs</a>';
                    return;
                }
                const doc = await res.json();
                el.innerHTML = `
                    <p class="text-xs text-zinc-500 mb-6"><a href="/docs" class="text-accent hover:text-accent-hover">Docs</a> / ${doc.category}</p>
                    <h1 class="text-3xl font-bold mb-6">${doc.title}</h1>
                    <div class="doc-body text-sm text-zinc-300 leading-relaxed">${doc.content}</div>
                `;
                // Apply syntax highlighting to code blocks
                document.querySelectorAll('.doc-body pre code').forEach(block => {
                    hljs.highlightElement(block);
                });
            })();
            "##
        </script>
    }
}

/// Sidebar — fetches doc list from DB.
#[component]
fn DocsSidebar() -> impl IntoView {
    view! {
        <aside class="w-64 shrink-0 border-r border-zinc-800 bg-surface sticky top-14 h-[calc(100vh-56px)] overflow-y-auto hidden lg:block">
            <div class="p-4" id="docs-sidebar-content">
                <p class="text-xs text-zinc-500 p-2">"Loading..."</p>
            </div>
        </aside>
        <script>
            r##"
            (async function() {
                const res = await fetch('/api/docs');
                if (!res.ok) return;
                const data = await res.json();
                const el = document.getElementById('docs-sidebar-content');
                const currentPath = window.location.pathname.replace('/docs/', '').replace('/docs', '');
                if (!data.categories?.length) {
                    el.innerHTML = '<p class="text-xs text-zinc-500 p-2">No docs yet.</p>';
                    return;
                }
                el.innerHTML = data.categories.map(cat => `
                    <div class="mb-6">
                        <h4 class="text-[11px] font-semibold uppercase tracking-[0.08em] text-zinc-500 mb-2 px-2">${cat.category}</h4>
                        <ul class="flex flex-col gap-px">
                            ${cat.pages.map(p => {
                                const isActive = currentPath === p.slug;
                                return `<li><a href="/docs/${p.slug}" class="block px-2 py-1.5 text-[13px] rounded transition-all ${isActive ? 'bg-accent/10 text-accent' : 'text-zinc-400 hover:text-zinc-50 hover:bg-white/5'}">${p.title}</a></li>`;
                            }).join('')}
                        </ul>
                    </div>
                `).join('');
            })();
            "##
        </script>
    }
}
