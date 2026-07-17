use leptos::prelude::*;

/// Direct + group + team messaging. Mirrors the engine's Messages panel:
/// conversation list with kind icons, message search, group creation, reply-to,
/// edit/delete own messages, and live updates over WS (via nav-dispatched events).
#[component]
pub fn MessagesPage() -> impl IntoView {
    view! {
        <section class="h-[calc(100vh-64px)] flex" id="messages-page">
            // Left: conversation list
            <div class="w-80 border-r border-zinc-800 flex flex-col bg-surface">
                <div class="p-3 border-b border-zinc-800 space-y-2">
                    <div class="flex items-center justify-between">
                        <h2 class="text-sm font-semibold text-zinc-300">"Messages"</h2>
                        <button id="new-group-btn" title="New group" class="w-7 h-7 rounded-lg flex items-center justify-center text-zinc-400 hover:text-accent hover:bg-white/[0.04] transition-colors"><i class="ph ph-users-three"></i></button>
                    </div>
                    <input id="conv-search" type="text" placeholder="Search messages…" class="w-full px-3 py-1.5 bg-zinc-900 border border-zinc-800 rounded-lg text-xs text-zinc-200 placeholder:text-zinc-600 outline-none focus:border-accent/50" />
                </div>
                <div id="conversation-list" class="flex-1 overflow-y-auto">
                    <div class="flex items-center justify-center py-12"><div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-accent rounded-full"></div></div>
                </div>
            </div>

            // Right: chat view
            <div class="flex-1 flex flex-col bg-surface">
                <div id="chat-header" class="p-4 border-b border-zinc-800 hidden flex items-center gap-2">
                    <i id="chat-kind-icon" class="ph ph-user text-zinc-500"></i>
                    <h3 id="chat-name" class="text-sm font-semibold text-zinc-200"></h3>
                </div>
                <div id="chat-empty" class="flex-1 flex items-center justify-center text-zinc-600 text-sm">"Select a conversation"</div>
                <div id="chat-messages" class="flex-1 overflow-y-auto p-4 space-y-1 hidden"></div>
                <div id="chat-input-bar" class="p-4 border-t border-zinc-800 hidden">
                    <div id="reply-banner" class="hidden items-center justify-between gap-2 mb-2 px-3 py-1.5 bg-white/[0.03] border border-zinc-800 rounded-lg text-xs text-zinc-400">
                        <span id="reply-text" class="truncate"></span>
                        <button id="reply-cancel" class="text-zinc-500 hover:text-white shrink-0"><i class="ph ph-x"></i></button>
                    </div>
                    <div class="flex gap-2">
                        <input id="message-input" type="text" class="flex-1 px-4 py-2 bg-zinc-900 border border-zinc-700 rounded-xl text-sm text-zinc-200 placeholder:text-zinc-600 focus:outline-none focus:border-accent/50" placeholder="Type a message..." />
                        <button id="send-btn" class="px-4 py-2 bg-accent hover:bg-accent-hover text-white text-sm font-medium rounded-xl transition-colors">"Send"</button>
                    </div>
                </div>
            </div>
        </section>

        // New group modal
        <div id="group-modal" class="hidden fixed inset-0 z-[100] items-center justify-center bg-black/70 backdrop-blur-sm p-4">
            <div class="w-full max-w-md bg-surface-card border border-zinc-800 rounded-2xl p-5">
                <div class="flex items-center justify-between mb-4"><h3 class="text-base font-semibold">"New group"</h3><button id="group-close" class="text-zinc-500 hover:text-white"><i class="ph ph-x"></i></button></div>
                <input id="group-name" type="text" placeholder="Group name" class="w-full px-3 py-2 mb-3 bg-surface border border-zinc-800 rounded-lg text-sm text-zinc-100 outline-none focus:border-accent" />
                <p class="text-xs text-zinc-500 mb-2">"Add friends"</p>
                <div id="group-friends" class="max-h-56 overflow-y-auto space-y-1 mb-4"></div>
                <button id="group-create" class="w-full px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">"Create group"</button>
            </div>
        </div>

        <script>
        r##"
        (async function() {
            var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
            if (!token) { window.location.href = '/login?redirect=/messages'; return; }
            var H = { 'Authorization': 'Bearer ' + token };
            var HJ = { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' };

            var currentConvId = null;
            var replyTo = null;
            var me = {};
            try { me = JSON.parse(decodeURIComponent(document.cookie.match('(^|;)\\s*user\\s*=\\s*([^;]+)')?.pop() || '')); } catch(e) {}

            var listEl = document.getElementById('conversation-list');
            var chatMessages = document.getElementById('chat-messages');
            var chatEmpty = document.getElementById('chat-empty');
            var chatHeader = document.getElementById('chat-header');
            var chatName = document.getElementById('chat-name');
            var chatKindIcon = document.getElementById('chat-kind-icon');
            var chatInputBar = document.getElementById('chat-input-bar');
            var messageInput = document.getElementById('message-input');
            var sendBtn = document.getElementById('send-btn');

            function esc(s) { var d = document.createElement('div'); d.textContent = s == null ? '' : String(s); return d.innerHTML; }
            function formatTime(iso) { var d = new Date(iso); return d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }); }
            function kindIcon(k) { return k === 'team' ? 'ph-users-three' : (k === 'group' ? 'ph-users' : 'ph-user'); }

            function convAvatar(c) {
                if (c.avatar_url) return '<img src="' + esc(c.avatar_url) + '" class="w-9 h-9 rounded-full object-cover shrink-0" />';
                if (c.kind === 'group' || c.kind === 'team') return '<div class="w-9 h-9 rounded-full bg-accent/20 flex items-center justify-center text-accent shrink-0"><i class="ph ' + kindIcon(c.kind) + '"></i></div>';
                return '<div class="w-9 h-9 rounded-full bg-accent/20 flex items-center justify-center text-xs font-bold text-accent shrink-0">' + esc((c.name || '?')[0].toUpperCase()) + '</div>';
            }

            async function loadConversations() {
                var res = await fetch('/api/messages/conversations', { headers: H });
                var data = await res.json();
                if (!Array.isArray(data) || !data.length) { listEl.innerHTML = '<p class="p-4 text-xs text-zinc-500 text-center">No conversations yet.</p>'; return; }
                listEl.innerHTML = data.map(function(c) {
                    var unread = c.unread_count > 0 ? '<span class="w-5 h-5 rounded-full bg-accent text-white text-[10px] flex items-center justify-center shrink-0">' + c.unread_count + '</span>' : '';
                    return '<button class="w-full text-left px-3 py-3 hover:bg-zinc-800/50 border-b border-zinc-800/50 flex items-center gap-3 transition-colors conv-item" data-id="' + c.id + '" data-kind="' + esc(c.kind) + '" data-name="' + esc(c.name || 'Chat') + '">' +
                        convAvatar(c) +
                        '<div class="flex-1 min-w-0">' +
                            '<div class="flex items-center justify-between gap-2"><span class="text-sm font-medium text-zinc-200 truncate">' + esc(c.name || 'Unknown') + '</span>' + unread + '</div>' +
                            '<p class="text-xs text-zinc-500 truncate mt-0.5">' + esc(c.last_message_body || 'No messages yet') + '</p>' +
                        '</div>' +
                    '</button>';
                }).join('');
                document.querySelectorAll('.conv-item').forEach(function(el) {
                    el.addEventListener('click', function() { openConversation(el.dataset.id, el.dataset.name, el.dataset.kind); });
                });
            }

            async function openConversation(convId, name, kind) {
                currentConvId = convId;
                clearReply();
                chatEmpty.classList.add('hidden');
                chatMessages.classList.remove('hidden');
                chatHeader.classList.remove('hidden');
                chatHeader.classList.add('flex');
                chatInputBar.classList.remove('hidden');
                chatName.textContent = name;
                chatKindIcon.className = 'ph ' + kindIcon(kind) + ' text-zinc-500';
                fetch('/api/messages/conversations/' + convId + '/read', { method: 'POST', headers: H });
                await loadMessages();
                scrollToBottom();
            }

            async function loadMessages() {
                if (!currentConvId) return;
                var res = await fetch('/api/messages/conversations/' + currentConvId + '/messages?limit=50', { headers: H });
                var data = await res.json();
                if (!Array.isArray(data)) return;
                data.reverse();
                var byId = {}; data.forEach(function(m) { byId[m.id] = m; });

                chatMessages.innerHTML = data.map(function(m) {
                    var isMe = m.sender_username === me.username;
                    var deleted = m.deleted;
                    var body = deleted ? '<em class="text-zinc-600">Message deleted</em>' : esc(m.body);
                    var edited = m.edited_at ? ' <span class="text-[10px] text-zinc-600">(edited)</span>' : '';
                    var quoted = '';
                    if (m.reply_to_id && byId[m.reply_to_id]) {
                        var q = byId[m.reply_to_id];
                        quoted = '<div class="text-[11px] text-zinc-500 border-l-2 border-zinc-700 pl-2 mb-1 truncate">' + esc(q.sender_username) + ': ' + esc((q.body || '').slice(0, 80)) + '</div>';
                    }
                    var actions = deleted ? '' : ('<div class="msg-actions hidden group-hover:flex items-center gap-1 text-zinc-500">' +
                        '<button title="Reply" onclick="__reply(\'' + m.id + '\',\'' + esc((m.sender_username||'').replace(/['"\\]/g, ' ')) + '\',\'' + esc((m.body||'').slice(0,60).replace(/['"\\]/g, ' ')) + '\')" class="hover:text-accent"><i class="ph ph-arrow-bend-up-left"></i></button>' +
                        (isMe ? '<button title="Edit" onclick="__edit(\'' + m.id + '\')" class="hover:text-accent"><i class="ph ph-pencil-simple"></i></button><button title="Delete" onclick="__del(\'' + m.id + '\')" class="hover:text-red-400"><i class="ph ph-trash"></i></button>' : '') +
                    '</div>');

                    if (isMe) {
                        return '<div class="group flex justify-end items-end gap-1.5" data-mid="' + m.id + '">' + actions +
                            '<div class="max-w-[70%]"><div class="bg-accent/20 border border-accent/10 rounded-2xl rounded-br-md px-4 py-2">' + quoted +
                                '<p class="text-sm text-zinc-200" data-body="1">' + body + edited + '</p></div>' +
                                '<p class="text-[10px] text-zinc-600 mt-0.5 text-right">' + formatTime(m.created_at) + '</p></div></div>';
                    }
                    return '<div class="group flex items-end gap-2" data-mid="' + m.id + '">' +
                        '<div class="w-7 h-7 rounded-full bg-zinc-800 flex items-center justify-center text-[10px] font-bold text-zinc-400 shrink-0">' + esc((m.sender_username || '?')[0].toUpperCase()) + '</div>' +
                        '<div class="max-w-[70%]"><p class="text-[10px] text-zinc-500 mb-0.5">' + esc(m.sender_username) + '</p>' +
                            '<div class="bg-zinc-800/50 border border-zinc-700/50 rounded-2xl rounded-bl-md px-4 py-2">' + quoted + '<p class="text-sm text-zinc-300" data-body="1">' + body + edited + '</p></div>' +
                            '<p class="text-[10px] text-zinc-600 mt-0.5">' + formatTime(m.created_at) + '</p></div>' + actions + '</div>';
                }).join('');
            }

            function scrollToBottom() { chatMessages.scrollTop = chatMessages.scrollHeight; }

            // ── Reply / edit / delete ──
            var replyBanner = document.getElementById('reply-banner');
            var replyText = document.getElementById('reply-text');
            function clearReply() { replyTo = null; replyBanner.classList.add('hidden'); replyBanner.classList.remove('flex'); }
            document.getElementById('reply-cancel').addEventListener('click', clearReply);
            window.__reply = function(id, user, snippet) { replyTo = id; replyText.textContent = 'Replying to ' + user + ': ' + snippet; replyBanner.classList.remove('hidden'); replyBanner.classList.add('flex'); messageInput.focus(); };
            window.__edit = async function(id) {
                var el = chatMessages.querySelector('[data-mid="' + id + '"] [data-body="1"]');
                var cur = el ? el.textContent.replace(/\s*\(edited\)\s*$/, '') : '';
                var next = prompt('Edit message', cur);
                if (next == null || !next.trim() || next === cur) return;
                await fetch('/api/messages/conversations/' + currentConvId + '/messages/' + id, { method: 'PUT', headers: HJ, body: JSON.stringify({ body: next.trim() }) });
                await loadMessages();
            };
            window.__del = async function(id) {
                if (!confirm('Delete this message?')) return;
                await fetch('/api/messages/conversations/' + currentConvId + '/messages/' + id, { method: 'DELETE', headers: H });
                await loadMessages();
            };

            async function sendMessage() {
                var body = messageInput.value.trim();
                if (!body || !currentConvId) return;
                messageInput.value = '';
                var payload = { body: body };
                if (replyTo) payload.reply_to_id = replyTo;
                clearReply();
                await fetch('/api/messages/conversations/' + currentConvId + '/messages', { method: 'POST', headers: HJ, body: JSON.stringify(payload) });
                await loadMessages();
                scrollToBottom();
            }
            sendBtn.addEventListener('click', sendMessage);
            messageInput.addEventListener('keydown', function(e) { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); sendMessage(); } });

            // ── Search ──
            var searchTimer;
            document.getElementById('conv-search').addEventListener('input', function(e) {
                clearTimeout(searchTimer);
                var q = e.target.value.trim();
                if (q.length < 2) { loadConversations(); return; }
                searchTimer = setTimeout(async function() {
                    try {
                        var res = await fetch('/api/messages/search?q=' + encodeURIComponent(q) + '&limit=30', { headers: H });
                        var hits = await res.json();
                        if (!Array.isArray(hits) || !hits.length) { listEl.innerHTML = '<p class="p-4 text-xs text-zinc-500 text-center">No matches.</p>'; return; }
                        listEl.innerHTML = hits.map(function(m) {
                            return '<button class="w-full text-left px-3 py-3 hover:bg-zinc-800/50 border-b border-zinc-800/50 conv-item" data-id="' + m.conversation_id + '" data-kind="' + esc(m.kind || 'dm') + '" data-name="' + esc(m.conversation_name || m.sender_username || 'Chat') + '">' +
                                '<div class="flex items-center justify-between gap-2"><span class="text-sm font-medium text-zinc-200 truncate">' + esc(m.conversation_name || m.sender_username || 'Chat') + '</span><span class="text-[10px] text-zinc-600">' + formatTime(m.created_at) + '</span></div>' +
                                '<p class="text-xs text-zinc-500 truncate mt-0.5">' + esc(m.body) + '</p></button>';
                        }).join('');
                        document.querySelectorAll('.conv-item').forEach(function(el) { el.addEventListener('click', function() { openConversation(el.dataset.id, el.dataset.name, el.dataset.kind); }); });
                    } catch(e) {}
                }, 250);
            });

            // ── New group ──
            var groupModal = document.getElementById('group-modal');
            document.getElementById('new-group-btn').addEventListener('click', async function() {
                groupModal.classList.remove('hidden'); groupModal.classList.add('flex');
                var box = document.getElementById('group-friends');
                box.innerHTML = '<p class="text-xs text-zinc-600">Loading…</p>';
                try {
                    var friends = await fetch('/api/gameservices/friends', { headers: H }).then(function(r){ return r.ok ? r.json() : []; });
                    box.innerHTML = friends.length ? friends.map(function(f) {
                        return '<label class="flex items-center gap-2 px-2 py-1.5 rounded-lg hover:bg-white/[0.03] cursor-pointer"><input type="checkbox" class="grp-check accent-accent" value="' + f.user_id + '" /><span class="text-sm text-zinc-300">' + esc(f.username) + '</span></label>';
                    }).join('') : '<p class="text-xs text-zinc-600">Add friends first to start a group.</p>';
                } catch(e) {}
            });
            document.getElementById('group-close').addEventListener('click', function() { groupModal.classList.add('hidden'); groupModal.classList.remove('flex'); });
            document.getElementById('group-create').addEventListener('click', async function() {
                var name = document.getElementById('group-name').value.trim();
                var ids = [...document.querySelectorAll('.grp-check:checked')].map(function(c){ return c.value; });
                if (!name || !ids.length) { alert('Give the group a name and pick at least one friend.'); return; }
                try {
                    var res = await fetch('/api/messages/conversations/group', { method: 'POST', headers: HJ, body: JSON.stringify({ name: name, member_ids: ids }) });
                    var d = await res.json();
                    groupModal.classList.add('hidden'); groupModal.classList.remove('flex');
                    await loadConversations();
                    if (d.id || d.conversation_id) openConversation(d.id || d.conversation_id, name, 'group');
                } catch(e) {}
            });

            // ── Live updates (nav owns the socket and re-dispatches events) ──
            window.addEventListener('renzora:new_message', function(e) {
                if (e.detail && e.detail.conversation_id === currentConvId) loadMessages().then(scrollToBottom);
                loadConversations();
            });
            window.addEventListener('renzora:message_edited', function(e) { if (e.detail && e.detail.conversation_id === currentConvId) loadMessages(); });
            window.addEventListener('renzora:message_deleted', function(e) { if (e.detail && e.detail.conversation_id === currentConvId) loadMessages(); });

            await loadConversations();

            var convParam = new URLSearchParams(window.location.search).get('conv');
            if (convParam) {
                setTimeout(function() {
                    var item = document.querySelector('.conv-item[data-id="' + convParam + '"]');
                    if (item) item.click(); else openConversation(convParam, 'Chat', 'dm');
                }, 400);
            }
        })();
        "##
        </script>
    }
}
