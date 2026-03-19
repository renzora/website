use leptos::prelude::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <section class="py-20 px-6">
            <div class="max-w-[1200px] mx-auto">
                <h1 class="text-4xl font-bold">"Creator Dashboard"</h1>
                <p class="text-zinc-400 mt-2 mb-8">"Manage your assets and track your earnings."</p>

                <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-12">
                    <StatCard label="Total Assets" value="0" />
                    <StatCard label="Total Downloads" value="0" />
                    <StatCard label="Total Earnings" value="0" suffix=" credits" />
                    <StatCard label="Balance" value="0" suffix=" credits" />
                </div>

                <div class="space-y-8">
                    <div class="p-6 bg-surface-card border border-zinc-800 rounded-lg">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-lg font-semibold">"Your Assets"</h2>
                            <a href="/marketplace/upload" class="inline-flex items-center justify-center px-4 py-2 rounded-lg text-xs font-medium bg-surface border border-zinc-800 text-zinc-50 hover:border-zinc-600 transition-colors">"Upload New"</a>
                        </div>
                        <p class="text-center text-zinc-500 py-8 text-sm">"No assets uploaded yet."</p>
                    </div>

                    <div class="p-6 bg-surface-card border border-zinc-800 rounded-lg">
                        <h2 class="text-lg font-semibold mb-4">"Recent Earnings"</h2>
                        <p class="text-center text-zinc-500 py-8 text-sm">"No earnings yet."</p>
                    </div>

                    <div class="p-6 bg-surface-card border border-zinc-800 rounded-lg">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-lg font-semibold">"Your Articles"</h2>
                            <a href="/community/write" class="inline-flex items-center justify-center px-4 py-2 rounded-lg text-xs font-medium bg-surface border border-zinc-800 text-zinc-50 hover:border-zinc-600 transition-colors">"Write New"</a>
                        </div>
                        <p class="text-center text-zinc-500 py-8 text-sm">"No articles written yet."</p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn StatCard(label: &'static str, value: &'static str, #[prop(default = "")] suffix: &'static str) -> impl IntoView {
    view! {
        <div class="p-5 bg-surface-card border border-zinc-800 rounded-lg">
            <span class="text-xs text-zinc-500 uppercase tracking-wider">{label}</span>
            <div class="text-2xl font-bold mt-1">{value}{suffix}</div>
        </div>
    }
}
