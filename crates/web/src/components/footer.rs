use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-zinc-800 pt-12 pb-6 mt-20">
            <div class="max-w-[1200px] mx-auto px-6">
                <div class="flex flex-col md:flex-row justify-between gap-8">
                    <div>
                        <span class="text-lg font-bold">"Renzora"</span>
                        <p class="text-zinc-500 text-sm mt-1">"A modern game engine built with Rust."</p>
                    </div>
                    <div class="flex gap-16">
                        <div class="flex flex-col gap-2">
                            <h4 class="text-xs font-semibold uppercase tracking-wider text-zinc-500 mb-1">"Product"</h4>
                            <a href="/download" class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors">"Download"</a>
                            <a href="/docs" class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors">"Documentation"</a>
                            <a href="/marketplace" class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors">"Marketplace"</a>
                        </div>
                        <div class="flex flex-col gap-2">
                            <h4 class="text-xs font-semibold uppercase tracking-wider text-zinc-500 mb-1">"Community"</h4>
                            <a href="https://github.com/renzora" class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors">"GitHub"</a>
                            <a href="https://discord.gg/renzora" class="text-sm text-zinc-400 hover:text-zinc-50 transition-colors">"Discord"</a>
                        </div>
                    </div>
                </div>
                <div class="mt-12 pt-6 border-t border-zinc-800">
                    <p class="text-xs text-zinc-500">"© 2026 Renzora. All rights reserved."</p>
                </div>
            </div>
        </footer>
    }
}
