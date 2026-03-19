use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="sticky top-0 z-50 bg-[rgba(10,10,11,0.8)] backdrop-blur-xl border-b border-zinc-800">
            <div class="max-w-[1200px] mx-auto px-6 h-14 flex items-center gap-8">
                <a href="/" class="text-lg font-bold tracking-tight">"Renzora"</a>
                <div class="flex gap-6 flex-1">
                    <a href="/docs" class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors flex items-center gap-1.5">
                        <i class="ph ph-book-open text-base"></i>"Docs"
                    </a>
                    <a href="/marketplace" class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors flex items-center gap-1.5">
                        <i class="ph ph-storefront text-base"></i>"Marketplace"
                    </a>
                    <a href="/community" class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors flex items-center gap-1.5">
                        <i class="ph ph-users-three text-base"></i>"Community"
                    </a>
                </div>
                <div class="flex gap-2">
                    <a href="/dashboard" class="text-sm text-zinc-400 hover:text-zinc-50 hover:bg-surface-card px-3 py-1.5 rounded-lg transition-all flex items-center gap-1.5">
                        <i class="ph ph-chart-bar text-base"></i>"Dashboard"
                    </a>
                    <a href="/login" class="text-sm text-zinc-400 hover:text-zinc-50 hover:bg-surface-card px-3 py-1.5 rounded-lg transition-all flex items-center gap-1.5">
                        <i class="ph ph-user text-base"></i>"Sign In"
                    </a>
                </div>
            </div>
        </nav>
    }
}
