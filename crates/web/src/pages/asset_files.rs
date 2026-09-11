use leptos::prelude::*;

/// The highlight.js theme, inlined so the source viewer is styled without a
/// render-blocking cross-origin stylesheet (same approach as the docs portal).
static HLJS_CSS: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    std::fs::read_to_string("assets/highlight/github-dark.min.css").unwrap_or_default()
});

/// Repository-style browser for an asset's uploaded files
/// (`/marketplace/asset/:slug/files/*path`).
///
/// The tree is public; file contents need ownership, except markdown and
/// licence files, which are the asset's documentation and render for everyone.
#[component]
pub fn AssetFilesPage() -> impl IntoView {
    view! {
        <style inner_html=HLJS_CSS.as_str()></style>
        <script defer src="/assets/highlight/highlight.min.js"></script>
        <section class="py-8 px-6">
            <div class="max-w-7xl mx-auto" id="files-root">
                <div class="text-center py-20">
                    <div class="inline-block animate-spin w-6 h-6 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                </div>
            </div>
        </section>
        <script>
            r##"
            // ── State ─────────────────────────────────────────────────────
            let A = null;          // asset detail
            let TREE = [];         // flat entries for the selected release
            let RELEASE = null;    // the release being browsed
            let RELEASES = [];     // every release, for the switcher
            let HAS_ACCESS = false;
            let README = null;     // rendered README of the release root

            const esc = s => String(s ?? '').replace(/[&<>"']/g, c =>
                ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));

            function fmtSize(bytes) {
                if (!bytes) return '0 B';
                if (bytes >= 1e9) return (bytes / 1e9).toFixed(1) + ' GB';
                if (bytes >= 1e6) return (bytes / 1e6).toFixed(1) + ' MB';
                if (bytes >= 1e3) return (bytes / 1e3).toFixed(1) + ' KB';
                return bytes + ' B';
            }

            function token() {
                return document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
            }

            function authHeaders() {
                const t = token();
                return t ? { 'Authorization': 'Bearer ' + t } : {};
            }

            // The release currently being browsed, as a query suffix. Only
            // pinned when it isn't the current release, so normal links stay
            // clean and shareable.
            function relQuery(prefix) {
                if (!RELEASE || RELEASE.is_current) return '';
                return (prefix || '?') + 'release=' + encodeURIComponent(RELEASE.version);
            }

            function filesUrl(path) {
                return '/marketplace/asset/' + A.slug + '/files' + (path ? '/' + path : '') + relQuery('?');
            }

            // ── URL parsing ───────────────────────────────────────────────
            // /marketplace/asset/<slug>/files/<path...>
            function parseLocation() {
                const parts = window.location.pathname.split('/').filter(Boolean);
                const i = parts.indexOf('files');
                const slug = parts[2] || '';
                const path = i >= 0 ? parts.slice(i + 1).map(decodeURIComponent).join('/') : '';
                const release = new URLSearchParams(window.location.search).get('release');
                return { slug, path, release };
            }

            // ── Icons ─────────────────────────────────────────────────────
            function iconFor(entry) {
                if (entry.kind === 'dir') return 'ph-folder-simple text-accent';
                const n = entry.name.toLowerCase();
                if (entry.is_doc) return 'ph-book-open text-cyan-400';
                if (entry.mime_type.startsWith('image/')) return 'ph-image text-purple-400';
                if (entry.mime_type.startsWith('audio/')) return 'ph-music-note text-pink-400';
                if (entry.mime_type.startsWith('video/')) return 'ph-video text-pink-400';
                if (/\.(lua|rhai|rs|js|ts|py|wgsl|glsl|json|toml|ron|yaml|yml)$/.test(n))
                    return 'ph-file-code text-zinc-500';
                return 'ph-file text-zinc-600';
            }

            // ── Tree helpers ──────────────────────────────────────────────
            function childrenOf(dir) {
                const prefix = dir ? dir + '/' : '';
                const depth = dir ? dir.split('/').length : 0;
                return TREE.filter(e =>
                    e.path.startsWith(prefix) && e.path.split('/').length === depth + 1);
            }

            function entryAt(path) {
                return TREE.find(e => e.path === path) || null;
            }

            function isDir(path) {
                if (!path) return true;
                const e = entryAt(path);
                return !!e && e.kind === 'dir';
            }

            // Every markdown file, for the docs sidebar.
            function docEntries() {
                return TREE.filter(e => e.kind === 'file' && /\.(md|markdown)$/i.test(e.name));
            }

            // ── Rendering ─────────────────────────────────────────────────
            function breadcrumbs(path) {
                const segs = path ? path.split('/') : [];
                let acc = '';
                const crumbs = [
                    `<a href="${filesUrl('')}" class="nav-link text-accent hover:underline">${esc(A.name)}</a>`
                ];
                segs.forEach((s, i) => {
                    acc = acc ? acc + '/' + s : s;
                    const last = i === segs.length - 1;
                    crumbs.push(last
                        ? `<span class="text-zinc-300">${esc(s)}</span>`
                        : `<a href="${filesUrl(acc)}" class="nav-link text-accent hover:underline">${esc(s)}</a>`);
                });
                return `<div class="flex items-center gap-1.5 text-sm flex-wrap">${crumbs.join('<span class="text-zinc-600">/</span>')}</div>`;
            }

            function releaseSwitcher() {
                if (RELEASES.length < 2) {
                    return `<span class="px-2.5 py-1 rounded-lg bg-white/[0.03] border border-zinc-800/50 text-xs text-zinc-400"><i class="ph ph-tag"></i> v${esc(RELEASE.version)}</span>`;
                }
                const opts = RELEASES.map(r =>
                    `<option value="${esc(r.version)}" ${r.id === RELEASE.id ? 'selected' : ''}>v${esc(r.version)}${r.is_current ? ' (latest)' : ''}</option>`
                ).join('');
                return `<select onchange="switchRelease(this.value)" class="px-2.5 py-1 rounded-lg bg-white/[0.03] border border-zinc-800/50 text-xs text-zinc-300 outline-none focus:border-accent/50 cursor-pointer">${opts}</select>`;
            }

            function docsSidebar(activePath) {
                const docs = docEntries();
                if (!docs.length) return '';

                // Group by directory so a docs/ folder reads as a section.
                const groups = new Map();
                docs.forEach(d => {
                    const dir = d.path.includes('/') ? d.path.slice(0, d.path.lastIndexOf('/')) : '';
                    if (!groups.has(dir)) groups.set(dir, []);
                    groups.get(dir).push(d);
                });
                // Root docs first, then folders alphabetically.
                const keys = [...groups.keys()].sort((a, b) => (a === '' ? -1 : b === '' ? 1 : a.localeCompare(b)));

                const body = keys.map(dir => {
                    const items = groups.get(dir).map(d => {
                        const active = d.path === activePath;
                        const label = d.name.replace(/\.(md|markdown)$/i, '').replace(/[-_]/g, ' ');
                        return `<a href="${filesUrl(d.path)}" class="nav-link block px-3 py-1.5 rounded-lg text-[13px] transition-colors ${active ? 'bg-accent/10 text-accent font-medium' : 'text-zinc-400 hover:text-zinc-200 hover:bg-white/[0.03]'}">${esc(label)}</a>`;
                    }).join('');
                    const heading = dir
                        ? `<div class="px-3 mt-4 mb-1.5 text-[10px] font-semibold uppercase tracking-[0.12em] text-zinc-600">${esc(dir)}</div>`
                        : '';
                    return heading + items;
                }).join('');

                return `
                    <aside class="hidden lg:block w-56 shrink-0">
                        <div class="sticky top-20">
                            <div class="px-3 mb-2 text-[10px] font-semibold uppercase tracking-[0.12em] text-zinc-500">Documentation</div>
                            ${body}
                        </div>
                    </aside>`;
            }

            function listing(dir) {
                const kids = childrenOf(dir);
                if (!kids.length) {
                    return '<p class="text-sm text-zinc-600 px-4 py-6">This folder is empty.</p>';
                }
                const up = dir
                    ? `<a href="${filesUrl(dir.includes('/') ? dir.slice(0, dir.lastIndexOf('/')) : '')}" class="nav-link flex items-center gap-2.5 px-4 py-2.5 hover:bg-white/[0.02] transition-colors border-b border-zinc-800/50 text-sm text-zinc-500"><i class="ph ph-arrow-bend-left-up"></i>..</a>`
                    : '';

                const rows = kids.map(e => `
                    <a href="${filesUrl(e.path)}" class="nav-link flex items-center gap-2.5 px-4 py-2.5 hover:bg-white/[0.02] transition-colors border-b border-zinc-800/50 last:border-0 group">
                        <i class="ph ${iconFor(e)}"></i>
                        <span class="flex-1 text-sm text-zinc-300 group-hover:text-accent transition-colors truncate">${esc(e.name)}</span>
                        ${e.is_doc ? '<span class="px-1.5 py-0.5 rounded bg-cyan-500/10 border border-cyan-500/20 text-[10px] text-cyan-400">docs</span>' : ''}
                        ${(!HAS_ACCESS && e.kind === 'file' && !e.is_doc) ? '<i class="ph ph-lock-simple text-xs text-zinc-600" title="Purchase to view"></i>' : ''}
                        <span class="text-xs text-zinc-600 tabular-nums w-20 text-right">${e.kind === 'dir' ? '' : fmtSize(e.size)}</span>
                    </a>`).join('');

                return `<div class="border border-zinc-800/50 rounded-2xl overflow-hidden bg-white/[0.01]">${up}${rows}</div>`;
            }

            function outlineNav(outline) {
                if (!outline || outline.length < 2) return '';
                const items = outline.filter(h => h.level <= 3).map(h =>
                    `<a href="#${esc(h.anchor)}" class="block py-1 pl-${h.level === 3 ? '6' : '3'} text-[12px] text-zinc-500 hover:text-accent transition-colors border-l border-zinc-800 -ml-px" style="padding-left:${h.level === 3 ? '1.5rem' : '0.75rem'}">${esc(h.text)}</a>`
                ).join('');
                return `
                    <aside class="hidden xl:block w-52 shrink-0">
                        <div class="sticky top-20">
                            <div class="text-[10px] font-semibold uppercase tracking-[0.12em] text-zinc-500 mb-3">On this page</div>
                            <nav class="border-l border-zinc-800 flex flex-col">${items}</nav>
                        </div>
                    </aside>`;
            }

            function readmeCard(doc) {
                if (!doc) return '';
                return `
                    <div class="mt-6 border border-zinc-800/50 rounded-2xl overflow-hidden bg-white/[0.01]">
                        <div class="flex items-center gap-2 px-4 py-2.5 border-b border-zinc-800/50 bg-white/[0.02]">
                            <i class="ph ph-book-open text-cyan-400"></i>
                            <span class="text-xs font-medium text-zinc-300">${esc(doc.path)}</span>
                        </div>
                        <div class="doc-body px-6 py-6">${doc.html}</div>
                    </div>`;
            }

            // Source view with line numbers.
            function codeBlock(view) {
                const lines = (view.content || '').split('\n');
                const lang = view.language ? ' class="language-' + esc(view.language) + '"' : '';
                const body = lines.map((l, i) =>
                    `<span class="ln">${i + 1}</span>${esc(l)}`).join('\n');
                return `
                    <div class="code-view">
                        <pre><code${lang}>${body}</code></pre>
                    </div>
                    ${view.truncated ? '<p class="text-xs text-amber-400/80 mt-2"><i class="ph ph-warning"></i> File is too large to show in full — download it to see the rest.</p>' : ''}`;
            }

            function lockedCard(view) {
                const price = A.price_credits.toLocaleString();
                return `
                    <div class="border border-zinc-800/50 rounded-2xl bg-white/[0.01] px-6 py-14 text-center">
                        <i class="ph ph-lock-simple text-3xl text-zinc-700"></i>
                        <p class="text-sm text-zinc-400 mt-3">This file is part of a paid asset.</p>
                        <p class="text-xs text-zinc-600 mt-1">${esc(view.name)} · ${fmtSize(view.size)}</p>
                        <a href="/marketplace/asset/${A.slug}" class="inline-flex items-center gap-2 mt-5 px-5 py-2.5 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all">
                            <i class="ph ph-shopping-cart"></i> Buy for ${price} credits
                        </a>
                        <p class="text-xs text-zinc-600 mt-4">Documentation and licence files are readable without purchasing.</p>
                    </div>`;
            }

            function fileHeader(view) {
                const raw = '/api/marketplace/' + A.id + '/raw?path=' + encodeURIComponent(view.path) + relQuery('&');
                return `
                    <div class="flex items-center gap-2 px-4 py-2.5 border border-zinc-800/50 rounded-t-2xl bg-white/[0.02] -mb-px">
                        <i class="ph ${view.kind === 'markdown' ? 'ph-book-open text-cyan-400' : 'ph-file-code text-zinc-500'}"></i>
                        <span class="text-xs font-medium text-zinc-300">${esc(view.name)}</span>
                        <span class="text-xs text-zinc-600">${fmtSize(view.size)}</span>
                        <span class="flex-1"></span>
                        ${view.kind === 'locked' ? '' : `<a href="${raw}" download class="text-xs text-zinc-500 hover:text-accent transition-colors"><i class="ph ph-download-simple"></i> Raw</a>`}
                    </div>`;
            }

            async function renderFile(path) {
                const url = '/api/marketplace/' + A.id + '/file?path=' + encodeURIComponent(path) + relQuery('&');
                const res = await fetch(url, { headers: authHeaders() });
                if (!res.ok) return '<p class="text-sm text-zinc-500 py-10 text-center">File not found in this release.</p>';
                const v = await res.json();

                let body;
                if (v.kind === 'markdown') {
                    body = `<div class="border border-zinc-800/50 rounded-b-2xl bg-white/[0.01] px-6 py-6"><div class="doc-body">${v.html}</div></div>`;
                } else if (v.kind === 'locked') {
                    return fileHeader(v) + lockedCard(v);
                } else if (v.kind === 'text') {
                    body = codeBlock(v);
                } else if (v.kind === 'image') {
                    body = `<div class="border border-zinc-800/50 rounded-b-2xl bg-white/[0.01] p-6 text-center"><img src="${esc(v.download_url)}" alt="${esc(v.name)}" class="max-w-full inline-block rounded-lg" /></div>`;
                } else {
                    body = `
                        <div class="border border-zinc-800/50 rounded-b-2xl bg-white/[0.01] px-6 py-14 text-center">
                            <i class="ph ph-file text-3xl text-zinc-700"></i>
                            <p class="text-sm text-zinc-400 mt-3">${esc(v.name)}</p>
                            <p class="text-xs text-zinc-600 mt-1">${fmtSize(v.size)} · ${esc(v.mime_type)}</p>
                            ${v.download_url ? `<a href="${esc(v.download_url)}" class="inline-flex items-center gap-2 mt-4 px-4 py-2 rounded-xl text-sm bg-white/[0.05] text-zinc-300 hover:bg-white/[0.08] transition-colors"><i class="ph ph-download-simple"></i> Download</a>` : ''}
                        </div>`;
                }
                window.__outline = v.outline || [];
                return fileHeader(v) + body;
            }

            // ── Page render ───────────────────────────────────────────────
            async function render() {
                const { path } = parseLocation();
                const root = document.getElementById('files-root');
                const dir = isDir(path);
                window.__outline = [];

                let main;
                if (dir) {
                    // A directory's own README renders below its listing, the
                    // way a repository front page does.
                    const readmeHere = path
                        ? TREE.find(e => e.kind === 'file' && e.path.toLowerCase() === (path + '/readme.md').toLowerCase())
                        : null;
                    let doc = path ? null : README;
                    if (readmeHere) {
                        const r = await fetch('/api/marketplace/' + A.id + '/file?path=' + encodeURIComponent(readmeHere.path) + relQuery('&'), { headers: authHeaders() });
                        if (r.ok) {
                            const v = await r.json();
                            doc = { path: v.path, html: v.html, outline: v.outline };
                        }
                    }
                    if (doc) window.__outline = doc.outline || [];
                    main = listing(path) + readmeCard(doc);
                } else {
                    main = await renderFile(path);
                }

                const fileCount = TREE.filter(e => e.kind === 'file').length;
                root.innerHTML = `
                    <a href="/marketplace/asset/${A.slug}" class="inline-flex items-center gap-1.5 text-sm text-zinc-500 hover:text-zinc-300 transition-colors mb-5">
                        <i class="ph ph-arrow-left"></i> Back to ${esc(A.name)}
                    </a>
                    <div class="flex items-center justify-between gap-4 mb-5 flex-wrap">
                        ${breadcrumbs(path)}
                        <div class="flex items-center gap-2">
                            ${releaseSwitcher()}
                            <span class="text-xs text-zinc-600">${fileCount} file${fileCount === 1 ? '' : 's'}</span>
                        </div>
                    </div>
                    ${(!HAS_ACCESS && A.price_credits > 0) ? `
                        <div class="mb-5 flex items-center gap-2 px-4 py-2.5 rounded-xl bg-amber-500/[0.06] border border-amber-500/20 text-xs text-amber-300/90">
                            <i class="ph ph-info"></i>
                            You're browsing the file list of a paid asset. Documentation and licence files are readable; the rest unlocks after purchase.
                        </div>` : ''}
                    <div class="flex gap-8 items-start">
                        ${docsSidebar(dir ? null : path)}
                        <div class="flex-1 min-w-0">${main}</div>
                        ${outlineNav(window.__outline)}
                    </div>
                `;

                if (window.hljs) {
                    root.querySelectorAll('pre code').forEach(b => window.hljs.highlightElement(b));
                }
                markLockedImages(root);
                bindSoftNav();
            }

            // An image a README points at lives inside the archive, so for a paid
            // asset it 403s until you own it. Say so, rather than leaving a
            // broken-image icon in the middle of the docs.
            function markLockedImages(root) {
                root.querySelectorAll('.doc-body img').forEach(img => {
                    img.addEventListener('error', () => {
                        const note = document.createElement('span');
                        note.className = 'inline-flex items-center gap-1.5 px-3 py-2 my-2 rounded-lg bg-white/[0.02] border border-zinc-800/50 text-xs text-zinc-500';
                        note.innerHTML = '<i class="ph ph-lock-simple"></i>' +
                            (img.alt ? esc(img.alt) + ' — ' : '') + 'image available after purchase';
                        img.replaceWith(note);
                    }, { once: true });
                });
            }

            // Navigating the tree re-renders from the already-loaded data
            // instead of reloading the page.
            function bindSoftNav() {
                document.querySelectorAll('#files-root a.nav-link').forEach(a => {
                    a.addEventListener('click', ev => {
                        if (ev.metaKey || ev.ctrlKey || ev.shiftKey || ev.button !== 0) return;
                        ev.preventDefault();
                        history.pushState({}, '', a.getAttribute('href'));
                        render();
                        window.scrollTo({ top: 0 });
                    });
                });
            }

            window.switchRelease = function(version) {
                const { path } = parseLocation();
                window.location.href = '/marketplace/asset/' + A.slug + '/files' +
                    (path ? '/' + path : '') + '?release=' + encodeURIComponent(version);
            };

            window.addEventListener('popstate', () => { if (A) render(); });

            // ── Boot ──────────────────────────────────────────────────────
            (async function() {
                const { slug, release } = parseLocation();
                const root = document.getElementById('files-root');

                const dRes = await fetch('/api/marketplace/detail/' + slug, { headers: authHeaders() });
                if (!dRes.ok) {
                    root.innerHTML = '<p class="text-center text-zinc-500 py-20">Asset not found.</p>';
                    return;
                }
                A = await dRes.json();

                const q = release ? '?release=' + encodeURIComponent(release) : '';
                const [tRes, rRes] = await Promise.all([
                    fetch('/api/marketplace/' + A.id + '/tree' + q, { headers: authHeaders() }),
                    fetch('/api/marketplace/' + A.id + '/releases'),
                ]);

                if (!tRes.ok) {
                    root.innerHTML = `
                        <div class="text-center py-20">
                            <i class="ph ph-folder-simple-dashed text-4xl text-zinc-700"></i>
                            <p class="text-sm text-zinc-500 mt-3">This asset has no browsable files.</p>
                            <a href="/marketplace/asset/${esc(slug)}" class="text-accent text-sm mt-3 inline-block">Back to the asset</a>
                        </div>`;
                    return;
                }

                const tree = await tRes.json();
                TREE = tree.entries || [];
                RELEASE = tree.release;
                README = tree.readme || null;
                HAS_ACCESS = !!tree.has_access;
                RELEASES = rRes.ok ? await rRes.json() : [RELEASE];

                render();
            })();
            "##
        </script>
    }
}
