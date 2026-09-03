use leptos::prelude::*;

#[component]
pub fn GiftsPage() -> impl IntoView {
    view! {
        <section class="max-w-2xl mx-auto py-12 px-4">
            <h1 class="text-2xl font-bold text-zinc-100 mb-2">"Gift Cards"</h1>
            <p class="text-sm text-zinc-400 mb-8">"Send credits to friends as a gift."</p>

            <div id="gift-auth" class="hidden">
                // Send gift form
                <div class="bg-surface-card border border-zinc-800 rounded-2xl p-6 mb-6">
                    <h2 class="text-base font-semibold text-zinc-200 mb-4">"Send a Gift"</h2>
                    <div class="space-y-3">
                        <div>
                            <label class="text-xs text-zinc-400 block mb-1">"Recipient (username, leave blank for a gift code)"</label>
                            <input id="gift-recipient" type="text" class="w-full px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl text-sm text-zinc-200 placeholder:text-zinc-600 focus:outline-none focus:border-accent/50" placeholder="username" />
                        </div>
                        <div>
                            <label class="text-xs text-zinc-400 block mb-1">"Amount (min 10 credits)"</label>
                            <input id="gift-amount" type="number" min="10" class="w-full px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl text-sm text-zinc-200 focus:outline-none focus:border-accent/50" placeholder="100" />
                        </div>
                        <div>
                            <label class="text-xs text-zinc-400 block mb-1">"Message (optional)"</label>
                            <input id="gift-message" type="text" class="w-full px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl text-sm text-zinc-200 placeholder:text-zinc-600 focus:outline-none focus:border-accent/50" placeholder="Happy birthday!" />
                        </div>
                        <button id="send-gift-btn" class="w-full py-2.5 bg-accent hover:bg-accent-hover text-white text-sm font-medium rounded-xl transition-colors">"Send Gift"</button>
                        <div id="gift-error" class="hidden text-xs text-red-400"></div>
                        <div id="gift-success" class="hidden text-xs text-green-400"></div>
                    </div>
                </div>

                // Redeem
                <div class="bg-surface-card border border-zinc-800 rounded-2xl p-6 mb-6">
                    <h2 class="text-base font-semibold text-zinc-200 mb-4">"Redeem a Gift Card"</h2>
                    <div class="flex gap-2">
                        <input id="redeem-code" type="text" class="flex-1 px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl text-sm text-zinc-200 font-mono uppercase placeholder:text-zinc-600 focus:outline-none focus:border-accent/50" placeholder="GIFT-XXXXXXXX" />
                        <button id="redeem-btn" class="px-6 py-2.5 bg-accent hover:bg-accent-hover text-white text-sm font-medium rounded-xl transition-colors">"Redeem"</button>
                    </div>
                    <div id="redeem-error" class="hidden text-xs text-red-400 mt-2"></div>
                    <div id="redeem-success" class="hidden text-xs text-green-400 mt-2"></div>
                </div>

                // History
                <div class="bg-surface-card border border-zinc-800 rounded-2xl p-6">
                    <h2 class="text-base font-semibold text-zinc-200 mb-4">"Gift History"</h2>
                    <div id="gift-history" class="space-y-2">
                        <p class="text-sm text-zinc-500">"Loading..."</p>
                    </div>
                </div>
            </div>

            <div id="gift-login" class="hidden text-center p-6 bg-surface-card border border-zinc-800 rounded-2xl">
                <p class="text-sm text-zinc-400">"Sign in to send and receive gifts"</p>
                <a href="/login" class="inline-block mt-3 px-6 py-2 bg-accent hover:bg-accent-hover text-white text-sm font-medium rounded-xl transition-colors">"Sign In"</a>
            </div>
        </section>

        <script>
        r##"
        (async function() {
            var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
            if (token) {
                document.getElementById('gift-auth').classList.remove('hidden');
            } else {
                document.getElementById('gift-login').classList.remove('hidden');
                return;
            }

            // Send gift
            document.getElementById('send-gift-btn').addEventListener('click', async function() {
                var recipient = document.getElementById('gift-recipient').value.trim();
                var amount = parseInt(document.getElementById('gift-amount').value);
                var message = document.getElementById('gift-message').value;
                var errorEl = document.getElementById('gift-error');
                var successEl = document.getElementById('gift-success');
                errorEl.classList.add('hidden'); successEl.classList.add('hidden');

                if (!amount || amount < 10) { errorEl.textContent = 'Minimum gift is 10 credits'; errorEl.classList.remove('hidden'); return; }

                var body = { amount: amount, message: message };
                if (recipient) body.recipient_username = recipient;

                var res = await fetch('/api/credits/gift-cards/send', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify(body)
                });
                var data = await res.json();
                if (data.code) {
                    if (data.auto_redeemed) {
                        successEl.textContent = 'Gift sent! ' + amount + ' credits delivered to ' + recipient;
                    } else {
                        successEl.textContent = 'Gift card created! Code: ' + data.code + ' (share this with the recipient)';
                    }
                    successEl.classList.remove('hidden');
                    loadHistory();
                } else {
                    errorEl.textContent = data.error || 'Failed'; errorEl.classList.remove('hidden');
                }
            });

            // Redeem
            document.getElementById('redeem-btn').addEventListener('click', async function() {
                var code = document.getElementById('redeem-code').value.trim();
                var errorEl = document.getElementById('redeem-error');
                var successEl = document.getElementById('redeem-success');
                errorEl.classList.add('hidden'); successEl.classList.add('hidden');
                if (!code) { errorEl.textContent = 'Enter a code'; errorEl.classList.remove('hidden'); return; }

                var res = await fetch('/api/credits/gift-cards/redeem', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ code: code })
                });
                var data = await res.json();
                if (data.ok) {
                    successEl.textContent = 'Redeemed! ' + data.amount + ' credits added to your account';
                    successEl.classList.remove('hidden');
                    document.getElementById('redeem-code').value = '';
                    loadHistory();
                } else {
                    errorEl.textContent = data.error || 'Invalid code'; errorEl.classList.remove('hidden');
                }
            });

            // Load history
            async function loadHistory() {
                var [sentRes, recvRes] = await Promise.all([
                    fetch('/api/credits/gift-cards/sent', { headers: { 'Authorization': 'Bearer ' + token } }),
                    fetch('/api/credits/gift-cards/received', { headers: { 'Authorization': 'Bearer ' + token } })
                ]);
                var sent = await sentRes.json();
                var received = await recvRes.json();
                var el = document.getElementById('gift-history');
                var all = [];
                if (Array.isArray(sent)) all = all.concat(sent.map(function(g) { return Object.assign(g, {dir: 'sent'}); }));
                if (Array.isArray(received)) all = all.concat(received.map(function(g) { return Object.assign(g, {dir: 'received'}); }));
                all.sort(function(a, b) { return new Date(b.created_at) - new Date(a.created_at); });

                if (all.length === 0) { el.innerHTML = '<p class="text-sm text-zinc-500">No gift history</p>'; return; }
                el.innerHTML = all.slice(0, 20).map(function(g) {
                    var badge = g.dir === 'sent' ? '<span class="text-[10px] px-2 py-0.5 rounded-full bg-warning/15 text-warning">Sent</span>' :
                        '<span class="text-[10px] px-2 py-0.5 rounded-full bg-success/15 text-success">Received</span>';
                    var status = g.status === 'redeemed' ? '<span class="text-[10px] text-zinc-500">Redeemed</span>' : '<span class="text-[10px] text-accent">Code: ' + g.code + '</span>';
                    return '<div class="flex items-center justify-between py-2 px-3 bg-zinc-800/30 rounded-lg">' +
                        '<div class="flex items-center gap-2">' + badge + '<span class="text-sm text-zinc-300">' + g.amount + ' credits</span></div>' +
                        '<div class="flex items-center gap-3">' + status + '<span class="text-[10px] text-zinc-600">' + new Date(g.created_at).toLocaleDateString() + '</span></div>' +
                    '</div>';
                }).join('');
            }

            loadHistory();
        })();
        "##
        </script>
    }
}
