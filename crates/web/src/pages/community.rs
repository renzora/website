use leptos::prelude::*;

#[component]
pub fn CommunityPage() -> impl IntoView {
    view! {
        <section class="py-12 px-6">
            <div class="max-w-[1000px] mx-auto">
                <div class="flex justify-between items-start mb-8">
                    <div>
                        <h1 class="text-3xl font-bold">"Community"</h1>
                        <p class="text-zinc-400 mt-2 text-sm">"Tutorials, guides, and resources from the Renzora community."</p>
                    </div>
                    <a href="/community/write" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                        <i class="ph ph-pen-nib text-base"></i>"Write an Article"
                    </a>
                </div>

                // Tags filter
                <div class="flex gap-2 flex-wrap mb-8" id="tag-filters">
                    <button onclick="filterArticles('all')" class="article-tag px-3.5 py-1.5 rounded-full text-xs font-medium bg-accent border border-accent text-white transition-all" data-tag="all">"All"</button>
                    <button onclick="filterArticles('tutorial')" class="article-tag px-3.5 py-1.5 rounded-full text-xs font-medium bg-transparent border border-zinc-800 text-zinc-400 hover:bg-accent hover:border-accent hover:text-white transition-all" data-tag="tutorial">"Tutorial"</button>
                    <button onclick="filterArticles('guide')" class="article-tag px-3.5 py-1.5 rounded-full text-xs font-medium bg-transparent border border-zinc-800 text-zinc-400 hover:bg-accent hover:border-accent hover:text-white transition-all" data-tag="guide">"Guide"</button>
                    <button onclick="filterArticles('tip')" class="article-tag px-3.5 py-1.5 rounded-full text-xs font-medium bg-transparent border border-zinc-800 text-zinc-400 hover:bg-accent hover:border-accent hover:text-white transition-all" data-tag="tip">"Tip"</button>
                    <button onclick="filterArticles('showcase')" class="article-tag px-3.5 py-1.5 rounded-full text-xs font-medium bg-transparent border border-zinc-800 text-zinc-400 hover:bg-accent hover:border-accent hover:text-white transition-all" data-tag="showcase">"Showcase"</button>
                    <button onclick="filterArticles('resource')" class="article-tag px-3.5 py-1.5 rounded-full text-xs font-medium bg-transparent border border-zinc-800 text-zinc-400 hover:bg-accent hover:border-accent hover:text-white transition-all" data-tag="resource">"Resource"</button>
                </div>

                <div id="articles-list" class="space-y-4">
                    <div class="text-center py-12"><div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-accent rounded-full"></div></div>
                </div>

                <div id="articles-pagination" class="flex justify-center gap-2 mt-6"></div>
            </div>
        </section>
        <script>
            r##"
            let currentTag = 'all';
            let currentPage = 1;

            async function loadArticles(tag, page) {
                currentTag = tag || 'all';
                currentPage = page || 1;
                const el = document.getElementById('articles-list');
                let url = '/api/articles/?page=' + currentPage;
                if (currentTag !== 'all') url += '&tag=' + currentTag;

                try {
                    const res = await fetch(url);
                    if (!res.ok) throw new Error();
                    const data = await res.json();

                    if (!data.articles?.length) {
                        el.innerHTML = '<p class="text-center text-zinc-500 py-16 text-sm">No articles yet. Be the first to share your knowledge!</p>';
                        return;
                    }

                    el.innerHTML = data.articles.map(a => {
                        const date = new Date(a.created_at);
                        const dateStr = isNaN(date) ? '' : date.toLocaleDateString('en-US', {month:'short',day:'numeric',year:'numeric'});
                        const tags = (a.tags || []).map(t => `<span class="px-2 py-0.5 rounded-full text-[10px] bg-zinc-800 text-zinc-400">${t}</span>`).join('');
                        return `
                        <a href="/community/${a.slug}" class="block group">
                            <div class="flex gap-4 p-5 bg-surface-card border border-zinc-800 rounded-xl hover:border-zinc-700 transition-all">
                                ${a.cover_image_url ? `<div class="w-24 h-24 rounded-lg overflow-hidden shrink-0 bg-surface"><img src="${a.cover_image_url}" class="w-full h-full object-cover" /></div>` : ''}
                                <div class="flex-1 min-w-0">
                                    <h3 class="text-base font-semibold group-hover:text-accent transition-colors mb-1">${a.title}</h3>
                                    <p class="text-xs text-zinc-500 mb-2 line-clamp-2">${a.summary || ''}</p>
                                    <div class="flex items-center gap-3 text-[11px] text-zinc-500">
                                        <span class="flex items-center gap-1"><i class="ph ph-user"></i>${a.author_name}</span>
                                        <span>${dateStr}</span>
                                        <span class="flex items-center gap-1"><i class="ph ph-heart"></i>${a.likes}</span>
                                        <span class="flex items-center gap-1"><i class="ph ph-eye"></i>${a.views}</span>
                                        ${tags}
                                    </div>
                                </div>
                            </div>
                        </a>`;
                    }).join('');
                } catch(e) {
                    el.innerHTML = '<p class="text-center text-zinc-500 py-16 text-sm">Failed to load articles.</p>';
                }
            }

            function filterArticles(tag) {
                document.querySelectorAll('.article-tag').forEach(t => {
                    t.classList.remove('bg-accent', 'border-accent', 'text-white');
                    t.classList.add('bg-transparent', 'border-zinc-800', 'text-zinc-400');
                });
                const active = document.querySelector(`.article-tag[data-tag="${tag}"]`);
                if (active) {
                    active.classList.add('bg-accent', 'border-accent', 'text-white');
                    active.classList.remove('bg-transparent', 'border-zinc-800', 'text-zinc-400');
                }
                loadArticles(tag, 1);
            }

            loadArticles('all', 1);
            "##
        </script>
    }
}

/// Article detail page
#[component]
pub fn ArticleDetailPage() -> impl IntoView {
    view! {
        <section class="py-12 px-6">
            <div class="max-w-[800px] mx-auto">
                <div id="article-loading" class="text-center py-20"><div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-accent rounded-full"></div></div>
                <div id="article-content" class="hidden"></div>
            </div>
        </section>
        <script>
            r##"
            (async function() {
                const slug = window.location.pathname.split('/').pop();
                const res = await fetch('/api/articles/detail/' + slug);
                if (!res.ok) {
                    document.getElementById('article-loading').innerHTML = '<p class="text-zinc-500">Article not found.</p>';
                    return;
                }
                const a = await res.json();
                document.getElementById('article-loading').classList.add('hidden');
                const el = document.getElementById('article-content');
                el.classList.remove('hidden');

                const date = new Date(a.created_at);
                const dateStr = isNaN(date) ? '' : date.toLocaleDateString('en-US', {month:'long',day:'numeric',year:'numeric'});
                const tags = (a.tags || []).map(t => `<span class="px-2.5 py-1 rounded-full text-xs bg-accent/10 text-accent border border-accent/20">${t}</span>`).join('');

                el.innerHTML = `
                    <a href="/community" class="inline-flex items-center gap-1 text-xs text-zinc-500 hover:text-zinc-300 transition-colors mb-6">
                        <i class="ph ph-arrow-left"></i>Back to Community
                    </a>
                    ${a.cover_image_url ? `<div class="rounded-2xl overflow-hidden mb-6 border border-zinc-800"><img src="${a.cover_image_url}" class="w-full max-h-80 object-cover" /></div>` : ''}
                    <h1 class="text-3xl font-bold mb-3">${a.title}</h1>
                    <div class="flex items-center gap-4 mb-6 text-sm text-zinc-500">
                        <a href="/profile/${a.author.username}" class="flex items-center gap-1.5 text-accent hover:text-accent-hover">
                            <i class="ph ph-user-circle text-base"></i>${a.author.username}
                        </a>
                        <span>${dateStr}</span>
                        <span class="flex items-center gap-1"><i class="ph ph-heart"></i>${a.likes}</span>
                        <span class="flex items-center gap-1"><i class="ph ph-eye"></i>${a.views}</span>
                        <button onclick="likeArticle('${a.id}')" id="like-btn" class="px-3 py-1 rounded-lg text-xs bg-surface-card border border-zinc-800 hover:border-accent hover:text-accent transition-all"><i class="ph ph-heart"></i> Like</button>
                    </div>
                    ${tags ? `<div class="flex flex-wrap gap-2 mb-6">${tags}</div>` : ''}
                    <div class="doc-body text-zinc-300 leading-relaxed mb-10">${a.content}</div>

                    <!-- Comments -->
                    <div class="border-t border-zinc-800 pt-6">
                        <h2 class="text-lg font-semibold mb-4">Comments (${a.comments?.length || 0})</h2>
                        <div id="comment-list" class="space-y-3 mb-4">
                            ${(a.comments || []).map(c => `
                                <div class="p-3 bg-surface-card border border-zinc-800 rounded-lg">
                                    <div class="flex justify-between items-center mb-1">
                                        <a href="/profile/${c.author_name}" class="text-xs text-accent">${c.author_name}</a>
                                        <span class="text-[10px] text-zinc-600">${new Date(c.created_at).toLocaleDateString()}</span>
                                    </div>
                                    <p class="text-sm text-zinc-300">${c.content}</p>
                                </div>
                            `).join('') || '<p class="text-sm text-zinc-600">No comments yet.</p>'}
                        </div>
                        <div class="flex gap-2">
                            <input type="text" id="comment-input" placeholder="Add a comment..." class="flex-1 px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent" />
                            <button onclick="addComment('${a.id}')" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover">Post</button>
                        </div>
                    </div>
                `;
            })();

            async function likeArticle(id) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }
                await fetch('/api/articles/' + id + '/like', { method: 'POST', headers: { 'Authorization': 'Bearer ' + token } });
                window.location.reload();
            }

            async function addComment(id) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }
                const content = document.getElementById('comment-input').value.trim();
                if (!content) return;
                await fetch('/api/articles/' + id + '/comment', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ content })
                });
                window.location.reload();
            }
            "##
        </script>
    }
}

/// Write article page with WYSIWYG
#[component]
pub fn WriteArticlePage() -> impl IntoView {
    view! {
        <link rel="stylesheet" href="https://cdn.quilljs.com/1.3.7/quill.snow.css" />
        <script src="https://cdn.quilljs.com/1.3.7/quill.min.js"></script>
        <section class="py-12 px-6">
            <div class="max-w-[800px] mx-auto">
                <a href="/community" class="inline-flex items-center gap-1 text-xs text-zinc-500 hover:text-zinc-300 transition-colors mb-4">
                    <i class="ph ph-arrow-left"></i>"Back to Community"
                </a>
                <h1 class="text-2xl font-bold mb-6">"Write an Article"</h1>

                <div id="write-error" class="hidden mb-4 p-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm"></div>

                <div class="space-y-4">
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Title"</label>
                        <input type="text" id="article-title" required placeholder="An awesome title..." class="w-full px-4 py-3 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 outline-none focus:border-accent" />
                    </div>
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Summary"</label>
                        <input type="text" id="article-summary" placeholder="A brief summary..." class="w-full px-4 py-3 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent" />
                    </div>
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Tags"</label>
                        <div class="flex flex-wrap gap-2">
                            <label class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg border border-zinc-800 cursor-pointer hover:border-accent transition-colors">
                                <input type="checkbox" value="tutorial" class="article-tag-check accent-accent" /><span class="text-sm text-zinc-300">"tutorial"</span>
                            </label>
                            <label class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg border border-zinc-800 cursor-pointer hover:border-accent transition-colors">
                                <input type="checkbox" value="guide" class="article-tag-check accent-accent" /><span class="text-sm text-zinc-300">"guide"</span>
                            </label>
                            <label class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg border border-zinc-800 cursor-pointer hover:border-accent transition-colors">
                                <input type="checkbox" value="tip" class="article-tag-check accent-accent" /><span class="text-sm text-zinc-300">"tip"</span>
                            </label>
                            <label class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg border border-zinc-800 cursor-pointer hover:border-accent transition-colors">
                                <input type="checkbox" value="showcase" class="article-tag-check accent-accent" /><span class="text-sm text-zinc-300">"showcase"</span>
                            </label>
                            <label class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg border border-zinc-800 cursor-pointer hover:border-accent transition-colors">
                                <input type="checkbox" value="resource" class="article-tag-check accent-accent" /><span class="text-sm text-zinc-300">"resource"</span>
                            </label>
                        </div>
                    </div>
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Content"</label>
                        <div id="article-editor" class="bg-surface-card border border-zinc-800 rounded-lg" style="min-height:400px"></div>
                    </div>
                    <div class="flex justify-end gap-3">
                        <button onclick="publishArticle()" id="publish-btn" class="inline-flex items-center gap-2 px-6 py-3 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                            <i class="ph ph-paper-plane-right text-base"></i>"Publish"
                        </button>
                    </div>
                </div>
            </div>
        </section>
        <script>
            r##"
            let quill;
            setTimeout(() => {
                quill = new Quill('#article-editor', {
                    theme: 'snow',
                    placeholder: 'Write your article content here...',
                    modules: {
                        toolbar: [
                            [{ header: [1, 2, 3, false] }],
                            ['bold', 'italic', 'underline', 'strike'],
                            [{ list: 'ordered' }, { list: 'bullet' }],
                            ['blockquote', 'code-block'],
                            ['link', 'image'],
                            ['clean']
                        ]
                    }
                });
            }, 100);

            async function publishArticle() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }

                const title = document.getElementById('article-title').value.trim();
                const summary = document.getElementById('article-summary').value.trim();
                const content = quill ? quill.root.innerHTML : '';
                const tags = [...document.querySelectorAll('.article-tag-check:checked')].map(c => c.value);

                if (!title) { showWriteError('Title is required'); return; }
                if (!content || content === '<p><br></p>') { showWriteError('Content is required'); return; }

                const btn = document.getElementById('publish-btn');
                btn.disabled = true;
                btn.innerHTML = '<i class="ph ph-spinner animate-spin"></i> Publishing...';

                try {
                    const res = await fetch('/api/articles/create', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({ title, summary, content, tags })
                    });
                    const data = await res.json();
                    if (!res.ok) throw new Error(data.error || 'Failed to publish');
                    window.location.href = '/community/' + data.slug;
                } catch(e) {
                    showWriteError(e.message);
                    btn.disabled = false;
                    btn.innerHTML = '<i class="ph ph-paper-plane-right text-base"></i> Publish';
                }
            }

            function showWriteError(msg) {
                const el = document.getElementById('write-error');
                el.textContent = msg;
                el.classList.remove('hidden');
                setTimeout(() => el.classList.add('hidden'), 5000);
            }
            "##
        </script>
    }
}
