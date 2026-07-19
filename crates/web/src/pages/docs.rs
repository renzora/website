use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};
use renzora_common::ssr::DocSsr;

/// The highlight.js github-dark theme, self-hosted and inlined so docs code
/// blocks are styled without a render-blocking cross-origin stylesheet.
static HLJS_CSS: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    std::fs::read_to_string("assets/highlight/github-dark.min.css").unwrap_or_default()
});

/// Docs landing (`/docs`), redirects to the default version's portal home.
#[component]
pub fn DocsPage() -> impl IntoView {
    view! {
        <section class="py-20 px-6 text-center">
            <p class="text-zinc-500 text-sm">"Loading documentation\u{2026}"</p>
        </section>
        <script>
            r##"
            (async function() {
                let def = 'r1-alpha6';
                try {
                    const res = await fetch('/api/docs/versions');
                    if (res.ok) { const data = await res.json(); if (data && data.default) def = data.default; }
                } catch (e) {}
                window.location.replace('/docs/' + def);
            })();
            "##
        </script>
    }
}

/// Individual doc page (`/docs/<version>/<slug...>`), or a version's portal home
/// (`/docs/<version>`). Shares the sidebar with the version switcher.
#[component]
pub fn DocArticle() -> impl IntoView {
    let ssr = use_context::<DocSsr>().filter(|d| d.found && d.is_page);
    let head = ssr.clone().map(|d| {
        let title = format!("{}, Renzora Docs", d.title);
        let canonical = format!("https://renzora.com/docs/{}/{}", d.version, d.slug);
        view! {
            <Title text=title />
            <Meta name="description" content=format!("{}, documentation for Renzora, the open-source Bevy editor.", d.title) />
            <Link rel="canonical" href=canonical />
        }
    });
    // Only article pages have code blocks, so only they load the highlighter; the
    // landing shouldn't pay for the 120KB library it has no use for.
    let is_article = ssr.is_some();
    // Server-rendered doc HTML (crawlable); the client script re-renders with the
    // sidebar + syntax highlighting on load.
    let ssr_body = ssr.map(|d| view! { <div class="doc-body" inner_html=d.content_html></div> });
    // Version-landing pages have no server-rendered article, paint a static
    // heading immediately so the LCP is a text element at first render, not the
    // client-rendered landing (which arrives ~2s later). renderLanding() then
    // swaps in the full landing with the same heading (no visible flash).
    let landing = ssr_body.is_none().then(|| view! {
        <h1 class="text-3xl font-bold mb-2">"Renzora Documentation"</h1>
        <p class="text-zinc-400 text-sm mb-8">"Guides, references and the API for the Renzora Bevy editor."</p>
    });
    let default_head = view! {
        <Title text="Renzora Documentation, Bevy Editor Guides & API" />
        <Meta name="description" content="Documentation for Renzora, the open-source Bevy editor: getting started, the scene editor, Lua & Rhai scripting, materials, plugins, physics and cross-platform export." />
    };
    view! {
        {head.is_none().then_some(default_head)}
        {head}
        <div class="flex min-h-[calc(100vh-56px)] max-w-[1200px] mx-auto">
            <DocsSidebar />
            <div class="flex-1 min-w-0 px-8 py-10 lg:px-12">
                <article id="doc-content">{ssr_body}{landing}</article>
            </div>
        </div>
        // Syntax highlighting, self-hosted (no cross-origin dependency), loaded
        // only on article pages that actually contain code. Theme CSS inlined;
        // the library is deferred so it never blocks paint.
        {is_article.then(|| view! {
            <style inner_html=HLJS_CSS.as_str()></style>
            <script defer src="/assets/highlight/highlight.min.js"></script>
        })}
        <script>
            r##"
            function docsNotFound() {
                return '<h1 class="text-2xl font-bold mb-4">Page not found</h1><p class="text-zinc-400 text-sm">This page hasn\'t been written yet.</p><a href="/docs" class="text-accent text-sm mt-4 inline-block">Back to docs</a>';
            }

            function renderLanding(sidebar, version) {
                window.__landingSidebar = sidebar; window.__landingVersion = version;
                const groups = (sidebar.groups || []);
                const el = document.getElementById('doc-content');
                el.innerHTML = `
                    <h1 class="text-3xl font-bold mb-2">${sidebar.label}</h1>
                    <p class="text-zinc-400 text-sm mb-8">${sidebar.description}</p>
                    ${groups.map(group => `
                        <div class="mb-10">
                            <h2 class="text-sm font-bold uppercase tracking-wider text-accent/80 mb-4" style="border:none;padding:0;">${group.group}</h2>
                            ${group.categories.map(cat => `
                                <div class="mb-6">
                                    <h3 class="text-xs font-semibold text-zinc-400 uppercase tracking-wider mb-3">${cat.category}</h3>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
                                        ${cat.pages.map(p => `
                                            <a href="/docs/${version}/${p.slug}" class="flex items-center gap-3 p-3 bg-surface-card border border-zinc-800 rounded-xl hover:border-zinc-700 transition-all group">
                                                <span class="text-sm font-medium group-hover:text-accent transition-colors">${p.title}</span>
                                                <span class="flex-1"></span>
                                                <i class="ph ph-caret-right text-zinc-600 group-hover:text-zinc-400 transition-colors"></i>
                                            </a>
                                        `).join('')}
                                    </div>
                                </div>
                            `).join('')}
                        </div>
                    `).join('')}
                `;
            }

            (async function() {
                const parts = window.location.pathname.split('/').filter(Boolean); // ['docs', version, ...slug]
                const el = document.getElementById('doc-content');
                if (parts.length < 2) { el.innerHTML = docsNotFound(); return; }

                // Resolve version; if the first segment isn't a known version (legacy/short link),
                // redirect to the default version, preserving the rest of the path as the slug.
                let known = [];
                let def = 'r1-alpha6';
                try {
                    const vres = await fetch('/api/docs/versions');
                    if (vres.ok) { const v = await vres.json(); known = (v.versions || []).map(x => x.id); if (v.default) def = v.default; }
                } catch (e) {}
                if (!known.includes(parts[1])) {
                    window.location.replace('/docs/' + def + '/' + parts.slice(1).join('/'));
                    return;
                }

                const version = parts[1];
                const pagePath = parts.slice(2).join('/');

                // Version home: /docs/<version>
                if (!pagePath) {
                    const sres = await fetch('/api/docs/sidebar/' + version);
                    if (!sres.ok) { el.innerHTML = docsNotFound(); return; }
                    renderLanding(await sres.json(), version);
                    return;
                }

                const res = await fetch('/api/docs/page/' + version + '/' + pagePath);
                if (!res.ok) { el.innerHTML = docsNotFound(); return; }
                const doc = await res.json();
                el.innerHTML = `
                    <div class="flex items-center gap-2 text-xs text-zinc-500 mb-6 flex-wrap">
                        <a href="/docs/${version}" class="text-accent hover:text-accent-hover">Docs</a>
                        <i class="ph ph-caret-right text-[10px]"></i>
                        <span>${doc.group}</span>
                        <i class="ph ph-caret-right text-[10px]"></i>
                        <span>${doc.category}</span>
                        <span class="ml-2 px-1.5 py-0.5 rounded bg-surface-card border border-zinc-800 text-[10px] text-zinc-400">${version}</span>
                    </div>
                    <div class="doc-body">${doc.content}</div>
                `;

                // Highlight code and add copy buttons
                document.querySelectorAll('.doc-body pre').forEach(pre => {
                    const code = pre.querySelector('code');
                    if (!code) return;
                    const langClass = [...code.classList].find(c => c.startsWith('language-'));
                    const lang = langClass ? langClass.replace('language-', '') : '';
                    if (window.hljs) hljs.highlightElement(code);
                    const wrapper = document.createElement('div');
                    wrapper.className = 'code-block-wrapper';
                    const header = document.createElement('div');
                    header.className = 'code-block-header';
                    header.innerHTML = `
                        <span class="code-lang">${lang || 'code'}</span>
                        <button class="code-copy-btn" onclick="copyCode(this)">
                            <i class="ph ph-copy"></i> Copy
                        </button>
                    `;
                    pre.parentNode.insertBefore(wrapper, pre);
                    wrapper.appendChild(header);
                    wrapper.appendChild(pre);
                });

            })();

            // Open any doc image in a lightbox. Event delegation on document so it
            // works for images injected at any time and survives SPA re-renders.
            if (!window.__docLightboxBound) {
                window.__docLightboxBound = true;
                document.addEventListener('click', (e) => {
                    const t = e.target;
                    const img = (t && t.closest) ? t.closest('.doc-body img') : null;
                    if (img) { e.preventDefault(); openLightbox(img.currentSrc || img.src, img.alt); }
                });
            }

            function copyCode(btn) {
                const wrapper = btn.closest('.code-block-wrapper');
                const code = wrapper.querySelector('code');
                const text = code.textContent;
                navigator.clipboard.writeText(text).then(() => {
                    btn.innerHTML = '<i class="ph ph-check"></i> Copied!';
                    btn.classList.add('copied');
                    setTimeout(() => {
                        btn.innerHTML = '<i class="ph ph-copy"></i> Copy';
                        btn.classList.remove('copied');
                    }, 2000);
                });
            }

            function openLightbox(src, alt) {
                let ov = document.getElementById('doc-lightbox');
                if (!ov) {
                    ov = document.createElement('div');
                    ov.id = 'doc-lightbox';
                    ov.className = 'doc-lightbox';
                    ov.innerHTML = '<button class="doc-lightbox-close" aria-label="Close">&times;</button><img alt="" /><div class="doc-lightbox-cap"></div>';
                    ov.addEventListener('click', (e) => { if (e.target === ov || e.target.classList.contains('doc-lightbox-close')) closeLightbox(); });
                    document.addEventListener('keydown', (e) => { if (e.key === 'Escape') closeLightbox(); });
                    document.body.appendChild(ov);
                }
                ov.querySelector('img').src = src;
                ov.querySelector('.doc-lightbox-cap').textContent = alt || '';
                ov.classList.add('open');
                document.body.style.overflow = 'hidden';
            }
            function closeLightbox() {
                const ov = document.getElementById('doc-lightbox');
                if (ov) { ov.classList.remove('open'); document.body.style.overflow = ''; }
            }
            "##
        </script>

        <style>
            r#"
            .code-block-wrapper {
                position: relative;
                margin-bottom: 1.25rem;
                border-radius: 10px;
                border: 1px solid #27272a;
                overflow: hidden;
                background: #0d0d0f;
            }
            .code-block-header {
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: 6px 12px;
                background: #18181b;
                border-bottom: 1px solid #27272a;
            }
            .code-lang {
                font-size: 11px;
                font-weight: 500;
                color: #71717a;
                text-transform: uppercase;
                letter-spacing: 0.05em;
                font-family: 'Cascadia Code', 'Fira Code', monospace;
            }
            .code-copy-btn {
                display: inline-flex;
                align-items: center;
                gap: 4px;
                padding: 3px 10px;
                border-radius: 6px;
                font-size: 11px;
                font-weight: 500;
                color: #a1a1aa;
                background: transparent;
                border: 1px solid transparent;
                cursor: pointer;
                transition: all 0.15s;
            }
            .code-copy-btn:hover {
                color: #fafafa;
                background: rgba(255,255,255,0.05);
                border-color: #3f3f46;
            }
            .code-copy-btn.copied {
                color: #4ade80;
            }
            .code-block-wrapper pre {
                margin: 0 !important;
                border: none !important;
                border-radius: 0 !important;
                background: #0d0d0f !important;
                padding: 1rem !important;
            }
            .code-block-wrapper pre code {
                font-size: 13px !important;
                line-height: 1.7 !important;
                font-family: 'Cascadia Code', 'Fira Code', monospace !important;
                tab-size: 4;
            }

            /* Override highlight.js background */
            .code-block-wrapper .hljs {
                background: transparent !important;
                padding: 0 !important;
            }

            /* Typography */
            .doc-body h1 { font-size: 1.75rem; font-weight: 800; margin-bottom: 1rem; margin-top: 2rem; color: #fafafa; letter-spacing: -0.02em; }
            .doc-body h2 { font-size: 1.35rem; font-weight: 700; margin-bottom: 0.75rem; margin-top: 1.75rem; color: #fafafa; letter-spacing: -0.01em; padding-bottom: 0.5rem; border-bottom: 1px solid #27272a; }
            .doc-body h3 { font-size: 1.1rem; font-weight: 600; margin-bottom: 0.5rem; margin-top: 1.5rem; color: #e4e4e7; }
            .doc-body h4 { font-size: 0.95rem; font-weight: 600; margin-bottom: 0.5rem; margin-top: 1.25rem; color: #d4d4d8; }
            .doc-body p { color: #a1a1aa; font-size: 0.875rem; line-height: 1.7; margin-bottom: 1rem; }
            .doc-body ul { list-style-type: disc; padding-left: 1.5rem; margin-bottom: 1rem; color: #a1a1aa; }
            .doc-body ol { list-style-type: decimal; padding-left: 1.5rem; margin-bottom: 1rem; color: #a1a1aa; }
            .doc-body li { font-size: 0.875rem; line-height: 1.7; margin-bottom: 0.25rem; }
            .doc-body li ul { margin-top: 0.25rem; margin-bottom: 0.25rem; }
            .doc-body a { color: #818cf8; text-decoration: none; transition: color 0.15s; }
            .doc-body a:hover { color: #a5b4fc; text-decoration: underline; }
            .doc-body blockquote { border-left: 3px solid #3f3f46; padding: 0.5rem 1rem; margin: 1rem 0; background: rgba(255,255,255,0.02); border-radius: 0 8px 8px 0; }
            .doc-body blockquote p { color: #71717a; margin-bottom: 0; }
            .doc-body strong { color: #e4e4e7; font-weight: 600; }
            .doc-body em { color: #a1a1aa; font-style: italic; }
            .doc-body hr { border: none; border-top: 1px solid #27272a; margin: 2rem 0; }
            .doc-body table { width: 100%; border-collapse: collapse; margin-bottom: 1rem; font-size: 0.8125rem; }
            .doc-body th { text-align: left; padding: 0.5rem 0.75rem; border-bottom: 2px solid #27272a; color: #d4d4d8; font-weight: 600; }
            .doc-body td { padding: 0.5rem 0.75rem; border-bottom: 1px solid #1e1e22; color: #a1a1aa; }
            .doc-body tr:hover td { background: rgba(255,255,255,0.02); }
            .doc-body img { max-width: 100%; border-radius: 8px; margin: 1rem 0; border: 1px solid #27272a; }
            .doc-body h1:first-child { margin-top: 0; }

            /* Inline code styling */
            .doc-body code:not(pre code) {
                background: #1e1e22;
                padding: 2px 6px;
                border-radius: 4px;
                font-size: 0.8125rem;
                font-family: 'Cascadia Code', 'Fira Code', monospace;
                color: #c4b5fd;
                border: 1px solid #27272a;
            }

            .code-block-wrapper pre code {
                counter-reset: line;
            }

            /* Clickable images + lightbox */
            .doc-body img { cursor: zoom-in; transition: border-color 0.15s; }
            .doc-body img:hover { border-color: #6366f1; }
            .doc-lightbox { position: fixed; inset: 0; z-index: 100; display: none; align-items: center; justify-content: center; background: rgba(0,0,0,0.85); backdrop-filter: blur(4px); padding: 2rem; }
            .doc-lightbox.open { display: flex; }
            .doc-lightbox img { max-width: 95vw; max-height: 88vh; border-radius: 10px; border: 1px solid #3f3f46; box-shadow: 0 20px 60px rgba(0,0,0,0.6); cursor: zoom-out; }
            .doc-lightbox-cap { position: absolute; bottom: 1.25rem; left: 0; right: 0; text-align: center; color: #a1a1aa; font-size: 0.8125rem; padding: 0 2rem; }
            .doc-lightbox-close { position: absolute; top: 1rem; right: 1.25rem; width: 40px; height: 40px; border-radius: 9999px; background: rgba(255,255,255,0.08); border: 1px solid #3f3f46; color: #fafafa; font-size: 1.5rem; line-height: 1; cursor: pointer; }
            .doc-lightbox-close:hover { background: rgba(255,255,255,0.15); }
            "#
        </style>
    }
}

/// Sidebar with search, a version switcher, and single-portal group/category nav.
#[component]
fn DocsSidebar() -> impl IntoView {
    view! {
        <aside class="w-64 shrink-0 border-r border-zinc-800 bg-surface sticky top-14 h-[calc(100vh-56px)] overflow-y-auto hidden lg:block">
            <div class="p-4">
                // Version switcher
                <div class="mb-4">
                    <label class="block text-[10px] font-semibold uppercase tracking-[0.08em] text-zinc-500 mb-1.5">"Version"</label>
                    <select id="version-select" onchange="switchVersion(this.value)" class="w-full px-2.5 py-2 bg-surface-card border border-zinc-800 rounded-lg text-xs text-zinc-50 outline-none focus:border-accent cursor-pointer">
                        <option>"\u{2026}"</option>
                    </select>
                </div>
                // Search
                <div class="relative mb-4">
                    <i class="ph ph-magnifying-glass absolute left-2.5 top-1/2 -translate-y-1/2 text-zinc-500 text-sm"></i>
                    <input type="text" id="doc-search" placeholder="Search docs..." oninput="searchDocs(this.value)" class="w-full pl-8 pr-3 py-2 bg-surface-card border border-zinc-800 rounded-lg text-xs text-zinc-50 outline-none focus:border-accent" />
                </div>
                <div id="search-results" class="hidden mb-4"></div>
                <div id="sidebar-nav">"Loading..."</div>
            </div>
        </aside>
        <script>
            r##"
            let sidebarData = null;
            let docVersions = null;
            let currentVersion = null;

            (async function() {
                const parts = window.location.pathname.split('/').filter(Boolean);
                try {
                    const vres = await fetch('/api/docs/versions');
                    docVersions = vres.ok ? await vres.json() : { default: 'r1-alpha6', versions: [] };
                } catch (e) { docVersions = { default: 'r1-alpha6', versions: [] }; }
                const known = (docVersions.versions || []).map(v => v.id);
                currentVersion = (parts[1] && known.includes(parts[1])) ? parts[1] : docVersions.default;
                renderVersionSelect();
                const res = await fetch('/api/docs/sidebar/' + currentVersion);
                if (!res.ok) return;
                sidebarData = await res.json();
                renderSidebar();
            })();

            function renderVersionSelect() {
                const sel = document.getElementById('version-select');
                if (!sel || !docVersions) return;
                sel.innerHTML = (docVersions.versions || []).map(v =>
                    `<option value="${v.id}" ${v.id === currentVersion ? 'selected' : ''}>${v.label}${v.status ? ' · ' + v.status : ''}</option>`
                ).join('');
            }

            function switchVersion(version) {
                const parts = window.location.pathname.split('/').filter(Boolean);
                const pagePath = parts.slice(2).join('/');
                window.location.href = pagePath ? `/docs/${version}/${pagePath}` : `/docs/${version}`;
            }

            function renderSidebar() {
                if (!sidebarData) return;
                const parts = window.location.pathname.split('/').filter(Boolean);
                const currentPath = parts.slice(2).join('/');
                const el = document.getElementById('sidebar-nav');
                el.innerHTML = (sidebarData.groups || []).map(group => `
                    <div class="mb-6">
                        <div class="text-[10px] font-bold uppercase tracking-[0.12em] text-accent/80 mb-3 px-2">${group.group}</div>
                        ${group.categories.map(cat => `
                            <div class="mb-4">
                                <h4 class="text-[11px] font-semibold uppercase tracking-[0.08em] text-zinc-500 mb-2 px-2">${cat.category}</h4>
                                <ul class="flex flex-col gap-px">
                                    ${cat.pages.map(p => {
                                        const isActive = currentPath === p.slug;
                                        return `<li><a href="/docs/${currentVersion}/${p.slug}" class="block px-2 py-1.5 text-[13px] rounded transition-all ${isActive ? 'bg-accent/10 text-accent' : 'text-zinc-400 hover:text-zinc-50 hover:bg-white/5'}">${p.title}</a></li>`;
                                    }).join('')}
                                </ul>
                            </div>
                        `).join('')}
                    </div>
                `).join('');
            }

            let searchTimeout;
            async function searchDocs(query) {
                clearTimeout(searchTimeout);
                const el = document.getElementById('search-results');
                if (!query.trim()) { el.classList.add('hidden'); return; }
                searchTimeout = setTimeout(async () => {
                    const res = await fetch('/api/docs/search/' + currentVersion + '?q=' + encodeURIComponent(query));
                    if (!res.ok) return;
                    const results = await res.json();
                    el.classList.remove('hidden');
                    if (!results.length) { el.innerHTML = '<p class="text-xs text-zinc-500 p-2">No results</p>'; return; }
                    el.innerHTML = results.map(r => `
                        <a href="/docs/${currentVersion}/${r.slug}" class="block px-2 py-1.5 text-[13px] text-zinc-300 hover:text-accent hover:bg-white/5 rounded transition-all">
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
