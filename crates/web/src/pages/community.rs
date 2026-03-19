use leptos::prelude::*;

#[component]
pub fn CommunityPage() -> impl IntoView {
    let tags = vec!["all", "tutorial", "guide", "tip", "showcase", "resource"];
    let (active_tag, set_active_tag) = signal("all".to_string());

    view! {
        <section class="py-20 px-6">
            <div class="max-w-[1200px] mx-auto">
                <div class="flex justify-between items-start mb-8">
                    <div>
                        <h1 class="text-4xl font-bold">"Community"</h1>
                        <p class="text-zinc-400 mt-2">"Tutorials, guides, and resources from the Renzora community."</p>
                    </div>
                    <a href="/community/write" class="inline-flex items-center justify-center px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">"Write an Article"</a>
                </div>

                <div class="flex gap-2 flex-wrap mb-8">
                    {tags.into_iter().map(|tag| {
                        let tag_str = tag.to_string();
                        let tag_clone = tag_str.clone();
                        let label = if tag == "all" { "All".to_string() } else {
                            let mut c = tag.chars();
                            match c.next() {
                                None => String::new(),
                                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                            }
                        };
                        view! {
                            <button
                                class=move || {
                                    if active_tag.get() == tag_str {
                                        "px-3.5 py-1.5 rounded-full text-xs font-medium bg-accent border border-accent text-white transition-all"
                                    } else {
                                        "px-3.5 py-1.5 rounded-full text-xs font-medium bg-transparent border border-zinc-800 text-zinc-400 hover:bg-accent hover:border-accent hover:text-white transition-all"
                                    }
                                }
                                on:click=move |_| set_active_tag.set(tag_clone.clone())
                            >
                                {label}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>

                <p class="text-center text-zinc-500 py-20 text-sm">"No articles yet. Be the first to share your knowledge!"</p>
            </div>
        </section>
    }
}
