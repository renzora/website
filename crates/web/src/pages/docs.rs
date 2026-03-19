use leptos::prelude::*;

/// Docs landing — pick your audience.
#[component]
pub fn DocsPage() -> impl IntoView {
    view! {
        <section class="py-12 px-6">
            <div class="max-w-[800px] mx-auto">
                <div class="text-center mb-10">
                    <h1 class="text-3xl font-bold">"Documentation"</h1>
                    <p class="text-zinc-400 mt-2 text-sm">"Choose your path."</p>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <a href="/docs/game-dev" class="block p-8 bg-surface-card border border-zinc-800 rounded-xl hover:border-accent transition-all group text-center">
                        <i class="ph ph-game-controller text-4xl text-accent mb-3"></i>
                        <h2 class="text-lg font-semibold group-hover:text-accent transition-colors">"Game Developer"</h2>
                        <p class="text-sm text-zinc-400 mt-2">"Learn how to use the engine: editor, scripting, exporting, and marketplace."</p>
                    </a>
                    <a href="/docs/developer" class="block p-8 bg-surface-card border border-zinc-800 rounded-xl hover:border-accent transition-all group text-center">
                        <i class="ph ph-code text-4xl text-accent mb-3"></i>
                        <h2 class="text-lg font-semibold group-hover:text-accent transition-colors">"Engine Developer"</h2>
                        <p class="text-sm text-zinc-400 mt-2">"Build, extend, and contribute: architecture, components, plugins, and rendering."</p>
                    </a>
                </div>
            </div>
        </section>
    }
}

/// Docs section landing (game-dev or developer).
#[component]
pub fn DocsSectionPage() -> impl IntoView {
    view! {
        <div class="flex min-h-[calc(100vh-56px)] max-w-[1200px] mx-auto">
            <DocsSidebar />
            <div class="flex-1 min-w-0 px-8 py-10 lg:px-12">
                <div id="section-landing">"Loading..."</div>
            </div>
        </div>
        <script>
            r##"
            (async function() {
                const parts = window.location.pathname.split('/').filter(Boolean);
                const sectionKey = parts[1]; // game-dev or developer
                const res = await fetch('/api/docs');
                if (!res.ok) return;
                const data = await res.json();
                const section = data[sectionKey];
                if (!section) { document.getElementById('section-landing').textContent = 'Section not found'; return; }
                const el = document.getElementById('section-landing');
                el.innerHTML = `
                    <h1 class="text-3xl font-bold mb-2">${section.label}</h1>
                    <p class="text-zinc-400 text-sm mb-8">${section.description}</p>
                    ${section.categories.map(cat => `
                        <div class="mb-6">
                            <h2 class="text-sm font-semibold text-zinc-400 uppercase tracking-wider mb-3">${cat.category}</h2>
                            <div class="space-y-2">
                                ${cat.pages.map(p => `
                                    <a href="/docs/${p.slug}" class="flex items-center gap-3 p-3 bg-surface-card border border-zinc-800 rounded-xl hover:border-zinc-700 transition-all group">
                                        <span class="text-sm font-medium group-hover:text-accent transition-colors">${p.title}</span>
                                        <span class="flex-1"></span>
                                        <i class="ph ph-caret-right text-zinc-600 group-hover:text-zinc-400 transition-colors"></i>
                                    </a>
                                `).join('')}
                            </div>
                        </div>
                    `).join('');
                `;
            })();
            "##
        </script>
    }
}

/// Individual doc page.
#[component]
pub fn DocArticle() -> impl IntoView {
    view! {
        <div class="flex min-h-[calc(100vh-56px)] max-w-[1200px] mx-auto">
            <DocsSidebar />
            <div class="flex-1 min-w-0 px-8 py-10 lg:px-12">
                <article id="doc-content">"Loading..."</article>
            </div>
        </div>
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github-dark.min.css" />
        <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
        <script>
            r##"
            (async function() {
                const path = window.location.pathname.replace('/docs/', '');
                const res = await fetch('/api/docs/' + path);
                const el = document.getElementById('doc-content');
                if (!res.ok) {
                    el.innerHTML = '<h1 class="text-2xl font-bold mb-4">Page not found</h1><p class="text-zinc-400 text-sm">This page hasn\'t been written yet.</p><a href="/docs" class="text-accent text-sm mt-4 inline-block">Back to docs</a>';
                    return;
                }
                const doc = await res.json();
                el.innerHTML = `
                    <div class="flex items-center gap-2 text-xs text-zinc-500 mb-6">
                        <a href="/docs" class="text-accent hover:text-accent-hover">Docs</a>
                        <i class="ph ph-caret-right text-[10px]"></i>
                        <a href="/docs/${doc.section}" class="text-accent hover:text-accent-hover">${doc.section === 'game-dev' ? 'Game Dev' : 'Developer'}</a>
                        <i class="ph ph-caret-right text-[10px]"></i>
                        <span>${doc.category}</span>
                    </div>
                    <h1 class="text-3xl font-bold mb-6">${doc.title}</h1>
                    <div class="doc-body">${doc.content}</div>
                `;
                document.querySelectorAll('.doc-body pre code').forEach(block => hljs.highlightElement(block));
            })();
            "##
        </script>
    }
}

/// Sidebar with search and section-aware navigation.
#[component]
fn DocsSidebar() -> impl IntoView {
    view! {
        <aside class="w-64 shrink-0 border-r border-zinc-800 bg-surface sticky top-14 h-[calc(100vh-56px)] overflow-y-auto hidden lg:block">
            <div class="p-4">
                // Search
                <div class="relative mb-4">
                    <i class="ph ph-magnifying-glass absolute left-2.5 top-1/2 -translate-y-1/2 text-zinc-500 text-sm"></i>
                    <input type="text" id="doc-search" placeholder="Search docs..." oninput="searchDocs(this.value)" class="w-full pl-8 pr-3 py-2 bg-surface-card border border-zinc-800 rounded-lg text-xs text-zinc-50 outline-none focus:border-accent" />
                </div>
                <div id="search-results" class="hidden mb-4"></div>
                // Section tabs
                <div class="flex gap-1 mb-4">
                    <button onclick="switchSection('game-dev')" id="tab-game-dev" class="flex-1 px-2 py-1.5 text-[11px] font-medium rounded-lg bg-accent text-white">"Game Dev"</button>
                    <button onclick="switchSection('developer')" id="tab-developer" class="flex-1 px-2 py-1.5 text-[11px] font-medium rounded-lg bg-surface-card text-zinc-400">"Developer"</button>
                </div>
                <div id="sidebar-nav">"Loading..."</div>
            </div>
        </aside>
        <script>
            r##"
            let sidebarData = null;
            let currentSection = 'game-dev';

            (async function() {
                const path = window.location.pathname;
                if (path.includes('/developer')) currentSection = 'developer';

                const res = await fetch('/api/docs');
                if (!res.ok) return;
                sidebarData = await res.json();
                renderSidebar();
            })();

            function switchSection(section) {
                currentSection = section;
                document.getElementById('tab-game-dev').className = section === 'game-dev' ? 'flex-1 px-2 py-1.5 text-[11px] font-medium rounded-lg bg-accent text-white' : 'flex-1 px-2 py-1.5 text-[11px] font-medium rounded-lg bg-surface-card text-zinc-400';
                document.getElementById('tab-developer').className = section === 'developer' ? 'flex-1 px-2 py-1.5 text-[11px] font-medium rounded-lg bg-accent text-white' : 'flex-1 px-2 py-1.5 text-[11px] font-medium rounded-lg bg-surface-card text-zinc-400';
                renderSidebar();
            }

            function renderSidebar() {
                if (!sidebarData) return;
                const section = sidebarData[currentSection];
                if (!section) return;
                const currentPath = window.location.pathname.replace('/docs/', '');
                const el = document.getElementById('sidebar-nav');
                el.innerHTML = section.categories.map(cat => `
                    <div class="mb-5">
                        <h4 class="text-[11px] font-semibold uppercase tracking-[0.08em] text-zinc-500 mb-2 px-2">${cat.category}</h4>
                        <ul class="flex flex-col gap-px">
                            ${cat.pages.map(p => {
                                const isActive = currentPath === p.slug;
                                return `<li><a href="/docs/${p.slug}" class="block px-2 py-1.5 text-[13px] rounded transition-all ${isActive ? 'bg-accent/10 text-accent' : 'text-zinc-400 hover:text-zinc-50 hover:bg-white/5'}">${p.title}</a></li>`;
                            }).join('')}
                        </ul>
                    </div>
                `).join('');
            }

            let searchTimeout;
            async function searchDocs(query) {
                clearTimeout(searchTimeout);
                const el = document.getElementById('search-results');
                if (!query.trim()) { el.classList.add('hidden'); return; }
                searchTimeout = setTimeout(async () => {
                    const res = await fetch('/api/docs/search?q=' + encodeURIComponent(query));
                    if (!res.ok) return;
                    const results = await res.json();
                    el.classList.remove('hidden');
                    if (!results.length) { el.innerHTML = '<p class="text-xs text-zinc-500 p-2">No results</p>'; return; }
                    el.innerHTML = results.map(r => `
                        <a href="/docs/${r.slug}" class="block px-2 py-1.5 text-[13px] text-zinc-300 hover:text-accent hover:bg-white/5 rounded transition-all">
                            <span class="font-medium">${r.title}</span>
                            <span class="text-[10px] text-zinc-500 ml-1">${r.category}</span>
                        </a>
                    `).join('');
                }, 300);
            }
            "##
        </script>
    }
}
