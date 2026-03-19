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
        <section class="wallet-page">
            <div class="container">
                <h1>"Wallet"</h1>
                <p class="wallet-intro">"Manage your Renzora credits."</p>

                <div class="wallet-grid">
                    <div class="wallet-balance-card">
                        <h2>"Credit Balance"</h2>
                        <div class="balance-amount">"0"</div>
                        <p class="balance-note">"1 credit = $0.01 USD"</p>
                    </div>

                    <div class="wallet-topup-card">
                        <h3>"Add Credits"</h3>
                        <div class="topup-tiers">
                            {credit_tiers.into_iter().map(|(credits, price)| {
                                view! {
                                    <button class="topup-btn">
                                        <span class="topup-credits">{credits} " credits"</span>
                                        <span class="topup-price">{price}</span>
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </div>

                <div class="transaction-history">
                    <h3>"Transaction History"</h3>
                    <div class="transaction-list">
                        <p class="empty-state">"No transactions yet."</p>
                    </div>
                </div>
            </div>
        </section>
    }
}
