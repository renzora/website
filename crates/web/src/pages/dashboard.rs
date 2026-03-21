use leptos::prelude::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <section class="py-12 px-6 min-h-[80vh]">
            <div class="max-w-[1200px] mx-auto">
                <div class="flex items-center justify-between mb-10">
                    <div>
                        <h1 class="text-3xl font-bold">"Creator Dashboard"</h1>
                        <p class="text-zinc-500 mt-1 text-sm">"Manage your assets and track your earnings."</p>
                    </div>
                    <a href="/marketplace/sell" class="inline-flex items-center gap-2 px-4 py-2 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)]">
                        <i class="ph ph-upload-simple text-base"></i>"Upload New"
                    </a>
                </div>

                <div id="dashboard-loading" class="text-center py-20">
                    <div class="inline-block animate-spin w-6 h-6 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                </div>

                <div id="dashboard-content" class="hidden">
                    // Stats cards
                    <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-10">
                        <div class="relative p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl overflow-hidden group hover:border-zinc-700 transition-all">
                            <div class="absolute -top-6 -right-6 w-20 h-20 bg-accent/5 rounded-full group-hover:scale-150 transition-transform duration-500"></div>
                            <span class="text-[11px] text-zinc-500 uppercase tracking-wider font-medium">"Total Assets"</span>
                            <div class="text-3xl font-bold mt-2" id="stat-assets">"0"</div>
                        </div>
                        <div class="relative p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl overflow-hidden group hover:border-zinc-700 transition-all">
                            <div class="absolute -top-6 -right-6 w-20 h-20 bg-cyan-500/5 rounded-full group-hover:scale-150 transition-transform duration-500"></div>
                            <span class="text-[11px] text-zinc-500 uppercase tracking-wider font-medium">"Total Downloads"</span>
                            <div class="text-3xl font-bold mt-2" id="stat-downloads">"0"</div>
                        </div>
                        <div class="relative p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl overflow-hidden group hover:border-zinc-700 transition-all">
                            <div class="absolute -top-6 -right-6 w-20 h-20 bg-green-500/5 rounded-full group-hover:scale-150 transition-transform duration-500"></div>
                            <span class="text-[11px] text-zinc-500 uppercase tracking-wider font-medium">"Total Earnings"</span>
                            <div class="text-3xl font-bold mt-2"><span id="stat-earnings">"0"</span><span class="text-base font-normal text-zinc-500">" cr"</span></div>
                        </div>
                        <div class="relative p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl overflow-hidden group hover:border-zinc-700 transition-all">
                            <div class="absolute -top-6 -right-6 w-20 h-20 bg-purple-500/5 rounded-full group-hover:scale-150 transition-transform duration-500"></div>
                            <span class="text-[11px] text-zinc-500 uppercase tracking-wider font-medium">"Balance"</span>
                            <div class="text-3xl font-bold mt-2"><span id="stat-balance">"0"</span><span class="text-base font-normal text-zinc-500">" cr"</span></div>
                        </div>
                    </div>

                    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                        // Assets list (2/3 width)
                        <div class="lg:col-span-2 p-6 bg-white/[0.02] border border-zinc-800/50 rounded-xl">
                            <div class="flex justify-between items-center mb-5">
                                <h2 class="text-base font-semibold flex items-center gap-2">
                                    <i class="ph ph-package text-accent"></i>"Your Assets"
                                </h2>
                            </div>
                            <div id="assets-list">
                                <p class="text-center text-zinc-600 py-10 text-sm">"No assets uploaded yet."</p>
                            </div>
                        </div>

                        // Earnings sidebar (1/3 width)
                        <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-xl">
                            <h2 class="text-base font-semibold mb-5 flex items-center gap-2">
                                <i class="ph ph-chart-line-up text-green-400"></i>"Recent Earnings"
                            </h2>
                            <div id="earnings-list">
                                <p class="text-center text-zinc-600 py-10 text-sm">"No earnings yet."</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <script>
            r#"
            function parseDate(s) {
                if (!s) return null;
                const iso = s.replace(/^(\d{4}-\d{2}-\d{2})\s+(\d{2}:\d{2}:\d{2}).*?\s+([+-]\d{2}):(\d{2}).*$/, '$1T$2$3:$4');
                const d = new Date(iso);
                return isNaN(d.getTime()) ? null : d;
            }
            function fmtDate(s) {
                const d = parseDate(s);
                return d ? d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' }) : '';
            }

            (async function() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) { window.location.href = '/login'; return; }

                const loading = document.getElementById('dashboard-loading');
                const content = document.getElementById('dashboard-content');

                try {
                    const headers = { 'Authorization': 'Bearer ' + token };
                    const [statsRes, earningsRes, assetsRes] = await Promise.all([
                        fetch('/api/creator/stats', { headers }),
                        fetch('/api/creator/earnings', { headers }),
                        fetch('/api/marketplace/my-assets', { headers }),
                    ]);

                    if (statsRes.ok) {
                        const stats = await statsRes.json();
                        animateCounter('stat-assets', stats.total_assets);
                        animateCounter('stat-downloads', stats.total_downloads);
                        animateCounter('stat-earnings', stats.total_earnings);
                        animateCounter('stat-balance', stats.credit_balance);
                    }

                    if (assetsRes.ok) {
                        const data = await assetsRes.json();
                        const el = document.getElementById('assets-list');
                        if (data.assets && data.assets.length > 0) {
                            el.innerHTML = `<div class="space-y-2">${data.assets.map((a, i) => `
                                <a href="/marketplace/asset/${a.slug}" class="flex items-center gap-4 p-3 rounded-lg hover:bg-white/[0.03] transition-all group" style="animation: fadeSlideUp 0.3s ease both; animation-delay: ${i * 40}ms">
                                    <div class="w-11 h-11 rounded-lg bg-surface-panel border border-zinc-800/50 flex items-center justify-center shrink-0 overflow-hidden">
                                        ${a.thumbnail_url
                                            ? `<img src="${a.thumbnail_url}" class="w-full h-full object-cover" />`
                                            : `<i class="ph ph-package text-lg text-zinc-600"></i>`}
                                    </div>
                                    <div class="flex-1 min-w-0">
                                        <div class="text-sm font-medium group-hover:text-accent transition-colors truncate">${a.name}</div>
                                        <div class="flex items-center gap-2 mt-0.5 text-[11px] text-zinc-600">
                                            <span>${a.category}</span>
                                            <span>·</span>
                                            <span>${a.downloads.toLocaleString()} downloads</span>
                                        </div>
                                    </div>
                                    <div class="text-right shrink-0">
                                        <span class="text-xs font-medium">${a.price_credits === 0 ? 'Free' : a.price_credits.toLocaleString() + ' cr'}</span>
                                        <div class="mt-0.5">
                                            <span class="text-[10px] px-1.5 py-0.5 rounded-full ${a.published ? 'bg-green-500/10 text-green-400' : 'bg-amber-500/10 text-amber-400'}">${a.published ? 'Live' : 'Draft'}</span>
                                        </div>
                                    </div>
                                </a>
                            `).join('')}</div>`;
                        }
                    }

                    if (earningsRes.ok) {
                        const data = await earningsRes.json();
                        const el = document.getElementById('earnings-list');
                        if (data.earnings && data.earnings.length > 0) {
                            el.innerHTML = `<div class="space-y-1">${data.earnings.map((e, i) => `
                                <div class="flex items-center justify-between py-2.5 px-2 rounded-lg hover:bg-white/[0.02] transition-all" style="animation: fadeSlideUp 0.3s ease both; animation-delay: ${i * 40}ms">
                                    <span class="text-xs text-zinc-400 truncate mr-3">${e.asset_name}</span>
                                    <div class="text-right shrink-0">
                                        <span class="text-xs font-semibold text-green-400">+${e.amount.toLocaleString()}</span>
                                        <div class="text-[10px] text-zinc-600">${fmtDate(e.created_at)}</div>
                                    </div>
                                </div>
                            `).join('')}</div>`;
                        }
                    }
                } catch(e) { console.error(e); }

                loading.classList.add('hidden');
                content.classList.remove('hidden');
            })();

            function animateCounter(id, target) {
                const el = document.getElementById(id);
                if (!el || target === 0) { el.textContent = '0'; return; }
                let current = 0;
                const step = Math.max(1, Math.floor(target / 30));
                const timer = setInterval(() => {
                    current += step;
                    if (current >= target) { current = target; clearInterval(timer); }
                    el.textContent = current.toLocaleString();
                }, 25);
            }
            "#
        </script>

        <style>
            r#"
            @keyframes fadeSlideUp {
                from { opacity: 0; transform: translateY(12px); }
                to { opacity: 1; transform: translateY(0); }
            }
            "#
        </style>
    }
}
