use leptos::prelude::*;

#[component]
pub fn ProfilePage() -> impl IntoView {
    view! {
        <section class="py-10 px-6">
            <div class="max-w-[800px] mx-auto">
                <div id="profile-content">"Loading..."</div>
            </div>
        </section>
        <script>
            r#"
            (async function() {
                const username = window.location.pathname.split('/').pop();
                const res = await fetch('/api/profiles/view/' + username);
                if (!res.ok) { document.getElementById('profile-content').textContent = 'User not found'; return; }
                const p = await res.json();
                const el = document.getElementById('profile-content');

                const badges = p.badges.map(b => `
                    <span class="inline-flex items-center gap-1 px-2 py-1 rounded-full text-[11px] font-medium border" style="border-color: ${b.color}30; color: ${b.color}; background: ${b.color}10" title="${b.description}">
                        <i class="ph ${b.icon}"></i>${b.name}
                    </span>
                `).join('');

                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                const userCookie = document.cookie.match('(^|;)\\s*user\\s*=\\s*([^;]+)')?.pop();
                let isOwnProfile = false;
                if (userCookie) { try { isOwnProfile = JSON.parse(decodeURIComponent(userCookie)).username === username; } catch(e) {} }

                const followBtn = !isOwnProfile && token ? `
                    <button onclick="toggleFollow('${username}')" id="follow-btn" class="inline-flex items-center gap-1.5 px-4 py-2 rounded-lg text-sm font-medium transition-colors ${p.is_following ? 'bg-surface border border-zinc-800 text-zinc-300 hover:border-red-500 hover:text-red-400' : 'bg-accent text-white hover:bg-accent-hover'}">
                        <i class="ph ${p.is_following ? 'ph-user-minus' : 'ph-user-plus'} text-base"></i>
                        ${p.is_following ? 'Unfollow' : 'Follow'}
                    </button>
                ` : '';

                const editBtn = isOwnProfile ? `
                    <button onclick="toggleEditProfile()" class="inline-flex items-center gap-1.5 px-4 py-2 rounded-lg text-sm font-medium bg-surface-card border border-zinc-800 text-zinc-300 hover:border-zinc-600 transition-colors">
                        <i class="ph ph-pencil-simple text-base"></i>Edit Profile
                    </button>
                ` : '';

                const infoItems = [
                    p.location ? `<span class="flex items-center gap-1"><i class="ph ph-map-pin text-sm"></i>${p.location}</span>` : '',
                    p.gender ? `<span class="flex items-center gap-1"><i class="ph ph-gender-intersex text-sm"></i>${p.gender}</span>` : '',
                    p.website ? `<a href="${p.website}" target="_blank" class="flex items-center gap-1 text-accent hover:text-accent-hover"><i class="ph ph-link text-sm"></i>${p.website.replace(/^https?:\/\//, '')}</a>` : '',
                ].filter(Boolean).join('<span class="text-zinc-700">·</span>');

                el.innerHTML = `
                    <!-- Banner -->
                    <div class="h-32 rounded-t-xl" style="background: ${p.banner_color}"></div>

                    <!-- Profile header -->
                    <div class="relative px-6 pb-6 bg-surface-card border border-t-0 border-zinc-800 rounded-b-xl">
                        <div class="flex items-end gap-4 -mt-10">
                            <div class="w-20 h-20 rounded-full border-4 flex items-center justify-center text-3xl" style="border-color: ${p.profile_color}; background: ${p.banner_color}; color: ${p.profile_color}">
                                <i class="ph ph-user-circle"></i>
                            </div>
                            <div class="flex-1 pt-12">
                                <div class="flex items-center gap-3">
                                    <h1 class="text-xl font-bold">${p.username}</h1>
                                    <span class="text-xs px-2 py-0.5 rounded" style="background: ${p.profile_color}20; color: ${p.profile_color}">${p.role}</span>
                                    ${followBtn}
                                    ${editBtn}
                                </div>
                            </div>
                        </div>

                        ${p.bio ? `<p class="text-sm text-zinc-400 mt-4">${p.bio}</p>` : ''}

                        ${infoItems ? `<div class="flex items-center gap-3 text-xs text-zinc-500 mt-3">${infoItems}</div>` : ''}

                        <div class="flex gap-6 mt-4 text-sm">
                            <div><span class="font-semibold text-zinc-50">${p.follower_count}</span> <span class="text-zinc-500">followers</span></div>
                            <div><span class="font-semibold text-zinc-50">${p.following_count}</span> <span class="text-zinc-500">following</span></div>
                            <div><span class="font-semibold text-zinc-50">${p.post_count}</span> <span class="text-zinc-500">posts</span></div>
                        </div>

                        ${badges ? `<div class="flex flex-wrap gap-2 mt-4">${badges}</div>` : ''}

                        <div class="text-xs text-zinc-500 mt-4">Joined ${new Date(p.created_at).toLocaleDateString()}</div>
                    </div>

                    <!-- Edit form (hidden by default) -->
                    <div id="edit-profile" class="hidden mt-4 p-6 bg-surface-card border border-zinc-800 rounded-xl space-y-4">
                        <h3 class="text-base font-semibold">Edit Profile</h3>
                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <label class="block text-xs text-zinc-500 mb-1">Bio</label>
                                <textarea id="edit-bio" rows="3" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent">${p.bio || ''}</textarea>
                            </div>
                            <div class="space-y-3">
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1">Location</label>
                                    <input id="edit-location" type="text" value="${p.location || ''}" placeholder="City, Country" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent" />
                                </div>
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1">Gender</label>
                                    <select id="edit-gender" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent">
                                        <option value="" ${!p.gender?'selected':''}>Prefer not to say</option>
                                        <option value="Male" ${p.gender==='Male'?'selected':''}>Male</option>
                                        <option value="Female" ${p.gender==='Female'?'selected':''}>Female</option>
                                        <option value="Non-binary" ${p.gender==='Non-binary'?'selected':''}>Non-binary</option>
                                        <option value="Other" ${p.gender==='Other'?'selected':''}>Other</option>
                                    </select>
                                </div>
                            </div>
                        </div>
                        <div>
                            <label class="block text-xs text-zinc-500 mb-1">Website</label>
                            <input id="edit-website" type="url" value="${p.website || ''}" placeholder="https://..." class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent" />
                        </div>
                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <label class="block text-xs text-zinc-500 mb-1">Profile Color</label>
                                <div class="flex items-center gap-2">
                                    <input id="edit-profile-color" type="color" value="${p.profile_color}" class="w-10 h-10 rounded cursor-pointer bg-transparent border-0" />
                                    <span class="text-xs text-zinc-400">${p.profile_color}</span>
                                </div>
                            </div>
                            <div>
                                <label class="block text-xs text-zinc-500 mb-1">Banner Color</label>
                                <div class="flex items-center gap-2">
                                    <input id="edit-banner-color" type="color" value="${p.banner_color}" class="w-10 h-10 rounded cursor-pointer bg-transparent border-0" />
                                    <span class="text-xs text-zinc-400">${p.banner_color}</span>
                                </div>
                            </div>
                        </div>
                        <div class="flex gap-2">
                            <button onclick="saveProfile()" class="inline-flex items-center gap-1.5 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                                <i class="ph ph-check"></i>Save
                            </button>
                            <button onclick="toggleEditProfile()" class="px-4 py-2 rounded-lg text-sm text-zinc-400 hover:text-zinc-50 transition-colors">Cancel</button>
                        </div>
                    </div>
                `;
            })();

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
                if (res.ok) { window.location.reload(); }
                else { alert('Failed to save'); }
            }

            async function toggleFollow(username) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }
                await fetch('/api/profiles/follow/' + username, { method: 'POST', headers: { 'Authorization': 'Bearer ' + token } });
                window.location.reload();
            }
            "#
        </script>
    }
}
