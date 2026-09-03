use leptos::prelude::*;

/// Publish an asset. One page, one form. Files come first because picking the
/// file is what the seller actually came to do, and it fills in the download
/// name for them. Version isn't asked for — every asset starts at 1.0.0 and
/// moves on through updates.
#[component]
pub fn UploadPage() -> impl IntoView {
    view! {
        <section class="py-12 px-6 min-h-[80vh]">
            <div class="max-w-3xl mx-auto">

                // ── Auth gate ──
                <div id="auth-required" class="hidden text-center py-20">
                    <div class="w-16 h-16 bg-zinc-800/50 rounded-full flex items-center justify-center mx-auto mb-4">
                        <i class="ph ph-lock text-2xl text-zinc-500"></i>
                    </div>
                    <p class="text-zinc-400 mb-4">"Sign in to publish content"</p>
                    <a href="/login" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all">"Sign in"</a>
                </div>

                <div id="upload-form" class="hidden">

                    <div class="mb-8">
                        <a href="/dashboard" class="inline-flex items-center gap-1.5 text-sm text-zinc-500 hover:text-zinc-300 transition-colors mb-4">
                            <i class="ph ph-arrow-left"></i>" Back to Dashboard"
                        </a>
                        <h1 class="text-3xl font-bold">"Publish an Asset"</h1>
                        <p class="text-zinc-400 text-sm mt-2">"3D models, scripts, audio, textures, plugins and more. Fields marked * are required."</p>
                    </div>

                    <div id="wizard-error" class="hidden mb-6 p-4 rounded-xl bg-red-500/10 border border-red-500/20 text-red-400 text-sm flex items-center gap-2">
                        <i class="ph ph-warning-circle text-lg"></i>
                        <span id="wizard-error-text"></span>
                    </div>
                    <div id="wizard-success" class="hidden mb-6 p-4 rounded-xl bg-green-500/10 border border-green-500/20 text-green-400 text-sm flex items-center gap-2">
                        <i class="ph ph-check-circle text-lg"></i>
                        <span id="wizard-success-text"></span>
                    </div>

                    <div class="space-y-6">

                        // ════════════════════════════════════════
                        // 1. The file itself
                        // ════════════════════════════════════════
                        <div class="p-6 md:p-8 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5">
                            <div>
                                <h2 class="text-base font-semibold flex items-center gap-2">
                                    <i class="ph ph-file-arrow-up text-cyan-400"></i>"Your file"
                                </h2>
                                <p class="text-xs text-zinc-600 mt-1">"Start here — everything else describes this file."</p>
                            </div>

                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">"File" <span class="text-red-400">"*"</span></label>
                                <div id="file-dropzone" class="relative border-2 border-dashed border-zinc-800/50 rounded-xl p-8 text-center hover:border-accent/30 transition-all cursor-pointer"
                                    onclick="document.getElementById('w-file').click()">
                                    <i class="ph ph-file-arrow-up text-2xl text-zinc-600 mb-2"></i>
                                    <p id="file-drop-label" class="text-sm text-zinc-500">"Drop a file or click to browse"</p>
                                    <p class="text-xs text-zinc-600 mt-2">"Accepted formats vary by category — max 50 MB"</p>
                                    <input type="file" id="w-file" class="hidden" onchange="previewMainFile(this)"
                                        accept=".zip,.rar,.7z,.lua,.rhai,.rs,.wgsl,.glsl,.fbx,.obj,.gltf,.glb,.blend,.png,.jpg,.svg,.hdr,.exr,.mp4,.webm,.mov,.wav,.ogg,.mp3,.flac,.ttf,.otf,.json,.ron" />
                                </div>
                                <p id="download-name-hint" class="hidden text-xs text-zinc-600 mt-1.5"></p>
                            </div>

                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">"Cover Image"</label>
                                <p class="text-xs text-zinc-600 mb-2">"Recommended: 1280x720 (16:9). PNG or JPG."</p>
                                <div id="thumb-dropzone" class="relative border-2 border-dashed border-zinc-800/50 rounded-xl p-6 text-center hover:border-accent/30 transition-all cursor-pointer"
                                    onclick="document.getElementById('w-thumbnail').click()">
                                    <i class="ph ph-image text-2xl text-zinc-600 mb-2" id="thumb-icon"></i>
                                    <p id="thumb-label" class="text-sm text-zinc-500">"Drop an image or click to browse"</p>
                                    <img id="thumb-preview" class="hidden mt-3 max-h-40 mx-auto rounded-lg" />
                                    <input type="file" id="w-thumbnail" accept="image/*" class="hidden" onchange="previewThumb(this)" />
                                </div>
                            </div>

                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">"Screenshots"</label>
                                <input type="file" id="w-screenshots" accept="image/*" multiple onchange="updateScreenshotCount(this)"
                                    class="w-full text-sm text-zinc-400 file:mr-4 file:py-2 file:px-4 file:rounded-xl file:border-0 file:text-sm file:font-medium file:bg-white/[0.05] file:text-zinc-300 hover:file:bg-white/[0.08] file:cursor-pointer file:transition-colors" />
                                <p class="text-xs text-zinc-600 mt-1" id="screenshot-count">"Up to 10 images, shown in the gallery. PNG or JPG."</p>
                                <div id="screenshot-previews" class="flex gap-2 mt-3 flex-wrap"></div>
                            </div>

                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">"Video Preview URL"</label>
                                <input type="text" id="w-video-url" placeholder="https://www.youtube.com/watch?v=... or direct .mp4 link"
                                    class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                                <p class="text-xs text-zinc-600 mt-1">"YouTube links are automatically embedded."</p>
                            </div>

                            // Audio previews only make sense for music.
                            <div data-show-for-category="music" class="hidden">
                                <label class="block text-sm text-zinc-400 mb-1.5">"Audio Previews"</label>
                                <input type="file" id="w-audio" accept="audio/mpeg,audio/wav,audio/ogg,audio/flac,.mp3,.wav,.ogg,.flac" multiple
                                    class="w-full text-sm text-zinc-400 file:mr-4 file:py-2 file:px-4 file:rounded-xl file:border-0 file:text-sm file:font-medium file:bg-white/[0.05] file:text-zinc-300 hover:file:bg-white/[0.08] file:cursor-pointer file:transition-colors" />
                                <p class="text-xs text-zinc-600 mt-1">"Let buyers listen before they buy. MP3, WAV, OGG or FLAC."</p>
                            </div>
                        </div>

                        // ════════════════════════════════════════
                        // 2. What it is
                        // ════════════════════════════════════════
                        <div class="p-6 md:p-8 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5">
                            <h2 class="text-base font-semibold flex items-center gap-2">
                                <i class="ph ph-info text-accent"></i>"Basics"
                            </h2>

                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">"Name" <span class="text-red-400">"*"</span></label>
                                <input type="text" id="w-name" required maxlength="128" placeholder="My Awesome Creation" oninput="updateDownloadNameHint()"
                                    class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                            </div>

                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">"Category" <span class="text-red-400">"*"</span></label>
                                <select id="w-category" onchange="applyCategoryFields()"
                                    class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all">
                                    <option value="">"Loading categories..."</option>
                                </select>
                            </div>

                            // Category-specific fields sit directly under the category
                            // that reveals them.
                            <div data-show-for-category="music" class="hidden space-y-4 p-4 bg-white/[0.01] border border-zinc-800/30 rounded-xl">
                                <p class="text-xs text-zinc-500 uppercase tracking-wider font-medium">"Music Details"</p>
                                <div class="grid grid-cols-2 gap-4">
                                    <div>
                                        <label class="block text-sm text-zinc-400 mb-1.5">"BPM"</label>
                                        <input type="number" id="w-bpm" min="1" max="999" placeholder="120"
                                            class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                                    </div>
                                    <div>
                                        <label class="block text-sm text-zinc-400 mb-1.5">"Genre"</label>
                                        <select id="w-genre" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all">
                                            <option value="">"Select genre..."</option>
                                            <option value="ambient">"Ambient"</option>
                                            <option value="orchestral">"Orchestral"</option>
                                            <option value="electronic">"Electronic"</option>
                                            <option value="retro">"Retro / Chiptune"</option>
                                            <option value="rock">"Rock"</option>
                                            <option value="cinematic">"Cinematic"</option>
                                            <option value="other">"Other"</option>
                                        </select>
                                    </div>
                                </div>
                                <label class="flex items-center gap-3 cursor-pointer select-none">
                                    <input type="checkbox" id="w-loopable" class="accent-accent w-4 h-4" />
                                    <span class="text-sm text-zinc-300">"Loop-friendly (seamless loop)"</span>
                                </label>
                            </div>

                            <div data-show-for-category="scripts,plugins,blueprints" class="hidden p-4 bg-white/[0.01] border border-zinc-800/30 rounded-xl">
                                <label class="block text-sm text-zinc-400 mb-1.5">"Scripting Language"</label>
                                <select id="w-script-lang" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all">
                                    <option value="">"Select..."</option>
                                    <option value="rust">"Rust"</option>
                                    <option value="lua">"Lua"</option>
                                    <option value="rhai">"Rhai"</option>
                                    <option value="wgsl">"WGSL (Shader)"</option>
                                    <option value="blueprint">"Visual Blueprint"</option>
                                    <option value="other">"Other"</option>
                                </select>
                            </div>

                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">"Description" <span class="text-red-400">"*"</span></label>
                                <textarea id="w-description" required rows="5" placeholder="Describe what this is, what's included, and how to use it..."
                                    class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all resize-y"></textarea>
                                <p class="text-xs text-zinc-600 mt-1">"Markdown is not supported. Keep it clear and concise."</p>
                            </div>

                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">"Price (credits)"</label>
                                <input type="number" id="w-price" min="0" value="0" oninput="updatePricePreview()"
                                    class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                                <p class="text-xs text-zinc-600 mt-1" id="price-preview">"Free, anyone can download"</p>
                                <p class="text-xs text-zinc-600 mt-0.5">"You earn 80% of each sale. 1 credit = $0.10 USD."</p>
                            </div>

                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">"Tags"</label>
                                <div class="relative">
                                    <div id="tags-pills" class="flex flex-wrap gap-1.5 mb-2"></div>
                                    <input type="text" id="w-tags-input" placeholder="Type to search tags..." autocomplete="off"
                                        class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                                    <input type="hidden" id="w-tags" />
                                    <div id="tags-dropdown" class="hidden absolute left-0 right-0 top-full mt-1 bg-zinc-900 border border-zinc-700 rounded-xl shadow-lg z-50 max-h-48 overflow-y-auto"></div>
                                </div>
                                <p class="text-xs text-zinc-600 mt-1">"Add up to 5 tags. Press comma or click a suggestion. New tags are submitted for review."</p>
                            </div>

                            <div class="grid grid-cols-2 gap-4">
                                <div>
                                    <label class="block text-sm text-zinc-400 mb-1.5">"Minimum Engine Version"</label>
                                    <select id="w-engine-versions" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all">
                                        <option value="">"Any version"</option>
                                    </select>
                                </div>
                                <div>
                                    <label class="block text-sm text-zinc-400 mb-1.5">"License"</label>
                                    <select id="w-license" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all">
                                        <option value="standard">"Standard Marketplace License"</option>
                                        <option value="extended">"Extended License"</option>
                                        <option value="mit">"MIT"</option>
                                        <option value="apache2">"Apache 2.0"</option>
                                        <option value="gpl3">"GPL 3.0"</option>
                                        <option value="cc0">"CC0 (Public Domain)"</option>
                                    </select>
                                </div>
                            </div>

                            <label class="flex items-start gap-3 cursor-pointer select-none">
                                <input type="checkbox" id="w-ai-generated" class="mt-1 accent-accent w-4 h-4" />
                                <div>
                                    <span class="text-sm text-zinc-300">"This asset was created with AI assistance"</span>
                                    <p class="text-xs text-zinc-600 mt-0.5">"Check this if AI tools were used to generate content in this asset."</p>
                                </div>
                            </label>
                        </div>

                        // ════════════════════════════════════════
                        // 3. Attribution
                        // ════════════════════════════════════════
                        <div class="p-6 md:p-8 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-4">
                            <h2 class="text-base font-semibold flex items-center gap-2">
                                <i class="ph ph-heart text-accent"></i>"Credit / Attribution"
                            </h2>
                            <p class="text-xs text-zinc-600">"If this asset is from another creator, credit them here. Credited assets are automatically free."</p>
                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">"Original Creator Name"</label>
                                <input type="text" id="w-credit-name" placeholder="e.g. KayKit, Kenney" oninput="updateCreditState()"
                                    class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                            </div>
                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">"Creator Website / Source Link"</label>
                                <input type="text" id="w-credit-url" placeholder="https://kaykit.itch.io"
                                    class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                            </div>
                            <div id="credit-free-notice" class="hidden p-3 bg-green-500/5 border border-green-500/10 rounded-lg">
                                <p class="text-xs text-green-400 flex items-center gap-1.5">
                                    <i class="ph ph-info"></i>
                                    "This asset will be published as free because it credits another creator."
                                </p>
                            </div>
                        </div>
                    </div>

                    <button type="button" onclick="handleSubmit()" id="publish-btn"
                        class="w-full mt-6 inline-flex items-center justify-center gap-2 px-5 py-3 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)]">
                        <i class="ph ph-rocket-launch text-lg"></i>"Publish"
                    </button>

                    <p class="text-xs text-zinc-600 text-center mt-3">"Published at version 1.0.0. By publishing, you agree to the Renzora "<a href="/docs/marketplace/publishing" class="text-accent hover:text-accent-hover">"content guidelines"</a>"."</p>
                </div>
            </div>
        </section>

        <script>
        r##"
        // Only the category needs tracking now that the form is a single page —
        // every other value is read straight off its input at submit time.
        let selectedCategory = '';

        async function safeJson(res) {
            const text = await res.text();
            if (!text) return {};
            try { return JSON.parse(text); }
            catch(e) { return { error: text.substring(0, 200) }; }
        }

        function escHtml(str) {
            const div = document.createElement('div');
            div.textContent = str;
            return div.innerHTML;
        }

        function showError(msg) {
            const el = document.getElementById('wizard-error');
            document.getElementById('wizard-error-text').textContent = msg;
            el.classList.remove('hidden');
            el.scrollIntoView({ behavior: 'smooth', block: 'center' });
        }

        // ──────────────────────────────────────
        // Category
        // ──────────────────────────────────────
        async function loadCategories() {
            const sel = document.getElementById('w-category');
            try {
                const res = await fetch('/api/marketplace/categories');
                if (!res.ok) throw new Error('Failed to load');
                const cats = await safeJson(res);
                if (!Array.isArray(cats)) throw new Error('Invalid response');
                sel.innerHTML = '<option value="">Select a category...</option>';
                cats.forEach(cat => {
                    const opt = document.createElement('option');
                    opt.value = cat.slug;
                    opt.textContent = cat.name;
                    sel.appendChild(opt);
                });
            } catch (e) {
                sel.innerHTML = '<option value="">Failed to load categories</option>';
            }
        }

        // Reveal only the detail fields that belong to the chosen category.
        function applyCategoryFields() {
            selectedCategory = document.getElementById('w-category').value;
            document.querySelectorAll('[data-show-for-category]').forEach(el => {
                const cats = el.dataset.showForCategory.split(',');
                el.classList.toggle('hidden', !cats.includes(selectedCategory));
            });
        }

        async function loadEngineVersions() {
            const sel = document.getElementById('w-engine-versions');
            try {
                const res = await fetch('/api/docs/versions');
                if (!res.ok) return;
                const d = await res.json();
                (d.versions || []).forEach(v => {
                    // Nightlies are never offered as a support target.
                    if (/nightly/i.test(v.id)) return;
                    const opt = document.createElement('option');
                    opt.value = v.id;
                    opt.textContent = v.label + (v.status === 'current' ? ' (current)' : '');
                    sel.appendChild(opt);
                });
            } catch (e) {}
        }

        // ──────────────────────────────────────
        // Download filename, derived from the title
        // ──────────────────────────────────────
        function slugify(s) {
            return s.toLowerCase().trim()
                .replace(/[^a-z0-9]+/g, '-')
                .replace(/^-+|-+$/g, '')
                .slice(0, 64);
        }
        // Buyers download "<asset-title>.<ext>" rather than whatever the seller
        // happened to call the file on disk.
        function downloadName() {
            const title = slugify(document.getElementById('w-name').value) || 'asset';
            const f = document.getElementById('w-file').files[0];
            const ext = f && f.name.includes('.') ? f.name.slice(f.name.lastIndexOf('.')) : '';
            return title + ext;
        }
        function updateDownloadNameHint() {
            const hint = document.getElementById('download-name-hint');
            const f = document.getElementById('w-file').files[0];
            if (!f) { hint.classList.add('hidden'); return; }
            hint.textContent = 'Buyers will download: ' + downloadName();
            hint.classList.remove('hidden');
        }

        // ──────────────────────────────────────
        // Price + credit/attribution
        // ──────────────────────────────────────
        function updatePricePreview() {
            const price = parseInt(document.getElementById('w-price').value) || 0;
            const el = document.getElementById('price-preview');
            if (price === 0) {
                el.textContent = 'Free — anyone can download';
            } else {
                const usd = (price * 0.10).toFixed(2);
                const earn = (price * 0.08).toFixed(2);
                el.textContent = price + ' credits ($' + usd + ') — you earn ' + Math.floor(price * 0.8) + ' credits ($' + earn + ')';
            }
        }

        function updateCreditState() {
            const creditName = document.getElementById('w-credit-name').value.trim();
            const notice = document.getElementById('credit-free-notice');
            const priceInput = document.getElementById('w-price');
            if (creditName) {
                notice.classList.remove('hidden');
                priceInput.value = '0';
                priceInput.disabled = true;
                updatePricePreview();
            } else {
                notice.classList.add('hidden');
                priceInput.disabled = false;
            }
        }

        // ──────────────────────────────────────
        // Tag autocomplete
        // ──────────────────────────────────────
        const selectedTags = [];
        let tagSearchTimeout = null;

        function renderTagPills() {
            const container = document.getElementById('tags-pills');
            const hidden = document.getElementById('w-tags');
            container.innerHTML = '';
            selectedTags.forEach((tag, i) => {
                const pill = document.createElement('span');
                pill.className = 'inline-flex items-center gap-1 px-2.5 py-1 bg-accent/15 text-accent text-xs font-medium rounded-lg';
                pill.innerHTML = escHtml(tag) + ' <button type="button" class="hover:text-white ml-0.5" onclick="removeTag(' + i + ')">&times;</button>';
                container.appendChild(pill);
            });
            hidden.value = selectedTags.join(',');
        }

        function addTag(name) {
            const clean = name.trim().toLowerCase();
            if (!clean || selectedTags.length >= 5 || selectedTags.includes(clean)) return;
            selectedTags.push(clean);
            renderTagPills();
            document.getElementById('w-tags-input').value = '';
            document.getElementById('tags-dropdown').classList.add('hidden');
        }

        function removeTag(index) {
            selectedTags.splice(index, 1);
            renderTagPills();
        }

        async function searchTags(query) {
            const dropdown = document.getElementById('tags-dropdown');
            if (!query || query.length < 1) { dropdown.classList.add('hidden'); return; }
            try {
                const res = await fetch('/api/marketplace/tags?q=' + encodeURIComponent(query));
                const tags = await safeJson(res);
                if (!Array.isArray(tags) || tags.length === 0) {
                    dropdown.innerHTML = '<div class="px-3 py-2 text-xs text-zinc-500">No matching tags</div>'
                        + '<button type="button" class="w-full px-3 py-2 text-left text-sm text-accent hover:bg-white/[0.05] transition-colors" onclick="submitNewTag(\'' + escHtml(query).replace(/'/g, "\\'") + '\')">+ Submit &quot;' + escHtml(query) + '&quot; as new tag</button>';
                    dropdown.classList.remove('hidden');
                    return;
                }
                dropdown.innerHTML = '';
                tags.forEach(t => {
                    if (selectedTags.includes(t.name)) return;
                    const btn = document.createElement('button');
                    btn.type = 'button';
                    btn.className = 'w-full px-3 py-2 text-left text-sm text-zinc-200 hover:bg-white/[0.05] transition-colors';
                    btn.textContent = t.name;
                    btn.onclick = () => addTag(t.name);
                    dropdown.appendChild(btn);
                });
                const names = tags.map(t => t.name.toLowerCase());
                if (!names.includes(query.toLowerCase())) {
                    const btn = document.createElement('button');
                    btn.type = 'button';
                    btn.className = 'w-full px-3 py-2 text-left text-sm text-accent hover:bg-white/[0.05] transition-colors border-t border-zinc-800';
                    btn.innerHTML = '+ Submit &quot;' + escHtml(query) + '&quot; as new tag';
                    btn.onclick = () => submitNewTag(query);
                    dropdown.appendChild(btn);
                }
                dropdown.classList.remove('hidden');
            } catch (e) {
                dropdown.classList.add('hidden');
            }
        }

        async function submitNewTag(name) {
            const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
            if (!token) return;
            try {
                const res = await fetch('/api/marketplace/tags/submit', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ name: name.trim() })
                });
                const data = await safeJson(res);
                if (res.ok) addTag(data.name || name.trim());
            } catch (e) {}
        }

        document.getElementById('w-tags-input').addEventListener('input', function(e) {
            clearTimeout(tagSearchTimeout);
            const val = e.target.value;
            if (val.includes(',')) {
                const parts = val.split(',');
                parts.forEach((p, i) => {
                    if (i < parts.length - 1 && p.trim()) addTag(p.trim());
                });
                e.target.value = parts[parts.length - 1];
                return;
            }
            tagSearchTimeout = setTimeout(() => searchTags(val.trim()), 200);
        });

        document.getElementById('w-tags-input').addEventListener('keydown', function(e) {
            if (e.key === 'Backspace' && !e.target.value && selectedTags.length > 0) {
                removeTag(selectedTags.length - 1);
            }
        });

        document.addEventListener('click', function(e) {
            const dropdown = document.getElementById('tags-dropdown');
            const input = document.getElementById('w-tags-input');
            if (dropdown && !dropdown.contains(e.target) && e.target !== input) {
                dropdown.classList.add('hidden');
            }
        });

        // ──────────────────────────────────────
        // File pickers
        // ──────────────────────────────────────
        function previewMainFile(input) {
            const label = document.getElementById('file-drop-label');
            if (input.files[0]) {
                const f = input.files[0];
                const sizeMB = (f.size / 1024 / 1024).toFixed(1);
                label.innerHTML = '<strong>' + escHtml(f.name) + '</strong> <span class="text-zinc-600">(' + sizeMB + ' MB)</span>';
                if (f.size > 50 * 1024 * 1024) {
                    label.innerHTML += ' <span class="text-red-400">— exceeds 50MB limit</span>';
                }
                updateDownloadNameHint();
            }
        }

        function previewThumb(input) {
            const preview = document.getElementById('thumb-preview');
            const icon = document.getElementById('thumb-icon');
            const label = document.getElementById('thumb-label');
            if (input.files && input.files[0]) {
                preview.src = URL.createObjectURL(input.files[0]);
                preview.classList.remove('hidden');
                if (icon) icon.classList.add('hidden');
                if (label) label.classList.add('hidden');
            }
        }

        function updateScreenshotCount(input) {
            const el = document.getElementById('screenshot-count');
            const previews = document.getElementById('screenshot-previews');
            const count = input.files.length;
            el.textContent = count > 0
                ? count + ' screenshot' + (count !== 1 ? 's' : '') + ' selected'
                : 'Up to 10 images, shown in the gallery. PNG or JPG.';
            previews.innerHTML = '';
            for (let i = 0; i < Math.min(count, 10); i++) {
                const url = URL.createObjectURL(input.files[i]);
                previews.innerHTML += '<div class="w-20 h-14 rounded-lg overflow-hidden border border-zinc-800/50 shrink-0"><img src="' + url + '" class="w-full h-full object-cover" /></div>';
            }
            if (count > 10) {
                previews.innerHTML += '<div class="w-20 h-14 rounded-lg bg-zinc-800/50 flex items-center justify-center text-xs text-zinc-500">+' + (count - 10) + '</div>';
            }
        }

        // Gather the category-specific detail fields into the free-form
        // `metadata` object the API stores alongside the asset. Only non-empty
        // values are sent, and only for the fields the chosen category shows.
        function collectDetailMetadata() {
            const meta = {};
            const put = (key, id) => {
                const el = document.getElementById(id);
                if (!el) return;
                const v = el.value.trim();
                if (v) meta[key] = v;
            };
            const putBool = (key, id) => {
                const el = document.getElementById(id);
                if (el && el.checked) meta[key] = true;
            };

            put('min_engine_version', 'w-engine-versions');

            const shown = (cats) => cats.includes(selectedCategory);
            if (shown(['music'])) {
                put('bpm', 'w-bpm');
                put('genre', 'w-genre');
                putBool('loopable', 'w-loopable');
            }
            if (shown(['scripts', 'plugins', 'blueprints'])) {
                put('script_language', 'w-script-lang');
            }
            return meta;
        }

        // ──────────────────────────────────────
        // Submit
        // ──────────────────────────────────────
        async function handleSubmit() {
            const errEl = document.getElementById('wizard-error');
            const okEl = document.getElementById('wizard-success');
            errEl.classList.add('hidden');
            okEl.classList.add('hidden');

            const name = document.getElementById('w-name').value.trim();
            const description = document.getElementById('w-description').value.trim();
            const category = document.getElementById('w-category').value;
            const file = document.getElementById('w-file').files[0];

            // Everything is on screen, so validate it all at once rather than
            // gating each group the way the old step flow did.
            if (!name) return showError('Name is required.');
            if (!category) return showError('Please choose a category.');
            if (!description) return showError('Description is required.');
            if (!file) return showError('Please select a file to upload.');

            const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
            if (!token) return showError('Please sign in first.');

            const btn = document.getElementById('publish-btn');
            const originalHtml = btn.innerHTML;
            btn.innerHTML = '<div class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div> Publishing...';
            btn.disabled = true;

            try {
                const thumbnail = document.getElementById('w-thumbnail').files[0];
                const screenshots = document.getElementById('w-screenshots').files;

                const metaObj = {
                    name: name,
                    description: description,
                    category: category,
                    price_credits: parseInt(document.getElementById('w-price').value) || 0,
                    version: '1.0.0',
                    tags: selectedTags,
                    download_filename: downloadName(),
                    licence: document.getElementById('w-license').value,
                    ai_generated: document.getElementById('w-ai-generated').checked,
                    metadata: collectDetailMetadata()
                };

                const creditName = document.getElementById('w-credit-name').value.trim();
                if (creditName) {
                    metaObj.credit_name = creditName;
                    metaObj.credit_url = document.getElementById('w-credit-url').value.trim();
                    metaObj.price_credits = 0;
                }

                const fd = new FormData();
                fd.append('metadata', JSON.stringify(metaObj));
                fd.append('file', file);
                if (thumbnail) fd.append('thumbnail', thumbnail);

                const headers = { 'Authorization': 'Bearer ' + token };
                const res = await fetch('/api/marketplace/upload', { method: 'POST', headers: headers, body: fd });
                const data = await safeJson(res);
                if (!res.ok) throw new Error(data.error || 'Upload failed');

                const itemId = data.id;

                for (let i = 0; i < Math.min(screenshots.length, 10); i++) {
                    const mfd = new FormData();
                    mfd.append('media_type', 'image');
                    mfd.append('file', screenshots[i]);
                    await fetch('/api/marketplace/' + itemId + '/media', { method: 'POST', headers: headers, body: mfd });
                }

                const videoUrl = document.getElementById('w-video-url').value.trim();
                if (videoUrl) {
                    const vfd = new FormData();
                    vfd.append('video_url', videoUrl);
                    await fetch('/api/marketplace/' + itemId + '/media', { method: 'POST', headers: headers, body: vfd });
                }

                const audioFiles = document.getElementById('w-audio').files || [];
                for (let i = 0; i < Math.min(audioFiles.length, 10); i++) {
                    const afd = new FormData();
                    afd.append('media_type', 'audio');
                    afd.append('file', audioFiles[i]);
                    await fetch('/api/marketplace/' + itemId + '/media', { method: 'POST', headers: headers, body: afd });
                }

                const assetLink = '/marketplace/asset/' + data.slug;
                document.getElementById('wizard-success-text').innerHTML =
                    'Asset published! <a href="' + assetLink + '" class="underline">View your asset <i class="ph ph-arrow-right"></i></a>';
                okEl.classList.remove('hidden');
                window.scrollTo({ top: 0, behavior: 'smooth' });
            } catch (error) {
                showError(error.message);
            }

            btn.innerHTML = originalHtml;
            btn.disabled = false;
        }

        // ──────────────────────────────────────
        // Auth & init
        // ──────────────────────────────────────
        (async function init() {
            let authed = false;
            try {
                const res = await fetch('/api/auth/me', { credentials: 'include' });
                if (res.ok) authed = true;
            } catch(e) {}
            if (document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop()) authed = true;

            if (!authed) {
                document.getElementById('auth-required').classList.remove('hidden');
                return;
            }

            document.getElementById('upload-form').classList.remove('hidden');
            loadCategories();
            loadEngineVersions();
        })();
        "##
        </script>
    }
}
