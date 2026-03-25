use leptos::prelude::*;

#[component]
pub fn AssetEditPage() -> impl IntoView {
    view! {
        <section class="py-12 px-6">
            <div class="max-w-3xl mx-auto" id="edit-root">
                <div class="text-center py-20">
                    <div class="inline-block animate-spin w-6 h-6 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                </div>
            </div>
        </section>
        <script>
            r##"
            let assetData = null;
            let assetId = null;

            (async function() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }

                const parts = window.location.pathname.split('/').filter(Boolean);
                const slug = parts[parts.length - 2]; // /marketplace/asset/:slug/edit

                // Fetch asset detail
                const res = await fetch('/api/marketplace/detail/' + slug, {
                    headers: { 'Authorization': 'Bearer ' + token }
                });
                if (!res.ok) {
                    document.getElementById('edit-root').innerHTML = '<p class="text-center text-zinc-500 py-20">Asset not found.</p>';
                    return;
                }
                assetData = await res.json();
                assetId = assetData.id;

                // Verify ownership
                const userCookie = document.cookie.match('(^|;)\\s*user\\s*=\\s*([^;]+)')?.pop();
                let currentUserId = null;
                if (userCookie) { try { currentUserId = JSON.parse(decodeURIComponent(userCookie)).id; } catch(e) {} }
                if (!currentUserId || (assetData.creator && assetData.creator.id !== currentUserId)) {
                    document.getElementById('edit-root').innerHTML = '<p class="text-center text-zinc-500 py-20">You don\'t have permission to edit this asset.</p>';
                    return;
                }

                // Fetch categories
                const catRes = await fetch('/api/marketplace/categories');
                const cats = catRes.ok ? await catRes.json() : [];

                // Fetch existing media
                const mediaRes = await fetch('/api/marketplace/' + assetId + '/media');
                const media = mediaRes.ok ? await mediaRes.json() : [];

                const a = assetData;
                const catOptions = cats.map(c =>
                    `<option value="${c.slug}" ${c.slug === a.category ? 'selected' : ''}>${c.name}</option>`
                ).join('');

                // Media gallery
                const mediaHtml = media.length ? media.map(m => `
                    <div class="relative group" data-media-id="${m.id}">
                        <div class="w-24 h-16 rounded-lg overflow-hidden border border-zinc-800/50">
                            ${m.media_type === 'video'
                                ? `<div class="w-full h-full bg-zinc-900 flex items-center justify-center"><i class="ph ph-play-circle text-xl text-zinc-500"></i></div>`
                                : `<img src="${m.url}" class="w-full h-full object-cover" />`}
                        </div>
                        <button onclick="deleteMedia('${m.id}')" class="absolute -top-1.5 -right-1.5 w-5 h-5 rounded-full bg-red-500 text-white flex items-center justify-center text-xs opacity-0 group-hover:opacity-100 transition-opacity hover:bg-red-400">
                            <i class="ph ph-x"></i>
                        </button>
                    </div>
                `).join('') : '<p class="text-xs text-zinc-600">No media uploaded yet.</p>';

                const root = document.getElementById('edit-root');
                root.innerHTML = `
                    <div class="mb-10">
                        <a href="/marketplace/asset/${a.slug}" class="inline-flex items-center gap-1.5 text-sm text-zinc-500 hover:text-zinc-300 transition-colors mb-4">
                            <i class="ph ph-arrow-left"></i> Back to Asset
                        </a>
                        <h1 class="text-3xl font-bold">Edit Asset</h1>
                        <p class="text-zinc-400 text-sm mt-2">Update your asset's details, files, and media.</p>
                    </div>

                    <div id="edit-error" class="hidden mb-6 p-4 rounded-xl bg-red-500/10 border border-red-500/20 text-red-400 text-sm flex items-center gap-2">
                        <i class="ph ph-warning-circle text-lg"></i>
                        <span id="edit-error-text"></span>
                    </div>
                    <div id="edit-success" class="hidden mb-6 p-4 rounded-xl bg-green-500/10 border border-green-500/20 text-green-400 text-sm flex items-center gap-2">
                        <i class="ph ph-check-circle text-lg"></i>
                        <span id="edit-success-text"></span>
                    </div>

                    <!-- Basic Info -->
                    <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5 mb-8">
                        <h2 class="text-base font-semibold flex items-center gap-2">
                            <i class="ph ph-info text-accent"></i> Basic Information
                        </h2>
                        <div>
                            <label class="block text-sm text-zinc-400 mb-1.5">Asset Name</label>
                            <input type="text" id="edit-name" value="${a.name}" maxlength="128" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                        </div>
                        <div>
                            <label class="block text-sm text-zinc-400 mb-1.5">Description</label>
                            <textarea id="edit-description" rows="5" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all resize-y">${a.description}</textarea>
                        </div>
                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">Category</label>
                                <select id="edit-category" disabled class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none opacity-60 cursor-not-allowed">
                                    ${catOptions}
                                </select>
                                <p class="text-xs text-zinc-600 mt-1">Category cannot be changed after creation.</p>
                            </div>
                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">Version</label>
                                <input type="text" id="edit-version" value="${a.version}" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                            </div>
                        </div>
                    </div>

                    <!-- Pricing -->
                    <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5 mb-8">
                        <h2 class="text-base font-semibold flex items-center gap-2">
                            <i class="ph ph-coins text-amber-400"></i> Pricing
                        </h2>
                        <div>
                            <label class="block text-sm text-zinc-400 mb-1.5">Price in credits (0 = free)</label>
                            <input type="number" id="edit-price" min="0" value="${a.price_credits}" oninput="updateEditPricePreview()"
                                class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                            <p class="text-xs text-zinc-600 mt-1" id="edit-price-preview"></p>
                        </div>
                    </div>

                    <!-- Thumbnail -->
                    <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5 mb-8">
                        <h2 class="text-base font-semibold flex items-center gap-2">
                            <i class="ph ph-image text-cyan-400"></i> Thumbnail
                        </h2>
                        <div class="flex items-start gap-4">
                            <div class="w-40 h-24 rounded-xl bg-white/[0.02] border border-zinc-800/50 flex items-center justify-center overflow-hidden shrink-0" id="edit-thumb-preview">
                                ${a.thumbnail_url ? `<img src="${a.thumbnail_url}" class="w-full h-full object-cover" />` : '<i class="ph ph-image text-2xl text-zinc-700"></i>'}
                            </div>
                            <div class="flex-1">
                                <input type="file" id="edit-thumbnail" accept="image/*" onchange="previewEditThumb(this)"
                                    class="w-full text-sm text-zinc-400 file:mr-4 file:py-2 file:px-4 file:rounded-xl file:border-0 file:text-sm file:font-medium file:bg-white/[0.05] file:text-zinc-300 hover:file:bg-white/[0.08] file:cursor-pointer file:transition-colors" />
                                <p class="text-xs text-zinc-600 mt-1">Upload a new thumbnail to replace the current one. 16:9 recommended.</p>
                            </div>
                        </div>
                    </div>

                    <!-- Asset File -->
                    <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5 mb-8">
                        <h2 class="text-base font-semibold flex items-center gap-2">
                            <i class="ph ph-file-arrow-up text-cyan-400"></i> Asset File
                        </h2>
                        <div>
                            <p class="text-sm text-zinc-400 mb-2">${a.file_url ? '<i class="ph ph-check-circle text-green-400"></i> File uploaded' : '<i class="ph ph-warning text-amber-400"></i> No file uploaded'}</p>
                            <input type="file" id="edit-file"
                                class="w-full text-sm text-zinc-400 file:mr-4 file:py-2 file:px-4 file:rounded-xl file:border-0 file:text-sm file:font-medium file:bg-white/[0.05] file:text-zinc-300 hover:file:bg-white/[0.08] file:cursor-pointer file:transition-colors" />
                            <p class="text-xs text-zinc-600 mt-1">Upload a new file to replace the current one. Max 50MB.</p>
                        </div>
                    </div>

                    <!-- Gallery Media -->
                    <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5 mb-8">
                        <h2 class="text-base font-semibold flex items-center gap-2">
                            <i class="ph ph-images text-purple-400"></i> Gallery Media
                        </h2>
                        <div class="flex flex-wrap gap-3" id="media-grid">${mediaHtml}</div>
                        <div class="border-t border-zinc-800/50 pt-4 mt-4">
                            <label class="block text-sm text-zinc-400 mb-1.5">Add Screenshots</label>
                            <input type="file" id="add-screenshots" accept="image/*" multiple
                                class="w-full text-sm text-zinc-400 file:mr-4 file:py-2 file:px-4 file:rounded-xl file:border-0 file:text-sm file:font-medium file:bg-white/[0.05] file:text-zinc-300 hover:file:bg-white/[0.08] file:cursor-pointer file:transition-colors" />
                        </div>
                        <div>
                            <label class="block text-sm text-zinc-400 mb-1.5">Add Video URL</label>
                            <div class="flex gap-2">
                                <input type="text" id="add-video-url" placeholder="https://youtube.com/watch?v=..."
                                    class="flex-1 px-4 py-2.5 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                                <button onclick="addVideoMedia()" class="px-4 py-2.5 rounded-xl text-sm font-medium bg-white/[0.05] text-zinc-300 hover:bg-white/[0.08] transition-colors">Add</button>
                            </div>
                        </div>
                    </div>

                    <!-- Publishing -->
                    <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5 mb-8">
                        <h2 class="text-base font-semibold flex items-center gap-2">
                            <i class="ph ph-globe text-green-400"></i> Publishing
                        </h2>
                        <label class="flex items-center gap-3 cursor-pointer select-none">
                            <input type="checkbox" id="edit-published" ${a.published ? 'checked' : ''} class="accent-accent w-4 h-4" />
                            <div>
                                <span class="text-sm text-zinc-300">Published</span>
                                <p class="text-xs text-zinc-600">When enabled, the asset is visible on the marketplace.</p>
                            </div>
                        </label>
                    </div>

                    <!-- Actions -->
                    <div class="flex items-center gap-4">
                        <button onclick="saveAsset()" id="save-btn" class="flex-1 inline-flex items-center justify-center gap-2 px-6 py-3.5 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)]">
                            <i class="ph ph-floppy-disk text-lg"></i> Save Changes
                        </button>
                        <a href="/marketplace/asset/${a.slug}" class="px-6 py-3.5 rounded-xl text-sm font-medium text-zinc-400 hover:text-zinc-200 transition-colors">Cancel</a>
                    </div>
                `;

                updateEditPricePreview();
            })();

            function updateEditPricePreview() {
                const price = parseInt(document.getElementById('edit-price')?.value) || 0;
                const el = document.getElementById('edit-price-preview');
                if (!el) return;
                if (price === 0) {
                    el.textContent = 'Free — anyone can download';
                } else {
                    const usd = (price * 0.10).toFixed(2);
                    const earn = (price * 0.08).toFixed(2);
                    el.textContent = `${price} credits ($${usd}) — you earn ${Math.floor(price * 0.8)} credits ($${earn})`;
                }
            }

            function previewEditThumb(input) {
                const el = document.getElementById('edit-thumb-preview');
                if (input.files[0]) {
                    const url = URL.createObjectURL(input.files[0]);
                    el.innerHTML = `<img src="${url}" class="w-full h-full object-cover" />`;
                }
            }

            function showError(msg) {
                const el = document.getElementById('edit-error');
                document.getElementById('edit-error-text').textContent = msg;
                el.classList.remove('hidden');
                window.scrollTo({ top: 0, behavior: 'smooth' });
            }

            function showSuccess(msg) {
                const el = document.getElementById('edit-success');
                document.getElementById('edit-success-text').textContent = msg;
                el.classList.remove('hidden');
                document.getElementById('edit-error').classList.add('hidden');
                window.scrollTo({ top: 0, behavior: 'smooth' });
            }

            async function saveAsset() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token || !assetId) return;

                const btn = document.getElementById('save-btn');
                btn.disabled = true;
                btn.innerHTML = '<i class="ph ph-spinner text-lg animate-spin"></i> Saving...';

                try {
                    // 1. Update metadata
                    const body = {
                        name: document.getElementById('edit-name').value,
                        description: document.getElementById('edit-description').value,
                        price_credits: parseInt(document.getElementById('edit-price').value) || 0,
                        version: document.getElementById('edit-version').value,
                        published: document.getElementById('edit-published').checked,
                    };

                    const res = await fetch('/api/marketplace/' + assetId + '/update', {
                        method: 'PUT',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify(body)
                    });
                    if (!res.ok) {
                        const d = await res.json().catch(() => ({}));
                        throw new Error(d.error || 'Failed to save');
                    }

                    // 2. Upload new file and/or thumbnail if selected
                    const thumbFile = document.getElementById('edit-thumbnail')?.files[0];
                    const newFile = document.getElementById('edit-file')?.files[0];
                    if (newFile || thumbFile) {
                        const fd = new FormData();
                        if (newFile) fd.append('file', newFile);
                        if (thumbFile) fd.append('thumbnail', thumbFile);
                        const fRes = await fetch('/api/marketplace/' + assetId + '/files', {
                            method: 'PUT',
                            headers: { 'Authorization': 'Bearer ' + token },
                            body: fd
                        });
                        if (!fRes.ok) {
                            const d = await fRes.json().catch(() => ({}));
                            throw new Error(d.error || 'Failed to upload files');
                        }
                    }

                    // 4. Upload new screenshots
                    const screenshots = document.getElementById('add-screenshots')?.files;
                    if (screenshots && screenshots.length > 0) {
                        for (let i = 0; i < Math.min(screenshots.length, 10); i++) {
                            const mfd = new FormData();
                            mfd.append('media_type', 'image');
                            mfd.append('file', screenshots[i]);
                            await fetch('/api/marketplace/' + assetId + '/media', {
                                method: 'POST',
                                headers: { 'Authorization': 'Bearer ' + token },
                                body: mfd
                            });
                        }
                    }

                    showSuccess('Asset updated successfully!');

                    // Refresh the page data after a moment
                    setTimeout(() => window.location.reload(), 1200);
                } catch (error) {
                    showError(error.message);
                }

                btn.disabled = false;
                btn.innerHTML = '<i class="ph ph-floppy-disk text-lg"></i> Save Changes';
            }

            async function deleteMedia(mediaId) {
                if (!confirm('Delete this media?')) return;
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;

                const res = await fetch('/api/marketplace/media/' + mediaId, {
                    method: 'DELETE',
                    headers: { 'Authorization': 'Bearer ' + token }
                });
                if (res.ok) {
                    const el = document.querySelector(`[data-media-id="${mediaId}"]`);
                    if (el) el.remove();
                } else {
                    alert('Failed to delete media');
                }
            }

            async function addVideoMedia() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                const url = document.getElementById('add-video-url')?.value.trim();
                if (!token || !url || !assetId) return;

                const fd = new FormData();
                fd.append('video_url', url);
                const res = await fetch('/api/marketplace/' + assetId + '/media', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token },
                    body: fd
                });
                if (res.ok) {
                    document.getElementById('add-video-url').value = '';
                    window.location.reload();
                } else {
                    alert('Failed to add video');
                }
            }
            "##
        </script>
    }
}
