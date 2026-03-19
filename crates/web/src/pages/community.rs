use leptos::prelude::*;

#[component]
pub fn CommunityPage() -> impl IntoView {
    let tags = vec![
        "all", "tutorial", "guide", "tip", "showcase", "resource",
    ];
    let (active_tag, set_active_tag) = signal("all".to_string());

    view! {
        <section class="community-page">
            <div class="container">
                <div class="community-header">
                    <div>
                        <h1>"Community"</h1>
                        <p class="community-intro">
                            "Tutorials, guides, and resources from the Renzora community."
                        </p>
                    </div>
                    <a href="/community/write" class="btn btn-primary">"Write an Article"</a>
                </div>

                <div class="tag-filters">
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
                                class=move || if active_tag.get() == tag_str { "tag active" } else { "tag" }
                                on:click=move |_| set_active_tag.set(tag_clone.clone())
                            >
                                {label}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>

                <div class="article-grid">
                    <p class="empty-state">"No articles yet. Be the first to share your knowledge!"</p>
                </div>
            </div>
        </section>
    }
}
