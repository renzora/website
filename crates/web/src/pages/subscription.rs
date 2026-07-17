use leptos::prelude::*;

#[component]
pub fn SubscriptionPage() -> impl IntoView {
    view! {
        <section class="py-12 px-6 min-h-screen">
            <div class="max-w-3xl mx-auto">
                <div class="mb-10 text-center">
                    <div class="inline-flex items-center justify-center w-16 h-16 rounded-full bg-accent/10 border border-accent/20 mb-4">
                        <i class="ph ph-heart text-3xl text-accent"></i>
                    </div>
                    <h1 class="text-3xl font-bold">"Become a Supporter"</h1>
                    <p class="text-zinc-400 mt-2 max-w-xl mx-auto">"Support Renzora with a monthly amount you choose — any amount from 10 credits. Paid from your credit balance, renews every 30 days."</p>
                </div>
                <div id="sub-content">
                    <div class="text-center py-12">
                        <div class="inline-block animate-spin w-6 h-6 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                    </div>
                </div>
            </div>
        </section>
        <script>
            r##"
            var MIN_AMOUNT = 10;

            (async function() {
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                var el = document.getElementById('sub-content');

                var current = null;
                if (token) {
                    try {
                        var curRes = await fetch('/api/subscriptions/current', { headers: { 'Authorization': 'Bearer ' + token } });
                        if (curRes.ok) current = await curRes.json();
                    } catch(e) {}
                }

                var sub = current && current.subscription;
                var isActive = sub && sub.status === 'active' && new Date(sub.current_period_end) > new Date();

                var html = '';

                // Active supporter banner
                if (isActive) {
                    var endDate = new Date(sub.current_period_end).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
                    html += '<div class="mb-8 p-6 bg-accent/5 border border-accent/20 rounded-2xl">';
                    html += '<div class="flex items-center justify-between flex-wrap gap-4"><div>';
                    html += '<div class="flex items-center gap-2"><i class="ph ph-heart-straight-fill text-accent"></i><span class="text-lg font-semibold">You are a Supporter</span>';
                    html += '<span class="px-2 py-0.5 rounded bg-accent/10 border border-accent/20 text-[10px] text-accent font-medium">ACTIVE</span></div>';
                    html += '<div class="flex items-center gap-4 mt-2 text-sm text-zinc-500 flex-wrap">';
                    html += '<span><i class="ph ph-coins"></i> ' + sub.monthly_amount.toLocaleString() + ' credits/month</span>';
                    if (sub.cancel_at_period_end || !sub.auto_renew) {
                        html += '<span class="text-amber-400"><i class="ph ph-warning"></i> Ends ' + endDate + '</span>';
                    } else {
                        html += '<span><i class="ph ph-repeat"></i> Renews ' + endDate + '</span>';
                    }
                    html += '<span><i class="ph ph-wallet"></i> Balance: ' + current.credit_balance.toLocaleString() + ' credits</span>';
                    html += '</div>';
                    html += '<label class="flex items-center gap-2 mt-3 text-sm text-zinc-400 cursor-pointer">';
                    html += '<input type="checkbox" id="auto-renew-toggle" ' + (sub.auto_renew && !sub.cancel_at_period_end ? 'checked' : '') + ' onchange="toggleAutoRenew(this.checked)" class="accent-[var(--accent,#6366f1)]">';
                    html += 'Auto-renew: deduct ' + sub.monthly_amount.toLocaleString() + ' credits automatically at the end of each term</label>';
                    html += '</div>';
                    if (!sub.cancel_at_period_end) {
                        html += '<button onclick="cancelSub()" class="px-4 py-2 rounded-lg text-sm text-red-400 hover:bg-red-950/30 border border-transparent hover:border-red-900/50 transition-all">Cancel</button>';
                    }
                    html += '</div></div>';
                    html += '<p class="text-xs text-zinc-600 mb-8 text-center">Want to change your monthly amount? Pick a new amount below — you\'ll be charged the new amount now and your 30-day period restarts.</p>';
                }

                // Amount picker
                html += '<div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl mb-8">';
                html += '<h3 class="text-sm font-semibold mb-4">' + (isActive ? 'Change your monthly amount' : 'Choose your monthly amount') + '</h3>';
                html += '<div class="grid grid-cols-4 gap-2 mb-4" id="amount-presets">';
                [10, 25, 50, 100].forEach(function(a) {
                    html += '<button onclick="pickAmount(' + a + ')" data-amount="' + a + '" class="preset-btn px-3 py-3 rounded-xl border border-zinc-800 bg-white/[0.02] hover:border-accent/50 transition-all text-center">';
                    html += '<div class="text-lg font-bold">' + a + '</div><div class="text-[10px] text-zinc-500">credits · $' + (a * 0.1).toFixed(2) + '/mo</div></button>';
                });
                html += '</div>';
                html += '<div class="flex items-center gap-3">';
                html += '<div class="relative flex-1"><i class="ph ph-coins absolute left-3 top-1/2 -translate-y-1/2 text-zinc-500"></i>';
                html += '<input type="number" id="custom-sub-amount" min="10" step="5" placeholder="Custom amount (min 10)" oninput="pickAmount(null)" class="w-full pl-9 pr-4 py-2.5 bg-white/[0.02] border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors"></div>';
                html += '<span id="sub-usd" class="text-xs text-zinc-500 w-20"></span>';
                html += '</div>';
                if (!isActive) {
                    html += '<label class="flex items-center gap-2 mt-4 text-sm text-zinc-400 cursor-pointer">';
                    html += '<input type="checkbox" id="sub-auto-renew" checked class="accent-[var(--accent,#6366f1)]">';
                    html += 'Auto-renew: deduct credits automatically at the end of each 30-day term</label>';
                }
                if (token) {
                    html += '<button onclick="subscribeSupporter()" id="sub-btn" class="w-full mt-5 px-4 py-3 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all disabled:opacity-50">' + (isActive ? 'Update amount' : 'Become a Supporter') + '</button>';
                } else {
                    html += '<a href="/login" class="block w-full mt-5 px-4 py-3 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all text-center">Sign in to support</a>';
                }
                html += '<p id="sub-error" class="hidden text-sm text-red-400 mt-3"></p>';
                html += '</div>';

                // Perks
                html += '<div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl">';
                html += '<h3 class="text-sm font-semibold mb-4">Supporter perks</h3>';
                html += '<div class="grid grid-cols-1 sm:grid-cols-2 gap-2 text-[13px]">';
                var perk = function(icon, text) { return '<div class="flex items-center gap-2 text-zinc-400"><i class="ph ph-' + icon + ' text-accent text-sm"></i>' + text + '</div>'; };
                html += perk('seal-check', 'Supporter profile badge');
                html += perk('discord-logo', 'Supporter Discord role');
                html += perk('palette', 'Custom profile colors & banner');
                html += perk('hard-drive', '10GB cloud storage');
                html += perk('users-three', 'Up to 5 team members');
                html += perk('lightning', '5,000 API requests/day');
                html += '</div>';
                html += '<p class="text-xs text-zinc-600 mt-4">Perks are the same at every amount — pay what feels right. Credits are deducted from your balance; if it runs low at renewal time your subscription simply ends.</p>';
                html += '</div>';

                el.innerHTML = html;
            })();

            function pickAmount(a) {
                document.querySelectorAll('.preset-btn').forEach(function(b) {
                    b.classList.toggle('border-accent', a !== null && parseInt(b.dataset.amount) === a);
                    b.classList.toggle('bg-accent/10', a !== null && parseInt(b.dataset.amount) === a);
                });
                if (a !== null) document.getElementById('custom-sub-amount').value = a;
                var val = parseInt(document.getElementById('custom-sub-amount').value) || 0;
                document.getElementById('sub-usd').textContent = val >= MIN_AMOUNT ? '= $' + (val * 0.1).toFixed(2) + '/mo' : '';
            }

            async function subscribeSupporter() {
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                var amount = parseInt(document.getElementById('custom-sub-amount').value);
                var errEl = document.getElementById('sub-error');
                errEl.classList.add('hidden');
                if (!amount || amount < MIN_AMOUNT) {
                    errEl.textContent = 'Minimum supporter amount is ' + MIN_AMOUNT + ' credits/month.';
                    errEl.classList.remove('hidden');
                    return;
                }
                if (!confirm('Support Renzora with ' + amount + ' credits/month? The first month is deducted from your balance now.')) return;
                var autoRenewEl = document.getElementById('sub-auto-renew');
                var btn = document.getElementById('sub-btn');
                btn.disabled = true;
                try {
                    var res = await fetch('/api/subscriptions/subscribe', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({ amount: amount, auto_renew: autoRenewEl ? autoRenewEl.checked : true })
                    });
                    var data = await res.json();
                    if (res.ok) { window.location.reload(); }
                    else {
                        errEl.textContent = data.error || data.message || 'Failed to subscribe';
                        errEl.classList.remove('hidden');
                        btn.disabled = false;
                    }
                } catch(e) {
                    errEl.textContent = 'Error: ' + e.message;
                    errEl.classList.remove('hidden');
                    btn.disabled = false;
                }
            }

            async function toggleAutoRenew(enabled) {
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                var res = await fetch('/api/subscriptions/auto-renew', {
                    method: 'PUT',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ enabled: enabled })
                });
                if (!res.ok) {
                    alert('Failed to update auto-renew');
                    document.getElementById('auto-renew-toggle').checked = !enabled;
                } else if (enabled) {
                    window.location.reload();
                }
            }

            async function cancelSub() {
                if (!confirm('Cancel your Supporter subscription? You keep your perks until the end of the billing period.')) return;
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                var res = await fetch('/api/subscriptions/cancel', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token }
                });
                if (res.ok) { window.location.reload(); }
                else { alert('Failed to cancel'); }
            }
            "##
        </script>
    }
}
