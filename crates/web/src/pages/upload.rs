use leptos::prelude::*;

#[component]
pub fn UploadPage() -> impl IntoView {
    view! {
        <section class="py-12 px-6">
            <div class="max-w-3xl mx-auto">
                // Header
                <div class="mb-10">
                    <a href="/dashboard" class="inline-flex items-center gap-1.5 text-sm text-zinc-500 hover:text-zinc-300 transition-colors mb-4">
                        <i class="ph ph-arrow-left"></i>" Back to Dashboard"
                    </a>
                    <h1 class="text-3xl font-bold">"Upload Asset"</h1>
                    <p class="text-zinc-400 text-sm mt-2">"Share your creation with the Renzora community."</p>
                </div>

                <div id="upload-error" class="hidden mb-6 p-4 rounded-xl bg-red-500/10 border border-red-500/20 text-red-400 text-sm flex items-center gap-2">
                    <i class="ph ph-warning-circle text-lg"></i>
                    <span id="upload-error-text"></span>
                </div>
                <div id="upload-success" class="hidden mb-6 p-4 rounded-xl bg-green-500/10 border border-green-500/20 text-green-400 text-sm flex items-center gap-2">
                    <i class="ph ph-check-circle text-lg"></i>
                    <span id="upload-success-text"></span>
                </div>

                <form id="upload-form" class="space-y-8" onsubmit="return handleUpload(event)">

                    // ── Basic Info ──
                    <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5">
                        <h2 class="text-base font-semibold flex items-center gap-2">
                            <i class="ph ph-info text-accent"></i>"Basic Information"
                        </h2>

                        <div>
                            <label class="block text-sm text-zinc-400 mb-1.5">"Asset Name"</label>
                            <input type="text" name="name" required maxlength="128" placeholder="My Awesome Plugin" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                        </div>

                        <div>
                            <label class="block text-sm text-zinc-400 mb-1.5">"Description"</label>
                            <textarea name="description" required rows="5" placeholder="Describe what your asset does, what's included, and how to use it..." class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all resize-y"></textarea>
                            <p class="text-xs text-zinc-600 mt-1">"Markdown is not supported. Keep it clear and concise."</p>
                        </div>

                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">"Category"</label>
                                <select name="category" id="upload-category" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all">
                                    <option value="">"Loading categories..."</option>
                                </select>
                            </div>
                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">"Version"</label>
                                <input type="text" name="version" value="1.0.0" placeholder="1.0.0" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                            </div>
                        </div>

                        <div>
                            <label class="block text-sm text-zinc-400 mb-1.5">"Tags"</label>
                            <input type="text" name="tags" placeholder="physics, 3d, multiplayer (comma separated)" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                            <p class="text-xs text-zinc-600 mt-1">"Add up to 5 tags to help people find your asset."</p>
                        </div>
                    </div>

                    // ── Pricing ──
                    <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5">
                        <h2 class="text-base font-semibold flex items-center gap-2">
                            <i class="ph ph-coins text-amber-400"></i>"Pricing"
                        </h2>

                        <div>
                            <label class="block text-sm text-zinc-400 mb-1.5">"Price in credits (0 = free)"</label>
                            <div class="relative">
                                <input type="number" name="price" min="0" value="0" id="price-input" oninput="updatePricePreview()"
                                    class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                            </div>
                            <p class="text-xs text-zinc-600 mt-1" id="price-preview">"Free — anyone can download"</p>
                            <p class="text-xs text-zinc-600 mt-0.5">"You earn 80% of each sale. 1 credit = $0.10 USD."</p>
                        </div>
                    </div>

                    // ── Files ──
                    <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5">
                        <h2 class="text-base font-semibold flex items-center gap-2">
                            <i class="ph ph-file-arrow-up text-cyan-400"></i>"Files"
                        </h2>

                        <div>
                            <label class="block text-sm text-zinc-400 mb-1.5">"Asset File" <span class="text-red-400">"*"</span></label>
                            <div class="relative">
                                <input type="file" name="file" required id="file-input" onchange="updateFileInfo(this)"
                                    class="w-full text-sm text-zinc-400 file:mr-4 file:py-2.5 file:px-5 file:rounded-xl file:border-0 file:text-sm file:font-medium file:bg-accent file:text-white hover:file:bg-accent-hover file:cursor-pointer file:transition-colors" />
                            </div>
                            <p class="text-xs text-zinc-600 mt-1.5" id="file-info">"Accepted formats: .zip, .rar, .7z, .lua, .rhai, .wgsl — Max 50MB"</p>
                        </div>

                        <div>
                            <label class="block text-sm text-zinc-400 mb-1.5">"Cover Thumbnail"</label>
                            <div class="flex items-start gap-4">
                                <div class="w-32 h-20 rounded-xl bg-white/[0.02] border border-zinc-800/50 flex items-center justify-center overflow-hidden shrink-0" id="thumb-preview">
                                    <i class="ph ph-image text-2xl text-zinc-700"></i>
                                </div>
                                <div class="flex-1">
                                    <input type="file" name="thumbnail" accept="image/*" onchange="previewThumb(this)"
                                        class="w-full text-sm text-zinc-400 file:mr-4 file:py-2 file:px-4 file:rounded-xl file:border-0 file:text-sm file:font-medium file:bg-white/[0.05] file:text-zinc-300 hover:file:bg-white/[0.08] file:cursor-pointer file:transition-colors" />
                                    <p class="text-xs text-zinc-600 mt-1">"16:9 ratio recommended (1280x720). PNG or JPG."</p>
                                </div>
                            </div>
                        </div>
                    </div>

                    // ── Screenshots & Videos ──
                    <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5">
                        <h2 class="text-base font-semibold flex items-center gap-2">
                            <i class="ph ph-images text-purple-400"></i>"Screenshots & Videos"
                        </h2>
                        <p class="text-xs text-zinc-500 -mt-2">"Add up to 10 screenshots or video previews. These will appear in the asset gallery."</p>

                        <div>
                            <label class="block text-sm text-zinc-400 mb-1.5">"Screenshots"</label>
                            <input type="file" name="screenshots" accept="image/*" multiple id="screenshots-input" onchange="updateScreenshotCount(this)"
                                class="w-full text-sm text-zinc-400 file:mr-4 file:py-2 file:px-4 file:rounded-xl file:border-0 file:text-sm file:font-medium file:bg-white/[0.05] file:text-zinc-300 hover:file:bg-white/[0.08] file:cursor-pointer file:transition-colors" />
                            <p class="text-xs text-zinc-600 mt-1" id="screenshot-count">"Select multiple images at once. PNG or JPG."</p>
                            <div id="screenshot-previews" class="flex gap-2 mt-3 flex-wrap"></div>
                        </div>

                        <div>
                            <label class="block text-sm text-zinc-400 mb-1.5">"Video Preview URL (optional)"</label>
                            <input type="text" name="video_url" placeholder="https://www.youtube.com/watch?v=... or direct .mp4 link"
                                class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                            <p class="text-xs text-zinc-600 mt-1">"YouTube links are automatically embedded. You can also use a direct video URL."</p>
                        </div>

                        <div>
                            <label class="block text-sm text-zinc-400 mb-1.5">"Audio Previews (optional)"</label>
                            <input type="file" name="audio_previews" accept="audio/mpeg,audio/wav,audio/ogg,audio/flac,.mp3,.wav,.ogg,.flac" multiple id="audio-input"
                                class="w-full text-sm text-zinc-400 file:mr-4 file:py-2 file:px-4 file:rounded-xl file:border-0 file:text-sm file:font-medium file:bg-white/[0.05] file:text-zinc-300 hover:file:bg-white/[0.08] file:cursor-pointer file:transition-colors" />
                            <p class="text-xs text-zinc-600 mt-1">"Upload audio samples for sound effects or music assets. MP3, WAV, OGG, or FLAC."</p>
                        </div>
                    </div>

                    // ── Additional Info ──
                    <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5">
                        <h2 class="text-base font-semibold flex items-center gap-2">
                            <i class="ph ph-gear text-zinc-400"></i>"Additional Information"
                        </h2>

                        <div>
                            <label class="flex items-start gap-3 cursor-pointer select-none">
                                <input type="checkbox" name="ai_generated" class="mt-1 accent-accent w-4 h-4" />
                                <div>
                                    <span class="text-sm text-zinc-300">"This asset was created with AI assistance"</span>
                                    <p class="text-xs text-zinc-600 mt-0.5">"Check this if AI tools were used to generate code, art, or other content in this asset."</p>
                                </div>
                            </label>
                        </div>

                        <div>
                            <label class="block text-sm text-zinc-400 mb-1.5">"Supported Engine Versions"</label>
                            <input type="text" name="engine_versions" placeholder="r1-alpha4+" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                        </div>

                        <div>
                            <label class="block text-sm text-zinc-400 mb-1.5">"License"</label>
                            <select name="license" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all">
                                <option value="standard">"Standard Marketplace License"</option>
                                <option value="mit">"MIT"</option>
                                <option value="apache2">"Apache 2.0"</option>
                                <option value="gpl3">"GPL 3.0"</option>
                                <option value="cc-by">"CC BY 4.0"</option>
                                <option value="cc0">"CC0 (Public Domain)"</option>
                            </select>
                        </div>
                    </div>

                    // ── Submit ──
                    <div class="flex items-center gap-4">
                        <button type="submit" id="upload-btn" class="flex-1 inline-flex items-center justify-center gap-2 px-6 py-3.5 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)]">
                            <i class="ph ph-upload-simple text-lg"></i>"Upload Asset"
                        </button>
                    </div>

                    <p class="text-xs text-zinc-600 text-center">"Your asset will be submitted as a draft. You can publish it from your dashboard after review."</p>
                </form>
            </div>
        </section>

        <script>
            r#"
            // Check onboarding status before allowing upload
            (async function() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }

                try {
                    const res = await fetch('/api/creator/onboard-status', { headers: { 'Authorization': 'Bearer ' + token } });
                    if (res.ok) {
                        const data = await res.json();
                        if (!data.policy_accepted) {
                            window.location.href = '/marketplace/sell';
                            return;
                        }
                    }
                } catch(e) {}

                const res = await fetch('/api/marketplace/categories');
                if (!res.ok) return;
                const cats = await res.json();
                const sel = document.getElementById('upload-category');
                sel.innerHTML = cats.map(c => `<option value="${c.slug}">${c.name}</option>`).join('');
            })();

            function updatePricePreview() {
                const price = parseInt(document.getElementById('price-input').value) || 0;
                const el = document.getElementById('price-preview');
                if (price === 0) {
                    el.textContent = 'Free — anyone can download';
                } else {
                    const usd = (price * 0.10).toFixed(2);
                    const earn = (price * 0.08).toFixed(2);
                    el.textContent = `${price} credits ($${usd}) — you earn ${Math.floor(price * 0.8)} credits ($${earn})`;
                }
            }

            function updateFileInfo(input) {
                const el = document.getElementById('file-info');
                if (input.files[0]) {
                    const f = input.files[0];
                    const sizeMB = (f.size / 1024 / 1024).toFixed(1);
                    const ext = f.name.split('.').pop().toLowerCase();
                    el.innerHTML = `<span class="text-zinc-300">${f.name}</span> <span class="text-zinc-600">(${sizeMB} MB)</span>`;
                    if (f.size > 50 * 1024 * 1024) {
                        el.innerHTML += ' <span class="text-red-400">— exceeds 50MB limit</span>';
                    }
                }
            }

            function previewThumb(input) {
                const el = document.getElementById('thumb-preview');
                if (input.files[0]) {
                    const url = URL.createObjectURL(input.files[0]);
                    el.innerHTML = `<img src="${url}" class="w-full h-full object-cover" />`;
                }
            }

            function updateScreenshotCount(input) {
                const el = document.getElementById('screenshot-count');
                const previews = document.getElementById('screenshot-previews');
                const count = input.files.length;
                el.textContent = count > 0 ? `${count} screenshot${count !== 1 ? 's' : ''} selected` : 'Select multiple images at once. PNG or JPG.';
                previews.innerHTML = '';
                for (let i = 0; i < Math.min(count, 10); i++) {
                    const url = URL.createObjectURL(input.files[i]);
                    previews.innerHTML += `<div class="w-20 h-14 rounded-lg overflow-hidden border border-zinc-800/50 shrink-0"><img src="${url}" class="w-full h-full object-cover" /></div>`;
                }
                if (count > 10) {
                    previews.innerHTML += `<div class="w-20 h-14 rounded-lg bg-zinc-800/50 flex items-center justify-center text-xs text-zinc-500">+${count - 10}</div>`;
                }
            }

            async function handleUpload(e) {
                e.preventDefault();
                const form = e.target;
                const btn = document.getElementById('upload-btn');
                const err = document.getElementById('upload-error');
                const errText = document.getElementById('upload-error-text');
                const ok = document.getElementById('upload-success');
                const okText = document.getElementById('upload-success-text');
                err.classList.add('hidden');
                ok.classList.add('hidden');

                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { errText.textContent = 'Please sign in first'; err.classList.remove('hidden'); return false; }

                if (!form.file.files[0]) { errText.textContent = 'Please select an asset file'; err.classList.remove('hidden'); return false; }

                btn.disabled = true;
                btn.innerHTML = '<i class="ph ph-spinner text-lg animate-spin"></i> Uploading...';

                const tags = form.tags.value.split(',').map(t => t.trim()).filter(t => t).slice(0, 5);

                const metadata = JSON.stringify({
                    name: form.name.value,
                    description: form.description.value,
                    category: form.category.value,
                    price_credits: parseInt(form.price.value) || 0,
                    version: form.version.value
                });

                const fd = new FormData();
                fd.append('metadata', metadata);
                fd.append('file', form.file.files[0]);
                if (form.thumbnail.files[0]) fd.append('thumbnail', form.thumbnail.files[0]);

                try {
                    // Upload asset
                    const res = await fetch('/api/marketplace/upload', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token },
                        body: fd
                    });
                    const data = await res.json();
                    if (!res.ok) throw new Error(data.error || 'Upload failed');

                    const assetId = data.id;

                    // Upload screenshots as gallery media
                    const screenshots = form.screenshots.files;
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

                    // Upload video URL as gallery media
                    const videoUrl = form.video_url.value.trim();
                    if (videoUrl) {
                        const vfd = new FormData();
                        vfd.append('video_url', videoUrl);
                        await fetch('/api/marketplace/' + assetId + '/media', {
                            method: 'POST',
                            headers: { 'Authorization': 'Bearer ' + token },
                            body: vfd
                        });
                    }

                    // Upload audio previews as gallery media
                    const audioFiles = document.getElementById('audio-input')?.files || [];
                    for (let i = 0; i < Math.min(audioFiles.length, 10); i++) {
                        const afd = new FormData();
                        afd.append('media_type', 'audio');
                        afd.append('file', audioFiles[i]);
                        await fetch('/api/marketplace/' + assetId + '/media', {
                            method: 'POST',
                            headers: { 'Authorization': 'Bearer ' + token },
                            body: afd
                        });
                    }

                    okText.textContent = 'Asset uploaded successfully! You can publish it from your dashboard.';
                    ok.classList.remove('hidden');
                    form.reset();
                    document.getElementById('thumb-preview').innerHTML = '<i class="ph ph-image text-2xl text-zinc-700"></i>';
                    document.getElementById('screenshot-previews').innerHTML = '';
                    document.getElementById('screenshot-count').textContent = 'Select multiple images at once. PNG or JPG.';
                    document.getElementById('price-preview').textContent = 'Free — anyone can download';
                    document.getElementById('file-info').textContent = 'Accepted formats: .zip, .rar, .7z, .lua, .rhai, .wgsl — Max 50MB';

                    // Scroll to top to show success
                    window.scrollTo({ top: 0, behavior: 'smooth' });
                } catch (error) {
                    errText.textContent = error.message;
                    err.classList.remove('hidden');
                }
                btn.disabled = false;
                btn.innerHTML = '<i class="ph ph-upload-simple text-lg"></i> Upload Asset';
                return false;
            }
            "#
        </script>
    }
}
