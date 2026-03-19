use leptos::prelude::*;

/// Forum landing — list categories.
#[component]
pub fn ForumPage() -> impl IntoView {
    view! {
        <section class="py-20 px-6">
            <div class="max-w-[900px] mx-auto">
                <div class="flex justify-between items-start mb-8">
                    <div>
                        <h1 class="text-3xl font-bold">"Forum"</h1>
                        <p class="text-zinc-400 mt-1 text-sm">"Discuss, ask questions, and share with the community."</p>
                    </div>
                    <a href="/forum/new" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                        <i class="ph ph-plus text-base"></i>"New Thread"
                    </a>
                </div>
                <div id="forum-categories" class="space-y-2">"Loading..."</div>
            </div>
        </section>
        <script>
            r#"
            (async function() {
                const res = await fetch('/api/forum/categories');
                const cats = await res.json();
                const el = document.getElementById('forum-categories');
                el.innerHTML = cats.map(c => `
                    <a href="/forum/${c.slug}" class="flex items-center gap-4 p-4 bg-surface-card border border-zinc-800 rounded-lg hover:border-accent transition-colors">
                        <i class="ph ${c.icon} text-2xl text-accent"></i>
                        <div class="flex-1">
                            <h3 class="text-sm font-semibold">${c.name}</h3>
                            <p class="text-xs text-zinc-400">${c.description}</p>
                        </div>
                        <div class="text-right">
                            <div class="text-xs text-zinc-400">${c.thread_count} threads</div>
                            <div class="text-xs text-zinc-500">${c.post_count} posts</div>
                        </div>
                    </a>
                `).join('');
            })();
            "#
        </script>
    }
}

/// Forum category — list threads.
#[component]
pub fn ForumCategoryPage() -> impl IntoView {
    view! {
        <section class="py-20 px-6">
            <div class="max-w-[900px] mx-auto">
                <div id="forum-cat-header" class="mb-6"></div>
                <div id="forum-threads" class="space-y-2">"Loading..."</div>
            </div>
        </section>
        <script>
            r#"
            (async function() {
                const slug = window.location.pathname.split('/').pop();
                const res = await fetch('/api/forum/categories/' + slug);
                if (!res.ok) { document.getElementById('forum-threads').textContent = 'Category not found'; return; }
                const data = await res.json();
                document.getElementById('forum-cat-header').innerHTML = `
                    <div class="flex justify-between items-center">
                        <div>
                            <a href="/forum" class="text-xs text-accent">&larr; Forum</a>
                            <h1 class="text-2xl font-bold mt-1">${data.category.name}</h1>
                            <p class="text-zinc-400 text-sm">${data.category.description}</p>
                        </div>
                        <a href="/forum/new?cat=${data.category.slug}" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                            <i class="ph ph-plus text-base"></i>New Thread
                        </a>
                    </div>
                `;
                const el = document.getElementById('forum-threads');
                if (!data.threads.length) { el.innerHTML = '<p class="text-zinc-500 text-sm py-8 text-center">No threads yet. Start one!</p>'; return; }
                el.innerHTML = data.threads.map(t => `
                    <a href="/forum/thread/${t.slug}" class="flex items-center gap-4 p-4 bg-surface-card border border-zinc-800 rounded-lg hover:border-accent transition-colors">
                        ${t.pinned ? '<i class="ph ph-push-pin text-amber-400"></i>' : ''}
                        <div class="flex-1">
                            <h3 class="text-sm font-medium">${t.title}</h3>
                            <p class="text-xs text-zinc-500">by ${t.author_name}</p>
                        </div>
                        <div class="text-right text-xs text-zinc-400">
                            <div>${t.post_count} posts</div>
                            <div>${t.views} views</div>
                        </div>
                    </a>
                `).join('');
            })();
            "#
        </script>
    }
}

/// Forum thread — view posts and reply.
#[component]
pub fn ForumThreadPage() -> impl IntoView {
    view! {
        <section class="py-20 px-6">
            <div class="max-w-[900px] mx-auto">
                <div id="thread-header" class="mb-6"></div>
                <div id="thread-posts" class="space-y-4 mb-8">"Loading..."</div>
                <div id="reply-form" class="hidden">
                    <h3 class="text-sm font-semibold mb-3">"Reply"</h3>
                    <textarea id="reply-content" rows="4" placeholder="Write your reply..." class="w-full px-3 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors resize-y mb-3"></textarea>
                    <button onclick="submitReply()" id="reply-btn" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                        <i class="ph ph-paper-plane-right text-base"></i>"Post Reply"
                    </button>
                </div>
            </div>
        </section>
        <script>
            r#"
            let threadSlug = '';
            (async function() {
                threadSlug = window.location.pathname.split('/').pop();
                const res = await fetch('/api/forum/threads/' + threadSlug);
                if (!res.ok) { document.getElementById('thread-posts').textContent = 'Thread not found'; return; }
                const data = await res.json();
                document.getElementById('thread-header').innerHTML = `
                    <a href="/forum" class="text-xs text-accent">&larr; Forum</a>
                    <h1 class="text-2xl font-bold mt-1">${data.thread.title}</h1>
                    <p class="text-xs text-zinc-500 mt-1">${data.total_posts} posts · ${data.thread.views} views</p>
                `;
                const el = document.getElementById('thread-posts');
                el.innerHTML = data.posts.map(p => `
                    <div class="p-4 bg-surface-card border border-zinc-800 rounded-lg">
                        <div class="flex items-center gap-2 mb-3">
                            <a href="/profile/${p.author_name}" class="text-sm font-medium text-accent hover:text-accent-hover">${p.author_name}</a>
                            <span class="text-[10px] px-1.5 py-0.5 rounded bg-zinc-800 text-zinc-400">${p.author_role}</span>
                            <span class="text-[10px] text-zinc-500">${p.author_post_count} posts</span>
                            <span class="flex-1"></span>
                            <span class="text-[11px] text-zinc-500">${new Date(p.created_at).toLocaleDateString()}</span>
                        </div>
                        <div class="text-sm text-zinc-300 leading-relaxed whitespace-pre-wrap">${p.content}</div>
                    </div>
                `).join('');
                // Show reply form if logged in
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (token && !data.thread.locked) document.getElementById('reply-form').classList.remove('hidden');
            })();

            async function submitReply() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }
                const content = document.getElementById('reply-content').value;
                if (!content.trim()) return;
                const btn = document.getElementById('reply-btn');
                btn.disabled = true;
                btn.innerHTML = '<i class="ph ph-spinner animate-spin"></i> Posting...';
                const res = await fetch('/api/forum/threads/' + threadSlug + '/reply', {
                    method: 'POST', headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ content })
                });
                if (res.ok) { window.location.reload(); } else {
                    const d = await res.json();
                    alert(d.error || 'Failed');
                    btn.disabled = false;
                    btn.innerHTML = '<i class="ph ph-paper-plane-right text-base"></i> Post Reply';
                }
            }
            "#
        </script>
    }
}

/// New thread form.
#[component]
pub fn NewThreadPage() -> impl IntoView {
    view! {
        <section class="py-20 px-6">
            <div class="max-w-2xl mx-auto">
                <h1 class="text-2xl font-bold mb-6">"New Thread"</h1>
                <div id="thread-error" class="hidden mb-4 p-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm"></div>
                <form onsubmit="return createThread(event)" class="flex flex-col gap-4">
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Category"</label>
                        <select id="thread-cat" class="w-full px-3 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent"></select>
                    </div>
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Title"</label>
                        <input type="text" id="thread-title" required class="w-full px-3 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent" />
                    </div>
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Content"</label>
                        <textarea id="thread-content" rows="8" required class="w-full px-3 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent resize-y"></textarea>
                    </div>
                    <button type="submit" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                        <i class="ph ph-paper-plane-right text-base"></i>"Create Thread"
                    </button>
                </form>
            </div>
        </section>
        <script>
            r#"
            (async function() {
                const res = await fetch('/api/forum/categories');
                const cats = await res.json();
                const sel = document.getElementById('thread-cat');
                const urlCat = new URLSearchParams(window.location.search).get('cat');
                sel.innerHTML = cats.map(c => `<option value="${c.slug}" ${c.slug===urlCat?'selected':''}>${c.name}</option>`).join('');
            })();
            async function createThread(e) {
                e.preventDefault();
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return false; }
                const err = document.getElementById('thread-error');
                err.classList.add('hidden');
                try {
                    const res = await fetch('/api/forum/threads', {
                        method: 'POST', headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({ category_slug: document.getElementById('thread-cat').value, title: document.getElementById('thread-title').value, content: document.getElementById('thread-content').value })
                    });
                    const data = await res.json();
                    if (!res.ok) throw new Error(data.error || 'Failed');
                    window.location.href = '/forum/thread/' + data.slug;
                } catch(e) { err.textContent = e.message; err.classList.remove('hidden'); }
                return false;
            }
            "#
        </script>
    }
}
