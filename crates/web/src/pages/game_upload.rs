use leptos::prelude::*;

#[component]
pub fn GameUploadPage() -> impl IntoView {
    view! {
        <section class="relative py-8 px-6 min-h-[80vh]">
            <div class="max-w-[700px] mx-auto">
                <div class="mb-8">
                    <h1 class="text-2xl font-bold">"Publish a Game"</h1>
                    <p class="text-zinc-500 text-sm mt-1">"Upload your game, set a price, and share it with the Renzora community."</p>
                </div>

                // Auth check
                <div id="upload-auth-required" class="hidden text-center py-16">
                    <div class="text-4xl mb-3 opacity-30">"🔒"</div>
                    <p class="text-zinc-500 mb-4">"Sign in to publish a game"</p>
                    <a href="/login" class="inline-flex items-center gap-2 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all">"Sign in"</a>
                </div>

                // Upload form
                <form id="upload-form" class="hidden space-y-6" onsubmit="return false;">
                    // Title
                    <div>
                        <label class="block text-xs text-zinc-400 mb-1.5 font-medium uppercase tracking-wider">"Game Title"</label>
                        <input type="text" id="game-name" required maxlength="128" placeholder="My Awesome Game"
                            class="w-full px-4 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                    </div>

                    // Description
                    <div>
                        <label class="block text-xs text-zinc-400 mb-1.5 font-medium uppercase tracking-wider">"Description"</label>
                        <textarea id="game-description" rows="5" placeholder="Describe your game, its features, how to play..."
                            class="w-full px-4 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all resize-y"></textarea>
                    </div>

                    // Category + Version row
                    <div class="grid grid-cols-2 gap-4">
                        <div>
                            <label class="block text-xs text-zinc-400 mb-1.5 font-medium uppercase tracking-wider">"Category"</label>
                            <select id="game-category"
                                class="w-full px-4 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm focus:border-accent/50 transition-all">
                                <option value="other">"Select category..."</option>
                            </select>
                        </div>
                        <div>
                            <label class="block text-xs text-zinc-400 mb-1.5 font-medium uppercase tracking-wider">"Version"</label>
                            <input type="text" id="game-version" value="1.0.0" placeholder="1.0.0"
                                class="w-full px-4 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                        </div>
                    </div>

                    // Price
                    <div>
                        <label class="block text-xs text-zinc-400 mb-1.5 font-medium uppercase tracking-wider">"Price (credits)"</label>
                        <p class="text-[11px] text-zinc-600 mb-2">"Set to 0 for a free game. 10 credits = $1 USD. You receive 80% of each sale."</p>
                        <input type="number" id="game-price" min="0" value="0" placeholder="0"
                            class="w-full px-4 py-2.5 bg-white/[0.03] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                    </div>

                    // Thumbnail
                    <div>
                        <label class="block text-xs text-zinc-400 mb-1.5 font-medium uppercase tracking-wider">"Cover Image"</label>
                        <p class="text-[11px] text-zinc-600 mb-2">"Recommended: 1920×1080 (16:9). PNG or JPG."</p>
                        <div id="thumb-dropzone" class="relative border-2 border-dashed border-zinc-800/50 rounded-xl p-8 text-center hover:border-accent/30 transition-all cursor-pointer"
                            onclick="document.getElementById('game-thumbnail').click()">
                            <i class="ph ph-image text-2xl text-zinc-600 mb-2"></i>
                            <p class="text-sm text-zinc-500">"Drop an image or click to browse"</p>
                            <img id="thumb-preview" class="hidden mt-4 max-h-40 mx-auto rounded-lg" />
                            <input type="file" id="game-thumbnail" accept="image/*" class="hidden" onchange="previewThumb(this)" />
                        </div>
                    </div>

                    // Game file
                    <div>
                        <label class="block text-xs text-zinc-400 mb-1.5 font-medium uppercase tracking-wider">"Game File"</label>
                        <p class="text-[11px] text-zinc-600 mb-2">"Upload a ZIP archive or executable. Max 2 GB."</p>
                        <div id="file-dropzone" class="relative border-2 border-dashed border-zinc-800/50 rounded-xl p-8 text-center hover:border-accent/30 transition-all cursor-pointer"
                            onclick="document.getElementById('game-file').click()">
                            <i class="ph ph-file-arrow-up text-2xl text-zinc-600 mb-2"></i>
                            <p id="file-label" class="text-sm text-zinc-500">"Drop a file or click to browse"</p>
                            <input type="file" id="game-file" accept=".zip,.exe,.tar.gz,.dmg,.appimage" class="hidden" onchange="previewFile(this)" />
                        </div>
                    </div>

                    // Screenshots
                    <div>
                        <label class="block text-xs text-zinc-400 mb-1.5 font-medium uppercase tracking-wider">"Screenshots (optional)"</label>
                        <p class="text-[11px] text-zinc-600 mb-2">"Add up to 10 screenshots. These can also be uploaded after publishing."</p>
                        <input type="file" id="game-screenshots" accept="image/*" multiple class="file-input file-input-sm w-full bg-white/[0.03] border border-zinc-800/50 rounded-xl text-sm" />
                    </div>

                    // Error / success
                    <div id="upload-error" class="hidden p-3 rounded-xl bg-red-500/10 border border-red-500/20 text-red-400 text-sm"></div>
                    <div id="upload-success" class="hidden p-3 rounded-xl bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-sm"></div>

                    // Submit
                    <div class="flex gap-3">
                        <button type="button" onclick="submitGame(false)" id="save-draft-btn"
                            class="flex-1 inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl text-sm font-medium bg-white/[0.05] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200 transition-all">
                            <i class="ph ph-floppy-disk"></i>"Save as Draft"
                        </button>
                        <button type="button" onclick="submitGame(true)" id="publish-btn"
                            class="flex-1 inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)]">
                            <i class="ph ph-rocket-launch"></i>"Publish"
                        </button>
                    </div>

                    <p class="text-[11px] text-zinc-600 text-center">"By publishing, you agree to the Renzora "<a href="/docs/marketplace/guidelines" class="text-accent hover:text-accent-hover">"content guidelines"</a>"."</p>
                </form>
            </div>
        </section>

        <script>
            r##"
            function previewThumb(input) {
                const preview = document.getElementById('thumb-preview');
                if (input.files && input.files[0]) {
                    const reader = new FileReader();
                    reader.onload = (e) => { preview.src = e.target.result; preview.classList.remove('hidden'); };
                    reader.readAsDataURL(input.files[0]);
                }
            }

            function previewFile(input) {
                if (input.files && input.files[0]) {
                    const file = input.files[0];
                    const sizeMB = (file.size / 1024 / 1024).toFixed(1);
                    document.getElementById('file-label').innerHTML = `<strong>${file.name}</strong> (${sizeMB} MB)`;
                }
            }

            async function submitGame(publish) {
                const error = document.getElementById('upload-error');
                const success = document.getElementById('upload-success');
                error.classList.add('hidden');
                success.classList.add('hidden');

                const name = document.getElementById('game-name').value.trim();
                const description = document.getElementById('game-description').value.trim();
                const category = document.getElementById('game-category').value;
                const version = document.getElementById('game-version').value.trim() || '1.0.0';
                const priceCredits = parseInt(document.getElementById('game-price').value) || 0;
                const fileInput = document.getElementById('game-file');
                const thumbInput = document.getElementById('game-thumbnail');

                if (!name) { error.textContent = 'Game title is required'; error.classList.remove('hidden'); return; }
                if (!fileInput.files[0]) { error.textContent = 'Game file is required'; error.classList.remove('hidden'); return; }

                const btn = publish ? document.getElementById('publish-btn') : document.getElementById('save-draft-btn');
                const originalText = btn.innerHTML;
                btn.innerHTML = '<div class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div> Uploading...';
                btn.disabled = true;

                const formData = new FormData();
                formData.append('metadata', JSON.stringify({
                    name, description, category, price_credits: priceCredits, version
                }));
                formData.append('file', fileInput.files[0]);
                if (thumbInput.files[0]) formData.append('thumbnail', thumbInput.files[0]);

                try {
                    const res = await fetch('/api/games/upload', {
                        method: 'POST',
                        credentials: 'include',
                        body: formData,
                    });
                    const data = await res.json();

                    if (!res.ok) {
                        error.textContent = data.error || 'Upload failed';
                        error.classList.remove('hidden');
                        btn.innerHTML = originalText;
                        btn.disabled = false;
                        return;
                    }

                    // If publishing, update metadata
                    if (publish) {
                        await fetch(`/api/games/${data.id}/update`, {
                            method: 'PUT',
                            credentials: 'include',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify({ published: true }),
                        });
                    }

                    // Upload screenshots
                    const screenshots = document.getElementById('game-screenshots').files;
                    for (let i = 0; i < screenshots.length && i < 10; i++) {
                        const mediaForm = new FormData();
                        mediaForm.append('type', 'image');
                        mediaForm.append('sort_order', i.toString());
                        mediaForm.append('file', screenshots[i]);
                        await fetch(`/api/games/${data.id}/media`, {
                            method: 'POST',
                            credentials: 'include',
                            body: mediaForm,
                        });
                    }

                    success.innerHTML = publish
                        ? `Game published! <a href="/games/${data.slug}" class="underline">View your game →</a>`
                        : `Draft saved! <a href="/games/${data.slug}" class="underline">View draft →</a>`;
                    success.classList.remove('hidden');
                    btn.innerHTML = originalText;
                    btn.disabled = false;

                } catch(e) {
                    error.textContent = 'Upload failed: ' + e.message;
                    error.classList.remove('hidden');
                    btn.innerHTML = originalText;
                    btn.disabled = false;
                }
            }

            // Init
            (async function() {
                const authRequired = document.getElementById('upload-auth-required');
                const form = document.getElementById('upload-form');

                try {
                    const res = await fetch('/api/auth/me', { credentials: 'include' });
                    if (!res.ok) {
                        authRequired.classList.remove('hidden');
                        return;
                    }
                    form.classList.remove('hidden');
                } catch(e) {
                    authRequired.classList.remove('hidden');
                    return;
                }

                // Load categories
                try {
                    const res = await fetch('/api/games/categories');
                    const cats = await res.json();
                    const select = document.getElementById('game-category');
                    select.innerHTML = '<option value="">Select category...</option>';
                    cats.forEach(cat => {
                        const opt = document.createElement('option');
                        opt.value = cat.slug;
                        opt.textContent = cat.name;
                        select.appendChild(opt);
                    });
                } catch(e) {}
            })();
            "##
        </script>
    }
}
