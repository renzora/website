use leptos::prelude::*;

#[component]
pub fn TeamsPage() -> impl IntoView {
    view! {
        <section class="py-12 px-6 min-h-screen">
            <div class="max-w-4xl mx-auto">
                <div class="flex items-center justify-between mb-10">
                    <div>
                        <h1 class="text-3xl font-bold">"Teams"</h1>
                        <p class="text-zinc-400 mt-2">"Collaborate with your studio or group."</p>
                    </div>
                    <button id="create-team-btn" onclick="createTeam()" class="hidden px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all">"Create Team"</button>
                </div>
                <div id="teams-content">
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
                const el = document.getElementById('teams-content');

                if (!token) {
                    el.innerHTML = '<p class="text-center text-zinc-500 py-12"><a href="/login" class="text-accent">Sign in</a> to manage teams.</p>';
                    return;
                }

                const headers = { 'Authorization': 'Bearer ' + token };

                const [teamsRes, invitesRes] = await Promise.all([
                    fetch('/api/teams', { headers }),
                    fetch('/api/teams/invites', { headers }),
                ]);

                const teams = teamsRes.ok ? await teamsRes.json() : [];
                const invites = invitesRes.ok ? await invitesRes.json() : [];

                document.getElementById('create-team-btn').classList.remove('hidden');

                let html = '';

                // Pending invites
                if (invites.length > 0) {
                    html += '<div class="mb-8"><h2 class="text-lg font-semibold mb-3">Pending Invites</h2><div class="space-y-2">';
                    invites.forEach(function(inv) {
                        html += '<div class="flex items-center justify-between p-4 bg-white/[0.02] border border-accent/30 rounded-xl">' +
                            '<div><span class="text-sm font-medium">Team invite</span> <span class="text-xs text-zinc-500">as ' + inv.role + '</span></div>' +
                            '<div class="flex gap-2">' +
                                '<button onclick="acceptInvite(\'' + inv.id + '\')" class="px-3 py-1.5 rounded-lg text-xs font-medium bg-accent text-white hover:bg-accent-hover transition-all">Accept</button>' +
                                '<button onclick="declineInvite(\'' + inv.id + '\')" class="px-3 py-1.5 rounded-lg text-xs text-zinc-400 hover:text-zinc-200 border border-zinc-800/50 hover:border-zinc-600 transition-all">Decline</button>' +
                            '</div>' +
                        '</div>';
                    });
                    html += '</div></div>';
                }

                // Teams
                if (teams.length === 0) {
                    html += '<div class="text-center py-12 bg-white/[0.02] border border-zinc-800/50 rounded-2xl">' +
                        '<i class="ph ph-users-three text-4xl text-zinc-700 mb-3"></i>' +
                        '<p class="text-zinc-500 mb-1">No teams yet</p>' +
                        '<p class="text-xs text-zinc-600">Create a team or get invited to one. Requires a Studio subscription.</p>' +
                    '</div>';
                } else {
                    html += '<div class="space-y-4">';
                    teams.forEach(function(team) {
                        html += '<div class="p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl cursor-pointer hover:border-zinc-600 transition-colors" onclick="viewTeam(\'' + team.id + '\')">' +
                            '<div class="flex items-center justify-between">' +
                                '<div class="flex items-center gap-3">' +
                                    '<div class="w-10 h-10 rounded-lg bg-accent/10 flex items-center justify-center"><i class="ph ph-users-three text-accent"></i></div>' +
                                    '<div>' +
                                        '<div class="font-medium">' + team.name + '</div>' +
                                        '<div class="text-xs text-zinc-600">' + (team.description || 'No description') + '</div>' +
                                    '</div>' +
                                '</div>' +
                                '<i class="ph ph-caret-right text-zinc-600"></i>' +
                            '</div>' +
                        '</div>';
                    });
                    html += '</div>';
                }

                el.innerHTML = html;

                // Team detail view
                window.viewTeam = async function(id) {
                    var res = await fetch('/api/teams/' + id, { headers: { 'Authorization': 'Bearer ' + token } });
                    if (!res.ok) { alert('Failed to load team'); return; }
                    var data = await res.json();
                    var team = data.team;
                    var members = data.members;

                    var userCookie = document.cookie.match('(^|;)\\s*user\\s*=\\s*([^;]+)')?.pop();
                    var currentUserId = null;
                    if (userCookie) { try { currentUserId = JSON.parse(decodeURIComponent(userCookie)).id; } catch(e) {} }
                    var isOwner = team.owner_id === currentUserId;

                    var html = '<button onclick="window.location.reload()" class="inline-flex items-center gap-1 text-sm text-zinc-500 hover:text-zinc-300 mb-6"><i class="ph ph-arrow-left"></i> Back to teams</button>';

                    html += '<div class="flex items-center justify-between mb-6"><div><h2 class="text-2xl font-bold">' + team.name + '</h2>';
                    if (team.description) html += '<p class="text-sm text-zinc-500 mt-1">' + team.description + '</p>';
                    html += '</div>';
                    if (isOwner) html += '<button onclick="deleteTeam(\'' + team.id + '\')" class="px-3 py-1.5 rounded-lg text-xs text-red-400 hover:bg-red-950/30 border border-transparent hover:border-red-900/50 transition-all">Delete Team</button>';
                    html += '</div>';

                    // Invite form
                    html += '<div class="mb-6 p-4 bg-white/[0.02] border border-zinc-800/50 rounded-xl">' +
                        '<div class="flex gap-2"><input id="invite-input" type="text" placeholder="Username or email" class="flex-1 px-3 py-2 bg-white/[0.02] border border-zinc-800/50 rounded-lg text-sm outline-none focus:border-accent/50" />' +
                        '<button onclick="inviteMember(\'' + team.id + '\')" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all">Invite</button></div></div>';

                    // Members list
                    html += '<h3 class="text-sm font-medium text-zinc-500 mb-3">Members (' + members.length + ')</h3><div class="space-y-2">';
                    members.forEach(function(m) {
                        var roleColor = m.role === 'owner' ? 'text-amber-400' : m.role === 'admin' ? 'text-accent' : 'text-zinc-500';
                        html += '<div class="flex items-center justify-between p-3 bg-white/[0.02] border border-zinc-800/50 rounded-lg">' +
                            '<div class="flex items-center gap-3">' +
                                '<div class="w-8 h-8 rounded-full bg-accent/10 flex items-center justify-center"><i class="ph ph-user text-accent text-sm"></i></div>' +
                                '<div><span class="text-sm font-medium">' + m.username + '</span> <span class="text-xs ' + roleColor + '">' + m.role + '</span></div>' +
                            '</div>';
                        if (isOwner && m.user_id !== currentUserId) {
                            html += '<div class="flex gap-2">' +
                                '<select onchange="changeRole(\'' + team.id + '\',\'' + m.user_id + '\',this.value)" class="px-2 py-1 bg-white/[0.02] border border-zinc-800/50 rounded text-xs text-zinc-400 outline-none">' +
                                    '<option value="member"' + (m.role === 'member' ? ' selected' : '') + '>Member</option>' +
                                    '<option value="admin"' + (m.role === 'admin' ? ' selected' : '') + '>Admin</option>' +
                                '</select>' +
                                '<button onclick="removeMember(\'' + team.id + '\',\'' + m.user_id + '\')" class="px-2 py-1 rounded text-xs text-red-400 hover:bg-red-950/30 transition-colors"><i class="ph ph-x"></i></button>' +
                            '</div>';
                        }
                        html += '</div>';
                    });
                    html += '</div>';

                    el.innerHTML = html;
                };
            })();

            async function createTeam() {
                var name = prompt('Team name:');
                if (!name) return;
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                var res = await fetch('/api/teams', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ name: name })
                });
                var data = await res.json();
                if (res.ok) { window.location.reload(); }
                else { alert(data.message || 'Failed to create team'); }
            }

            async function deleteTeam(id) {
                if (!confirm('Delete this team? All members will be removed.')) return;
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                var res = await fetch('/api/teams/' + id, { method: 'DELETE', headers: { 'Authorization': 'Bearer ' + token } });
                if (res.ok) { window.location.reload(); } else { alert('Failed to delete'); }
            }

            async function inviteMember(teamId) {
                var input = document.getElementById('invite-input');
                var identifier = input.value.trim();
                if (!identifier) return;
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                var res = await fetch('/api/teams/' + teamId + '/invite', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ identifier: identifier })
                });
                var data = await res.json();
                if (res.ok) { input.value = ''; alert('Invite sent!'); window.viewTeam(teamId); }
                else { alert(data.message || 'Failed to invite'); }
            }

            async function changeRole(teamId, userId, role) {
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                await fetch('/api/teams/' + teamId + '/members/' + userId + '/role', {
                    method: 'PUT',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ role: role })
                });
            }

            async function removeMember(teamId, userId) {
                if (!confirm('Remove this member?')) return;
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                await fetch('/api/teams/' + teamId + '/members/' + userId, {
                    method: 'DELETE', headers: { 'Authorization': 'Bearer ' + token }
                });
                window.viewTeam(teamId);
            }

            async function acceptInvite(id) {
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                var res = await fetch('/api/teams/invites/' + id + '/accept', {
                    method: 'POST', headers: { 'Authorization': 'Bearer ' + token }
                });
                if (res.ok) { window.location.reload(); } else { alert('Failed to accept'); }
            }

            async function declineInvite(id) {
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                await fetch('/api/teams/invites/' + id + '/decline', {
                    method: 'POST', headers: { 'Authorization': 'Bearer ' + token }
                });
                window.location.reload();
            }
            "##
        </script>
    }
}
