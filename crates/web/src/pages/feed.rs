use leptos::prelude::*;

#[component]
pub fn FeedPage() -> impl IntoView {
    view! {
        <section class="max-w-2xl mx-auto py-8 px-4">
            <div class="flex items-center justify-between mb-6">
                <h1 class="text-xl font-bold text-zinc-100">"Feed"</h1>
            </div>

            // Post composer
            <div id="post-composer" class="hidden mb-6 p-4 bg-surface-card border border-zinc-800 rounded-2xl">
                <textarea id="post-body" class="w-full bg-transparent border-none text-sm text-zinc-200 placeholder:text-zinc-600 resize-none focus:outline-none" rows="3" placeholder="What are you working on?"></textarea>
                <div class="flex items-center justify-between mt-3 pt-3 border-t border-zinc-800">
                    <div class="flex items-center gap-2">
                        <select id="post-visibility" class="text-xs bg-zinc-900 border border-zinc-700 rounded-lg px-2 py-1 text-zinc-400">
                            <option value="public">"Public"</option>
                            <option value="followers">"Followers"</option>
                            <option value="friends">"Friends"</option>
                        </select>
                    </div>
                    <button id="post-submit" class="px-4 py-1.5 bg-accent hover:bg-accent-hover text-white text-sm font-medium rounded-lg transition-colors">"Post"</button>
                </div>
                <div id="post-error" class="text-xs text-red-400 mt-2 hidden"></div>
            </div>

            // Feed
            <div id="feed-loading" class="flex items-center justify-center py-12">
                <span class="loading loading-spinner loading-md text-accent"></span>
            </div>
            <div id="feed-list" class="space-y-4"></div>
            <div id="feed-empty" class="hidden text-center py-12 text-zinc-500 text-sm">"Follow people to see their posts here."</div>
            <div id="load-more" class="hidden text-center py-4">
                <button id="load-more-btn" class="text-sm text-accent hover:text-accent-hover">"Load more"</button>
            </div>
        </section>

        <script>
        r##"
        (async function() {
            var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
            if (!token) { window.location.href = '/login'; return; }

            var composer = document.getElementById('post-composer');
            var postBody = document.getElementById('post-body');
            var postSubmit = document.getElementById('post-submit');
            var postVisibility = document.getElementById('post-visibility');
            var postError = document.getElementById('post-error');
            var feedLoading = document.getElementById('feed-loading');
            var feedList = document.getElementById('feed-list');
            var feedEmpty = document.getElementById('feed-empty');
            var loadMoreDiv = document.getElementById('load-more');
            var loadMoreBtn = document.getElementById('load-more-btn');

            var lastPostId = null;
            composer.classList.remove('hidden');

            // Create post
            postSubmit.addEventListener('click', async function() {
                var body = postBody.value.trim();
                if (!body) return;
                postError.classList.add('hidden');
                postSubmit.disabled = true;
                postSubmit.textContent = 'Posting...';

                try {
                    var res = await fetch('/api/feed/posts', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({ body: body, visibility: postVisibility.value })
                    });
                    var data = await res.json();
                    if (data.id) {
                        postBody.value = '';
                        await loadFeed(true);
                    } else {
                        postError.textContent = data.error || 'Failed to post';
                        postError.classList.remove('hidden');
                    }
                } catch(e) { postError.textContent = 'Network error'; postError.classList.remove('hidden'); }

                postSubmit.disabled = false;
                postSubmit.textContent = 'Post';
            });

            async function loadFeed(refresh) {
                if (refresh) {
                    feedList.innerHTML = '';
                    lastPostId = null;
                }

                var url = '/api/feed/feed?limit=20';
                if (lastPostId) url += '&before=' + lastPostId;

                var res = await fetch(url, { headers: { 'Authorization': 'Bearer ' + token } });
                var data = await res.json();
                feedLoading.classList.add('hidden');

                if (!Array.isArray(data) || data.length === 0) {
                    if (!lastPostId) feedEmpty.classList.remove('hidden');
                    loadMoreDiv.classList.add('hidden');
                    return;
                }

                feedEmpty.classList.add('hidden');
                data.forEach(function(p) {
                    lastPostId = p.id;
                    feedList.insertAdjacentHTML('beforeend', renderPost(p));
                });

                if (data.length >= 20) {
                    loadMoreDiv.classList.remove('hidden');
                } else {
                    loadMoreDiv.classList.add('hidden');
                }

                // Attach like handlers
                feedList.querySelectorAll('.like-btn:not([data-bound])').forEach(function(btn) {
                    btn.dataset.bound = '1';
                    btn.addEventListener('click', function() { toggleLike(btn.dataset.postId, btn); });
                });
            }

            function renderPost(p) {
                var liked = p.is_liked ? 'text-accent' : 'text-zinc-500';
                var likeIcon = p.is_liked ? 'ph-heart-fill' : 'ph-heart';
                return '<div class="p-5 bg-surface-card border border-zinc-800 rounded-2xl">' +
                    '<div class="flex items-center gap-3 mb-3">' +
                        '<a href="/profile/' + p.username + '" class="flex items-center gap-3 group">' +
                            '<div class="w-9 h-9 rounded-full bg-accent/20 flex items-center justify-center text-xs font-bold text-accent">' + (p.username || '?')[0].toUpperCase() + '</div>' +
                            '<div>' +
                                '<span class="text-sm font-medium text-zinc-200 group-hover:text-accent transition-colors">' + escapeHtml(p.username) + '</span>' +
                                '<p class="text-[10px] text-zinc-500">' + timeAgo(p.created_at) + '</p>' +
                            '</div>' +
                        '</a>' +
                    '</div>' +
                    '<p class="text-sm text-zinc-300 whitespace-pre-wrap mb-3">' + escapeHtml(p.body) + '</p>' +
                    (p.media_urls && p.media_urls.length > 0 ? '<div class="grid grid-cols-2 gap-2 mb-3">' + p.media_urls.map(function(u) { return '<img src="' + u + '" class="rounded-xl w-full object-cover max-h-64" />'; }).join('') + '</div>' : '') +
                    '<div class="flex items-center gap-4 pt-3 border-t border-zinc-800">' +
                        '<button class="like-btn flex items-center gap-1.5 ' + liked + ' hover:text-accent transition-colors text-xs" data-post-id="' + p.id + '">' +
                            '<i class="ph ' + likeIcon + ' text-base"></i> ' + p.like_count +
                        '</button>' +
                        '<a href="#" class="flex items-center gap-1.5 text-zinc-500 hover:text-zinc-300 transition-colors text-xs">' +
                            '<i class="ph ph-chat-circle text-base"></i> ' + p.comment_count +
                        '</a>' +
                    '</div>' +
                '</div>';
            }

            async function toggleLike(postId, btn) {
                var res = await fetch('/api/feed/posts/' + postId + '/like', {
                    method: 'POST', headers: { 'Authorization': 'Bearer ' + token }
                });
                var data = await res.json();
                if (data.liked !== undefined) {
                    btn.className = 'like-btn flex items-center gap-1.5 ' + (data.liked ? 'text-accent' : 'text-zinc-500') + ' hover:text-accent transition-colors text-xs';
                    var count = parseInt(btn.textContent.trim()) || 0;
                    var icon = data.liked ? 'ph-heart-fill' : 'ph-heart';
                    btn.innerHTML = '<i class="ph ' + icon + ' text-base"></i> ' + (data.liked ? count + 1 : Math.max(0, count - 1));
                }
            }

            function escapeHtml(s) { var d = document.createElement('div'); d.textContent = s; return d.innerHTML; }

            function timeAgo(iso) {
                var d = new Date(iso);
                var diff = (Date.now() - d.getTime()) / 1000;
                if (diff < 60) return 'just now';
                if (diff < 3600) return Math.floor(diff / 60) + 'm ago';
                if (diff < 86400) return Math.floor(diff / 3600) + 'h ago';
                return d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
            }

            loadMoreBtn.addEventListener('click', function() { loadFeed(false); });

            await loadFeed(true);
        })();
        "##
        </script>
    }
}
