use leptos::prelude::*;

#[component]
pub fn ProfilePage() -> impl IntoView {
    view! {
        <section class="min-h-screen relative overflow-hidden">
            // Glow orbs
            <div class="fixed top-20 right-1/4 w-[500px] h-[500px] bg-accent/10 rounded-full blur-[150px] pointer-events-none"></div>
            <div class="fixed bottom-20 left-1/4 w-[400px] h-[400px] bg-purple-600/10 rounded-full blur-[120px] pointer-events-none"></div>

            <div id="pf-loading" class="flex items-center justify-center py-32">
                <div class="w-8 h-8 border-2 border-zinc-800 border-t-accent rounded-full animate-spin"></div>
            </div>
            <div id="pf-404" class="hidden flex items-center justify-center py-32">
                <div class="text-center"><i class="ph ph-user-circle text-5xl text-zinc-700 mb-3"></i><p class="text-zinc-500">"User not found."</p></div>
            </div>
            <div id="pf-content" class="hidden"></div>

            // Edit modal
            <div id="pf-edit-overlay" class="hidden fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm">
                <div class="w-[480px] bg-[#12121a] border border-white/[0.08] rounded-2xl p-6 shadow-2xl">
                    <div class="flex items-center justify-between mb-4">
                        <h3 class="text-sm font-semibold">"Edit Profile"</h3>
                        <button onclick="window._closeEdit()" class="text-zinc-500 hover:text-white"><i class="ph ph-x text-lg"></i></button>
                    </div>
                    <div class="grid grid-cols-2 gap-3 mb-4">
                        <div class="col-span-2"><label class="block text-[10px] text-zinc-500 mb-1">"Bio"</label><textarea id="ed-bio" rows="2" class="w-full px-3 py-2 bg-black/40 border border-white/[0.06] rounded-xl text-sm text-zinc-50 outline-none focus:border-accent resize-none"></textarea></div>
                        <div><label class="block text-[10px] text-zinc-500 mb-1">"Location"</label><input id="ed-loc" type="text" class="w-full px-3 py-2 bg-black/40 border border-white/[0.06] rounded-xl text-sm text-zinc-50 outline-none focus:border-accent" /></div>
                        <div><label class="block text-[10px] text-zinc-500 mb-1">"Website"</label><input id="ed-web" type="url" class="w-full px-3 py-2 bg-black/40 border border-white/[0.06] rounded-xl text-sm text-zinc-50 outline-none focus:border-accent" /></div>
                        <div><label class="block text-[10px] text-zinc-500 mb-1">"Profile Color"</label><input id="ed-pc" type="color" value="#6366f1" class="w-full h-9 rounded-xl cursor-pointer bg-transparent border border-white/[0.06]" /></div>
                        <div><label class="block text-[10px] text-zinc-500 mb-1">"Banner Color"</label><input id="ed-bc" type="color" value="#1e1b4b" class="w-full h-9 rounded-xl cursor-pointer bg-transparent border border-white/[0.06]" /></div>
                    </div>
                    <div class="flex gap-2">
                        <button onclick="window._saveProfile()" class="px-5 py-2 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover">"Save"</button>
                        <button onclick="window._closeEdit()" class="px-5 py-2 rounded-xl text-sm text-zinc-400 hover:text-white">"Cancel"</button>
                    </div>
                </div>
            </div>
        </section>

        <script>
            r#"
            const username = window.location.pathname.split('/').pop();
            const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
            const hdrs = token ? { 'Authorization': 'Bearer ' + token } : {};

            // Register functions immediately
            window._ptab = ptab;
            window._openEdit = () => document.getElementById('pf-edit-overlay').classList.remove('hidden');
            window._closeEdit = () => document.getElementById('pf-edit-overlay').classList.add('hidden');
            window._saveProfile = saveProfile;
            window._follow = doFollow;
            window._msg = doMsg;

            let assetPage = 0;
            let assetHasMore = true;
            let assetLoading = false;

            (async function() {
                const res = await fetch('/api/profiles/view/' + username, { headers: hdrs });
                document.getElementById('pf-loading').classList.add('hidden');

                if (!res.ok) { document.getElementById('pf-404').classList.remove('hidden'); return; }

                const p = await res.json();

                const userCookie = document.cookie.match('(^|;)\\s*user\\s*=\\s*([^;]+)')?.pop();
                let isOwn = false;
                if (userCookie) { try { isOwn = JSON.parse(decodeURIComponent(userCookie)).username === username; } catch(e) {} }

                // XP
                const lvl = p.level || 1, xp = p.total_xp || 0;
                const curXp = lvl*(lvl-1)*50, nxtXp = (lvl+1)*lvl*50;
                const pct = nxtXp > curXp ? Math.min(100, (xp-curXp)/(nxtXp-curXp)*100).toFixed(0) : 100;

                // Seller
                const sNames = ['','Bronze','Silver','Gold','Platinum','Diamond'];
                const sColors = ['','#CD7F32','#C0C0C0','#FFD700','#E5E4E2','#B9F2FF'];
                const sLvl = p.seller_level || 0;
                const sellerBadge = sLvl > 0 ? `<span class="text-[10px] font-bold px-2 py-0.5 rounded-md" style="background:${sColors[sLvl]}15;color:${sColors[sLvl]};border:1px solid ${sColors[sLvl]}30">${sNames[sLvl]} Seller</span>` : '';

                // Role
                const rc = {admin:'#ef4444',creator:'#8b5cf6',moderator:'#f59e0b',user:'#6b7280'}[p.role]||'#6b7280';

                // Avatar — 3D canvas or fallback image
                const avatarFallback = p.avatar_url
                    ? `<img src="${p.avatar_url}" class="w-full h-full object-cover" />`
                    : `<div class="w-full h-full flex items-center justify-center bg-gradient-to-br from-accent/20 to-purple-500/20"><i class="ph ph-user text-4xl" style="color:${p.profile_color||'#6366f1'}"></i></div>`;
                const avatar = `<canvas id="pf-avatar-3d" class="w-full h-full hidden"></canvas><div id="pf-avatar-fallback">${avatarFallback}</div>`;

                // Joined
                const jd = new Date(p.created_at);
                const joined = !isNaN(jd) ? jd.toLocaleDateString('en-US',{month:'long',year:'numeric'}) : '';

                // Actions
                let actions = '';
                if (!isOwn && token) {
                    actions += `<button onclick="window._follow('${p.username}')" class="px-3 py-1.5 rounded-lg text-xs font-medium transition-all ${p.is_following?'bg-white/[0.06] border border-white/[0.06] text-zinc-300 hover:text-red-400':'bg-accent text-white hover:bg-accent-hover'}">${p.is_following?'Unfollow':'Follow'}</button>`;
                    actions += `<button onclick="window._friend('${p.username}')" id="pf-friend-btn" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-white/[0.06] border border-white/[0.06] text-zinc-300 hover:text-white transition-all"><i class="ph ph-user-plus mr-1"></i>Add Friend</button>`;
                    actions += `<button onclick="window._msg('${p.id}')" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-white/[0.06] border border-white/[0.06] text-zinc-300 hover:text-white transition-all"><i class="ph ph-chat-circle-dots mr-1"></i>Message</button>`;
                    actions += `<button onclick="window._block('${p.username}')" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-white/[0.04] border border-red-900/30 text-red-400/70 hover:text-red-400 hover:bg-red-950/20 transition-all"><i class="ph ph-prohibit mr-1"></i>Block</button>`;
                }
                if (isOwn) {
                    actions += `<button onclick="window._openEdit()" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-white/[0.06] border border-white/[0.06] text-zinc-300 hover:text-white transition-all"><i class="ph ph-pencil-simple mr-1"></i>Edit</button>`;
                    actions += `<a href="/avatar/edit" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-accent/20 border border-accent/30 text-accent hover:bg-accent/30 transition-all inline-flex items-center"><i class="ph ph-person-arms-spread mr-1"></i>Avatar</a>`;
                }

                // Check friend status
                if (!isOwn && token) {
                    try {
                        var fres = await fetch('/api/gameservices/friends', { headers: { 'Authorization': 'Bearer ' + token } });
                        var friends = await fres.json();
                        if (Array.isArray(friends)) {
                            var match = friends.find(function(f) { return f.friend_username === username; });
                            if (match) {
                                var fbtn = document.getElementById('pf-friend-btn');
                                if (fbtn) { fbtn.innerHTML = '<i class="ph ph-user-minus mr-1"></i>Unfriend'; }
                            }
                        }
                    } catch(e) {}
                }

                // Social icons
                const si = {discord:'ph-discord-logo',twitch:'ph-twitch-logo',youtube:'ph-youtube-logo',twitter:'ph-twitter-logo',github:'ph-github-logo'};
                const socialHtml = (p.connections||[]).map(c =>
                    `<a href="${c.platform_url||'#'}" target="_blank" class="w-8 h-8 rounded-lg bg-white/[0.04] border border-white/[0.04] flex items-center justify-center text-zinc-400 hover:text-white hover:bg-white/[0.08] transition-all" title="${c.platform}"><i class="ph ${si[c.platform]||'ph-link'}"></i></a>`
                ).join('');

                // Badges
                const badgesHtml = p.badges.length ? p.badges.map(b =>
                    `<div class="flex items-center gap-2 p-2.5 rounded-xl bg-white/[0.03] border border-white/[0.04] hover:border-white/[0.08] transition-all" title="${b.description}"><div class="w-8 h-8 rounded-lg flex items-center justify-center" style="background:${b.color}15"><i class="ph ${b.icon} text-lg" style="color:${b.color}"></i></div><div><div class="text-xs font-medium" style="color:${b.color}">${b.name}</div><div class="text-[9px] text-zinc-600">${b.description}</div></div></div>`
                ).join('') : '<p class="text-zinc-600 text-sm text-center py-6 col-span-full">No badges yet</p>';

                const el = document.getElementById('pf-content');
                el.classList.remove('hidden');
                el.innerHTML = `
                <div class="max-w-[1100px] mx-auto px-6 py-8 relative z-10">

                    <!-- Top row: Avatar card + Info -->
                    <div class="flex flex-col lg:flex-row gap-5 mb-5">

                        <!-- Avatar card -->
                        <div class="w-full lg:w-[280px] shrink-0">
                            <div class="rounded-2xl border border-white/[0.06] bg-[#0e0e16]/80 backdrop-blur-xl overflow-hidden shadow-xl shadow-black/30">
                                <div class="aspect-square overflow-hidden relative">
                                    ${avatar}
                                </div>

                                <!-- XP bar under avatar -->
                                <div class="px-4 pt-3 pb-1">
                                    <div class="flex items-center gap-2 text-xs mb-1">
                                        <span class="font-bold text-accent">Lv ${lvl}</span>
                                        <div class="flex-1 h-2 bg-black/40 border border-white/[0.06] rounded-full overflow-hidden">
                                            <div class="h-full bg-gradient-to-r from-accent to-purple-500 rounded-full relative" style="width:${pct}%">
                                                <div class="absolute inset-0 bg-[linear-gradient(90deg,transparent_25%,rgba(255,255,255,0.15)_50%,transparent_75%)] bg-[length:200%_100%] animate-[xpShimmer_2s_linear_infinite]"></div>
                                            </div>
                                        </div>
                                        <span class="text-[9px] text-zinc-500">${pct}%</span>
                                    </div>
                                    <div class="text-[9px] text-zinc-600 text-right">${xp.toLocaleString()} / ${nxtXp.toLocaleString()} XP</div>
                                </div>
                                <!-- Name under avatar -->
                                <div class="p-4 pt-2">
                                    <div class="flex items-center gap-2 flex-wrap">
                                        <h1 class="text-lg font-bold">${p.username}</h1>
                                        <span class="text-[9px] font-bold uppercase tracking-wider px-1.5 py-0.5 rounded-md" style="background:${rc}15;color:${rc};border:1px solid ${rc}30">${p.role}</span>
                                        ${sellerBadge}
                                    </div>
                                    ${p.bio ? `<p class="text-xs text-zinc-400 mt-2 leading-relaxed">${p.bio}</p>` : ''}
                                    <div class="flex flex-wrap gap-x-3 gap-y-1 mt-2 text-[10px] text-zinc-500">
                                        ${p.location ? `<span class="flex items-center gap-1"><i class="ph ph-map-pin"></i>${p.location}</span>` : ''}
                                        ${p.website ? `<a href="${p.website}" target="_blank" class="flex items-center gap-1 text-accent hover:underline"><i class="ph ph-link"></i>${p.website.replace('https://','').replace('http://','')}</a>` : ''}
                                        ${joined ? `<span class="flex items-center gap-1"><i class="ph ph-calendar"></i>${joined}</span>` : ''}
                                    </div>
                                    ${socialHtml ? `<div class="flex gap-1.5 mt-3">${socialHtml}</div>` : ''}
                                    <div class="flex flex-wrap gap-2 mt-3">${actions}</div>
                                </div>
                            </div>
                        </div>

                        <!-- Right column -->
                        <div class="flex-1 flex flex-col gap-5 min-w-0">

                            <!-- Stats row -->
                            <div class="grid grid-cols-4 gap-3">
                                <div class="rounded-xl border border-white/[0.06] bg-[#0e0e16]/80 backdrop-blur-xl p-4 text-center">
                                    <div class="text-2xl font-bold text-white">${(p.follower_count||0).toLocaleString()}</div>
                                    <div class="text-[10px] text-zinc-500 mt-0.5">Followers</div>
                                </div>
                                <div class="rounded-xl border border-white/[0.06] bg-[#0e0e16]/80 backdrop-blur-xl p-4 text-center">
                                    <div class="text-2xl font-bold text-white">${(p.following_count||0).toLocaleString()}</div>
                                    <div class="text-[10px] text-zinc-500 mt-0.5">Following</div>
                                </div>
                                <div class="rounded-xl border border-white/[0.06] bg-[#0e0e16]/80 backdrop-blur-xl p-4 text-center">
                                    <div class="text-2xl font-bold text-white">${(p.asset_count||0).toLocaleString()}</div>
                                    <div class="text-[10px] text-zinc-500 mt-0.5">Assets</div>
                                </div>
                                <div class="rounded-xl border border-white/[0.06] bg-[#0e0e16]/80 backdrop-blur-xl p-4 text-center">
                                    <div class="text-2xl font-bold text-accent">${xp.toLocaleString()}</div>
                                    <div class="text-[10px] text-zinc-500 mt-0.5">Total XP</div>
                                </div>
                            </div>

                            <!-- Badges -->
                            <div class="rounded-2xl border border-white/[0.06] bg-[#0e0e16]/80 backdrop-blur-xl p-5">
                                <div class="flex items-center justify-between mb-3">
                                    <h2 class="text-sm font-semibold flex items-center gap-1.5"><i class="ph ph-medal text-accent"></i>Badges</h2>
                                    <span class="text-[10px] text-zinc-600">${p.badges.length} earned</span>
                                </div>
                                <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">${badgesHtml}</div>
                            </div>
                        </div>
                    </div>

                    <!-- Tabs: Assets & Activity -->
                    <div class="rounded-2xl border border-white/[0.06] bg-[#0e0e16]/80 backdrop-blur-xl overflow-hidden">
                        <div class="flex border-b border-white/[0.05]">
                            <button onclick="window._ptab('assets')" id="ptab-assets" class="ptab flex-1 py-3 text-sm font-medium text-white border-b-2 border-accent transition-all"><i class="ph ph-package mr-1.5"></i>Assets</button>
                            <button onclick="window._ptab('activity')" id="ptab-activity" class="ptab flex-1 py-3 text-sm font-medium text-zinc-500 border-b-2 border-transparent hover:text-zinc-300 transition-all"><i class="ph ph-clock-counter-clockwise mr-1.5"></i>Activity</button>
                        </div>
                        <div id="pf-tab" class="p-5"></div>
                    </div>
                </div>`;

                // Prefill edit form
                if (isOwn) {
                    document.getElementById('ed-bio').value = p.bio || '';
                    document.getElementById('ed-loc').value = p.location || '';
                    document.getElementById('ed-web').value = p.website || '';
                    document.getElementById('ed-pc').value = p.profile_color || '#6366f1';
                    document.getElementById('ed-bc').value = p.banner_color || '#1e1b4b';
                }

                ptab('assets');

                // Animate
                if (typeof anime !== 'undefined') {
                    anime({ targets: '#pf-content .rounded-2xl, #pf-content .rounded-xl', opacity: [0,1], translateY: [25,0], delay: anime.stagger(60), duration: 600, easing: 'easeOutCubic' });
                }

                // Load 3D avatar
                load3DAvatar(p.id);
            })();

            function renderAssetCard(a) {
                return '<a href="/marketplace/asset/' + a.slug + '" class="block group"><div class="rounded-xl border border-white/[0.04] bg-white/[0.02] overflow-hidden hover:border-white/[0.1] transition-all"><div class="aspect-[4/3] bg-black/20 overflow-hidden">' + (a.thumbnail_url ? '<img src="' + a.thumbnail_url + '" class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300" />' : '<div class="w-full h-full flex items-center justify-center"><i class="ph ph-package text-2xl text-zinc-700"></i></div>') + '</div><div class="p-3"><div class="text-sm font-medium truncate group-hover:text-accent transition-colors">' + a.name + '</div><div class="flex justify-between mt-1.5 text-xs text-zinc-500"><span>' + a.downloads + ' dl</span><span class="font-semibold ' + (a.price_credits===0?'text-emerald-400':'text-zinc-300') + '">' + (a.price_credits===0?'Free':a.price_credits+' cr') + '</span></div></div></div></a>';
            }

            async function loadAssets() {
                if (assetLoading || !assetHasMore) return;
                assetLoading = true;
                assetPage++;
                var grid = document.getElementById('pf-assets-grid');
                var btn = document.getElementById('pf-assets-more');
                if (btn) btn.disabled = true;
                try {
                    var r = await fetch('/api/profiles/' + username + '/assets?page=' + assetPage);
                    if (!r.ok) { assetLoading = false; return; }
                    var data = await r.json();
                    assetHasMore = data.has_more;
                    if (!grid) return;
                    var items = data.assets || [];
                    if (assetPage === 1 && items.length === 0) {
                        grid.parentElement.innerHTML = '<p class="text-zinc-600 text-sm text-center py-10">No published assets yet.</p>';
                        return;
                    }
                    grid.insertAdjacentHTML('beforeend', items.map(renderAssetCard).join(''));
                    if (!assetHasMore && btn) btn.remove();
                    else if (btn) btn.disabled = false;
                } catch(e) {}
                assetLoading = false;
            }
            window._loadMoreAssets = loadAssets;

            function ptab(name) {
                document.querySelectorAll('.ptab').forEach(t => {
                    const a = t.id === 'ptab-' + name;
                    t.className = 'ptab flex-1 py-3 text-sm font-medium transition-all ' + (a ? 'text-white border-b-2 border-accent' : 'text-zinc-500 border-b-2 border-transparent hover:text-zinc-300');
                });
                const el = document.getElementById('pf-tab');
                if (!el) return;
                if (name === 'assets') {
                    assetPage = 0;
                    assetHasMore = true;
                    el.innerHTML = '<div id="pf-assets-grid" class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-3"></div><div class="flex justify-center mt-4" id="pf-assets-more-wrap"><button id="pf-assets-more" onclick="window._loadMoreAssets()" class="px-5 py-2 rounded-xl text-sm font-medium bg-white/[0.06] border border-white/[0.06] text-zinc-300 hover:text-white hover:bg-white/[0.08] transition-all">Load More</button></div>';
                    loadAssets();
                } else {
                    el.innerHTML = '<div id="pf-feed"><p class="text-zinc-600 text-sm text-center py-10">Loading...</p></div>';
                    loadFeed();
                }
            }

            async function loadFeed() {
                var el = document.getElementById('pf-feed');
                if (!el) return;
                try {
                    var r = await fetch('/api/feed/users/' + username + '/posts?limit=20', { headers: hdrs });
                    var posts = await r.json();
                    if (Array.isArray(posts) && posts.length) {
                        el.innerHTML = posts.map(function(p) {
                            var d = new Date(p.created_at);
                            var diff = (Date.now() - d) / 1000;
                            var ago = diff < 3600 ? Math.floor(diff/60) + 'm ago' : diff < 86400 ? Math.floor(diff/3600) + 'h ago' : d.toLocaleDateString('en-US', {month:'short',day:'numeric'});
                            var div = document.createElement('div'); div.textContent = p.body;
                            return '<div class="p-4 rounded-xl bg-white/[0.02] border border-white/[0.04] mb-2"><p class="text-sm text-zinc-300">' + div.innerHTML + '</p><div class="flex gap-4 mt-2 text-xs text-zinc-600"><span><i class="ph ph-heart"></i> ' + p.like_count + '</span><span><i class="ph ph-chat-circle"></i> ' + p.comment_count + '</span><span>' + ago + '</span></div></div>';
                        }).join('');
                    } else { el.innerHTML = '<p class="text-zinc-600 text-sm text-center py-10">No activity yet.</p>'; }
                } catch(e) { el.innerHTML = '<p class="text-zinc-600 text-sm text-center py-10">No activity yet.</p>'; }
            }

            async function saveProfile() {
                if (!token) return;
                var body = { bio: document.getElementById('ed-bio').value, location: document.getElementById('ed-loc').value, website: document.getElementById('ed-web').value, profile_color: document.getElementById('ed-pc').value, banner_color: document.getElementById('ed-bc').value };
                var r = await fetch('/api/auth/me', { method: 'PUT', headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' }, body: JSON.stringify(body) });
                if (r.ok) window.location.reload(); else alert('Failed to save');
            }

            async function doFollow(u) {
                if (!token) { window.location.href = '/login'; return; }
                await fetch('/api/profiles/follow/' + u, { method: 'POST', headers: { 'Authorization': 'Bearer ' + token } });
                window.location.reload();
            }

            async function doMsg(uid) {
                if (!token) { window.location.href = '/login'; return; }
                var r = await fetch('/api/messages/conversations/dm/' + uid, { method: 'POST', headers: { 'Authorization': 'Bearer ' + token } });
                var d = await r.json();
                if (d.conversation_id) window.location.href = '/messages?conv=' + d.conversation_id;
            }

            async function doFriend(u) {
                if (!token) { window.location.href = '/login'; return; }
                var r = await fetch('/api/profiles/friend/' + u, { method: 'POST', headers: { 'Authorization': 'Bearer ' + token } });
                var d = await r.json();
                var btn = document.getElementById('pf-friend-btn');
                if (btn) {
                    if (d.status === 'accepted') btn.innerHTML = '<i class="ph ph-user-minus mr-1"></i>Unfriend';
                    else if (d.status === 'pending') btn.innerHTML = '<i class="ph ph-clock mr-1"></i>Pending';
                    else btn.innerHTML = '<i class="ph ph-user-plus mr-1"></i>Add Friend';
                }
            }

            async function doBlock(u) {
                if (!confirm('Block ' + u + '?')) return;
                if (!token) return;
                await fetch('/api/profiles/block/' + u, { method: 'POST', headers: { 'Authorization': 'Bearer ' + token } });
                window.location.reload();
            }

            // ── 3D Avatar render ──
            async function load3DAvatar(userId) {
                try {
                    var avRes = await fetch('/api/avatar/user/' + userId);
                    if (!avRes.ok) return;
                    var av = await avRes.json();
                    if (!av.equipped_parts || !av.equipped_parts.character) return;

                    var s = document.createElement('script');
                    s.type = 'module';
                    s.textContent = `
                        import * as THREE from 'three';
                        import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';
                        const loader = new GLTFLoader();
                        const load = u => new Promise((r,e) => loader.load(u, r, undefined, e));
                        const canvas = document.getElementById('pf-avatar-3d');
                        if (!canvas) throw 'no canvas';
                        const w = canvas.parentElement.clientWidth, h = canvas.parentElement.clientHeight;
                        const scene = new THREE.Scene();
                        const cam = new THREE.PerspectiveCamera(25, w/h, 0.1, 20);
                        cam.position.set(0, 0.9, 3.2);
                        const renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true });
                        renderer.setSize(w, h);
                        renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
                        renderer.setClearColor(0x000000, 0);
                        renderer.outputColorSpace = THREE.SRGBColorSpace;
                        scene.add(new THREE.AmbientLight(0xffffff, 1.0));
                        const dir = new THREE.DirectionalLight(0xffffff, 1.2);
                        dir.position.set(2, 3, 3); scene.add(dir);
                        const gltf = await load('/assets/avatar/characters/${av.equipped_parts.character}.glb');
                        const model = gltf.scene;
                        const colors = ${JSON.stringify(av.equipped_parts.colors || {})};
                        model.traverse(ch => {
                            if (ch.isMesh) { ch.material.side = THREE.FrontSide; ch.material.depthWrite = true; ch.material.transparent = false;
                                if (colors[ch.name]) { ch.material = ch.material.clone(); ch.material.color.set(colors[ch.name]); }
                            }
                        });
                        scene.add(model);
                        // Attach items
                        let rSlot, lSlot;
                        model.traverse(ch => { if(ch.isBone && ch.name==='handslotr') rSlot=ch; if(ch.isBone && ch.name==='handslotl') lSlot=ch; });
                        const wp = '${av.equipped_parts.weapon||'none'}';
                        const sh = '${av.equipped_parts.shield||'none'}';
                        if (wp !== 'none' && rSlot) { try { const g = await load('/assets/avatar/items/'+wp+'.gltf'); rSlot.add(g.scene); } catch(e){} }
                        if (sh !== 'none' && lSlot) { try { const g = await load('/assets/avatar/items/'+sh+'.gltf'); lSlot.add(g.scene); } catch(e){} }
                        // Animation
                        let mixer;
                        try {
                            const animG = await load('/assets/avatar/animations/Rig_Medium_General.glb');
                            const animName = '${av.equipped_parts.anim||'Idle_A'}';
                            const clip = animG.animations.find(a => a.name === animName) || animG.animations.find(a => a.name === 'Idle_A');
                            if (clip) { mixer = new THREE.AnimationMixer(model); mixer.clipAction(clip).play(); }
                        } catch(e) {}
                        canvas.classList.remove('hidden');
                        document.getElementById('pf-avatar-fallback').classList.add('hidden');
                        const clock = new THREE.Clock();
                        let angle = 0;
                        function anim() { requestAnimationFrame(anim); if(mixer) mixer.update(clock.getDelta()); angle += 0.003; model.rotation.y = Math.sin(angle)*0.3; renderer.render(scene, cam); }
                        anim();
                    `;
                    document.body.appendChild(s);
                } catch(e) { /* 3D avatar load failed, fallback stays visible */ }
            }

            window._friend = doFriend;
            window._block = doBlock;
            "#
        </script>

        <style>
            r#"
            @keyframes xpShimmer { 0% { background-position: -200% 0; } 100% { background-position: 200% 0; } }
            "#
        </style>
    }
}
