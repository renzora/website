use leptos::prelude::*;
use leptos_meta::{Title, Meta};

#[component]
pub fn DevelopersPage() -> impl IntoView {
    view! {
        <Title text="Renzora for Developers, API, Plugins & Source" />
        <Meta name="description" content="Build on Renzora: REST API reference, plugin development and the architecture of the open-source Bevy editor. Extend the engine with hot-loadable cdylibs." />

        <section class="py-12 px-6 min-h-screen">
            <div class="max-w-4xl mx-auto">
                <div class="mb-10">
                    <h1 class="text-3xl font-bold">"Developers"</h1>
                    <p class="text-zinc-400 mt-2">"Build integrations, automate uploads, and extend the Renzora ecosystem with our API."</p>
                </div>

                <div id="dev-content">
                    <div class="text-center py-12">
                        <div class="inline-block animate-spin w-6 h-6 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                    </div>
                </div>
            </div>
        </section>
        <script>
            r##"
            (async function() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                const el = document.getElementById('dev-content');

                // Fetch existing tokens if logged in
                let tokens = [];
                if (token) {
                    try {
                        const res = await fetch('/api/api-tokens', { headers: { 'Authorization': 'Bearer ' + token } });
                        if (res.ok) tokens = await res.json();
                    } catch(e) {}
                }

                el.innerHTML = `
                    <!-- API Tokens Section -->
                    <div class="mb-10">
                        <div class="flex items-center justify-between mb-4">
                            <h2 class="text-xl font-semibold">API Tokens</h2>
                            ${token ? '<button onclick="createToken()" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all">Create Token</button>' : ''}
                        </div>

                        ${!token ? '<p class="text-sm text-zinc-500"><a href="/login" class="text-accent hover:text-accent-hover">Sign in</a> to manage API tokens.</p>' : `
                            <!-- Today's allowance. Filled by loadUsage(); until
                                 then it says nothing rather than showing a bar
                                 at zero, which would read as "no requests left"
                                 and is the one wrong answer to give here. -->
                            <div id="usage-card" class="hidden mb-5 p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl"></div>

                            <div id="token-list" class="space-y-2">
                                ${tokens.length === 0 ? '<p class="text-sm text-zinc-500">No API tokens yet. Create one to get started.</p>' :
                                    tokens.map(t => tokenRow(t)).join('')}
                            </div>
                            <div id="new-token-banner" class="hidden mt-4 p-4 bg-green-950/30 border border-green-800/50 rounded-xl">
                                <div class="flex items-center gap-2 mb-2">
                                    <i class="ph ph-check-circle text-green-400"></i>
                                    <span class="text-sm font-medium text-green-400">Token created</span>
                                </div>
                                <p class="text-xs text-zinc-400 mb-2">Copy this token now. It will not be shown again.</p>
                                <div class="flex items-center gap-2">
                                    <code id="new-token-value" class="flex-1 px-3 py-2 bg-black/40 rounded-lg text-xs text-zinc-300 font-mono select-all overflow-x-auto"></code>
                                    <button onclick="copyToken()" class="px-3 py-2 rounded-lg text-xs bg-white/5 hover:bg-white/10 transition-colors">Copy</button>
                                </div>
                            </div>
                        `}
                    </div>

                `;
            })();

            // ── Today's allowance ──
            //
            // Counted per account, not per token: ten tokens share one daily
            // budget rather than getting ten, so this is one bar for the page
            // and not a row on each token.
            async function loadUsage() {
                if (!token) return;
                const el = document.getElementById('usage-card');
                if (!el) return;
                let u;
                try {
                    const res = await fetch('/api/api-tokens/usage', { headers: { 'Authorization': 'Bearer ' + token } });
                    if (!res.ok) return;
                    u = await res.json();
                } catch(e) { return; }

                // A rejected request still increments the count, so `used` can
                // pass `limit`. The bar is clamped and `remaining` comes from
                // the server already floored, but the NUMBERS are printed as
                // they are: someone over the line should see how far over.
                const pct = u.limit > 0 ? Math.min(100, Math.round((u.used / u.limit) * 100)) : 0;
                const spent = u.remaining === 0;
                const low = !spent && u.remaining <= u.limit * 0.1;
                const barColour = spent ? 'bg-red-500' : low ? 'bg-amber-400' : 'bg-accent';
                const numColour = spent ? 'text-red-300' : low ? 'text-amber-300' : 'text-zinc-100';

                const resets = new Date(u.resets_at);
                const resetLabel = isNaN(resets) ? '' : resets.toLocaleTimeString([], { hour: 'numeric', minute: '2-digit' });

                el.className = 'mb-5 p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl';
                el.innerHTML = `
                    <div class="flex items-baseline justify-between gap-3 flex-wrap mb-3">
                        <div>
                            <div class="text-sm text-zinc-400">Requests remaining today</div>
                            <div class="mt-1 flex items-baseline gap-2">
                                <span class="text-3xl font-bold ${numColour}">${u.remaining.toLocaleString()}</span>
                                <span class="text-sm text-zinc-400">of ${u.limit.toLocaleString()}</span>
                            </div>
                        </div>
                        <div class="text-sm text-zinc-400 text-right">
                            <div>${u.used.toLocaleString()} used</div>
                            ${resetLabel ? '<div class="text-zinc-500">Resets at ' + resetLabel + '</div>' : ''}
                        </div>
                    </div>
                    <div class="h-2.5 w-full rounded-full bg-white/[0.08] overflow-hidden" role="progressbar"
                         aria-valuenow="${u.used}" aria-valuemin="0" aria-valuemax="${u.limit}"
                         aria-label="API requests used today">
                        <div class="h-full ${barColour} rounded-full transition-all" style="width:${pct}%"></div>
                    </div>
                    ${spent ? '<p class="mt-3 text-sm text-red-300">The daily limit is spent. Requests are rejected until it resets.</p>' : ''}
                `;
                el.classList.remove('hidden');
            }
            loadUsage();

            // A date the server sent, or nothing. The API now emits RFC 3339,
            // which is what `new Date()` needs; this stays defensive because the
            // failure mode is a row reading "Last used Invalid Date", and a
            // missing date is better than a wrong one.
            function tokenDate(v) {
                if (!v) return null;
                const d = new Date(v);
                return isNaN(d) ? null : d.toLocaleDateString();
            }

            function tokenRow(t) {
                const created = tokenDate(t.created_at);
                const used = tokenDate(t.last_used_at);
                const expires = tokenDate(t.expires_at);
                return '<div class="flex items-center justify-between p-3 bg-white/[0.02] border border-zinc-800/50 rounded-lg">' +
                    '<div class="flex items-center gap-3">' +
                        '<div class="w-8 h-8 rounded-lg bg-accent/10 flex items-center justify-center"><i class="ph ph-key text-accent text-sm"></i></div>' +
                        '<div>' +
                            '<div class="text-sm font-medium">' + t.name + '</div>' +
                            '<div class="text-xs text-zinc-400">' + t.prefix + '...' +
                            (created ? ' · Created ' + created : '') +
                            (used ? ' · Last used ' + used : ' · Never used') +
                            (expires ? ' · Expires ' + expires : '') + '</div>' +
                        '</div>' +
                    '</div>' +
                    '<button onclick="revokeToken(\'' + t.id + '\', this)" class="px-3 py-1.5 rounded-lg text-xs text-red-400 hover:bg-red-950/30 hover:text-red-300 border border-transparent hover:border-red-900/50 transition-all">Revoke</button>' +
                '</div>';
            }

            async function createToken() {
                var name = prompt('Token name (e.g. "upload-bot"):');
                if (!name) return;
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                try {
                    var res = await fetch('/api/api-tokens', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({ name: name })
                    });
                    var data = await res.json();
                    if (!res.ok) { alert(data.message || 'Failed to create token'); return; }
                    document.getElementById('new-token-value').textContent = data.token;
                    document.getElementById('new-token-banner').classList.remove('hidden');
                    // Add new token row to the list without reloading the page
                    var list = document.getElementById('token-list');
                    var empty = list.querySelector('p');
                    if (empty) empty.remove();
                    list.insertAdjacentHTML('afterbegin', tokenRow({
                        id: data.id,
                        name: data.name,
                        prefix: data.prefix,
                        created_at: data.created_at,
                        last_used_at: null,
                        expires_at: data.expires_at || null
                    }));
                } catch(e) { alert('Error: ' + e.message); }
            }

            async function revokeToken(id, btn) {
                if (!confirm('Revoke this API token? Any integrations using it will stop working.')) return;
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                var res = await fetch('/api/api-tokens/' + id, {
                    method: 'DELETE',
                    headers: { 'Authorization': 'Bearer ' + token }
                });
                if (res.ok) { btn.closest('[class*="flex items-center justify-between"]').remove(); }
                else { alert('Failed to revoke token'); }
            }

            function copyToken() {
                var val = document.getElementById('new-token-value').textContent;
                navigator.clipboard.writeText(val);
            }
            "##
        </script>
    }
}
