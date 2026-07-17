use leptos::prelude::*;

/// The community hub — a channel-based social feed. Mirrors the engine editor's
/// Community/Feed panel: a channel rail, a composer that targets the active
/// channel, and post cards with likes, arbitrary reactions, threaded comments,
/// media, reporting/hide moderation, and live "new posts" updates over WS.
///
/// Follows the site convention: a static SSR shell + one client-side IIFE that
/// talks to `/api/feed/*` with the `token` cookie and renders via innerHTML.
#[component]
pub fn CommunityPage() -> impl IntoView {
    view! {
        <section class="py-8 px-4 md:px-6 min-h-[80vh] bg-gradient-to-b from-[#0c0a10] via-[#060608] to-[#060608]">
            <div class="max-w-[1100px] mx-auto grid grid-cols-1 lg:grid-cols-[240px_minmax(0,1fr)] gap-6">

                // ── Channel rail ──
                <aside class="lg:sticky lg:top-20 self-start order-2 lg:order-1">
                    <div class="bg-surface-card border border-zinc-800 rounded-xl p-3">
                        <div class="flex items-center justify-between px-1 mb-2">
                            <h2 class="text-xs font-semibold uppercase tracking-wide text-zinc-500">"Channels"</h2>
                            <button id="suggest-toggle" title="Suggest a channel" class="w-6 h-6 rounded-md flex items-center justify-center text-zinc-500 hover:text-accent hover:bg-white/[0.04] transition-colors">
                                <i class="ph ph-plus"></i>
                            </button>
                        </div>
                        <div id="suggest-box" class="hidden mb-3 space-y-2">
                            <input id="suggest-name" type="text" maxlength="48" placeholder="Channel name" class="w-full px-2.5 py-1.5 bg-surface border border-zinc-800 rounded-lg text-xs text-zinc-50 outline-none focus:border-accent" />
                            <button id="suggest-submit" class="w-full px-2.5 py-1.5 rounded-lg text-xs font-medium bg-accent text-white hover:bg-accent-hover transition-colors">"Suggest"</button>
                            <p class="text-[10px] text-zinc-600 leading-tight">"Suggestions are reviewed by an admin before going live."</p>
                        </div>
                        <div id="channel-list" class="space-y-0.5">
                            <div class="text-center py-6"><div class="inline-block animate-spin w-4 h-4 border-2 border-zinc-700 border-t-accent rounded-full"></div></div>
                        </div>
                    </div>
                    <a href="/articles" class="mt-2 flex items-center gap-2.5 px-3 py-2.5 rounded-xl bg-surface-card border border-zinc-800 text-sm text-zinc-300 hover:border-zinc-700 hover:text-white transition-colors">
                        <i class="ph ph-article text-base text-accent"></i><span class="flex-1">"Articles"</span><i class="ph ph-arrow-right text-zinc-600"></i>
                    </a>
                </aside>

                // ── Feed column ──
                <div class="min-w-0 order-1 lg:order-2">
                    <div class="flex items-center justify-between mb-4">
                        <div class="min-w-0">
                            <h1 class="text-2xl font-bold flex items-center gap-2"><i id="channel-icon" class="ph ph-globe-hemisphere-west text-accent"></i><span id="channel-title">"Community"</span></h1>
                            <p id="channel-sub" class="text-zinc-500 text-sm mt-0.5">"Everything happening across Renzora"</p>
                        </div>
                        <button id="live-toggle" title="Toggle live updates" class="shrink-0 inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-white/[0.04] border border-white/[0.06] text-zinc-300 hover:text-white transition-colors">
                            <span id="live-dot" class="w-2 h-2 rounded-full bg-emerald-400"></span><span id="live-label">"Live"</span>
                        </button>
                    </div>

                    // Composer (signed-in) / sign-in prompt
                    <div id="composer" class="hidden mb-4 p-4 bg-surface-card border border-zinc-800 rounded-2xl">
                        <textarea id="post-body" rows="3" placeholder="Share what you're building… (@name to tag someone)" class="w-full bg-transparent text-sm text-zinc-100 placeholder-zinc-600 outline-none resize-none"></textarea>
                        <div id="media-previews" class="hidden grid grid-cols-4 gap-2 mt-2"></div>
                        <div class="flex items-center justify-between mt-3 pt-3 border-t border-zinc-800/70">
                            <div class="flex items-center gap-2">
                                <button id="attach-btn" title="Attach image" class="w-8 h-8 rounded-lg flex items-center justify-center text-zinc-400 hover:text-accent hover:bg-white/[0.04] transition-colors"><i class="ph ph-image text-lg"></i></button>
                                <input id="attach-input" type="file" accept="image/png,image/jpeg,image/webp,image/gif" class="hidden" />
                                <select id="post-visibility" class="bg-surface border border-zinc-800 rounded-lg text-xs text-zinc-300 px-2 py-1.5 outline-none focus:border-accent">
                                    <option value="public">"Public"</option>
                                    <option value="followers">"Followers"</option>
                                    <option value="friends">"Friends"</option>
                                </select>
                                <span id="post-target" class="text-[11px] text-zinc-600"></span>
                            </div>
                            <button id="post-btn" class="inline-flex items-center gap-1.5 px-4 py-1.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors disabled:opacity-50"><i class="ph ph-paper-plane-right"></i>"Post"</button>
                        </div>
                    </div>
                    <div id="composer-signin" class="hidden mb-4 p-6 bg-surface-card border border-zinc-800 rounded-2xl text-center">
                        <p class="text-sm text-zinc-400 mb-3">"Sign in to join the conversation and post to channels."</p>
                        <a href="/login?redirect=/community" class="inline-flex items-center gap-1.5 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">"Sign In"</a>
                    </div>

                    // New posts pill
                    <button id="new-posts-pill" class="hidden w-full mb-4 px-4 py-2 rounded-lg text-sm font-medium bg-accent/15 border border-accent/30 text-accent hover:bg-accent/25 transition-colors">
                        <i class="ph ph-arrow-up"></i>" New posts — see what's fresh"
                    </button>

                    // Filter bar
                    <div id="filter-bar" class="hidden flex-wrap items-center gap-2 mb-4 text-xs">
                        <select id="f-sort" class="bg-surface-card border border-zinc-800 rounded-lg text-zinc-300 px-2.5 py-1.5 outline-none focus:border-accent">
                            <option value="recent">"Recent"</option>
                            <option value="popular">"Most popular"</option>
                        </select>
                        <select id="f-time" class="bg-surface-card border border-zinc-800 rounded-lg text-zinc-300 px-2.5 py-1.5 outline-none focus:border-accent">
                            <option value="all">"All time"</option>
                            <option value="today">"Today"</option>
                            <option value="week">"This week"</option>
                            <option value="month">"This month"</option>
                        </select>
                        <select id="f-audience" class="bg-surface-card border border-zinc-800 rounded-lg text-zinc-300 px-2.5 py-1.5 outline-none focus:border-accent">
                            <option value="everyone">"Everyone"</option>
                            <option value="following">"Following"</option>
                            <option value="friends">"Friends"</option>
                        </select>
                    </div>

                    <div id="feed-list" class="space-y-4"></div>
                    <div id="feed-loading" class="text-center py-12"><div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-accent rounded-full"></div></div>
                    <div id="feed-empty" class="hidden text-center py-16 text-sm text-zinc-500">"Nothing here yet. Be the first to post!"</div>
                    <button id="load-more" class="hidden w-full mt-4 px-4 py-2.5 rounded-lg text-sm font-medium bg-surface-card border border-zinc-800 text-zinc-300 hover:border-zinc-700 transition-colors">"Load more"</button>
                </div>
            </div>
        </section>
        <script>
            r##"
            (function() {
              const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
              let me = null;
              try { me = JSON.parse(decodeURIComponent(document.cookie.match('(^|;)\\s*user\\s*=\\s*([^;]+)')?.pop() || '')); } catch(e) {}
              const isMod = me && (me.role === 'admin' || me.role === 'moderator');
              const authHeaders = token ? { 'Authorization': 'Bearer ' + token } : {};

              const REACTIONS = ['ph-heart','ph-thumbs-up','ph-fire','ph-star','ph-smiley','ph-confetti','ph-rocket','ph-lightning','ph-trophy','ph-hand-heart','ph-sparkle','ph-eyes','ph-thumbs-down','ph-smiley-sad','ph-brain','ph-game-controller','ph-paint-brush','ph-code','ph-check-circle','ph-question','ph-warning','ph-skull','ph-crown','ph-coffee'];

              const state = { channel: null, sort: 'recent', timeframe: 'all', audience: 'everyone', live: true, lastId: null, more: false, loading: false };
              let channels = [];
              let allPosts = [];
              let mediaUrls = [];
              let followingSet = null, friendsSet = null;
              let confirmDelete = {};

              const $ = id => document.getElementById(id);
              function esc(s){ const d = document.createElement('div'); d.textContent = s == null ? '' : String(s); return d.innerHTML; }
              function timeAgo(iso){
                const t = new Date(iso).getTime(); if (isNaN(t)) return '';
                const s = Math.floor((Date.now()-t)/1000);
                if (s < 60) return 'just now';
                const m = Math.floor(s/60); if (m < 60) return m+'m';
                const h = Math.floor(m/60); if (h < 24) return h+'h';
                const d = Math.floor(h/24); if (d < 7) return d+'d';
                return new Date(iso).toLocaleDateString('en-US',{month:'short',day:'numeric'});
              }
              function avatarHtml(url, name, size){
                const cls = 'w-'+size+' h-'+size;
                if (url) return `<img src="${esc(url)}" class="${cls} rounded-full object-cover shrink-0" />`;
                const ch = (name||'?').charAt(0).toUpperCase();
                return `<div class="${cls} rounded-full bg-accent/20 text-accent flex items-center justify-center font-semibold shrink-0">${esc(ch)}</div>`;
              }

              // ── Channels ──────────────────────────────────────────────────
              async function loadChannels(){
                try {
                  const res = await fetch('/api/feed/channels', { headers: authHeaders });
                  channels = res.ok ? await res.json() : [];
                } catch(e){ channels = []; }
                renderChannels();
              }
              function renderChannels(){
                const el = $('channel-list');
                let html = channelRow(null, 'ph-globe-hemisphere-west', 'All', '');
                html += channels.map(c => channelRow(c.slug, c.icon, c.name, c.post_count)).join('');
                el.innerHTML = html;
              }
              function channelRow(slug, icon, name, count){
                const active = state.channel === slug;
                return `<button onclick="__selChannel(${slug ? "'"+esc(slug)+"'" : 'null'})" class="w-full flex items-center gap-2.5 px-2 py-1.5 rounded-lg text-sm text-left transition-colors ${active ? 'bg-accent/15 text-accent' : 'text-zinc-300 hover:bg-white/[0.04]'}">
                  <i class="ph ${esc(icon||'ph-hash')} text-base ${active?'text-accent':'text-zinc-500'}"></i>
                  <span class="flex-1 truncate">${esc(name)}</span>
                  ${count !== '' ? `<span class="text-[10px] text-zinc-600">${count}</span>` : ''}
                </button>`;
              }
              window.__selChannel = function(slug){
                state.channel = slug;
                const c = slug ? channels.find(x => x.slug === slug) : null;
                $('channel-title').textContent = c ? c.name : 'Community';
                $('channel-icon').className = 'ph ' + (c ? (c.icon||'ph-hash') : 'ph-globe-hemisphere-west') + ' text-accent';
                $('channel-sub').textContent = c ? (c.description || ('#'+c.slug)) : 'Everything happening across Renzora';
                updateComposerTarget();
                renderChannels();
                loadFeed(true);
              };

              function updateComposerTarget(){
                const t = $('post-target'); if (!t) return;
                const c = state.channel ? channels.find(x => x.slug === state.channel) : null;
                t.textContent = c ? ('Posting to #'+c.slug) : 'Posting to the main feed';
              }

              // ── Feed ──────────────────────────────────────────────────────
              async function loadFeed(refresh){
                if (!token){ $('feed-loading').classList.add('hidden'); return; }
                if (state.loading) return;
                state.loading = true;
                if (refresh){ allPosts = []; state.lastId = null; $('feed-list').innerHTML = ''; state.pendingNew = false; $('new-posts-pill').classList.add('hidden'); }
                $('feed-loading').classList.remove('hidden');
                let url = '/api/feed/feed?limit=20';
                if (state.lastId) url += '&before=' + state.lastId;
                if (state.channel) url += '&channel=' + encodeURIComponent(state.channel);
                try {
                  const res = await fetch(url, { headers: authHeaders });
                  const data = res.ok ? await res.json() : [];
                  if (data.length){
                    allPosts = allPosts.concat(data);
                    state.lastId = data[data.length-1].id;
                  }
                  state.more = data.length >= 20;
                } catch(e){}
                state.loading = false;
                $('feed-loading').classList.add('hidden');
                renderFeed();
              }

              function visiblePosts(){
                let list = allPosts.filter(p => !(p.hidden && (!me || p.user_id !== me.id)));
                if (state.audience === 'following' && followingSet) list = list.filter(p => (me && p.user_id === me.id) || followingSet.has((p.username||'').toLowerCase()));
                if (state.audience === 'friends' && friendsSet) list = list.filter(p => friendsSet.has((p.username||'').toLowerCase()));
                const spans = { today: 864e5, week: 6048e5, month: 2592e6 };
                if (spans[state.timeframe]){ const now = Date.now(); list = list.filter(p => now - new Date(p.created_at).getTime() <= spans[state.timeframe]); }
                if (state.sort === 'popular') list = list.slice().sort((a,b) => (b.like_count-a.like_count) || (new Date(b.created_at)-new Date(a.created_at)));
                return list;
              }

              function renderFeed(){
                const list = visiblePosts();
                $('feed-list').innerHTML = list.map(postCard).join('');
                $('feed-empty').classList.toggle('hidden', list.length > 0 || state.loading);
                $('load-more').classList.toggle('hidden', !state.more);
              }

              function reactionsHtml(p){
                const chips = (p.reactions||[]).map(r =>
                  `<button onclick="__react('${p.id}','${esc(r.icon)}')" class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs border transition-colors ${r.reacted ? 'bg-accent/15 border-accent/40 text-accent' : 'bg-white/[0.03] border-zinc-800 text-zinc-400 hover:border-zinc-700'}"><i class="ph ${esc(r.icon)}"></i>${r.count}</button>`
                ).join('');
                return `<div id="reactions-${p.id}" class="flex flex-wrap items-center gap-1.5">${chips}
                  <button onclick="__openPicker(event,'${p.id}')" class="w-6 h-6 rounded-full flex items-center justify-center text-zinc-500 hover:text-accent hover:bg-white/[0.04] transition-colors" title="Add reaction"><i class="ph ph-smiley-sticker"></i></button>
                </div>`;
              }

              function postCard(p){
                if (p.hidden && me && p.user_id === me.id){
                  return `<div id="post-${p.id}" class="p-4 bg-surface-card border border-amber-500/20 rounded-2xl text-sm text-amber-400/90 flex items-center justify-between gap-3">
                    <span><i class="ph ph-eye-slash"></i> This post is hidden pending review.</span>
                    ${p.review_requested ? '<span class="text-xs text-zinc-500">Review requested</span>' : `<button onclick="__requestReview('${p.id}')" class="px-3 py-1 rounded-lg text-xs bg-white/[0.06] border border-white/[0.08] text-zinc-200 hover:text-white">Request review</button>`}
                  </div>`;
                }
                const channelChip = p.channel_slug ? `<a href="/community/channel/${esc(p.channel_slug)}" class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded-md text-[11px] bg-white/[0.04] text-zinc-400 hover:text-accent"><i class="ph ${esc(p.channel_icon||'ph-hash')}"></i>${esc(p.channel_name||p.channel_slug)}</a>` : '';
                const vis = p.visibility && p.visibility !== 'public' ? `<span class="text-[11px] text-zinc-600" title="${esc(p.visibility)}">· <i class="ph ${p.visibility==='friends'?'ph-users':'ph-user-check'}"></i></span>` : '';
                const longBody = (p.body||'').length > 500;
                const bodyShown = longBody ? esc(p.body.slice(0,500))+'…' : esc(p.body||'');
                const media = mediaGrid(p.media_urls||[]);
                const canDelete = me && (p.user_id === me.id || isMod);
                return `<article id="post-${p.id}" class="p-5 bg-surface-card border border-zinc-800 rounded-2xl">
                  <div class="flex items-start gap-3">
                    <a href="/profile/${esc(p.username)}">${avatarHtml(p.avatar_url, p.username, 10)}</a>
                    <div class="flex-1 min-w-0">
                      <div class="flex items-center gap-1.5 flex-wrap">
                        <a href="/profile/${esc(p.username)}" class="text-sm font-semibold hover:text-accent transition-colors">${esc(p.username)}</a>
                        ${p.role && p.role!=='user' ? `<i class="ph ph-seal-check text-accent text-xs" title="${esc(p.role)}"></i>` : ''}
                        ${channelChip}
                        <span class="text-[11px] text-zinc-600">· ${timeAgo(p.created_at)}</span>
                        ${vis}
                      </div>
                      <p class="text-sm text-zinc-200 whitespace-pre-wrap mt-1.5 break-words" id="body-${p.id}">${bodyShown}</p>
                      ${longBody ? `<button onclick="__expand('${p.id}')" data-full="${esc(p.body)}" class="text-xs text-accent mt-1">See more</button>` : ''}
                      ${media}
                      <div class="mt-3">${reactionsHtml(p)}</div>
                      <div class="flex items-center gap-4 mt-3 text-zinc-500">
                        <button onclick="__like('${p.id}')" class="like-btn inline-flex items-center gap-1.5 text-xs hover:text-accent transition-colors">
                          <i class="ph ${p.is_liked?'ph-arrow-fat-up-fill text-accent':'ph-arrow-fat-up'}"></i><span id="like-${p.id}" class="${p.is_liked?'text-accent':''}">${p.like_count}</span>
                        </button>
                        <button onclick="__toggleComments('${p.id}')" class="inline-flex items-center gap-1.5 text-xs hover:text-accent transition-colors">
                          <i class="ph ph-chat-circle"></i><span id="ccount-${p.id}">${p.comment_count}</span>
                        </button>
                        <div class="flex-1"></div>
                        ${me && p.user_id !== me.id ? `<button onclick="__report('${p.id}')" class="text-xs hover:text-amber-400 transition-colors" title="Report"><i class="ph ph-flag"></i></button>` : ''}
                        ${canDelete ? `<button onclick="__delete(event,'${p.id}')" class="text-xs hover:text-red-400 transition-colors" title="Delete"><i class="ph ph-trash"></i></button>` : ''}
                      </div>
                      <div id="comments-${p.id}" class="hidden mt-3 pt-3 border-t border-zinc-800/60"></div>
                    </div>
                  </div>
                </article>`;
              }

              function mediaGrid(urls){
                if (!urls.length) return '';
                if (urls.length === 1) return `<div class="mt-3 rounded-xl overflow-hidden border border-zinc-800"><img data-zoom src="${esc(urls[0])}" class="w-full max-h-96 object-cover" /></div>`;
                const cells = urls.slice(0,4).map(u => `<img data-zoom src="${esc(u)}" class="w-full h-40 object-cover rounded-lg border border-zinc-800" />`).join('');
                return `<div class="grid grid-cols-2 gap-2 mt-3">${cells}</div>`;
              }

              function findPost(id){ return allPosts.find(p => p.id === id); }

              window.__expand = function(id){
                const btn = event.target; const p = findPost(id); if (!p) return;
                $('body-'+id).textContent = p.body; btn.remove();
              };

              // ── Post actions ──────────────────────────────────────────────
              window.__like = async function(id){
                if (!token){ window.location.href='/login?redirect=/community'; return; }
                const p = findPost(id); if (!p) return;
                try {
                  const res = await fetch('/api/feed/posts/'+id+'/like', { method:'POST', headers: authHeaders });
                  const d = await res.json();
                  p.is_liked = d.liked; p.like_count += d.liked ? 1 : -1;
                  const cnt = $('like-'+id); if (cnt){ cnt.textContent = p.like_count; cnt.className = d.liked?'text-accent':''; cnt.previousElementSibling.className = 'ph ' + (d.liked?'ph-arrow-fat-up-fill text-accent':'ph-arrow-fat-up'); }
                } catch(e){}
              };

              window.__react = async function(id, icon){
                if (!token){ window.location.href='/login?redirect=/community'; return; }
                const p = findPost(id); if (!p) return;
                try {
                  const res = await fetch('/api/feed/posts/'+id+'/reactions', { method:'POST', headers:{...authHeaders,'Content-Type':'application/json'}, body: JSON.stringify({icon}) });
                  const d = await res.json();
                  p.reactions = p.reactions || [];
                  const ex = p.reactions.find(r => r.icon === icon);
                  if (d.reacted){ if (ex){ ex.count++; ex.reacted = true; } else p.reactions.push({icon,count:1,reacted:true}); }
                  else if (ex){ ex.count--; ex.reacted = false; if (ex.count <= 0) p.reactions = p.reactions.filter(r => r.icon !== icon); }
                  const box = $('reactions-'+id); if (box) box.outerHTML = reactionsHtml(p);
                } catch(e){}
              };

              window.__openPicker = function(ev, id){
                ev.stopPropagation();
                let pop = $('reaction-pop');
                if (!pop){ pop = document.createElement('div'); pop.id='reaction-pop'; pop.className='fixed z-50 p-2 bg-surface-card border border-zinc-700 rounded-xl shadow-2xl grid grid-cols-6 gap-1'; document.body.appendChild(pop);
                  document.addEventListener('click', e => { if (pop && !pop.contains(e.target)) pop.classList.add('hidden'); }); }
                pop.innerHTML = REACTIONS.map(i => `<button onclick="__react('${id}','${i}');document.getElementById('reaction-pop').classList.add('hidden')" class="w-8 h-8 rounded-lg flex items-center justify-center text-lg text-zinc-300 hover:bg-accent/20 hover:text-accent"><i class="ph ${i}"></i></button>`).join('');
                const r = ev.currentTarget.getBoundingClientRect();
                pop.style.left = Math.min(r.left, window.innerWidth-230)+'px';
                pop.style.top = (r.bottom+6)+'px';
                pop.classList.remove('hidden');
              };

              window.__report = async function(id){
                if (!confirm('Report this post to moderators?')) return;
                try {
                  const res = await fetch('/api/feed/posts/'+id+'/report', { method:'POST', headers:{...authHeaders,'Content-Type':'application/json'}, body: JSON.stringify({}) });
                  const d = await res.json();
                  alert(d.hidden ? 'Reported. This post has been hidden pending review.' : 'Thanks — this post has been reported.');
                } catch(e){}
              };

              window.__requestReview = async function(id){
                try { await fetch('/api/feed/posts/'+id+'/request-review', { method:'POST', headers: authHeaders }); const p=findPost(id); if(p){p.review_requested=true;} const el=$('post-'+id); if(el) el.outerHTML = postCard(findPost(id)); } catch(e){}
              };

              window.__delete = async function(ev, id){
                if (!confirmDelete[id]){ confirmDelete[id]=true; ev.currentTarget.classList.add('text-red-400'); ev.currentTarget.title='Click again to confirm'; setTimeout(()=>{confirmDelete[id]=false;},4000); return; }
                try {
                  const res = await fetch('/api/feed/posts/'+id, { method:'DELETE', headers: authHeaders });
                  if (res.ok){ allPosts = allPosts.filter(p => p.id !== id); const el=$('post-'+id); if(el) el.remove(); }
                } catch(e){}
              };

              // ── Comments ──────────────────────────────────────────────────
              window.__toggleComments = async function(id){
                const box = $('comments-'+id); if (!box) return;
                if (!box.classList.contains('hidden')){ box.classList.add('hidden'); return; }
                box.classList.remove('hidden');
                box.innerHTML = '<div class="text-xs text-zinc-600 py-2">Loading…</div>';
                await renderComments(id);
              };

              async function renderComments(id){
                const box = $('comments-'+id); if (!box) return;
                let comments = [];
                try { const res = await fetch('/api/feed/posts/'+id+'/comments', { headers: authHeaders }); comments = res.ok ? await res.json() : []; } catch(e){}
                const tops = comments.filter(c => !c.parent_id);
                const kids = c0 => comments.filter(c => c.parent_id === c0.id);
                const one = (c, nested) => `<div class="flex items-start gap-2 ${nested?'ml-8':''} py-1.5">
                    <a href="/profile/${esc(c.username)}">${avatarHtml(c.avatar_url, c.username, 7)}</a>
                    <div class="flex-1 min-w-0">
                      <div class="bg-white/[0.03] rounded-xl px-3 py-2">
                        <div class="flex items-center gap-1.5"><a href="/profile/${esc(c.username)}" class="text-xs font-semibold hover:text-accent">${esc(c.username)}</a><span class="text-[10px] text-zinc-600">${timeAgo(c.created_at)}</span></div>
                        <p class="text-sm text-zinc-200 whitespace-pre-wrap break-words">${esc(c.body)}</p>
                      </div>
                      <div class="flex items-center gap-3 mt-1 text-[11px] text-zinc-500">
                        <button onclick="__clike('${c.id}',this)" class="hover:text-accent"><i class="ph ${c.is_liked?'ph-heart-fill text-accent':'ph-heart'}"></i> <span>${c.like_count}</span></button>
                        ${!nested ? `<button onclick="__replyTo('${id}','${c.id}','${esc(c.username)}')" class="hover:text-accent">Reply</button>` : ''}
                        ${me && c.user_id === me.id ? `<button onclick="__cdelete('${id}','${c.id}')" class="hover:text-red-400">Delete</button>` : ''}
                      </div>
                    </div>
                  </div>`;
                const listHtml = tops.map(c => one(c,false) + kids(c).map(k => one(k,true)).join('')).join('') || '<p class="text-xs text-zinc-600 py-2">No comments yet.</p>';
                box.innerHTML = `<div class="space-y-1">${listHtml}</div>
                  ${token ? `<div class="flex items-center gap-2 mt-3" id="cform-${id}" data-parent="">
                    <input id="cinput-${id}" type="text" placeholder="Write a comment…" class="flex-1 px-3 py-1.5 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-100 outline-none focus:border-accent" />
                    <button onclick="__comment('${id}')" class="px-3 py-1.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover">Send</button>
                  </div>` : ''}`;
              }

              window.__replyTo = function(postId, parentId, name){
                const form = $('cform-'+postId); if (!form) return;
                form.dataset.parent = parentId;
                const inp = $('cinput-'+postId); inp.value = '@'+name+' '; inp.focus();
              };

              window.__comment = async function(id){
                const inp = $('cinput-'+id); const body = inp.value.trim(); if (!body) return;
                const form = $('cform-'+id); const parent = form && form.dataset.parent ? form.dataset.parent : null;
                try {
                  const res = await fetch('/api/feed/posts/'+id+'/comments', { method:'POST', headers:{...authHeaders,'Content-Type':'application/json'}, body: JSON.stringify({ body, parent_id: parent }) });
                  if (res.ok){ const p = findPost(id); if (p){ p.comment_count++; const c=$('ccount-'+id); if(c) c.textContent = p.comment_count; } inp.value=''; if(form) form.dataset.parent=''; await renderComments(id); }
                } catch(e){}
              };

              window.__clike = async function(cid, btn){
                try { const res = await fetch('/api/feed/comments/'+cid+'/like', { method:'POST', headers: authHeaders }); const d = await res.json();
                  const i = btn.querySelector('i'); const s = btn.querySelector('span'); let n = parseInt(s.textContent)||0;
                  i.className = 'ph ' + (d.liked?'ph-heart-fill text-accent':'ph-heart'); s.textContent = n + (d.liked?1:-1);
                } catch(e){}
              };

              window.__cdelete = async function(postId, cid){
                try { const res = await fetch('/api/feed/comments/'+cid, { method:'DELETE', headers: authHeaders });
                  if (res.ok){ const p=findPost(postId); if(p){ p.comment_count=Math.max(0,p.comment_count-1); const c=$('ccount-'+postId); if(c) c.textContent=p.comment_count; } await renderComments(postId); }
                } catch(e){}
              };

              // ── Composer ──────────────────────────────────────────────────
              function renderMediaPreviews(){
                const box = $('media-previews');
                box.classList.toggle('hidden', mediaUrls.length === 0);
                box.innerHTML = mediaUrls.map((u,i) => `<div class="relative"><img src="${esc(u)}" class="w-full h-16 object-cover rounded-lg border border-zinc-800" /><button onclick="__rmMedia(${i})" class="absolute top-0.5 right-0.5 w-5 h-5 rounded-full bg-black/70 text-white text-xs">&times;</button></div>`).join('');
              }
              window.__rmMedia = function(i){ mediaUrls.splice(i,1); renderMediaPreviews(); };

              function bindComposer(){
                $('attach-btn').addEventListener('click', () => $('attach-input').click());
                $('attach-input').addEventListener('change', async e => {
                  const file = e.target.files[0]; if (!file) return;
                  if (file.size > 5*1024*1024){ alert('Image must be under 5MB'); return; }
                  if (mediaUrls.length >= 4){ alert('Up to 4 images'); return; }
                  const fd = new FormData(); fd.append('image', file);
                  try { const res = await fetch('/api/feed/upload', { method:'POST', headers: authHeaders, body: fd }); const d = await res.json(); if (d.url){ mediaUrls.push(d.url); renderMediaPreviews(); } }
                  catch(err){ alert('Upload failed'); }
                  e.target.value = '';
                });
                $('post-btn').addEventListener('click', async () => {
                  const body = $('post-body').value.trim();
                  if (!body && mediaUrls.length === 0) return;
                  const btn = $('post-btn'); btn.disabled = true;
                  try {
                    const res = await fetch('/api/feed/posts', { method:'POST', headers:{...authHeaders,'Content-Type':'application/json'}, body: JSON.stringify({ body, visibility: $('post-visibility').value, media_urls: mediaUrls, channel: state.channel }) });
                    if (res.ok){ $('post-body').value=''; mediaUrls=[]; renderMediaPreviews(); loadFeed(true); }
                    else { const d = await res.json().catch(()=>({})); alert(d.error || 'Failed to post'); }
                  } catch(e){ alert('Failed to post'); }
                  btn.disabled = false;
                });
              }

              // ── Suggest channel ───────────────────────────────────────────
              function bindSuggest(){
                $('suggest-toggle').addEventListener('click', () => { if(!token){window.location.href='/login?redirect=/community';return;} $('suggest-box').classList.toggle('hidden'); });
                $('suggest-submit').addEventListener('click', async () => {
                  const name = $('suggest-name').value.trim(); if (!name) return;
                  try { const res = await fetch('/api/feed/channels/suggest', { method:'POST', headers:{...authHeaders,'Content-Type':'application/json'}, body: JSON.stringify({ name }) });
                    const d = await res.json();
                    if (res.ok){ $('suggest-name').value=''; $('suggest-box').classList.add('hidden'); alert('Thanks! Your channel suggestion is pending review.'); }
                    else alert(d.error || 'Could not suggest channel');
                  } catch(e){}
                });
              }

              // ── Filters + live ────────────────────────────────────────────
              async function ensureFollowingSet(){
                if (followingSet || !me) return;
                try { const res = await fetch('/api/profiles/'+encodeURIComponent(me.username)+'/following', { headers: authHeaders }); const d = res.ok?await res.json():[]; followingSet = new Set(d.map(u => (u.username||'').toLowerCase())); } catch(e){ followingSet = new Set(); }
              }
              async function ensureFriendsSet(){
                if (friendsSet) return;
                try { const res = await fetch('/api/gameservices/friends', { headers: authHeaders }); const d = res.ok?await res.json():[]; friendsSet = new Set(d.map(u => (u.username||'').toLowerCase())); } catch(e){ friendsSet = new Set(); }
              }
              function bindFilters(){
                $('f-sort').addEventListener('change', e => { state.sort = e.target.value; renderFeed(); });
                $('f-time').addEventListener('change', e => { state.timeframe = e.target.value; renderFeed(); });
                $('f-audience').addEventListener('change', async e => { state.audience = e.target.value; if (state.audience==='following') await ensureFollowingSet(); if (state.audience==='friends') await ensureFriendsSet(); renderFeed(); });
                $('live-toggle').addEventListener('click', () => {
                  state.live = !state.live;
                  $('live-label').textContent = state.live ? 'Live' : 'Paused';
                  $('live-dot').className = 'w-2 h-2 rounded-full ' + (state.live ? 'bg-emerald-400' : 'bg-zinc-500');
                  if (state.live && state.pendingNew) loadFeed(true);
                });
                $('new-posts-pill').addEventListener('click', () => loadFeed(true));
                $('load-more').addEventListener('click', () => loadFeed(false));
              }

              window.addEventListener('renzora:new_post', () => {
                if (state.live) { loadFeed(true); }
                else { state.pendingNew = true; $('new-posts-pill').classList.remove('hidden'); }
              });

              // ── Init ──────────────────────────────────────────────────────
              // Channel from /community/channel/:slug
              const m = window.location.pathname.match(/\/community\/channel\/([^\/]+)/);
              if (m) state.channel = decodeURIComponent(m[1]);

              if (token){ $('composer').classList.remove('hidden'); $('filter-bar').classList.remove('hidden'); $('filter-bar').classList.add('flex'); bindComposer(); }
              else { $('composer-signin').classList.remove('hidden'); }
              bindSuggest(); bindFilters();
              loadChannels().then(() => { updateComposerTarget(); if (state.channel) window.__selChannel(state.channel); });
              if (token) loadFeed(true); else { $('feed-loading').classList.add('hidden'); }
              loadMarketplaceStrip();

              async function loadMarketplaceStrip(){
                try {
                  const res = await fetch('/api/marketplace?page=1&sort=newest');
                  if (!res.ok) return;
                  const d = await res.json();
                  const assets = (d.assets||[]).slice(0,6);
                  if (!assets.length) return;
                  const strip = document.createElement('div');
                  strip.className = 'mb-4 p-4 bg-surface-card border border-zinc-800 rounded-2xl';
                  strip.innerHTML = `<div class="flex items-center justify-between mb-3"><h3 class="text-xs font-semibold uppercase tracking-wide text-zinc-500">New in the Marketplace</h3><a href="/marketplace" class="text-xs text-accent hover:text-accent-hover">Browse all</a></div>
                    <div class="grid grid-cols-3 md:grid-cols-6 gap-2">${assets.map(a => `<a href="/marketplace/asset/${esc(a.slug)}" class="group"><div class="aspect-square rounded-lg overflow-hidden bg-surface border border-zinc-800">${a.thumbnail_url?`<img src="${esc(a.thumbnail_url)}" class="w-full h-full object-cover group-hover:scale-105 transition-transform" loading="lazy" />`:'<div class="w-full h-full flex items-center justify-center text-zinc-700"><i class="ph ph-cube text-2xl"></i></div>'}</div><p class="text-[10px] text-zinc-400 mt-1 truncate">${esc(a.title)}</p></a>`).join('')}</div>`;
                  const composer = $('composer').classList.contains('hidden') ? $('composer-signin') : $('composer');
                  composer.insertAdjacentElement('afterend', strip);
                } catch(e){}
              }
            })();
            "##
        </script>
    }
}
