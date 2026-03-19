use leptos::prelude::*;

#[component]
pub fn AssetDetailPage() -> impl IntoView {
    view! {
        <section class="py-8 px-6">
            <div class="max-w-[1000px] mx-auto" id="asset-detail">"Loading..."</div>
        </section>
        <script>
            r##"
            (async function() {
                const slug = window.location.pathname.split('/').pop();
                const res = await fetch('/api/marketplace/detail/' + slug);
                if (!res.ok) { document.getElementById('asset-detail').textContent = 'Asset not found'; return; }
                const a = await res.json();
                const el = document.getElementById('asset-detail');
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                const userCookie = document.cookie.match('(^|;)\\s*user\\s*=\\s*([^;]+)')?.pop();
                let currentUserId = null;
                if (userCookie) { try { currentUserId = JSON.parse(decodeURIComponent(userCookie)).id; } catch(e) {} }
                const isCreator = currentUserId && a.creator && a.creator.id === currentUserId;

                el.innerHTML = `
                    <a href="/marketplace" class="inline-flex items-center gap-1 text-xs text-zinc-500 hover:text-zinc-300 transition-colors mb-4"><i class="ph ph-arrow-left"></i>Marketplace</a>

                    <div class="flex flex-col lg:flex-row gap-6">
                        <!-- Main -->
                        <div class="flex-1">
                            <!-- Thumbnail -->
                            <div class="h-64 bg-surface-card border border-zinc-800 rounded-xl flex items-center justify-center overflow-hidden mb-6">
                                ${a.thumbnail_url ? `<img src="${a.thumbnail_url}" class="w-full h-full object-cover" />` : '<i class="ph ph-package text-5xl text-zinc-700"></i>'}
                            </div>

                            <h1 class="text-2xl font-bold">${a.name}</h1>
                            <div class="flex items-center gap-3 mt-2 text-xs text-zinc-500">
                                <span class="px-2 py-0.5 rounded bg-surface-card border border-zinc-800">${a.category}</span>
                                <span>v${a.version}</span>
                                <span><i class="ph ph-download-simple"></i> ${a.downloads} downloads</span>
                            </div>

                            <div class="mt-4 flex items-center gap-2">
                                <a href="/profile/${a.creator.username}" class="text-sm text-accent hover:text-accent-hover font-medium">${a.creator.username}</a>
                                <span class="text-xs px-1.5 py-0.5 rounded bg-zinc-800 text-zinc-400">${a.creator.role}</span>
                            </div>

                            <div class="mt-6">
                                <h2 class="text-sm font-semibold mb-2">Description</h2>
                                <p class="text-sm text-zinc-400 leading-relaxed whitespace-pre-wrap">${a.description}</p>
                            </div>

                            <div class="mt-6 text-xs text-zinc-500">
                                Published ${new Date(a.created_at).toLocaleDateString()} · Updated ${new Date(a.updated_at).toLocaleDateString()}
                            </div>

                            <!-- Comments -->
                            <div class="mt-8">
                                <h2 class="text-sm font-semibold mb-3">Comments</h2>
                                <div id="comments-list" class="space-y-3 mb-4">Loading...</div>
                                ${token ? `
                                    <div class="flex gap-2">
                                        <textarea id="comment-input" rows="2" placeholder="Leave a comment..." class="flex-1 px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent resize-y"></textarea>
                                        <button onclick="postComment('${a.id}')" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors self-end">Post</button>
                                    </div>
                                ` : '<p class="text-xs text-zinc-500"><a href="/login" class="text-accent">Sign in</a> to comment.</p>'}
                            </div>
                        </div>

                        <!-- Sidebar -->
                        <div class="w-full lg:w-72 shrink-0">
                            <div class="bg-surface-card border border-zinc-800 rounded-xl p-5 sticky top-20">
                                <div class="text-2xl font-bold mb-1">${a.price_credits === 0 ? 'Free' : a.price_credits + ' credits'}</div>
                                ${token ? `
                                    ${isCreator ? `
                                        <button onclick="downloadAsset('${a.id}')" class="w-full mt-3 px-4 py-2.5 rounded-lg text-sm font-medium bg-green-500 text-white hover:bg-green-600 transition-colors flex items-center justify-center gap-2"><i class="ph ph-download-simple"></i>Download</button>
                                        <p class="text-[10px] text-zinc-400 text-center mt-2">This is your asset</p>
                                    ` : a.owned ? `
                                        <button onclick="downloadAsset('${a.id}')" class="w-full mt-3 px-4 py-2.5 rounded-lg text-sm font-medium bg-green-500 text-white hover:bg-green-600 transition-colors flex items-center justify-center gap-2"><i class="ph ph-download-simple"></i>Download</button>
                                        <p class="text-[10px] text-green-400 text-center mt-2">You own this asset</p>
                                    ` : a.price_credits === 0 ? `
                                        <button onclick="purchaseAsset('${a.id}')" class="w-full mt-3 px-4 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors flex items-center justify-center gap-2"><i class="ph ph-download-simple"></i>Download</button>
                                    ` : `
                                        <button onclick="purchaseAsset('${a.id}')" class="w-full mt-3 px-4 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">Purchase</button>
                                    `}
                                ` : `
                                    <a href="/login" class="block w-full mt-3 px-4 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors text-center">Sign in</a>
                                `}
                                <div class="mt-4 space-y-2 text-xs text-zinc-500">
                                    <div class="flex justify-between"><span>Category</span><span class="text-zinc-300">${a.category}</span></div>
                                    <div class="flex justify-between"><span>Version</span><span class="text-zinc-300">${a.version}</span></div>
                                    <div class="flex justify-between"><span>Downloads</span><span class="text-zinc-300">${a.downloads}</span></div>
                                </div>
                            </div>
                        </div>
                    </div>
                `;
            })();

            async function purchaseAsset(id) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }
                const res = await fetch('/api/credits/purchase', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ asset_id: id })
                });
                const data = await res.json();
                if (res.ok) { window.location.reload(); } else { alert(data.error || 'Purchase failed'); }
            }

            async function downloadAsset(id) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                const res = await fetch('/api/marketplace/' + id + '/download', {
                    headers: { 'Authorization': 'Bearer ' + token }
                });
                const data = await res.json();
                if (res.ok && data.download_url) {
                    window.open(data.download_url, '_blank');
                } else { alert(data.error || 'Download failed'); }
            }

            // Comments
            let assetId = '';
            (async function loadComments() {
                assetId = window.location.pathname.split('/').pop();
                // Get asset to know creator
                const aRes = await fetch('/api/marketplace/detail/' + assetId);
                const asset = aRes.ok ? await aRes.json() : null;
                const creatorId = asset?.creator?.id;

                const res = await fetch('/api/marketplace/' + (asset?.id || '') + '/comments');
                if (!res.ok) return;
                const data = await res.json();
                const el = document.getElementById('comments-list');
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                const userCookie = document.cookie.match('(^|;)\\s*user\\s*=\\s*([^;]+)')?.pop();
                let myId = null;
                let myRole = '';
                if (userCookie) { try { const u = JSON.parse(decodeURIComponent(userCookie)); myId = u.id; myRole = u.role; } catch(e) {} }

                if (!data.comments?.length) { el.innerHTML = '<p class="text-xs text-zinc-500">No comments yet.</p>'; return; }
                el.innerHTML = data.comments.map(c => {
                    const canDelete = myId && (myId === c.author_id || myId === creatorId || myRole === 'admin');
                    return `
                    <div class="p-3 bg-surface border border-zinc-800 rounded-lg">
                        <div class="flex justify-between items-start">
                            <div>
                                <a href="/profile/${c.author_name}" class="text-xs text-accent hover:text-accent-hover font-medium">${c.author_name}</a>
                                <span class="text-[10px] text-zinc-600 ml-2">${new Date(c.created_at).toLocaleDateString()}</span>
                            </div>
                            ${canDelete ? `<button onclick="deleteComment('${c.id}','${asset?.id}')" class="text-[10px] text-red-400 hover:text-red-300"><i class="ph ph-trash"></i></button>` : ''}
                        </div>
                        <p class="text-sm text-zinc-300 mt-1">${c.content}</p>
                    </div>`;
                }).join('');
            })();

            async function postComment(id) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }
                const input = document.getElementById('comment-input');
                if (!input.value.trim()) return;
                const res = await fetch('/api/marketplace/' + id + '/comments', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ content: input.value })
                });
                if (res.ok) { input.value = ''; window.location.reload(); } else { const d = await res.json(); alert(d.error || 'Failed'); }
            }

            async function deleteComment(commentId, assetId) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                await fetch('/api/marketplace/comments/' + commentId, {
                    method: 'DELETE',
                    headers: { 'Authorization': 'Bearer ' + token }
                });
                window.location.reload();
            }
            "##
        </script>
    }
}
