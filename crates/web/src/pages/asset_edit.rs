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
                        <h1 class="text-3xl font-bold">Editing <span class="text-accent">${relEsc(a.name)}</span></h1>
                        <p class="text-zinc-300 text-sm mt-2">The listing: its description, price, artwork and gallery, plus the engine each published release targets. Files and version numbers come from publishing a release.</p>
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
                                <label class="block text-sm text-zinc-400 mb-1.5">Current version</label>
                                <div class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-300 text-sm">${a.version}</div>
                                <p class="text-xs text-zinc-500 mt-1">Set by publishing a release, not here. Typing a version would rename the listing without any release matching it.</p>
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

                    <!--
                        No file replacement here. Swapping a published release's
                        files in place makes the version a lie: everyone who
                        downloaded v1.0.0 has different bytes from everyone who
                        downloads it after, under the same number, and nothing
                        records that it happened. It also went round the
                        publishing tool, which is the thing that checks the
                        manifest, the engine version and the ordering. Shipping
                        changed files is publishing a release.
                    -->

                    <!-- Releases -->
                    <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5 mb-8">
                        <div class="flex items-center justify-between">
                            <h2 class="text-base font-semibold flex items-center gap-2">
                                <i class="ph ph-tag text-green-400"></i> Releases
                            </h2>
                            <a href="/marketplace/asset/${a.slug}/releases/new" class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-accent/10 border border-accent/30 text-accent hover:bg-accent/15 transition-colors">
                                <i class="ph ph-rocket-launch"></i> New release
                            </a>
                        </div>
                        <p class="text-sm text-zinc-400">Every version keeps its own files, so people can still download the one they were using. Each release states the oldest engine it runs on, and an editor is offered the newest release it can actually run.</p>
                        <div id="releases-list" class="space-y-3">
                            <p class="text-sm text-zinc-400 px-1 py-3">Loading...</p>
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
                loadReleasesList();
            })();

            // ── Releases ──────────────────────────────────────────────────

            function relEsc(s) {
                return String(s ?? '').replace(/[&<>"']/g, c =>
                    ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
            }

            // Engine versions, newest first. Loaded once and reused by every
            // release row's dropdown.
            let EDIT_ENGINES = [];

            // "Any engine" is first and is what an unset value means. It is not
            // a placeholder: a release with no floor is offered to everybody,
            // and that is the honest default rather than a floor invented for
            // releases that never declared one.
            function engineOptions(selected) {
                return ['<option value="">Any engine</option>'].concat(
                    EDIT_ENGINES.map(e =>
                        `<option value="${relEsc(e.version)}"${e.version === selected ? ' selected' : ''}>${relEsc(e.version)} and newer</option>`)
                ).join('');
            }

            async function loadReleasesList() {
                const el = document.getElementById('releases-list');
                if (!el || !assetId) return;

                if (!EDIT_ENGINES.length) {
                    // Not fatal. Without it the dropdown still offers "Any
                    // engine", which is a valid answer for every release.
                    const eRes = await fetch('/api/marketplace/engine-versions');
                    EDIT_ENGINES = eRes.ok ? await eRes.json() : [];
                }

                const res = await fetch('/api/marketplace/' + assetId + '/releases');
                if (!res.ok) { el.innerHTML = '<p class="text-sm text-zinc-400 px-1 py-3">Could not load releases.</p>'; return; }
                const releases = await res.json();
                if (!releases.length) {
                    el.innerHTML = '<p class="text-sm text-zinc-400 px-1 py-3">No releases yet.</p>';
                    return;
                }

                el.innerHTML = releases.map(r => `
                    <div class="p-4 bg-white/[0.02] border border-zinc-800/50 rounded-xl" data-release="${r.id}">
                        <div class="flex items-center gap-3 flex-wrap mb-3">
                            <span class="text-base font-semibold text-zinc-100">v${relEsc(r.version)}</span>
                            ${r.is_current ? '<span class="px-2 py-0.5 rounded-full bg-green-500/15 border border-green-500/40 text-xs font-medium text-green-300">Latest</span>' : ''}
                            <span class="text-sm text-zinc-400">${r.file_count} files &middot; ${formatFileSize(r.total_size)} &middot; ${r.downloads} downloads</span>
                            <span class="flex-1"></span>
                            ${r.is_current
                                ? '<span class="text-sm text-zinc-500" title="Publish a newer version before removing this one">Delete</span>'
                                : `<button onclick="deleteRelease('${r.id}','${relEsc(r.version)}')" class="text-sm text-red-400 hover:text-red-300 transition-colors">Delete</button>`}
                        </div>
                        <div class="grid sm:grid-cols-2 gap-4">
                            <div>
                                <label class="block text-sm text-zinc-300 mb-1.5">Built for</label>
                                <select id="eng-${r.id}" data-previous="${relEsc(r.min_engine_version || '')}"
                                    onchange="saveReleaseEngine('${r.id}', this)"
                                    class="w-full px-3 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-lg text-zinc-100 text-sm outline-none focus:border-accent/50 transition-all">${engineOptions(r.min_engine_version || '')}</select>
                                <p class="text-xs text-zinc-500 mt-1">Saves as soon as you change it.</p>
                            </div>
                            <div>
                                <label class="block text-sm text-zinc-300 mb-1.5">Release notes</label>
                                <textarea id="notes-input-${r.id}" rows="3" placeholder="Markdown"
                                    class="w-full px-3 py-2 bg-white/[0.03] border border-zinc-800/50 rounded-lg text-zinc-100 text-sm outline-none focus:border-accent/50 resize-y font-mono">${relEsc(r.notes)}</textarea>
                                <button onclick="saveReleaseNotes('${r.id}')" class="mt-2 px-3 py-1.5 rounded-lg text-sm font-medium bg-white/[0.06] text-zinc-200 hover:bg-white/[0.1] transition-colors">Save notes</button>
                            </div>
                        </div>
                    </div>`).join('');
            }

            // Retarget one release at a different engine, without republishing.
            //
            // The engine a release needs is a claim about code that already
            // shipped, and the usual way it turns out to be wrong is somebody
            // installing it and watching it fail to load. A republish would
            // spend a version number on a metadata fix.
            //
            // Only this release moves. Correcting an old one must not disturb
            // what anybody on a newer engine resolves, which is the whole point
            // of the value living on the release.
            async function saveReleaseEngine(id, sel) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                const previous = sel.dataset.previous ?? '';
                sel.disabled = true;
                try {
                    const res = await fetch('/api/marketplace/' + assetId + '/releases/' + id, {
                        method: 'PUT',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({ min_engine_version: sel.value })
                    });
                    if (!res.ok) {
                        const d = await res.json().catch(() => ({}));
                        throw new Error(d.error || 'Could not change the engine for this release');
                    }
                    sel.dataset.previous = sel.value;
                    showSuccess(sel.value
                        ? 'This release now targets ' + sel.value + ' and newer.'
                        : 'This release now runs on any engine.');
                } catch (e) {
                    // Put the control back to what the server still holds, so it
                    // never shows a value that was not saved.
                    sel.value = previous;
                    showError(e.message);
                } finally {
                    sel.disabled = false;
                }
            }

            async function saveReleaseNotes(id) {
                const token = document.cookie.match('(^|;)\s*token\s*=\s*([^;]+)')?.pop();
                const notes = document.getElementById('notes-input-' + id)?.value ?? '';
                const res = await fetch('/api/marketplace/' + assetId + '/releases/' + id, {
                    method: 'PUT',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ notes })
                });
                if (res.ok) { showSuccess('Release notes saved.'); loadReleasesList(); }
                else { showError('Could not save the release notes.'); }
            }

            async function deleteRelease(id, version) {
                if (!confirm('Delete v' + version + '? Its files are removed for good, including for people who downloaded it.')) return;
                const token = document.cookie.match('(^|;)\s*token\s*=\s*([^;]+)')?.pop();
                const res = await fetch('/api/marketplace/' + assetId + '/releases/' + id, {
                    method: 'DELETE',
                    headers: { 'Authorization': 'Bearer ' + token }
                });
                if (res.ok) { loadReleasesList(); }
                else {
                    const d = await res.json().catch(() => ({}));
                    showError(d.error || 'Could not delete that release.');
                }
            }

            function updateEditPricePreview() {
                const price = parseInt(document.getElementById('edit-price')?.value) || 0;
                const el = document.getElementById('edit-price-preview');
                if (!el) return;
                if (price === 0) {
                    el.textContent = 'Free, anyone can download';
                } else {
                    const usd = (price * 0.10).toFixed(2);
                    const earn = (price * 0.08).toFixed(2);
                    el.textContent = `${price} credits ($${usd}), you earn ${Math.floor(price * 0.8)} credits ($${earn})`;
                }
            }

            function formatFileSize(bytes) {
                if (!bytes) return '';
                if (bytes > 1e9) return (bytes / 1e9).toFixed(1) + ' GB';
                if (bytes > 1e6) return (bytes / 1e6).toFixed(1) + ' MB';
                return (bytes / 1e3).toFixed(0) + ' KB';
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
                        // No `version`. It mirrors the current release and is
                        // only moved by publishing one; sending it from here
                        // could name a version no release has.
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

                    // 2. Thumbnail only. Release files are never replaced from
                    // here: swapping a published version's bytes in place makes
                    // the version number a lie and goes round the publishing
                    // tool that checks the manifest and the engine version.
                    const thumbFile = document.getElementById('edit-thumbnail')?.files[0];
                    if (thumbFile) {
                        const fd = new FormData();
                        fd.append('thumbnail', thumbFile);
                        const fRes = await fetch('/api/marketplace/' + assetId + '/files', {
                            method: 'PUT',
                            headers: { 'Authorization': 'Bearer ' + token },
                            body: fd
                        });
                        if (!fRes.ok) {
                            const d = await fRes.json().catch(() => ({}));
                            throw new Error(d.error || 'Failed to upload the thumbnail');
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
