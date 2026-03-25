use leptos::prelude::*;

#[component]
pub fn WalletPage() -> impl IntoView {
    view! {
        <section class="py-12 px-6 min-h-[80vh] bg-gradient-to-b from-[#0a0c10] via-[#060608] to-[#060608]">
            <div class="max-w-[960px] mx-auto">

                // Success / cancelled banners
                <div id="wallet-success" class="hidden mb-6 p-4 rounded-xl bg-green-500/10 border border-green-500/20 text-green-400 text-sm flex items-center gap-3">
                    <i class="ph ph-check-circle text-xl"></i>
                    <span>"Payment successful! Your credits have been added."</span>
                </div>
                <div id="wallet-cancelled" class="hidden mb-6 p-4 rounded-xl bg-amber-500/10 border border-amber-500/20 text-amber-400 text-sm flex items-center gap-3">
                    <i class="ph ph-warning text-xl"></i>
                    <span>"Payment was cancelled. No credits were charged."</span>
                </div>
                <div id="wallet-error" class="hidden mb-6 p-4 rounded-xl bg-red-500/10 border border-red-500/20 text-red-400 text-sm flex items-center gap-3">
                    <i class="ph ph-x-circle text-xl"></i>
                    <span id="wallet-error-text"></span>
                </div>

                // Header
                <div class="mb-10">
                    <h1 class="text-3xl font-bold tracking-tight">"Credits"</h1>
                    <p class="text-zinc-500 text-sm mt-1">"Purchase credits to buy assets, plugins, and themes from the marketplace."</p>
                </div>

                // Balance + quick stats row
                <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-10">
                    <div class="sm:col-span-1 p-6 bg-surface-card border border-zinc-800 rounded-xl relative overflow-hidden">
                        <div class="absolute top-0 right-0 w-24 h-24 bg-accent/5 rounded-full -translate-y-8 translate-x-8"></div>
                        <p class="text-[11px] font-medium text-zinc-500 uppercase tracking-widest mb-1">"Balance"</p>
                        <div class="flex items-baseline gap-2">
                            <span id="wallet-balance" class="text-4xl font-extrabold tracking-tight text-zinc-50">"..."</span>
                            <span class="text-sm text-zinc-500">"credits"</span>
                        </div>
                        <p class="text-[11px] text-zinc-600 mt-2">"1 credit = $0.10 USD"</p>
                    </div>
                    <div class="p-6 bg-surface-card border border-zinc-800 rounded-xl">
                        <p class="text-[11px] font-medium text-zinc-500 uppercase tracking-widest mb-1">"Total Spent"</p>
                        <span id="wallet-spent" class="text-2xl font-bold text-zinc-50">"..."</span>
                    </div>
                    <div class="p-6 bg-surface-card border border-zinc-800 rounded-xl">
                        <p class="text-[11px] font-medium text-zinc-500 uppercase tracking-widest mb-1">"Total Earned"</p>
                        <span id="wallet-earned" class="text-2xl font-bold text-zinc-50">"..."</span>
                    </div>
                </div>

                // Credit tiers
                <div class="mb-12">
                    <h2 class="text-lg font-semibold mb-1">"Add Credits"</h2>
                    <p class="text-zinc-500 text-xs mb-5">"Select a pack below. You'll be redirected to Stripe for secure payment."</p>

                    <div class="grid grid-cols-2 lg:grid-cols-4 gap-3" id="tier-grid">
                        <TierCard credits=50    price="$5"    tag=""           color="zinc"   />
                        <TierCard credits=100   price="$10"   tag=""           color="zinc"   />
                        <TierCard credits=250   price="$25"   tag="Popular"    color="accent" />
                        <TierCard credits=500   price="$50"   tag="Best Value" color="accent" />
                    </div>

                    // Custom amount
                    <div class="mt-4 flex items-center gap-3">
                        <div class="relative flex-1 max-w-xs">
                            <i class="ph ph-coins absolute left-3 top-1/2 -translate-y-1/2 text-zinc-500"></i>
                            <input type="number" id="custom-amount" min="50" step="10" placeholder="Custom amount (min 50)"
                                class="w-full pl-9 pr-4 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors" />
                        </div>
                        <button onclick="buyCredits(parseInt(document.getElementById('custom-amount').value))"
                            class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-surface-card border border-zinc-800 text-zinc-300 hover:border-accent hover:text-white transition-all">
                            <i class="ph ph-arrow-right text-base"></i>"Purchase"
                        </button>
                    </div>
                </div>

                // Referral section
                <div id="referral-section" class="mb-12 hidden">
                    <h2 class="text-lg font-semibold mb-1">"Invite & Earn"</h2>
                    <p class="text-zinc-500 text-xs mb-5">"Share your referral link. Earn 5% of every purchase your referrals make — forever."</p>

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <div class="md:col-span-2 p-5 bg-surface-card border border-zinc-800 rounded-xl">
                            <p class="text-[11px] font-medium text-zinc-500 uppercase tracking-widest mb-2">"Your Referral Link"</p>
                            <div class="flex gap-2">
                                <input type="text" id="referral-link" readonly class="flex-1 px-3 py-2.5 bg-surface border border-zinc-800 rounded-lg text-zinc-50 text-sm font-mono outline-none" />
                                <button onclick="copyReferralLink()" id="copy-btn" class="px-4 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                                    <i class="ph ph-copy"></i>" Copy"
                                </button>
                            </div>
                            <p class="text-[10px] text-zinc-600 mt-2">"Anyone who signs up through this link is permanently linked to you."</p>
                        </div>

                        <div class="p-5 bg-surface-card border border-zinc-800 rounded-xl">
                            <div class="space-y-3">
                                <div>
                                    <p class="text-[11px] font-medium text-zinc-500 uppercase tracking-widest">"Referrals"</p>
                                    <span id="referral-count" class="text-2xl font-bold text-zinc-50">"0"</span>
                                </div>
                                <div>
                                    <p class="text-[11px] font-medium text-zinc-500 uppercase tracking-widest">"Total Earned"</p>
                                    <span id="referral-earned" class="text-2xl font-bold text-purple-400">"0"</span>
                                    <span class="text-xs text-zinc-500">" credits"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                // Transaction history
                <div>
                    <div class="flex items-center justify-between mb-4">
                        <h2 class="text-lg font-semibold">"Transaction History"</h2>
                        <div class="flex gap-1 text-xs">
                            <button onclick="filterTx('all')"   id="tx-filter-all"      class="px-2.5 py-1 rounded-md bg-accent text-white">"All"</button>
                            <button onclick="filterTx('topup')" id="tx-filter-topup"     class="px-2.5 py-1 rounded-md text-zinc-400 hover:bg-zinc-800">"Top-ups"</button>
                            <button onclick="filterTx('purchase')" id="tx-filter-purchase" class="px-2.5 py-1 rounded-md text-zinc-400 hover:bg-zinc-800">"Purchases"</button>
                            <button onclick="filterTx('earning')" id="tx-filter-earning"  class="px-2.5 py-1 rounded-md text-zinc-400 hover:bg-zinc-800">"Earnings"</button>
                            <button onclick="filterTx('referral')" id="tx-filter-referral" class="px-2.5 py-1 rounded-md text-zinc-400 hover:bg-zinc-800">"Referrals"</button>
                        </div>
                    </div>
                    <div id="tx-list" class="bg-surface-card border border-zinc-800 rounded-xl overflow-hidden">
                        <div class="text-center py-12 text-zinc-600">
                            <div class="inline-block animate-spin w-5 h-5 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                        </div>
                    </div>
                    <div id="tx-pagination" class="flex justify-center gap-2 mt-4"></div>
                </div>

                // Sign-in prompt
                <div id="wallet-signin" class="hidden text-center py-20">
                    <i class="ph ph-wallet text-5xl text-zinc-700 mb-4"></i>
                    <p class="text-zinc-500 mb-5">"Sign in to manage your credits."</p>
                    <a href="/login" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">"Sign In"</a>
                </div>

            </div>
        </section>

        <script>
            r##"
            function parseDate(s) {
                if (!s) return null;
                const iso = s.replace(/^(\d{4}-\d{2}-\d{2})\s+(\d{2}:\d{2}:\d{2}).*?\s+([+-]\d{2}):(\d{2}).*$/, '$1T$2$3:$4');
                const d = new Date(iso);
                return isNaN(d.getTime()) ? null : d;
            }
            function fmtDate(s) {
                const d = parseDate(s);
                return d ? d.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' }) : '';
            }

            let token = null;
            let allTx = [];
            let txFilter = 'all';
            let txPage = 1;

            (async function init() {
                token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) {
                    document.getElementById('wallet-signin').classList.remove('hidden');
                    document.getElementById('tier-grid')?.closest('.mb-12')?.classList.add('hidden');
                    return;
                }

                // Check URL params for Stripe redirect
                const params = new URLSearchParams(window.location.search);
                if (params.get('success') === 'true') {
                    document.getElementById('wallet-success').classList.remove('hidden');
                    fireConfetti();
                    history.replaceState({}, '', '/wallet');
                }
                if (params.get('cancelled') === 'true') {
                    document.getElementById('wallet-cancelled').classList.remove('hidden');
                    history.replaceState({}, '', '/wallet');
                }

                await Promise.all([loadBalance(), loadHistory(), loadReferralStats()]);
            })();

            function fireConfetti() {
                const colors = ['#6366f1', '#818cf8', '#a78bfa', '#22c55e', '#f59e0b', '#ec4899', '#06b6d4'];
                const container = document.createElement('div');
                container.style.cssText = 'position:fixed;inset:0;pointer-events:none;z-index:9999;overflow:hidden';
                document.body.appendChild(container);
                for (let i = 0; i < 80; i++) {
                    const p = document.createElement('div');
                    const size = Math.random() * 8 + 4;
                    const x = Math.random() * 100;
                    const color = colors[Math.floor(Math.random() * colors.length)];
                    const delay = Math.random() * 0.3;
                    const drift = (Math.random() - 0.5) * 200;
                    const shape = Math.random() > 0.5 ? '50%' : '0';
                    p.style.cssText = `position:absolute;top:-10px;left:${x}%;width:${size}px;height:${size}px;background:${color};border-radius:${shape};opacity:0.9;animation:confettiFall ${1.5 + Math.random()}s ease-out ${delay}s forwards`;
                    p.style.setProperty('--drift', drift + 'px');
                    container.appendChild(p);
                }
                setTimeout(() => container.remove(), 3000);
            }

            async function loadBalance() {
                try {
                    const res = await fetch('/api/credits/balance', { headers: { 'Authorization': 'Bearer ' + token } });
                    if (!res.ok) throw new Error();
                    const data = await res.json();
                    document.getElementById('wallet-balance').textContent = data.credit_balance.toLocaleString();
                    // Update nav credits too
                    const navCredits = document.getElementById('nav-credits');
                    if (navCredits) navCredits.textContent = data.credit_balance.toLocaleString();
                } catch(e) {
                    document.getElementById('wallet-balance').textContent = '0';
                }
            }

            async function loadHistory() {
                try {
                    const res = await fetch('/api/credits/history?page=' + txPage, { headers: { 'Authorization': 'Bearer ' + token } });
                    if (!res.ok) throw new Error();
                    const data = await res.json();
                    allTx = data.transactions || [];

                    // Compute stats
                    let spent = 0, earned = 0;
                    allTx.forEach(t => {
                        if (t.type === 'purchase') spent += Math.abs(t.amount);
                        if (t.type === 'earning') earned += t.amount;
                    });
                    document.getElementById('wallet-spent').textContent = spent.toLocaleString();
                    document.getElementById('wallet-earned').textContent = earned.toLocaleString();

                    renderTx();
                } catch(e) {
                    document.getElementById('tx-list').innerHTML = '<p class="text-center text-zinc-600 py-12 text-sm">Failed to load history.</p>';
                    document.getElementById('wallet-spent').textContent = '0';
                    document.getElementById('wallet-earned').textContent = '0';
                }
            }

            function filterTx(type) {
                txFilter = type;
                // Update button styles
                ['all','topup','purchase','earning','referral'].forEach(t => {
                    const btn = document.getElementById('tx-filter-' + t);
                    if (!btn) return;
                    if (t === type) {
                        btn.className = 'px-2.5 py-1 rounded-md bg-accent text-white';
                    } else {
                        btn.className = 'px-2.5 py-1 rounded-md text-zinc-400 hover:bg-zinc-800';
                    }
                });
                renderTx();
            }

            function renderTx() {
                const filtered = txFilter === 'all' ? allTx : allTx.filter(t => t.type === txFilter);
                const el = document.getElementById('tx-list');

                if (!filtered.length) {
                    el.innerHTML = `
                        <div class="text-center py-12">
                            <i class="ph ph-receipt text-3xl text-zinc-700 mb-2"></i>
                            <p class="text-zinc-600 text-sm">No transactions found.</p>
                        </div>`;
                    return;
                }

                el.innerHTML = filtered.map((t, i) => {
                    const isPositive = t.amount > 0;
                    const icon = t.type === 'topup' ? 'ph-plus-circle' : t.type === 'earning' ? 'ph-arrow-down-left' : t.type === 'referral' ? 'ph-users' : 'ph-shopping-cart';
                    const iconColor = t.type === 'topup' ? 'text-green-400' : t.type === 'earning' ? 'text-blue-400' : t.type === 'referral' ? 'text-purple-400' : 'text-amber-400';
                    const label = t.type === 'topup' ? 'Credit Top-up' : t.type === 'earning' ? 'Creator Earning' : t.type === 'referral' ? 'Referral Reward' : 'Asset Purchase';
                    const amountColor = isPositive ? 'text-green-400' : 'text-red-400';
                    const sign = isPositive ? '+' : '';
                    const date = fmtDate(t.created_at);
                    const border = i > 0 ? 'border-t border-zinc-800/50' : '';

                    return `
                        <div class="flex items-center gap-4 px-5 py-3.5 ${border} hover:bg-white/[0.02] transition-colors">
                            <div class="w-8 h-8 rounded-full bg-zinc-800/80 flex items-center justify-center flex-shrink-0">
                                <i class="ph ${icon} text-base ${iconColor}"></i>
                            </div>
                            <div class="flex-1 min-w-0">
                                <p class="text-sm text-zinc-200">${label}</p>
                                <p class="text-xs text-zinc-600">${date}</p>
                            </div>
                            <span class="text-sm font-semibold ${amountColor} tabular-nums">${sign}${Math.abs(t.amount).toLocaleString()}</span>
                        </div>`;
                }).join('');
            }

            async function buyCredits(amount) {
                if (!token) { window.location.href = '/login'; return; }
                if (!amount || amount < 50) {
                    showError('Minimum purchase is 50 credits ($5.00).');
                    return;
                }

                // Disable all buy buttons
                document.querySelectorAll('[data-buy-btn]').forEach(b => { b.disabled = true; b.style.opacity = '0.5'; });

                try {
                    const res = await fetch('/api/credits/topup', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({ amount })
                    });
                    const data = await res.json();
                    if (!res.ok) throw new Error(data.error || 'Failed to create checkout session');
                    // Redirect to Stripe
                    window.location.href = data.checkout_url;
                } catch(e) {
                    showError(e.message);
                    document.querySelectorAll('[data-buy-btn]').forEach(b => { b.disabled = false; b.style.opacity = '1'; });
                }
            }

            async function loadReferralStats() {
                try {
                    const res = await fetch('/api/credits/referral-stats', { headers: { 'Authorization': 'Bearer ' + token } });
                    if (!res.ok) return;
                    const data = await res.json();
                    document.getElementById('referral-section').classList.remove('hidden');
                    document.getElementById('referral-link').value = window.location.origin + '/register?ref=' + data.referral_code;
                    document.getElementById('referral-count').textContent = data.referral_count.toLocaleString();
                    document.getElementById('referral-earned').textContent = data.total_earned.toLocaleString();
                } catch(e) {}
            }

            function copyReferralLink() {
                const input = document.getElementById('referral-link');
                navigator.clipboard.writeText(input.value).then(() => {
                    const btn = document.getElementById('copy-btn');
                    btn.innerHTML = '<i class="ph ph-check"></i> Copied!';
                    setTimeout(() => { btn.innerHTML = '<i class="ph ph-copy"></i> Copy'; }, 2000);
                });
            }

            function showError(msg) {
                const el = document.getElementById('wallet-error');
                document.getElementById('wallet-error-text').textContent = msg;
                el.classList.remove('hidden');
                setTimeout(() => el.classList.add('hidden'), 6000);
            }
            "##
        </script>
        <style>
            r#"
            @keyframes confettiFall {
                0% { transform: translateY(0) translateX(0) rotate(0deg); opacity: 1; }
                100% { transform: translateY(100vh) translateX(var(--drift, 0px)) rotate(720deg); opacity: 0; }
            }
            "#
        </style>
    }
}

#[component]
fn TierCard(
    credits: u32,
    price: &'static str,
    tag: &'static str,
    color: &'static str,
) -> impl IntoView {
    let has_tag = !tag.is_empty();
    let is_accent = color == "accent";

    let border_class = if is_accent {
        "border-accent/40 hover:border-accent"
    } else {
        "border-zinc-800 hover:border-zinc-600"
    };

    let tag_view = if has_tag {
        view! {
            <span class="absolute -top-2.5 left-1/2 -translate-x-1/2 px-2.5 py-0.5 rounded-full text-[10px] font-semibold bg-accent text-white tracking-wide">{tag}</span>
        }.into_any()
    } else {
        view! { <span></span> }.into_any()
    };

    view! {
        <button data-buy-btn onclick={format!("buyCredits({})", credits)}
            class={format!("relative flex flex-col items-center gap-2 p-6 bg-surface-card rounded-xl border {} hover:bg-white/[0.02] transition-all cursor-pointer group", border_class)}>
            {tag_view}
            <span class="text-2xl font-bold text-zinc-50 group-hover:text-white transition-colors">{credits.to_string()}</span>
            <span class="text-xs text-zinc-500">"credits"</span>
            <span class="mt-1 text-sm font-semibold text-accent">{price}</span>
        </button>
    }
}
