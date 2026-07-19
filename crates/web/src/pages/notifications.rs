use leptos::prelude::*;

/// Notifications page, a full-page view of the bell dropdown. Lists the most
/// recent notifications with actor avatars/type icons, mark-read, and deep links.
#[component]
pub fn NotificationsPage() -> impl IntoView {
    view! {
        <section class="py-8 px-4 md:px-6 min-h-[80vh] bg-gradient-to-b from-[#0c0a10] via-[#060608] to-[#060608]">
            <div class="max-w-[640px] mx-auto">
                <div class="flex items-center justify-between mb-6">
                    <h1 class="text-2xl font-bold">"Notifications"</h1>
                    <button id="mark-all" class="text-xs text-accent hover:text-accent-hover transition-colors">"Mark all read"</button>
                </div>
                <div id="notif-list" class="space-y-2">
                    <div class="text-center py-12"><div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-accent rounded-full"></div></div>
                </div>
            </div>
        </section>
        <script>
            r##"
            (function(){
              const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
              if (!token){ window.location.href = '/login?redirect=/notifications'; return; }
              const H = { 'Authorization': 'Bearer ' + token };
              const $ = id => document.getElementById(id);
              function esc(s){ const d=document.createElement('div'); d.textContent=s==null?'':String(s); return d.innerHTML; }
              function timeAgo(iso){ const t=new Date(iso).getTime(); if(isNaN(t)) return ''; const s=Math.floor((Date.now()-t)/1000); if(s<60)return 'just now'; const m=Math.floor(s/60); if(m<60)return m+'m'; const h=Math.floor(m/60); if(h<24)return h+'h'; const d=Math.floor(h/24); if(d<7)return d+'d'; return new Date(iso).toLocaleDateString('en-US',{month:'short',day:'numeric'}); }

              const ICONS = { follow:'ph-user-plus', friend_request:'ph-user-plus', friend_accepted:'ph-users', mention:'ph-at', reply:'ph-chat-circle', comment:'ph-chat-circle', like:'ph-heart', reaction:'ph-smiley', team_invite:'ph-users-three', team_member_joined:'ph-users-three' };

              function iconFor(type){ return ICONS[type] || 'ph-bell'; }

              async function load(){
                try {
                  const data = await fetch('/api/notifications', { headers: H }).then(r => r.ok?r.json():{notifications:[]});
                  const list = data.notifications || [];
                  const el = $('notif-list');
                  if (!list.length){ el.innerHTML = '<p class="text-center text-zinc-500 py-16 text-sm">No notifications yet.</p>'; return; }
                  el.innerHTML = list.map(n => `
                    <div onclick="__open('${n.id}', ${n.link?`'${esc(n.link)}'`:'null'})" class="flex items-start gap-3 p-3.5 rounded-xl border cursor-pointer transition-colors ${n.read?'bg-surface-card border-zinc-800 hover:border-zinc-700':'bg-accent/[0.06] border-accent/25 hover:border-accent/40'}">
                      <div class="relative shrink-0">
                        ${n.actor_avatar_url ? `<img src="${esc(n.actor_avatar_url)}" class="w-10 h-10 rounded-full object-cover" />` : `<div class="w-10 h-10 rounded-full bg-accent/15 text-accent flex items-center justify-center"><i class="ph ${iconFor(n.type)} text-lg"></i></div>`}
                        <span class="absolute -bottom-0.5 -right-0.5 w-5 h-5 rounded-full bg-surface-card border border-zinc-800 flex items-center justify-center"><i class="ph ${iconFor(n.type)} text-[11px] text-accent"></i></span>
                      </div>
                      <div class="flex-1 min-w-0">
                        <p class="text-sm text-zinc-100">${esc(n.title)}</p>
                        ${n.body ? `<p class="text-xs text-zinc-500 mt-0.5 line-clamp-2">${esc(n.body)}</p>` : ''}
                        <p class="text-[11px] text-zinc-600 mt-1">${timeAgo(n.created_at)}</p>
                      </div>
                      ${!n.read ? '<span class="w-2 h-2 rounded-full bg-accent shrink-0 mt-2"></span>' : ''}
                    </div>`).join('');
                } catch(e){}
              }

              window.__open = async function(id, link){
                try { await fetch('/api/notifications/'+id+'/read', { method:'PUT', headers: H }); } catch(e){}
                if (link) window.location.href = link; else load();
              };

              $('mark-all').addEventListener('click', async () => {
                try { await fetch('/api/notifications/read-all', { method:'PUT', headers: H }); load();
                  const badge = document.getElementById('notif-badge'); if (badge) badge.classList.add('hidden');
                } catch(e){}
              });

              load();
            })();
            "##
        </script>
    }
}
