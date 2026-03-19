use leptos::prelude::*;

#[component]
pub fn MarketplacePage() -> impl IntoView {
    let (search, set_search) = signal(String::new());
    let (category, set_category) = signal("all".to_string());

    let categories = vec![
        ("all", "All"),
        ("plugin", "Plugins"),
        ("theme", "Themes"),
        ("asset", "Assets"),
    ];

    view! {
        <section class="marketplace-page">
            <div class="container">
                <h1>"Marketplace"</h1>
                <p class="marketplace-intro">
                    "Discover plugins, themes, and assets for your Renzora projects."
                </p>

                <div class="marketplace-filters">
                    <div class="search-bar">
                        <input
                            type="text"
                            placeholder="Search assets..."
                            class="search-input"
                            prop:value=search
                            on:input=move |ev| {
                                set_search.set(event_target_value(&ev));
                            }
                        />
                    </div>
                    <div class="category-tabs">
                        {categories.into_iter().map(|(key, label)| {
                            let key = key.to_string();
                            let key_clone = key.clone();
                            view! {
                                <button
                                    class=move || if category.get() == key { "tab active" } else { "tab" }
                                    on:click=move |_| set_category.set(key_clone.clone())
                                >
                                    {label}
                                </button>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                <div class="asset-grid">
                    <p class="empty-state">"Marketplace launching soon. Be the first to publish!"</p>
                </div>
            </div>
        </section>
    }
}
