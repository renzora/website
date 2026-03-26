use leptos::prelude::*;

#[component]
pub fn GameUploadPage() -> impl IntoView {
    view! {
        <section class="py-20 px-6 text-center">
            <p class="text-zinc-500 text-sm">"Redirecting..."</p>
        </section>
        <script>"window.location.replace('/marketplace/upload?type=game');"</script>
    }
}
