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
        <section class="py-20 px-6">
            <div class="max-w-[1200px] mx-auto">
                <h1 class="text-4xl font-bold">"Marketplace"</h1>
                <p class="text-zinc-400 mt-2 mb-8">"Discover plugins, themes, and assets for your Renzora projects."</p>

                <div class="mb-8">
                    <input
                        type="text"
                        placeholder="Search assets..."
                        class="w-full max-w-md px-4 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors"
                        prop:value=search
                        on:input=move |ev| set_search.set(event_target_value(&ev))
                    />
                    <div class="flex gap-2 mt-4">
                        {categories.into_iter().map(|(key, label)| {
                            let key = key.to_string();
                            let key_clone = key.clone();
                            view! {
                                <button
                                    class=move || {
                                        if category.get() == key {
                                            "px-4 py-1.5 rounded-lg text-xs font-medium bg-accent border border-accent text-white transition-all"
                                        } else {
                                            "px-4 py-1.5 rounded-lg text-xs font-medium bg-transparent border border-zinc-800 text-zinc-400 hover:bg-accent hover:border-accent hover:text-white transition-all"
                                        }
                                    }
                                    on:click=move |_| set_category.set(key_clone.clone())
                                >
                                    {label}
                                </button>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                <p class="text-center text-zinc-500 py-20 text-sm">"Marketplace launching soon. Be the first to publish!"</p>
            </div>
        </section>
    }
}
