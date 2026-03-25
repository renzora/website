use leptos::prelude::*;

#[component]
pub fn ProfilePage() -> impl IntoView {
    view! {
        <section class="py-0 px-0">
            <div id="profile-content" class="min-h-[60vh]">
                <div class="flex items-center justify-center py-20">
                    <div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                </div>
            </div>
        </section>
        <script>
            r##"
            let profileData = null;
            let isOwnProfile = false;
            let isStaff = false;

            function parseDate(s) {
                if (!s) return null;
                // Try direct parse first (ISO 8601 / RFC 3339)
                let d = new Date(s);
                if (!isNaN(d.getTime())) return d;
                // Fallback: Rust OffsetDateTime format "2024-01-15 10:30:00.123 +00:00:00"
                const iso = s.replace(/^(\d{4}-\d{2}-\d{2})\s+(\d{2}:\d{2}:\d{2}).*?\s*([+-]\d{2}):?(\d{2}).*$/, '$1T$2$3:$4');
                d = new Date(iso);
                return isNaN(d.getTime()) ? null : d;
            }

            (async function() {
                const username = window.location.pathname.split('/').pop();
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();

                const headers = {};
                if (token) headers['Authorization'] = 'Bearer ' + token;

                const res = await fetch('/api/profiles/view/' + username, { headers });
                if (!res.ok) {
                    document.getElementById('profile-content').innerHTML = `
                        <div class="text-center py-20">
                            <i class="ph ph-user-circle text-5xl text-zinc-700 mb-3"></i>
                            <p class="text-zinc-500">User not found.</p>
                        </div>`;
                    return;
                }

                profileData = await res.json();
                const p = profileData;

                const userCookie = document.cookie.match('(^|;)\\s*user\\s*=\\s*([^;]+)')?.pop();
                if (userCookie) {
                    try {
                        const u = JSON.parse(decodeURIComponent(userCookie));
                        isOwnProfile = u.username === username;
                        isStaff = u.role === 'admin' || u.role === 'moderator';
                    } catch(e) {}
                }

                // Badges
                const badges = p.badges.map(b => `
                    <span class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-[11px] font-medium border" style="border-color: ${b.color}30; color: ${b.color}; background: ${b.color}10" title="${b.description}">
                        <i class="ph ${b.icon}"></i>${b.name}
                    </span>
                `).join('');

                // Follow button
                const followBtn = !isOwnProfile && token ? `
                    <button onclick="toggleFollow('${p.username}')" id="follow-btn"
                        class="inline-flex items-center gap-1.5 px-4 py-2 rounded-lg text-sm font-medium transition-colors ${p.is_following
                            ? 'bg-surface-card border border-zinc-800 text-zinc-300 hover:border-red-500 hover:text-red-400'
                            : 'bg-accent text-white hover:bg-accent-hover'}">
                        <i class="ph ${p.is_following ? 'ph-user-minus' : 'ph-user-plus'} text-base"></i>
                        ${p.is_following ? 'Unfollow' : 'Follow'}
                    </button>` : '';

                // Avatar
                const avatarImg = p.avatar_url
                    ? `<img src="${p.avatar_url}" class="w-full h-full object-cover rounded-full" />`
                    : `<i class="ph ph-user text-4xl" style="color: ${p.profile_color}"></i>`;

                const avatarOverlay = isOwnProfile ? `
                    <label class="absolute inset-0 flex items-center justify-center bg-black/50 rounded-full opacity-0 hover:opacity-100 transition-opacity cursor-pointer">
                        <i class="ph ph-camera text-white text-xl"></i>
                        <input type="file" accept="image/*" onchange="uploadAvatar(this)" class="hidden" />
                    </label>` : '';

                // Info pills
                const joinedDate = parseDate(p.created_at);
                const joinedStr = joinedDate ? joinedDate.toLocaleDateString('en-US', { month: 'long', year: 'numeric' }) : 'Unknown';
                const infoPills = [
                    p.location ? `<span class="inline-flex items-center gap-1 text-xs text-zinc-400"><i class="ph ph-map-pin"></i>${p.location}</span>` : '',
                    p.website ? `<a href="${p.website}" target="_blank" class="inline-flex items-center gap-1 text-xs text-accent hover:text-accent-hover"><i class="ph ph-link"></i>${p.website.replace(/^https?:\/\//, '')}</a>` : '',
                    `<span class="inline-flex items-center gap-1 text-xs text-zinc-500"><i class="ph ph-calendar"></i>Joined ${joinedStr}</span>`,
                ].filter(Boolean).join('');

                // Role badge
                const roleColors = { admin: '#ef4444', creator: '#8b5cf6', moderator: '#f59e0b', user: '#6b7280' };
                const roleColor = roleColors[p.role] || roleColors.user;

                // Assets grid
                const assetsHtml = (p.assets && p.assets.length) ? p.assets.map(a => `
                    <a href="/marketplace/asset/${a.slug}" class="block group">
                        <div class="bg-surface-card border border-zinc-800 rounded-xl overflow-hidden hover:border-zinc-700 transition-all">
                            <div class="h-32 bg-surface flex items-center justify-center relative">
                                ${a.thumbnail_url ? `<img src="${a.thumbnail_url}" class="w-full h-full object-cover" />` : `<i class="ph ph-package text-3xl text-zinc-700"></i>`}
                                <span class="absolute top-2 right-2 text-[10px] px-1.5 py-0.5 rounded bg-black/50 text-zinc-300 backdrop-blur-sm">${a.category}</span>
                            </div>
                            <div class="p-3">
                                <h3 class="text-sm font-semibold group-hover:text-accent transition-colors truncate">${a.name}</h3>
                                <div class="flex items-center justify-between mt-2">
                                    <span class="text-xs text-zinc-500"><i class="ph ph-download-simple"></i> ${a.downloads}</span>
                                    <span class="text-xs font-semibold ${a.price_credits === 0 ? 'text-green-400' : 'text-zinc-50'}">${a.price_credits === 0 ? 'Free' : a.price_credits + ' credits'}</span>
                                </div>
                            </div>
                        </div>
                    </a>
                `).join('') : '<p class="text-zinc-500 text-sm py-8 text-center">No published assets yet.</p>';

                // Staff notes section (only for staff viewing other profiles)
                const staffNotesHtml = isStaff ? `
                    <div class="bg-surface-card border border-amber-500/20 rounded-xl p-4">
                        <div class="flex items-center gap-2 mb-3">
                            <i class="ph ph-shield-check text-amber-400"></i>
                            <h3 class="text-sm font-semibold text-amber-400">Staff Notes</h3>
                        </div>
                        <div id="profile-notes-list" class="space-y-2 mb-3"><p class="text-xs text-zinc-500">Loading...</p></div>
                        <div class="flex gap-2">
                            <input type="text" id="profile-note-input" placeholder="Add a staff note..." class="flex-1 px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent" />
                            <button onclick="addProfileNote()" class="px-3 py-2 rounded-lg text-xs font-medium bg-accent text-white hover:bg-accent-hover">Add</button>
                        </div>
                    </div>` : '';

                const el = document.getElementById('profile-content');
                el.innerHTML = `
                    <!-- Banner -->
                    <div class="h-40 relative" style="background: linear-gradient(135deg, ${p.banner_color}, ${p.banner_color}88, ${p.profile_color}44)">
                        <div class="absolute inset-0 bg-gradient-to-b from-transparent to-[#0a0a0b]"></div>
                    </div>

                    <!-- Profile card -->
                    <div class="max-w-[1000px] mx-auto px-6 -mt-16 relative z-10">
                        <div class="flex flex-col sm:flex-row gap-5">
                            <!-- Avatar -->
                            <div class="relative w-28 h-28 rounded-full border-4 border-[#0a0a0b] bg-surface-card flex items-center justify-center flex-shrink-0 overflow-hidden" style="box-shadow: 0 0 0 3px ${p.profile_color}40">
                                ${avatarImg}
                                ${avatarOverlay}
                            </div>

                            <!-- Info -->
                            <div class="flex-1 pt-2">
                                <div class="flex flex-wrap items-center gap-3 mb-2">
                                    <h1 class="text-2xl font-bold">${p.username}</h1>
                                    <span class="text-[10px] font-semibold uppercase tracking-wider px-2 py-0.5 rounded-full" style="background: ${roleColor}15; color: ${roleColor}; border: 1px solid ${roleColor}30">${p.role}</span>
                                    ${followBtn}
                                    ${isOwnProfile ? `
                                        <button onclick="toggleEditProfile()" class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-surface-card border border-zinc-800 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200 transition-colors"><i class="ph ph-pencil-simple"></i>Edit</button>
                                    ` : ''}
                                </div>

                                ${p.bio ? `<p class="text-sm text-zinc-400 mb-3 max-w-lg">${p.bio}</p>` : ''}

                                <div class="flex flex-wrap items-center gap-3 mb-3">${infoPills}</div>

                                <div class="flex gap-5 text-sm">
                                    <div><span class="font-semibold text-zinc-50">${p.follower_count.toLocaleString()}</span> <span class="text-zinc-500">followers</span></div>
                                    <div><span class="font-semibold text-zinc-50">${p.following_count.toLocaleString()}</span> <span class="text-zinc-500">following</span></div>
                                    ${p.assets ? `<div><span class="font-semibold text-zinc-50">${p.assets.length}</span> <span class="text-zinc-500">assets</span></div>` : ''}
                                </div>

                                ${badges ? `<div class="flex flex-wrap gap-2 mt-3">${badges}</div>` : ''}
                            </div>
                        </div>

                        <!-- Edit form (hidden) -->
                        <div id="edit-profile" class="hidden mt-6 p-6 bg-surface-card border border-zinc-800 rounded-xl">
                            <h3 class="text-base font-semibold mb-4">Edit Profile</h3>
                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-4">
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1">Bio</label>
                                    <textarea id="edit-bio" rows="3" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent resize-y">${p.bio || ''}</textarea>
                                </div>
                                <div class="space-y-3">
                                    <div>
                                        <label class="block text-xs text-zinc-500 mb-1">Location</label>
                                        <input id="edit-location" type="text" value="${p.location || ''}" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent" />
                                    </div>
                                    <div>
                                        <label class="block text-xs text-zinc-500 mb-1">Website</label>
                                        <input id="edit-website" type="url" value="${p.website || ''}" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent" />
                                    </div>
                                </div>
                            </div>
                            <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-4">
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1">Gender</label>
                                    <select id="edit-gender" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50">
                                        <option value="" ${!p.gender?'selected':''}>—</option>
                                        <option value="Male" ${p.gender==='Male'?'selected':''}>Male</option>
                                        <option value="Female" ${p.gender==='Female'?'selected':''}>Female</option>
                                        <option value="Non-binary" ${p.gender==='Non-binary'?'selected':''}>Non-binary</option>
                                    </select>
                                </div>
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1">Profile Color</label>
                                    <input id="edit-profile-color" type="color" value="${p.profile_color || '#6366f1'}" class="w-full h-9 rounded cursor-pointer bg-transparent border border-zinc-800" />
                                </div>
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1">Banner Color</label>
                                    <input id="edit-banner-color" type="color" value="${p.banner_color || '#1e1b4b'}" class="w-full h-9 rounded cursor-pointer bg-transparent border border-zinc-800" />
                                </div>
                            </div>
                            <div class="flex gap-2">
                                <button onclick="saveProfile()" class="inline-flex items-center gap-1.5 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover"><i class="ph ph-check"></i>Save</button>
                                <button onclick="toggleEditProfile()" class="px-4 py-2 rounded-lg text-sm text-zinc-400 hover:text-zinc-50">Cancel</button>
                            </div>
                        </div>
                    </div>

                    <!-- Tabbed content area -->
                    <div class="max-w-[1000px] mx-auto px-6 mt-8 mb-12">
                        <div class="flex gap-1 border-b border-zinc-800 mb-6">
                            <button onclick="showProfileTab('assets')" id="ptab-assets" class="profile-tab px-4 py-2.5 text-sm font-medium border-b-2 border-accent text-zinc-50">
                                <i class="ph ph-package"></i> Assets
                            </button>
                            <button onclick="showProfileTab('badges')" id="ptab-badges" class="profile-tab px-4 py-2.5 text-sm font-medium border-b-2 border-transparent text-zinc-400 hover:text-zinc-200">
                                <i class="ph ph-medal"></i> Badges
                            </button>
                            ${isStaff ? `<button onclick="showProfileTab('staff')" id="ptab-staff" class="profile-tab px-4 py-2.5 text-sm font-medium border-b-2 border-transparent text-amber-400 hover:text-amber-300">
                                <i class="ph ph-shield-check"></i> Staff
                            </button>` : ''}
                        </div>

                        <div id="ptab-content-assets">
                            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                                ${assetsHtml}
                            </div>
                        </div>

                        <div id="ptab-content-badges" class="hidden">
                            ${p.badges.length ? `
                                <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                                    ${p.badges.map(b => `
                                        <div class="flex items-center gap-3 p-4 bg-surface-card border border-zinc-800 rounded-xl">
                                            <div class="w-10 h-10 rounded-full flex items-center justify-center" style="background: ${b.color}15; border: 1px solid ${b.color}30">
                                                <i class="ph ${b.icon} text-xl" style="color: ${b.color}"></i>
                                            </div>
                                            <div>
                                                <div class="text-sm font-semibold" style="color: ${b.color}">${b.name}</div>
                                                <div class="text-xs text-zinc-500">${b.description}</div>
                                            </div>
                                        </div>
                                    `).join('')}
                                </div>
                            ` : '<p class="text-zinc-500 text-sm py-8 text-center">No badges earned yet.</p>'}
                        </div>

                        <div id="ptab-content-staff" class="hidden">
                            ${staffNotesHtml}
                        </div>
                    </div>
                `;

                // Load staff notes if staff
                if (isStaff && p.id) loadProfileNotes(p.id);
            })();

            function showProfileTab(name) {
                document.querySelectorAll('.profile-tab').forEach(t => {
                    t.classList.remove('border-accent', 'text-zinc-50', 'border-amber-400', 'text-amber-400');
                    t.classList.add('border-transparent', 'text-zinc-400');
                });
                const tab = document.getElementById('ptab-' + name);
                if (tab) {
                    if (name === 'staff') {
                        tab.classList.add('border-amber-400', 'text-amber-400');
                    } else {
                        tab.classList.add('border-accent', 'text-zinc-50');
                    }
                    tab.classList.remove('border-transparent', 'text-zinc-400');
                }
                ['assets', 'badges', 'staff'].forEach(n => {
                    const el = document.getElementById('ptab-content-' + n);
                    if (el) el.classList.toggle('hidden', n !== name);
                });
            }

            function toggleEditProfile() {
                document.getElementById('edit-profile')?.classList.toggle('hidden');
            }

            async function saveProfile() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                const body = {
                    bio: document.getElementById('edit-bio').value,
                    location: document.getElementById('edit-location').value,
                    gender: document.getElementById('edit-gender').value,
                    website: document.getElementById('edit-website').value,
                    profile_color: document.getElementById('edit-profile-color').value,
                    banner_color: document.getElementById('edit-banner-color').value,
                };
                const res = await fetch('/api/auth/me', {
                    method: 'PUT',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify(body)
                });
                if (res.ok) window.location.reload();
                else alert('Failed to save');
            }

            async function uploadAvatar(input) {
                const file = input.files[0];
                if (!file) return;
                if (file.size > 2 * 1024 * 1024) { alert('Avatar must be under 2MB'); return; }
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                const form = new FormData();
                form.append('avatar', file);
                const res = await fetch('/api/profiles/avatar', { method: 'PUT', headers: { 'Authorization': 'Bearer ' + token }, body: form });
                if (res.ok) window.location.reload();
                else { const d = await res.json().catch(() => ({})); alert(d.error || 'Upload failed'); }
            }

            async function toggleFollow(username) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }
                await fetch('/api/profiles/follow/' + username, { method: 'POST', headers: { 'Authorization': 'Bearer ' + token } });
                window.location.reload();
            }

            // ── Staff notes on profile ──
            async function loadProfileNotes(userId) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                try {
                    const res = await fetch('/api/admin/users/' + userId + '/notes', { headers: { 'Authorization': 'Bearer ' + token } });
                    if (!res.ok) return;
                    const notes = await res.json();
                    const el = document.getElementById('profile-notes-list');
                    if (!el) return;
                    el.innerHTML = notes.length ? notes.map(n => `
                        <div class="p-2.5 bg-surface border border-zinc-800 rounded-lg">
                            <div class="flex justify-between items-center">
                                <span class="text-xs text-accent">${n.author_name}</span>
                                <span class="text-[10px] text-zinc-600">${new Date(n.created_at).toLocaleString()}</span>
                            </div>
                            <p class="text-xs text-zinc-300 mt-1">${n.content}</p>
                        </div>
                    `).join('') : '<p class="text-xs text-zinc-600">No notes yet.</p>';
                } catch(e) {}
            }

            async function addProfileNote() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token || !profileData?.id) return;
                const input = document.getElementById('profile-note-input');
                const content = input.value.trim();
                if (!content) return;
                await fetch('/api/admin/users/' + profileData.id + '/notes', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ content })
                });
                input.value = '';
                loadProfileNotes(profileData.id);
            }
            "##
        </script>
    }
}
