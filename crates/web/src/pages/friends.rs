use leptos::prelude::*;

/// Friends hub, mirrors the engine's Friends panel: Friends / Requests / Add
/// tabs with live presence dots, message + remove actions, and user search.
#[component]
pub fn FriendsPage() -> impl IntoView {
    view! {
        <section class="py-8 px-4 md:px-6 min-h-[80vh] bg-gradient-to-b from-[#0c0a10] via-[#060608] to-[#060608]">
            <div class="max-w-[700px] mx-auto">
                <h1 class="text-2xl font-bold mb-1">"Friends"</h1>
                <p class="text-zinc-500 text-sm mb-6">"Connect with other creators across Renzora."</p>

                <div class="flex items-center gap-1 mb-5 border-b border-zinc-800">
                    <button data-tab="friends" class="fr-tab px-4 py-2.5 text-sm font-medium border-b-2 border-accent text-white">"Friends"</button>
                    <button data-tab="requests" class="fr-tab px-4 py-2.5 text-sm font-medium border-b-2 border-transparent text-zinc-400 hover:text-white transition-colors">"Requests"<span id="req-count" class="hidden ml-1.5 px-1.5 py-0.5 rounded-full text-[10px] bg-accent text-white"></span></button>
                    <button data-tab="add" class="fr-tab px-4 py-2.5 text-sm font-medium border-b-2 border-transparent text-zinc-400 hover:text-white transition-colors">"Add friends"</button>
                </div>

                <div id="tab-friends" class="fr-panel">
                    <div id="friends-list" class="space-y-2"><div class="text-center py-10"><div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-accent rounded-full"></div></div></div>
                </div>
                <div id="tab-requests" class="fr-panel hidden">
                    <div id="requests-list" class="space-y-2"></div>
                </div>
                <div id="tab-add" class="fr-panel hidden">
                    <input id="user-search" type="text" placeholder="Search users by name…" class="w-full px-4 py-2.5 mb-4 bg-surface-card border border-zinc-800 rounded-lg text-sm text-zinc-100 outline-none focus:border-accent" />
                    <div id="search-results" class="space-y-2"></div>
                </div>
            </div>
        </section>
        <script>
            r##"
            (function(){
              const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
              if (!token){ window.location.href = '/login?redirect=/friends'; return; }
              const H = { 'Authorization': 'Bearer ' + token };
              const HJ = { ...H, 'Content-Type': 'application/json' };
              const $ = id => document.getElementById(id);
              function esc(s){ const d=document.createElement('div'); d.textContent=s==null?'':String(s); return d.innerHTML; }
              let presence = {};

              function avatar(url, name, dot){
                const inner = url ? `<img src="${esc(url)}" class="w-10 h-10 rounded-full object-cover" />` : `<div class="w-10 h-10 rounded-full bg-accent/20 text-accent flex items-center justify-center font-semibold">${esc((name||'?').charAt(0).toUpperCase())}</div>`;
                const online = dot !== undefined;
                return `<div class="relative shrink-0">${inner}${online?`<span class="absolute bottom-0 right-0 w-3 h-3 rounded-full border-2 border-surface-card ${dot?'bg-emerald-400':'bg-zinc-600'}"></span>`:''}</div>`;
              }
              function row(inner){ return `<div class="flex items-center gap-3 p-3 bg-surface-card border border-zinc-800 rounded-xl">${inner}</div>`; }

              // ── Tabs ──
              document.querySelectorAll('.fr-tab').forEach(t => t.addEventListener('click', () => {
                document.querySelectorAll('.fr-tab').forEach(x => { x.classList.remove('border-accent','text-white'); x.classList.add('border-transparent','text-zinc-400'); });
                t.classList.add('border-accent','text-white'); t.classList.remove('border-transparent','text-zinc-400');
                document.querySelectorAll('.fr-panel').forEach(p => p.classList.add('hidden'));
                $('tab-'+t.dataset.tab).classList.remove('hidden');
                if (t.dataset.tab === 'add' && !$('search-results').dataset.loaded) loadPopular();
              }));

              // ── Friends ──
              async function loadFriends(){
                try {
                  const [fr, pr] = await Promise.all([
                    fetch('/api/gameservices/friends', { headers: H }).then(r => r.ok?r.json():[]),
                    fetch('/api/gameservices/friends/presence', { headers: H }).then(r => r.ok?r.json():[]),
                  ]);
                  presence = {}; pr.forEach(p => presence[p.user_id] = p.online);
                  fr.sort((a,b) => (presence[b.user_id]?1:0)-(presence[a.user_id]?1:0) || (a.username||'').localeCompare(b.username||''));
                  const el = $('friends-list');
                  if (!fr.length){ el.innerHTML = '<p class="text-center text-zinc-500 py-12 text-sm">No friends yet. Head to the Add friends tab to find creators.</p>'; return; }
                  el.innerHTML = fr.map(f => row(`
                    ${avatar(f.avatar_url, f.username, !!presence[f.user_id])}
                    <div class="flex-1 min-w-0"><a href="/profile/${esc(f.username)}" class="text-sm font-semibold hover:text-accent">${esc(f.username)}</a><p class="text-[11px] text-zinc-600">${presence[f.user_id]?'Online':'Offline'}</p></div>
                    <button onclick="__msg('${f.user_id}')" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-white/[0.06] border border-white/[0.06] text-zinc-200 hover:text-white transition-colors"><i class="ph ph-chat-circle"></i> Message</button>
                    <button onclick="__remove('${f.user_id}',this)" class="px-2.5 py-1.5 rounded-lg text-xs text-zinc-500 hover:text-red-400 transition-colors" title="Remove friend"><i class="ph ph-user-minus"></i></button>
                  `)).join('');
                } catch(e){}
              }

              async function loadRequests(){
                try {
                  const reqs = await fetch('/api/gameservices/friends/requests', { headers: H }).then(r => r.ok?r.json():[]);
                  const badge = $('req-count');
                  if (reqs.length){ badge.textContent = reqs.length; badge.classList.remove('hidden'); } else badge.classList.add('hidden');
                  const el = $('requests-list');
                  el.innerHTML = reqs.length ? reqs.map(r => row(`
                    ${avatar(r.avatar_url, r.username)}
                    <div class="flex-1 min-w-0"><a href="/profile/${esc(r.username)}" class="text-sm font-semibold hover:text-accent">${esc(r.username)}</a><p class="text-[11px] text-zinc-600">wants to be friends</p></div>
                    <button onclick="__accept('${r.from_user_id}')" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-accent text-white hover:bg-accent-hover transition-colors">Accept</button>
                    <button onclick="__remove('${r.from_user_id}',this)" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-white/[0.06] border border-white/[0.06] text-zinc-300 hover:text-white transition-colors">Decline</button>
                  `)).join('') : '<p class="text-center text-zinc-500 py-12 text-sm">No pending requests.</p>';
                } catch(e){}
              }

              // ── Add / search ──
              let searchTimer;
              $('user-search').addEventListener('input', e => {
                clearTimeout(searchTimer);
                const q = e.target.value.trim();
                if (q.length < 2){ loadPopular(); return; }
                searchTimer = setTimeout(() => doSearch(q), 250);
              });
              async function doSearch(q){
                try {
                  const users = await fetch('/api/profiles/search?q=' + encodeURIComponent(q), { headers: H }).then(r => r.ok?r.json():[]);
                  renderSearch(users, 'No users found.');
                } catch(e){}
              }
              async function loadPopular(){
                try {
                  const users = await fetch('/api/profiles/popular', { headers: H }).then(r => r.ok?r.json():[]);
                  renderSearch(users, 'No suggestions yet.', 'Popular in the community');
                } catch(e){}
              }
              function renderSearch(users, empty, heading){
                const el = $('search-results'); el.dataset.loaded = '1';
                if (!users.length){ el.innerHTML = `<p class="text-center text-zinc-500 py-12 text-sm">${empty}</p>`; return; }
                el.innerHTML = (heading?`<p class="text-xs uppercase tracking-wide text-zinc-600 mb-2">${heading}</p>`:'') + users.map(u => row(`
                  ${avatar(u.avatar_url, u.username)}
                  <div class="flex-1 min-w-0"><a href="/profile/${esc(u.username)}" class="text-sm font-semibold hover:text-accent">${esc(u.username)}</a></div>
                  <button onclick="__add('${esc(u.username)}',this)" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-accent text-white hover:bg-accent-hover transition-colors"><i class="ph ph-user-plus"></i> Add</button>
                `)).join('');
              }

              // ── Actions ──
              window.__msg = async function(uid){
                try { const res = await fetch('/api/messages/conversations/dm/'+uid, { method:'POST', headers: H }); const d = await res.json(); if (d.conversation_id) window.location.href = '/messages?conv='+d.conversation_id; } catch(e){}
              };
              window.__add = async function(username, btn){
                // Username-based toggle works for both search hits and popular
                // suggestions (the /popular payload carries no user id).
                try { const res = await fetch('/api/profiles/friend/'+encodeURIComponent(username), { method:'POST', headers: HJ });
                  if (res.ok){ btn.outerHTML = '<span class="px-3 py-1.5 text-xs text-zinc-500">Requested</span>'; } } catch(e){}
              };
              window.__accept = async function(uid){
                try { await fetch('/api/gameservices/friends/accept', { method:'POST', headers: HJ, body: JSON.stringify({ user_id: uid }) }); loadRequests(); loadFriends(); } catch(e){}
              };
              window.__remove = async function(uid, btn){
                try { const res = await fetch('/api/gameservices/friends/remove', { method:'POST', headers: HJ, body: JSON.stringify({ user_id: uid }) });
                  if (res.ok){ const rowEl = btn.closest('.bg-surface-card'); if (rowEl) rowEl.remove(); loadRequests(); } } catch(e){}
              };

              loadFriends();
              loadRequests();
            })();
            "##
        </script>
    }
}
