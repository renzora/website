use leptos::prelude::*;

/// Forum landing — Reddit-style board.
#[component]
pub fn ForumPage() -> impl IntoView {
    view! {
        <section class="py-8 px-6 min-h-[80vh] bg-gradient-to-b from-[#0a0a0f] via-[#060608] to-[#060608]">
            <div class="max-w-[900px] mx-auto">
                // Header
                <div class="flex justify-between items-center mb-6">
                    <div>
                        <h1 class="text-2xl font-bold">"Community Forum"</h1>
                        <p class="text-zinc-500 text-sm mt-1">"Discuss, ask questions, and share with fellow developers."</p>
                    </div>
                    <a href="/forum/new" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                        <i class="ph ph-plus-circle text-base"></i>"New Post"
                    </a>
                </div>

                // Categories
                <div id="forum-categories" class="space-y-3">"Loading..."</div>
            </div>
        </section>
        <script>
            r##"
            (async function() {
                const res = await fetch('/api/forum/categories');
                const cats = await res.json();
                const el = document.getElementById('forum-categories');
                el.innerHTML = cats.map(c => `
                    <a href="/forum/${c.slug}" class="block group">
                        <div class="flex items-center gap-4 p-4 bg-surface-card border border-zinc-800 rounded-xl hover:border-zinc-700 transition-all">
                            <div class="w-12 h-12 rounded-xl bg-accent/10 flex items-center justify-center shrink-0">
                                <i class="ph ${c.icon} text-2xl text-accent"></i>
                            </div>
                            <div class="flex-1 min-w-0">
                                <h3 class="text-sm font-semibold group-hover:text-accent transition-colors">${c.name}</h3>
                                <p class="text-xs text-zinc-500 mt-0.5 truncate">${c.description}</p>
                            </div>
                            <div class="flex gap-6 text-center shrink-0">
                                <div>
                                    <div class="text-sm font-semibold">${c.thread_count}</div>
                                    <div class="text-[10px] text-zinc-500 uppercase">threads</div>
                                </div>
                                <div>
                                    <div class="text-sm font-semibold">${c.post_count}</div>
                                    <div class="text-[10px] text-zinc-500 uppercase">posts</div>
                                </div>
                            </div>
                            <i class="ph ph-caret-right text-zinc-600 group-hover:text-zinc-400 transition-colors"></i>
                        </div>
                    </a>
                `).join('');
            })();
            "##
        </script>
    }
}

/// Forum category — thread listing (Reddit-style).
#[component]
pub fn ForumCategoryPage() -> impl IntoView {
    view! {
        <section class="py-8 px-6">
            <div class="max-w-[900px] mx-auto">
                <div id="cat-header" class="mb-6"></div>
                <div id="thread-list">"Loading..."</div>
            </div>
        </section>
        <script>
            r##"
            (async function() {
                const slug = window.location.pathname.split('/').filter(Boolean).pop();
                const res = await fetch('/api/forum/categories/' + slug);
                if (!res.ok) { document.getElementById('thread-list').textContent = 'Category not found'; return; }
                const data = await res.json();
                const cat = data.category;

                document.getElementById('cat-header').innerHTML = `
                    <div class="flex items-center gap-3 mb-1">
                        <a href="/forum" class="text-zinc-500 hover:text-zinc-300 transition-colors"><i class="ph ph-arrow-left text-lg"></i></a>
                        <div class="w-10 h-10 rounded-xl bg-accent/10 flex items-center justify-center">
                            <i class="ph ${cat.icon} text-xl text-accent"></i>
                        </div>
                        <div class="flex-1">
                            <h1 class="text-xl font-bold">${cat.name}</h1>
                            <p class="text-xs text-zinc-500">${cat.description}</p>
                        </div>
                        <a href="/forum/new?cat=${cat.slug}" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                            <i class="ph ph-plus-circle text-base"></i>New Post
                        </a>
                    </div>
                `;

                const el = document.getElementById('thread-list');
                if (!data.threads.length) {
                    el.innerHTML = '<div class="text-center py-16"><i class="ph ph-chat-circle-dots text-4xl text-zinc-700 mb-3"></i><p class="text-zinc-500 text-sm">No threads yet. Start the conversation!</p></div>';
                    return;
                }

                el.innerHTML = '<div class="space-y-2">' + data.threads.map(t => {
                    const timeAgo = getTimeAgo(new Date(t.created_at));
                    return `
                    <a href="/forum/thread/${t.slug}" class="block group">
                        <div class="flex items-start gap-3 p-4 bg-surface-card border border-zinc-800 rounded-xl hover:border-zinc-700 transition-all">
                            <div class="flex flex-col items-center gap-0.5 pt-0.5 shrink-0 w-10">
                                <div class="text-sm font-semibold">${t.post_count - 1}</div>
                                <div class="text-[10px] text-zinc-500">${t.post_count - 1 === 1 ? 'reply' : 'replies'}</div>
                            </div>
                            <div class="flex-1 min-w-0">
                                <div class="flex items-center gap-2">
                                    ${t.pinned ? '<span class="text-[10px] px-1.5 py-0.5 rounded bg-amber-500/10 text-amber-400 font-medium">PINNED</span>' : ''}
                                    ${t.locked ? '<i class="ph ph-lock text-xs text-zinc-500"></i>' : ''}
                                    <h3 class="text-sm font-medium group-hover:text-accent transition-colors truncate">${t.title}</h3>
                                </div>
                                <div class="flex items-center gap-2 mt-1.5 text-[11px] text-zinc-500">
                                    <span class="flex items-center gap-1"><i class="ph ph-user text-xs"></i>${t.author_name}</span>
                                    <span>·</span>
                                    <span>${timeAgo}</span>
                                    <span>·</span>
                                    <span class="flex items-center gap-1"><i class="ph ph-eye text-xs"></i>${t.views}</span>
                                </div>
                            </div>
                        </div>
                    </a>`;
                }).join('') + '</div>';
            })();

            function getTimeAgo(date) {
                const s = Math.floor((Date.now() - date.getTime()) / 1000);
                if (s < 60) return 'just now';
                if (s < 3600) return Math.floor(s/60) + 'm ago';
                if (s < 86400) return Math.floor(s/3600) + 'h ago';
                if (s < 2592000) return Math.floor(s/86400) + 'd ago';
                return date.toLocaleDateString();
            }
            "##
        </script>
    }
}

/// Forum thread — post view with replies.
#[component]
pub fn ForumThreadPage() -> impl IntoView {
    view! {
        <section class="py-8 px-6">
            <div class="max-w-[900px] mx-auto">
                <div id="thread-header" class="mb-4"></div>
                <div id="thread-posts" class="space-y-3">"Loading..."</div>
                <div id="reply-section" class="hidden mt-6">
                    <div class="bg-surface-card border border-zinc-800 rounded-xl p-4">
                        <h3 class="text-sm font-semibold mb-3 flex items-center gap-2">
                            <i class="ph ph-chat-circle text-base text-accent"></i>"Reply"
                        </h3>
                        <textarea id="reply-content" rows="4" placeholder="Write your reply..." class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors resize-y"></textarea>
                        <div class="flex justify-end mt-3">
                            <button onclick="submitReply()" id="reply-btn" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                                <i class="ph ph-paper-plane-right text-base"></i>"Post Reply"
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </section>
        <script>
            r##"
            let threadSlug = '';
            (async function() {
                const parts = window.location.pathname.split('/');
                threadSlug = parts[parts.length - 1];
                const res = await fetch('/api/forum/threads/' + threadSlug);
                if (!res.ok) { document.getElementById('thread-posts').textContent = 'Thread not found'; return; }
                const data = await res.json();
                const t = data.thread;

                document.getElementById('thread-header').innerHTML = `
                    <a href="/forum" class="inline-flex items-center gap-1 text-xs text-zinc-500 hover:text-zinc-300 transition-colors mb-3">
                        <i class="ph ph-arrow-left"></i>Back to Forum
                    </a>
                    <h1 class="text-xl font-bold">${t.title}</h1>
                    <div class="flex items-center gap-3 mt-2 text-xs text-zinc-500">
                        <span class="flex items-center gap-1"><i class="ph ph-chat-circle"></i>${data.total_posts} posts</span>
                        <span class="flex items-center gap-1"><i class="ph ph-eye"></i>${t.views} views</span>
                        ${t.locked ? '<span class="flex items-center gap-1 text-amber-400"><i class="ph ph-lock"></i>Locked</span>' : ''}
                    </div>
                `;

                const el = document.getElementById('thread-posts');
                el.innerHTML = data.posts.map((p, i) => {
                    const timeAgo = getTimeAgo(new Date(p.created_at));
                    const isOP = p.is_first_post;
                    const avatarHtml = p.author_avatar_url
                        ? `<img src="${p.author_avatar_url}" class="w-full h-full object-cover rounded-full" />`
                        : `<i class="ph ph-user-circle text-lg"></i>`;
                    return `
                    <div class="bg-surface-card border border-zinc-800 rounded-xl overflow-hidden ${isOP ? 'border-accent/20' : ''}">
                        <div class="flex gap-4 p-4">
                            <!-- Author sidebar -->
                            <div class="flex flex-col items-center gap-1 shrink-0 w-16">
                                <a href="/profile/${p.author_name}" class="w-10 h-10 rounded-full bg-surface border border-zinc-800 flex items-center justify-center text-zinc-400 hover:border-accent transition-colors overflow-hidden">
                                    ${avatarHtml}
                                </a>
                                <a href="/profile/${p.author_name}" class="text-[11px] font-medium text-zinc-300 hover:text-accent transition-colors text-center truncate w-full">${p.author_name}</a>
                                <span class="text-[9px] px-1.5 py-0.5 rounded-full ${p.author_role === 'admin' ? 'bg-red-500/10 text-red-400' : p.author_role === 'moderator' ? 'bg-amber-500/10 text-amber-400' : 'bg-zinc-800 text-zinc-500'}">${p.author_role}</span>
                                <span class="text-[9px] text-zinc-600">${p.author_post_count} posts</span>
                            </div>
                            <!-- Post content -->
                            <div class="flex-1 min-w-0">
                                <div class="flex items-center justify-between mb-2">
                                    <div class="flex items-center gap-2">
                                        ${isOP ? '<span class="text-[10px] px-1.5 py-0.5 rounded bg-accent/10 text-accent font-medium">OP</span>' : ''}
                                        <span class="text-[11px] text-zinc-500">${timeAgo}</span>
                                        ${p.edited ? '<span class="text-[10px] text-zinc-600 italic">edited</span>' : ''}
                                    </div>
                                    <span class="text-[10px] text-zinc-600">#${i + 1}</span>
                                </div>
                                <div class="text-sm text-zinc-300 leading-relaxed whitespace-pre-wrap">${escapeHtml(p.content)}</div>
                            </div>
                        </div>
                    </div>`;
                }).join('');

                // Show reply form if logged in and thread not locked
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (token && !t.locked) document.getElementById('reply-section').classList.remove('hidden');
            })();

            function escapeHtml(text) {
                const div = document.createElement('div');
                div.textContent = text;
                return div.innerHTML;
            }

            function getTimeAgo(date) {
                const s = Math.floor((Date.now() - date.getTime()) / 1000);
                if (s < 60) return 'just now';
                if (s < 3600) return Math.floor(s/60) + 'm ago';
                if (s < 86400) return Math.floor(s/3600) + 'h ago';
                if (s < 2592000) return Math.floor(s/86400) + 'd ago';
                return date.toLocaleDateString();
            }

            async function submitReply() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }
                const content = document.getElementById('reply-content').value.trim();
                if (!content) return;
                const btn = document.getElementById('reply-btn');
                btn.disabled = true;
                btn.innerHTML = '<i class="ph ph-spinner animate-spin"></i> Posting...';
                try {
                    const res = await fetch('/api/forum/threads/' + threadSlug + '/reply', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({ content })
                    });
                    if (res.ok) { window.location.reload(); }
                    else { const d = await res.json(); alert(d.error || 'Failed to post'); }
                } catch(e) { alert('Network error'); }
                btn.disabled = false;
                btn.innerHTML = '<i class="ph ph-paper-plane-right text-base"></i> Post Reply';
            }
            "##
        </script>
    }
}

/// New thread form.
#[component]
pub fn NewThreadPage() -> impl IntoView {
    view! {
        <section class="py-8 px-6">
            <div class="max-w-[700px] mx-auto">
                <a href="/forum" class="inline-flex items-center gap-1 text-xs text-zinc-500 hover:text-zinc-300 transition-colors mb-4">
                    <i class="ph ph-arrow-left"></i>"Back to Forum"
                </a>
                <h1 class="text-xl font-bold mb-6">"Create a Post"</h1>

                <div id="thread-error" class="hidden mb-4 p-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm"></div>

                <div class="bg-surface-card border border-zinc-800 rounded-xl p-6">
                    <form onsubmit="return createThread(event)" class="flex flex-col gap-4">
                        <div>
                            <label class="block text-xs text-zinc-500 mb-1.5">"Category"</label>
                            <select id="thread-cat" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent"></select>
                        </div>
                        <div>
                            <label class="block text-xs text-zinc-500 mb-1.5">"Title"</label>
                            <input type="text" id="thread-title" required placeholder="What's on your mind?" class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent" />
                        </div>
                        <div>
                            <label class="block text-xs text-zinc-500 mb-1.5">"Body"</label>
                            <textarea id="thread-content" rows="8" required placeholder="Share your thoughts, ask a question, or start a discussion..." class="w-full px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent resize-y"></textarea>
                        </div>
                        <div class="flex justify-end">
                            <button type="submit" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                                <i class="ph ph-paper-plane-right text-base"></i>"Post"
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </section>
        <script>
            r##"
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
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({
                            category_slug: document.getElementById('thread-cat').value,
                            title: document.getElementById('thread-title').value,
                            content: document.getElementById('thread-content').value
                        })
                    });
                    const data = await res.json();
                    if (!res.ok) throw new Error(data.error || 'Failed');
                    window.location.href = '/forum/thread/' + data.slug;
                } catch(e) {
                    err.textContent = e.message;
                    err.classList.remove('hidden');
                }
                return false;
            }
            "##
        </script>
    }
}
