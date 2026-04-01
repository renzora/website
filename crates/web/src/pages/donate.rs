use leptos::prelude::*;

#[component]
pub fn DonatePage() -> impl IntoView {
    view! {
        <section class="max-w-3xl mx-auto py-12 px-4">
            <div class="text-center mb-8">
                <h1 class="text-2xl font-bold text-zinc-100">"Support Renzora"</h1>
                <p class="text-sm text-zinc-400 mt-2">"Your donations help us keep the platform running and fund new features."</p>
                <div class="mt-4 inline-flex items-center gap-2 px-4 py-2 bg-accent/10 border border-accent/20 rounded-xl">
                    <i class="ph ph-heart-fill text-accent"></i>
                    <span class="text-sm text-accent font-medium" id="total-donated">"Loading..."</span>
                    <span class="text-xs text-zinc-500">"credits donated"</span>
                </div>
            </div>

            // Donate form (auth required)
            <div id="donate-form" class="hidden bg-surface-card border border-zinc-800 rounded-2xl p-6 mb-8">
                <h2 class="text-base font-semibold text-zinc-200 mb-4">"Make a Donation"</h2>
                <div class="grid grid-cols-4 gap-2 mb-4">
                    <button class="donate-preset px-4 py-3 bg-zinc-800/50 border border-zinc-700 rounded-xl text-sm text-zinc-300 hover:border-accent/50 hover:text-accent transition-colors" data-amount="10">"10"</button>
                    <button class="donate-preset px-4 py-3 bg-zinc-800/50 border border-zinc-700 rounded-xl text-sm text-zinc-300 hover:border-accent/50 hover:text-accent transition-colors" data-amount="50">"50"</button>
                    <button class="donate-preset px-4 py-3 bg-zinc-800/50 border border-zinc-700 rounded-xl text-sm text-zinc-300 hover:border-accent/50 hover:text-accent transition-colors" data-amount="100">"100"</button>
                    <button class="donate-preset px-4 py-3 bg-zinc-800/50 border border-zinc-700 rounded-xl text-sm text-zinc-300 hover:border-accent/50 hover:text-accent transition-colors" data-amount="500">"500"</button>
                </div>
                <div class="flex gap-3 mb-4">
                    <input id="donate-amount" type="number" min="1" class="flex-1 px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl text-sm text-zinc-200 placeholder:text-zinc-600 focus:outline-none focus:border-accent/50" placeholder="Custom amount" />
                    <input id="donate-message" type="text" class="flex-1 px-4 py-2.5 bg-zinc-900 border border-zinc-700 rounded-xl text-sm text-zinc-200 placeholder:text-zinc-600 focus:outline-none focus:border-accent/50" placeholder="Message (optional)" />
                </div>
                <div class="flex items-center justify-between">
                    <label class="flex items-center gap-2 text-sm text-zinc-400 cursor-pointer">
                        <input type="checkbox" id="donate-anon" class="checkbox checkbox-sm checkbox-accent" />
                        "Donate anonymously"
                    </label>
                    <button id="donate-btn" class="px-6 py-2.5 bg-accent hover:bg-accent-hover text-white text-sm font-medium rounded-xl transition-colors">"Donate"</button>
                </div>
                <div id="donate-error" class="hidden text-xs text-red-400 mt-2"></div>
                <div id="donate-success" class="hidden text-xs text-green-400 mt-2"></div>
            </div>

            <div id="donate-login" class="hidden text-center mb-8 p-6 bg-surface-card border border-zinc-800 rounded-2xl">
                <p class="text-sm text-zinc-400">"Sign in to make a donation"</p>
                <a href="/login" class="inline-block mt-3 px-6 py-2 bg-accent hover:bg-accent-hover text-white text-sm font-medium rounded-xl transition-colors">"Sign In"</a>
            </div>

            // Leaderboard
            <div class="bg-surface-card border border-zinc-800 rounded-2xl p-6">
                <h2 class="text-base font-semibold text-zinc-200 mb-4">"Donation Leaderboard"</h2>
                <div id="leaderboard" class="space-y-2">
                    <div class="flex justify-center py-4"><span class="loading loading-spinner loading-sm text-accent"></span></div>
                </div>
            </div>

            // Badge info
            <div class="mt-8 grid grid-cols-4 gap-3">
                <div class="text-center p-4 bg-surface-card border border-zinc-800 rounded-xl">
                    <div class="text-2xl mb-1" style="color: #cd7f32">"♥"</div>
                    <div class="text-xs font-medium text-zinc-300">"Bronze"</div>
                    <div class="text-[10px] text-zinc-500">"100+ credits"</div>
                </div>
                <div class="text-center p-4 bg-surface-card border border-zinc-800 rounded-xl">
                    <div class="text-2xl mb-1" style="color: #c0c0c0">"♥"</div>
                    <div class="text-xs font-medium text-zinc-300">"Silver"</div>
                    <div class="text-[10px] text-zinc-500">"500+ credits"</div>
                </div>
                <div class="text-center p-4 bg-surface-card border border-zinc-800 rounded-xl">
                    <div class="text-2xl mb-1" style="color: #ffd700">"♥"</div>
                    <div class="text-xs font-medium text-zinc-300">"Gold"</div>
                    <div class="text-[10px] text-zinc-500">"1000+ credits"</div>
                </div>
                <div class="text-center p-4 bg-surface-card border border-zinc-800 rounded-xl">
                    <div class="text-2xl mb-1" style="color: #e5e4e2">"♥"</div>
                    <div class="text-xs font-medium text-zinc-300">"Platinum"</div>
                    <div class="text-[10px] text-zinc-500">"5000+ credits"</div>
                </div>
            </div>
        </section>

        <script>
        r##"
        (async function() {
            var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();

            // Load total
            fetch('/api/credits/donate/total').then(r => r.json()).then(data => {
                document.getElementById('total-donated').textContent = (data.total || 0).toLocaleString();
            }).catch(() => {});

            // Load leaderboard
            fetch('/api/credits/donate/leaderboard').then(r => r.json()).then(data => {
                var el = document.getElementById('leaderboard');
                if (!Array.isArray(data) || data.length === 0) {
                    el.innerHTML = '<p class="text-sm text-zinc-500 text-center py-4">No donations yet. Be the first!</p>';
                    return;
                }
                el.innerHTML = data.map(function(d, i) {
                    var medal = i === 0 ? '🥇' : i === 1 ? '🥈' : i === 2 ? '🥉' : (i + 1) + '.';
                    var name = d.username || 'Anonymous';
                    return '<div class="flex items-center gap-3 py-2.5 px-3 ' + (i < 3 ? 'bg-accent/5 border border-accent/10' : 'bg-zinc-800/30') + ' rounded-lg">' +
                        '<span class="text-base w-8 text-center">' + medal + '</span>' +
                        '<div class="w-8 h-8 rounded-full bg-zinc-800 flex items-center justify-center text-xs font-bold text-zinc-400">' + name[0].toUpperCase() + '</div>' +
                        '<span class="flex-1 text-sm text-zinc-200">' + name + '</span>' +
                        '<span class="text-sm font-medium text-accent">' + (d.total || 0).toLocaleString() + ' credits</span>' +
                    '</div>';
                }).join('');
            }).catch(() => {});

            // Auth check
            if (token) {
                document.getElementById('donate-form').classList.remove('hidden');
            } else {
                document.getElementById('donate-login').classList.remove('hidden');
            }

            // Preset buttons
            document.querySelectorAll('.donate-preset').forEach(function(btn) {
                btn.addEventListener('click', function() {
                    document.getElementById('donate-amount').value = btn.dataset.amount;
                    document.querySelectorAll('.donate-preset').forEach(b => b.classList.remove('border-accent', 'text-accent'));
                    btn.classList.add('border-accent', 'text-accent');
                });
            });

            // Donate
            document.getElementById('donate-btn')?.addEventListener('click', async function() {
                var amount = parseInt(document.getElementById('donate-amount').value);
                var message = document.getElementById('donate-message').value;
                var anonymous = document.getElementById('donate-anon').checked;
                var errorEl = document.getElementById('donate-error');
                var successEl = document.getElementById('donate-success');

                errorEl.classList.add('hidden');
                successEl.classList.add('hidden');

                if (!amount || amount < 1) { errorEl.textContent = 'Enter an amount'; errorEl.classList.remove('hidden'); return; }

                var res = await fetch('/api/credits/donate', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ amount: amount, message: message, anonymous: anonymous })
                });
                var data = await res.json();
                if (data.ok) {
                    successEl.textContent = 'Thank you! You donated ' + amount + ' credits. Total: ' + (data.total_donated || 0).toLocaleString();
                    successEl.classList.remove('hidden');
                    document.getElementById('donate-amount').value = '';
                    document.getElementById('donate-message').value = '';
                    // Reload leaderboard and total
                    setTimeout(function() { window.location.reload(); }, 1500);
                } else {
                    errorEl.textContent = data.error || 'Failed to donate';
                    errorEl.classList.remove('hidden');
                }
            });
        })();
        "##
        </script>
    }
}
