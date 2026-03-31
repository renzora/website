use leptos::prelude::*;

#[component]
pub fn MessagesPage() -> impl IntoView {
    view! {
        <section class="h-[calc(100vh-64px)] flex" id="messages-page">
            // Left: conversation list
            <div class="w-80 border-r border-zinc-800 flex flex-col bg-surface">
                <div class="p-4 border-b border-zinc-800">
                    <h2 class="text-sm font-semibold text-zinc-300">Messages</h2>
                </div>
                <div id="conversation-list" class="flex-1 overflow-y-auto">
                    <div class="flex items-center justify-center py-12">
                        <span class="loading loading-spinner loading-sm text-accent"></span>
                    </div>
                </div>
            </div>

            // Right: chat view
            <div class="flex-1 flex flex-col bg-surface">
                <div id="chat-header" class="p-4 border-b border-zinc-800 hidden">
                    <h3 id="chat-name" class="text-sm font-semibold text-zinc-200"></h3>
                </div>
                <div id="chat-empty" class="flex-1 flex items-center justify-center text-zinc-600 text-sm">
                    "Select a conversation"
                </div>
                <div id="chat-messages" class="flex-1 overflow-y-auto p-4 space-y-3 hidden"></div>
                <div id="chat-input-bar" class="p-4 border-t border-zinc-800 hidden">
                    <div class="flex gap-2">
                        <input id="message-input" type="text" class="flex-1 px-4 py-2 bg-zinc-900 border border-zinc-700 rounded-xl text-sm text-zinc-200 placeholder:text-zinc-600 focus:outline-none focus:border-accent/50" placeholder="Type a message..." />
                        <button id="send-btn" class="px-4 py-2 bg-accent hover:bg-accent-hover text-white text-sm font-medium rounded-xl transition-colors">"Send"</button>
                    </div>
                    <div id="typing-indicator" class="text-xs text-zinc-500 mt-1 hidden"></div>
                </div>
            </div>
        </section>

        <script>
        r##"
        (async function() {
            var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
            if (!token) { window.location.href = '/login'; return; }

            var currentConvId = null;
            var listEl = document.getElementById('conversation-list');
            var chatMessages = document.getElementById('chat-messages');
            var chatEmpty = document.getElementById('chat-empty');
            var chatHeader = document.getElementById('chat-header');
            var chatName = document.getElementById('chat-name');
            var chatInputBar = document.getElementById('chat-input-bar');
            var messageInput = document.getElementById('message-input');
            var sendBtn = document.getElementById('send-btn');
            var typingIndicator = document.getElementById('typing-indicator');

            // Load conversations
            async function loadConversations() {
                var res = await fetch('/api/messages/conversations', {
                    headers: { 'Authorization': 'Bearer ' + token }
                });
                var data = await res.json();
                if (!Array.isArray(data)) { listEl.innerHTML = '<p class="p-4 text-xs text-zinc-500">No conversations</p>'; return; }

                listEl.innerHTML = data.map(function(c) {
                    var unread = c.unread_count > 0 ? '<span class="w-5 h-5 rounded-full bg-accent text-white text-[10px] flex items-center justify-center">' + c.unread_count + '</span>' : '';
                    return '<button class="w-full text-left px-4 py-3 hover:bg-zinc-800/50 border-b border-zinc-800/50 flex items-center gap-3 transition-colors conv-item" data-id="' + c.id + '">' +
                        '<div class="w-9 h-9 rounded-full bg-accent/20 flex items-center justify-center text-xs font-bold text-accent shrink-0">' + (c.name || '?')[0].toUpperCase() + '</div>' +
                        '<div class="flex-1 min-w-0">' +
                            '<div class="flex items-center justify-between">' +
                                '<span class="text-sm font-medium text-zinc-200 truncate">' + (c.name || 'Unknown') + '</span>' +
                                unread +
                            '</div>' +
                            '<p class="text-xs text-zinc-500 truncate mt-0.5">' + (c.last_message_body || 'No messages yet') + '</p>' +
                        '</div>' +
                    '</button>';
                }).join('');

                // Attach click handlers
                document.querySelectorAll('.conv-item').forEach(function(el) {
                    el.addEventListener('click', function() {
                        openConversation(el.dataset.id, el.querySelector('.text-sm').textContent);
                    });
                });
            }

            async function openConversation(convId, name) {
                currentConvId = convId;
                chatEmpty.classList.add('hidden');
                chatMessages.classList.remove('hidden');
                chatHeader.classList.remove('hidden');
                chatInputBar.classList.remove('hidden');
                chatName.textContent = name;

                // Mark as read
                fetch('/api/messages/conversations/' + convId + '/read', {
                    method: 'POST', headers: { 'Authorization': 'Bearer ' + token }
                });

                await loadMessages();
                scrollToBottom();
            }

            async function loadMessages() {
                if (!currentConvId) return;
                var res = await fetch('/api/messages/conversations/' + currentConvId + '/messages?limit=50', {
                    headers: { 'Authorization': 'Bearer ' + token }
                });
                var data = await res.json();
                if (!Array.isArray(data)) return;

                var userCookie = document.cookie.match('(^|;)\\s*user\\s*=\\s*([^;]+)')?.pop();
                var me = userCookie ? JSON.parse(decodeURIComponent(userCookie)) : {};

                // Messages come newest first, reverse for display
                data.reverse();

                chatMessages.innerHTML = data.map(function(m) {
                    var isMe = m.sender_username === me.username;
                    var deleted = m.deleted;
                    var body = deleted ? '<em class="text-zinc-600">Message deleted</em>' : escapeHtml(m.body);
                    var edited = m.edited_at ? ' <span class="text-[10px] text-zinc-600">(edited)</span>' : '';

                    if (isMe) {
                        return '<div class="flex justify-end"><div class="max-w-[70%]">' +
                            '<div class="bg-accent/20 border border-accent/10 rounded-2xl rounded-br-md px-4 py-2">' +
                                '<p class="text-sm text-zinc-200">' + body + edited + '</p>' +
                            '</div>' +
                            '<p class="text-[10px] text-zinc-600 mt-1 text-right">' + formatTime(m.created_at) + '</p>' +
                        '</div></div>';
                    } else {
                        return '<div class="flex gap-2"><div class="w-7 h-7 rounded-full bg-zinc-800 flex items-center justify-center text-[10px] font-bold text-zinc-400 shrink-0">' + (m.sender_username || '?')[0].toUpperCase() + '</div>' +
                            '<div class="max-w-[70%]">' +
                                '<p class="text-[10px] text-zinc-500 mb-0.5">' + m.sender_username + '</p>' +
                                '<div class="bg-zinc-800/50 border border-zinc-700/50 rounded-2xl rounded-bl-md px-4 py-2">' +
                                    '<p class="text-sm text-zinc-300">' + body + edited + '</p>' +
                                '</div>' +
                                '<p class="text-[10px] text-zinc-600 mt-1">' + formatTime(m.created_at) + '</p>' +
                            '</div></div>';
                    }
                }).join('');
            }

            function scrollToBottom() {
                chatMessages.scrollTop = chatMessages.scrollHeight;
            }

            function formatTime(iso) {
                var d = new Date(iso);
                return d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
            }

            function escapeHtml(s) {
                var div = document.createElement('div');
                div.textContent = s;
                return div.innerHTML;
            }

            // Send message
            async function sendMessage() {
                var body = messageInput.value.trim();
                if (!body || !currentConvId) return;
                messageInput.value = '';

                await fetch('/api/messages/conversations/' + currentConvId + '/messages', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ body: body })
                });

                await loadMessages();
                scrollToBottom();
            }

            sendBtn.addEventListener('click', sendMessage);
            messageInput.addEventListener('keydown', function(e) {
                if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); sendMessage(); }
            });

            // Listen for WS messages
            if (window._wsSocket) {
                var origHandler = window._wsSocket.onmessage;
                window._wsSocket.addEventListener('message', function(evt) {
                    try {
                        var msg = JSON.parse(evt.data);
                        if (msg.event === 'new_message' && msg.data.conversation_id === currentConvId) {
                            loadMessages().then(scrollToBottom);
                        }
                        if (msg.event === 'new_message') {
                            loadConversations();
                        }
                        if (msg.event === 'typing' && msg.data.conversation_id === currentConvId) {
                            typingIndicator.textContent = msg.data.username + ' is typing...';
                            typingIndicator.classList.remove('hidden');
                            clearTimeout(window._typingTimeout);
                            window._typingTimeout = setTimeout(function() { typingIndicator.classList.add('hidden'); }, 3000);
                        }
                    } catch(e) {}
                });
            }

            await loadConversations();
        })();
        "##
        </script>
    }
}
