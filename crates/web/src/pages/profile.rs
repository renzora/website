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
                if (userCookie) { try { isOwnProfile = JSON.parse(decodeURIComponent(userCookie)).username === username; } catch(e) {} }

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
                const infoPills = [
                    p.location ? `<span class="inline-flex items-center gap-1 text-xs text-zinc-400"><i class="ph ph-map-pin"></i>${p.location}</span>` : '',
                    p.website ? `<a href="${p.website}" target="_blank" class="inline-flex items-center gap-1 text-xs text-accent hover:text-accent-hover"><i class="ph ph-link"></i>${p.website.replace(/^https?:\/\//, '')}</a>` : '',
                    `<span class="inline-flex items-center gap-1 text-xs text-zinc-500"><i class="ph ph-calendar"></i>Joined ${new Date(p.created_at).toLocaleDateString('en-US', { month: 'long', year: 'numeric' })}</span>`,
                ].filter(Boolean).join('');

                // Role badge
                const roleColors = { admin: '#ef4444', creator: '#8b5cf6', moderator: '#f59e0b', user: '#6b7280' };
                const roleColor = roleColors[p.role] || roleColors.user;

                // Assets grid
                const assetsHtml = (p.assets && p.assets.length) ? `
                    <div class="max-w-[1000px] mx-auto px-6 mt-8 mb-12">
                        <h2 class="text-lg font-semibold mb-4">Published Assets <span class="text-zinc-500 text-sm font-normal">(${p.assets.length})</span></h2>
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                            ${p.assets.map(a => `
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
                            `).join('')}
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
                                        <button onclick="toggleStorefrontEditor()" class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium bg-surface-card border border-zinc-800 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200 transition-colors"><i class="ph ph-storefront"></i>Storefront</button>
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
                                        <input id="edit-location" type="text" value="${p.location || ''}" placeholder="City, Country" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent" />
                                    </div>
                                    <div>
                                        <label class="block text-xs text-zinc-500 mb-1">Website</label>
                                        <input id="edit-website" type="url" value="${p.website || ''}" placeholder="https://..." class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent" />
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

                        <!-- Storefront editor (hidden) -->
                        ${isOwnProfile ? `
                        <div id="edit-storefront" class="hidden mt-6 p-6 bg-surface-card border border-zinc-800 rounded-xl">
                            <div class="flex items-center justify-between mb-4">
                                <h3 class="text-base font-semibold">Storefront Settings</h3>
                                ${p.storefront_enabled ? `<a href="/shop/${p.username}" target="_blank" class="text-xs text-accent hover:underline"><i class="ph ph-arrow-square-out"></i> View Shop</a>` : ''}
                            </div>
                            <div class="mb-4">
                                <label class="flex items-center gap-2 cursor-pointer">
                                    <input type="checkbox" id="sf-enabled" ${p.storefront_enabled ? 'checked' : ''} class="accent-accent w-4 h-4" />
                                    <span class="text-sm">Enable public storefront at <span class="text-accent">/shop/${p.username}</span></span>
                                </label>
                            </div>
                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-4">
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1">Tagline</label>
                                    <input id="sf-tagline" type="text" value="" placeholder="A short catchy tagline..." maxlength="128" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent" />
                                </div>
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1">Layout</label>
                                    <select id="sf-layout" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent">
                                        <option value="grid">Grid</option>
                                        <option value="list">List</option>
                                    </select>
                                </div>
                            </div>
                            <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-4">
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1">Background</label>
                                    <input id="sf-bg-color" type="color" value="#0a0a0b" class="w-full h-9 rounded cursor-pointer bg-transparent border border-zinc-800" />
                                </div>
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1">Text Color</label>
                                    <input id="sf-text-color" type="color" value="#e4e4e7" class="w-full h-9 rounded cursor-pointer bg-transparent border border-zinc-800" />
                                </div>
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1">Accent</label>
                                    <input id="sf-accent-color" type="color" value="#6366f1" class="w-full h-9 rounded cursor-pointer bg-transparent border border-zinc-800" />
                                </div>
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1">Card Background</label>
                                    <input id="sf-card-bg" type="color" value="#18181b" class="w-full h-9 rounded cursor-pointer bg-transparent border border-zinc-800" />
                                </div>
                            </div>
                            <div class="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-4">
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1">Card Border</label>
                                    <input id="sf-card-border" type="color" value="#27272a" class="w-full h-9 rounded cursor-pointer bg-transparent border border-zinc-800" />
                                </div>
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1">Font</label>
                                    <select id="sf-font" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent">
                                        <option value="Inter">Inter</option>
                                        <option value="Roboto">Roboto</option>
                                        <option value="Poppins">Poppins</option>
                                        <option value="Space Grotesk">Space Grotesk</option>
                                        <option value="JetBrains Mono">JetBrains Mono</option>
                                        <option value="Outfit">Outfit</option>
                                        <option value="Nunito">Nunito</option>
                                        <option value="Lexend">Lexend</option>
                                        <option value="DM Sans">DM Sans</option>
                                        <option value="Playfair Display">Playfair Display</option>
                                    </select>
                                </div>
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1">Font Size</label>
                                    <select id="sf-font-size" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent">
                                        <option value="12px">Small (12px)</option>
                                        <option value="14px">Default (14px)</option>
                                        <option value="16px">Large (16px)</option>
                                        <option value="18px">Extra Large (18px)</option>
                                    </select>
                                </div>
                                <div>
                                    <label class="block text-xs text-zinc-500 mb-1">Cursor</label>
                                    <select id="sf-cursor" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent">
                                        <option value="default">Default</option>
                                        <option value="crosshair">Crosshair</option>
                                        <option value="pointer">Pointer</option>
                                        <option value="cell">Cell</option>
                                        <option value="grab">Grab</option>
                                    </select>
                                </div>
                            </div>
                            <div class="mb-4">
                                <label class="block text-xs text-zinc-500 mb-1">Background Image URL (optional)</label>
                                <input id="sf-bg-image" type="url" value="" placeholder="https://..." class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-50 outline-none focus:border-accent" />
                            </div>
                            <div class="mb-4">
                                <label class="block text-xs text-zinc-500 mb-1">Custom CSS <span class="text-zinc-600">(max 4KB)</span></label>
                                <textarea id="sf-css" rows="3" placeholder="#shop-page .shop-card { border-radius: 1rem; }" class="w-full px-3 py-2 bg-surface border border-zinc-800 rounded-lg text-xs text-zinc-50 outline-none focus:border-accent resize-y font-mono"></textarea>
                            </div>
                            <div class="flex gap-2">
                                <button onclick="saveStorefront()" class="inline-flex items-center gap-1.5 px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover"><i class="ph ph-check"></i>Save Storefront</button>
                                <button onclick="toggleStorefrontEditor()" class="px-4 py-2 rounded-lg text-sm text-zinc-400 hover:text-zinc-50">Cancel</button>
                            </div>
                        </div>
                        ` : ''}
                    </div>

                    <!-- Assets -->
                    ${assetsHtml}
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

                const res = await fetch('/api/profiles/avatar', {
                    method: 'PUT',
                    headers: { 'Authorization': 'Bearer ' + token },
                    body: form
                });

                if (res.ok) window.location.reload();
                else { const d = await res.json().catch(() => ({})); alert(d.error || 'Upload failed'); }
            }

            function toggleStorefrontEditor() {
                const el = document.getElementById('edit-storefront');
                if (!el) return;
                const wasHidden = el.classList.contains('hidden');
                el.classList.toggle('hidden');
                // Load current settings when opening
                if (wasHidden) loadStorefrontSettings();
            }

            async function loadStorefrontSettings() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token || !profileData) return;
                const username = profileData.username;
                // Try to fetch current storefront data (may 404 if not enabled)
                const res = await fetch('/api/profiles/shop/' + username);
                if (res.ok) {
                    const sf = await res.json();
                    const set = (id, val) => { const el = document.getElementById(id); if (el) el.value = val || ''; };
                    set('sf-tagline', sf.tagline);
                    set('sf-bg-color', sf.bg_color);
                    set('sf-bg-image', sf.bg_image);
                    set('sf-text-color', sf.text_color);
                    set('sf-accent-color', sf.accent_color);
                    set('sf-card-bg', sf.card_bg);
                    set('sf-card-border', sf.card_border);
                    set('sf-font', sf.font);
                    set('sf-font-size', sf.font_size);
                    set('sf-cursor', sf.cursor);
                    set('sf-layout', sf.layout);
                    set('sf-css', sf.css);
                }
            }

            async function saveStorefront() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                const body = {
                    enabled: document.getElementById('sf-enabled')?.checked ?? false,
                    tagline: document.getElementById('sf-tagline')?.value || '',
                    bg_color: document.getElementById('sf-bg-color')?.value || '#0a0a0b',
                    bg_image: document.getElementById('sf-bg-image')?.value || '',
                    text_color: document.getElementById('sf-text-color')?.value || '#e4e4e7',
                    accent_color: document.getElementById('sf-accent-color')?.value || '#6366f1',
                    card_bg: document.getElementById('sf-card-bg')?.value || '#18181b',
                    card_border: document.getElementById('sf-card-border')?.value || '#27272a',
                    font: document.getElementById('sf-font')?.value || 'Inter',
                    font_size: document.getElementById('sf-font-size')?.value || '14px',
                    cursor: document.getElementById('sf-cursor')?.value || 'default',
                    layout: document.getElementById('sf-layout')?.value || 'grid',
                    css: document.getElementById('sf-css')?.value || '',
                };
                const res = await fetch('/api/profiles/storefront', {
                    method: 'PUT',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify(body)
                });
                if (res.ok) {
                    window.location.reload();
                } else {
                    const d = await res.json().catch(() => ({}));
                    alert(d.error || 'Failed to save');
                }
            }

            async function toggleFollow(username) {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }
                await fetch('/api/profiles/follow/' + username, { method: 'POST', headers: { 'Authorization': 'Bearer ' + token } });
                window.location.reload();
            }
            "##
        </script>
    }
}
