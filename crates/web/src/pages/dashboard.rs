use leptos::prelude::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <section class="dashboard-page">
            <div class="container">
                <h1>"Creator Dashboard"</h1>
                <p class="dashboard-intro">"Manage your assets and track your earnings."</p>

                <div class="stats-grid">
                    <StatCard label="Total Assets" value="0" />
                    <StatCard label="Total Downloads" value="0" />
                    <StatCard label="Total Earnings" value="0" suffix=" credits" />
                    <StatCard label="Balance" value="0" suffix=" credits" />
                </div>

                <div class="dashboard-sections">
                    <div class="dashboard-section">
                        <div class="section-header">
                            <h2>"Your Assets"</h2>
                            <a href="/marketplace/upload" class="btn btn-secondary">"Upload New"</a>
                        </div>
                        <div class="assets-table">
                            <p class="empty-state">"No assets uploaded yet."</p>
                        </div>
                    </div>

                    <div class="dashboard-section">
                        <div class="section-header">
                            <h2>"Recent Earnings"</h2>
                        </div>
                        <div class="earnings-table">
                            <p class="empty-state">"No earnings yet."</p>
                        </div>
                    </div>

                    <div class="dashboard-section">
                        <div class="section-header">
                            <h2>"Your Articles"</h2>
                            <a href="/community/write" class="btn btn-secondary">"Write New"</a>
                        </div>
                        <div class="articles-table">
                            <p class="empty-state">"No articles written yet."</p>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn StatCard(label: &'static str, value: &'static str, #[prop(default = "")] suffix: &'static str) -> impl IntoView {
    view! {
        <div class="stat-card">
            <span class="stat-label">{label}</span>
            <span class="stat-value">{value}{suffix}</span>
        </div>
    }
}
