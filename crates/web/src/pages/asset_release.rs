use leptos::prelude::*;

/// Publish a new version of an asset (`/marketplace/asset/:slug/releases/new`).
///
/// Distinct from Edit on purpose: editing fixes the current version in place,
/// while a release is a new version that keeps the old files downloadable for
/// people who already bought them.
#[component]
pub fn AssetReleasePage() -> impl IntoView {
    view! {
        <section class="py-12 px-6">
            <div class="max-w-3xl mx-auto" id="release-root">
                <div class="text-center py-20">
                    <div class="inline-block animate-spin w-6 h-6 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                </div>
            </div>
        </section>
        <script>
            r##"
            let RA = null;        // asset detail
            let RELEASES = [];

            const resc = s => String(s ?? '').replace(/[&<>"']/g, c =>
                ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));

            function rToken() {
                return document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
            }

            function rFmtSize(bytes) {
                if (!bytes) return '0 B';
                if (bytes >= 1e9) return (bytes / 1e9).toFixed(1) + ' GB';
                if (bytes >= 1e6) return (bytes / 1e6).toFixed(1) + ' MB';
                if (bytes >= 1e3) return (bytes / 1e3).toFixed(1) + ' KB';
                return bytes + ' B';
            }

            // Suggest the next patch version, so the common case is one click.
            function suggestVersion(current) {
                const m = /^(\d+)\.(\d+)\.(\d+)(.*)$/.exec(current || '');
                if (m) return `${m[1]}.${m[2]}.${parseInt(m[3], 10) + 1}${m[4] || ''}`;
                const n = /^(\d+)\.(\d+)$/.exec(current || '');
                if (n) return `${n[1]}.${parseInt(n[2], 10) + 1}`;
                return '';
            }

            (async function() {
                const root = document.getElementById('release-root');
                const t = rToken();
                if (!t) { window.location.href = '/login'; return; }

                const parts = window.location.pathname.split('/').filter(Boolean);
                const slug = parts[2];

                const res = await fetch('/api/marketplace/detail/' + slug, {
                    headers: { 'Authorization': 'Bearer ' + t }
                });
                if (!res.ok) {
                    root.innerHTML = '<p class="text-center text-zinc-500 py-20">Asset not found.</p>';
                    return;
                }
                RA = await res.json();

                // Only the creator can publish a release.
                const userCookie = document.cookie.match('(^|;)\\s*user\\s*=\\s*([^;]+)')?.pop();
                let uid = null;
                if (userCookie) { try { uid = JSON.parse(decodeURIComponent(userCookie)).id; } catch(e) {} }
                if (!uid || !RA.creator || RA.creator.id !== uid) {
                    root.innerHTML = '<p class="text-center text-zinc-500 py-20">You don\'t have permission to publish releases for this asset.</p>';
                    return;
                }

                const rRes = await fetch('/api/marketplace/' + RA.id + '/releases');
                RELEASES = rRes.ok ? await rRes.json() : [];

                const history = RELEASES.length ? RELEASES.map(r => `
                    <div class="flex items-center gap-3 px-4 py-2.5 border-b border-zinc-800/50 last:border-0">
                        <span class="text-sm font-medium text-zinc-300">v${resc(r.version)}</span>
                        ${r.is_current ? '<span class="px-1.5 py-0.5 rounded bg-green-500/10 border border-green-500/20 text-[10px] text-green-400">current</span>' : ''}
                        <span class="flex-1"></span>
                        <span class="text-xs text-zinc-600">${r.file_count} files · ${rFmtSize(r.total_size)}</span>
                    </div>`).join('') : '<p class="text-xs text-zinc-600 px-4 py-3">No releases yet.</p>';

                root.innerHTML = `
                    <div class="mb-10">
                        <a href="/marketplace/asset/${RA.slug}" class="inline-flex items-center gap-1.5 text-sm text-zinc-500 hover:text-zinc-300 transition-colors mb-4">
                            <i class="ph ph-arrow-left"></i> Back to Asset
                        </a>
                        <h1 class="text-3xl font-bold">New Release</h1>
                        <p class="text-zinc-400 text-sm mt-2">Ship a new version of <span class="text-zinc-200">${resc(RA.name)}</span>. The current version stays downloadable for people who already have it.</p>
                    </div>

                    <div id="rel-error" class="hidden mb-6 p-4 rounded-xl bg-red-500/10 border border-red-500/20 text-red-400 text-sm flex items-center gap-2">
                        <i class="ph ph-warning-circle text-lg"></i><span id="rel-error-text"></span>
                    </div>

                    <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5 mb-8">
                        <h2 class="text-base font-semibold flex items-center gap-2"><i class="ph ph-tag text-accent"></i> Version</h2>
                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">New version</label>
                                <input type="text" id="rel-version" maxlength="32" value="${resc(suggestVersion(RA.version))}" placeholder="1.2.0"
                                    class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                            </div>
                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">Current version</label>
                                <div class="px-4 py-3 bg-white/[0.01] border border-zinc-800/50 rounded-xl text-zinc-500 text-sm">${resc(RA.version)}</div>
                            </div>
                        </div>
                    </div>

                    <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5 mb-8">
                        <h2 class="text-base font-semibold flex items-center gap-2"><i class="ph ph-note-pencil text-cyan-400"></i> Release notes</h2>
                        <div>
                            <textarea id="rel-notes" rows="8" placeholder="- Fixed a crash on startup&#10;- Added the async loader&#10;&#10;Markdown is supported."
                                class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all resize-y font-mono"></textarea>
                            <p class="text-xs text-zinc-600 mt-1">Markdown. Leave this empty and a <span class="text-zinc-500">CHANGELOG.md</span> in your archive will be used instead.</p>
                        </div>
                    </div>

                    <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5 mb-8">
                        <h2 class="text-base font-semibold flex items-center gap-2"><i class="ph ph-file-arrow-up text-cyan-400"></i> Files</h2>
                        <input type="file" id="rel-files" multiple onchange="onRelFiles(this)"
                            class="w-full text-sm text-zinc-400 file:mr-4 file:py-2 file:px-4 file:rounded-xl file:border-0 file:text-sm file:font-medium file:bg-white/[0.05] file:text-zinc-300 hover:file:bg-white/[0.08] file:cursor-pointer file:transition-colors" />
                        <p class="text-xs text-zinc-600">Max 200MB each. Upload a .zip to publish a whole folder tree.</p>

                        <div id="rel-zip-options" class="hidden p-3 bg-white/[0.02] rounded-xl border border-zinc-800/50">
                            <p class="text-xs text-zinc-400 mb-2">This is a .zip. How should it be stored?</p>
                            <label class="flex items-start gap-2 cursor-pointer mb-2">
                                <input type="radio" name="rel_zip" value="extract" checked class="accent-accent mt-0.5" />
                                <span class="text-xs text-zinc-300">Unpack it<span class="block text-zinc-600">Each file is stored separately.</span></span>
                            </label>
                            <label class="flex items-start gap-2 cursor-pointer">
                                <input type="radio" name="rel_zip" value="keep" class="accent-accent mt-0.5" />
                                <span class="text-xs text-zinc-300">Keep it as a single .zip<span class="block text-zinc-600">The archive is downloaded as-is. Always used for plugins, since the editor builds from these exact bytes.</span></span>
                            </label>
                            <p class="text-[11px] text-zinc-600 mt-2">Either way buyers get a browsable file tree and your README renders as documentation.</p>
                        </div>

                        <div id="rel-file-list" class="space-y-1"></div>
                    </div>

                    <div class="border border-zinc-800/50 rounded-2xl overflow-hidden bg-white/[0.01] mb-8">
                        <div class="px-4 py-2.5 border-b border-zinc-800/50 bg-white/[0.02] text-xs font-medium text-zinc-400">Release history</div>
                        ${history}
                    </div>

                    <div class="flex items-center gap-4">
                        <button onclick="publishRelease()" id="rel-btn" class="flex-1 inline-flex items-center justify-center gap-2 px-6 py-3.5 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)]">
                            <i class="ph ph-rocket-launch text-lg"></i> Publish Release
                        </button>
                        <a href="/marketplace/asset/${RA.slug}" class="px-6 py-3.5 rounded-xl text-sm font-medium text-zinc-400 hover:text-zinc-200 transition-colors">Cancel</a>
                    </div>
                `;
            })();

            window.onRelFiles = function(input) {
                const files = input.files;
                const zipOpts = document.getElementById('rel-zip-options');
                const list = document.getElementById('rel-file-list');

                if (files.length === 1 && files[0].name.toLowerCase().endsWith('.zip')) {
                    zipOpts.classList.remove('hidden');
                } else {
                    zipOpts.classList.add('hidden');
                }

                list.innerHTML = Array.from(files).map(f =>
                    `<div class="flex items-center gap-2 text-xs text-zinc-400 px-3 py-2 bg-white/[0.02] rounded-lg">
                        <i class="ph ph-file text-zinc-600"></i>
                        <span class="flex-1 truncate">${resc(f.name)}</span>
                        <span class="text-zinc-600">${rFmtSize(f.size)}</span>
                    </div>`).join('');
            };

            function relError(msg) {
                const el = document.getElementById('rel-error');
                document.getElementById('rel-error-text').textContent = msg;
                el.classList.remove('hidden');
                window.scrollTo({ top: 0, behavior: 'smooth' });
            }

            window.publishRelease = async function() {
                const t = rToken();
                if (!t || !RA) return;

                const version = document.getElementById('rel-version').value.trim();
                const files = document.getElementById('rel-files').files;
                if (!version) { relError('Give the release a version number.'); return; }
                if (!files.length) { relError('A release needs at least one file.'); return; }

                const btn = document.getElementById('rel-btn');
                btn.disabled = true;
                btn.innerHTML = '<i class="ph ph-spinner text-lg animate-spin"></i> Publishing...';

                try {
                    const zip = document.querySelector('input[name="rel_zip"]:checked');
                    const fd = new FormData();
                    fd.append('metadata', JSON.stringify({
                        version,
                        notes: document.getElementById('rel-notes').value,
                        zip_action: zip ? zip.value : 'extract',
                    }));
                    for (let i = 0; i < files.length; i++) fd.append('file', files[i], files[i].name);

                    const res = await fetch('/api/marketplace/' + RA.id + '/releases', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + t },
                        body: fd
                    });
                    if (!res.ok) {
                        const d = await res.json().catch(() => ({}));
                        throw new Error(d.error || 'Failed to publish the release');
                    }
                    window.location.href = '/marketplace/asset/' + RA.slug + '?tab=releases';
                } catch (e) {
                    relError(e.message);
                    btn.disabled = false;
                    btn.innerHTML = '<i class="ph ph-rocket-launch text-lg"></i> Publish Release';
                }
            };
            "##
        </script>
    }
}
