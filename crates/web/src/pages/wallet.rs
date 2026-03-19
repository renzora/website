use leptos::prelude::*;

#[component]
pub fn WalletPage() -> impl IntoView {
    let credit_tiers = vec![
        (500, "$5.00"),
        (1000, "$10.00"),
        (2500, "$25.00"),
        (5000, "$50.00"),
    ];

    view! {
        <section class="py-20 px-6">
            <div class="max-w-[1200px] mx-auto">
                <h1 class="text-4xl font-bold">"Wallet"</h1>
                <p class="text-zinc-400 mt-2 mb-8">"Manage your Renzora credits."</p>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-5 mb-12">
                    <div class="p-8 bg-surface-card border border-zinc-800 rounded-lg">
                        <h2 class="text-xs font-medium text-zinc-500 uppercase tracking-wider mb-2">"Credit Balance"</h2>
                        <div class="text-5xl font-extrabold tracking-tight gradient-text">"0"</div>
                        <p class="text-xs text-zinc-500 mt-2">"1 credit = $0.01 USD"</p>
                    </div>

                    <div class="p-8 bg-surface-card border border-zinc-800 rounded-lg">
                        <h3 class="text-base font-semibold mb-4">"Add Credits"</h3>
                        <div class="grid grid-cols-2 gap-2.5">
                            {credit_tiers.into_iter().map(|(credits, price)| {
                                view! {
                                    <button class="flex flex-col items-center gap-1 p-4 bg-surface rounded-lg border border-zinc-800 hover:border-accent hover:bg-accent-subtle transition-all cursor-pointer">
                                        <span class="text-sm font-semibold">{credits} " credits"</span>
                                        <span class="text-xs text-zinc-400">{price}</span>
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </div>

                <div>
                    <h3 class="text-lg font-semibold mb-4">"Transaction History"</h3>
                    <div class="p-6 bg-surface-card border border-zinc-800 rounded-lg">
                        <p class="text-center text-zinc-500 py-8 text-sm">"No transactions yet."</p>
                    </div>
                </div>
            </div>
        </section>
    }
}
