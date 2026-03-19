use leptos::prelude::*;

#[component]
pub fn UploadPage() -> impl IntoView {
    view! {
        <section class="py-20 px-6">
            <div class="max-w-2xl mx-auto">
                <h1 class="text-3xl font-bold mb-2">"Upload Asset"</h1>
                <p class="text-zinc-400 text-sm mb-8">"Share your creation with the Renzora community."</p>

                <div id="upload-error" class="hidden mb-4 p-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm"></div>
                <div id="upload-success" class="hidden mb-4 p-3 rounded-lg bg-green-500/10 border border-green-500/20 text-green-400 text-sm"></div>

                <form id="upload-form" class="flex flex-col gap-5" onsubmit="return handleUpload(event)">
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Name"</label>
                        <input type="text" name="name" required placeholder="My Awesome Plugin" class="w-full px-3 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors" />
                    </div>
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Description"</label>
                        <textarea name="description" required rows="4" placeholder="Describe what your asset does..." class="w-full px-3 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors resize-y"></textarea>
                    </div>
                    <div class="grid grid-cols-2 gap-4">
                        <div>
                            <label class="block text-xs text-zinc-500 mb-1.5">"Category"</label>
                            <select name="category" id="upload-category" class="w-full px-3 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors">
                                <option value="plugin">"Plugin"</option>
                                <option value="theme">"Theme"</option>
                                <option value="asset-pack">"Asset Pack"</option>
                                <option value="script">"Script"</option>
                                <option value="material">"Material"</option>
                            </select>
                        </div>
                        <div>
                            <label class="block text-xs text-zinc-500 mb-1.5">"Price (credits, 0 = free)"</label>
                            <input type="number" name="price" min="0" value="0" class="w-full px-3 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors" />
                        </div>
                    </div>
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Version"</label>
                        <input type="text" name="version" value="1.0.0" class="w-full px-3 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors" />
                    </div>
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Asset File"</label>
                        <input type="file" name="file" required class="w-full text-sm text-zinc-400 file:mr-4 file:py-2 file:px-4 file:rounded-lg file:border-0 file:text-sm file:font-medium file:bg-accent file:text-white hover:file:bg-accent-hover file:cursor-pointer" />
                    </div>
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Thumbnail (optional)"</label>
                        <input type="file" name="thumbnail" accept="image/*" class="w-full text-sm text-zinc-400 file:mr-4 file:py-2 file:px-4 file:rounded-lg file:border-0 file:text-sm file:font-medium file:bg-surface-card file:text-zinc-50 file:border file:border-zinc-800 hover:file:bg-zinc-800 file:cursor-pointer" />
                    </div>
                    <button type="submit" id="upload-btn" class="w-full mt-2 inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                        <i class="ph ph-upload-simple text-lg"></i>"Upload Asset"
                    </button>
                </form>
            </div>
        </section>

        <script>
            r#"
            async function handleUpload(e) {
                e.preventDefault();
                const form = e.target;
                const btn = document.getElementById('upload-btn');
                const err = document.getElementById('upload-error');
                const ok = document.getElementById('upload-success');
                err.classList.add('hidden');
                ok.classList.add('hidden');

                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { err.textContent = 'Please sign in first'; err.classList.remove('hidden'); return false; }

                btn.disabled = true;
                btn.innerHTML = '<i class="ph ph-spinner text-lg animate-spin"></i> Uploading...';

                const metadata = JSON.stringify({
                    name: form.name.value,
                    description: form.description.value,
                    category: form.category.value,
                    price_credits: parseInt(form.price.value) || 0,
                    version: form.version.value
                });

                const fd = new FormData();
                fd.append('metadata', metadata);
                if (form.file.files[0]) fd.append('file', form.file.files[0]);
                if (form.thumbnail.files[0]) fd.append('thumbnail', form.thumbnail.files[0]);

                try {
                    const res = await fetch('/api/marketplace/upload', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token },
                        body: fd
                    });
                    const data = await res.json();
                    if (!res.ok) throw new Error(data.error || 'Upload failed');
                    ok.textContent = 'Asset uploaded! It will be visible after review.';
                    ok.classList.remove('hidden');
                    form.reset();
                } catch (error) {
                    err.textContent = error.message;
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
