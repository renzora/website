use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-zinc-800 py-4 mt-20">
            <div class="max-w-[1200px] mx-auto px-6 flex items-center justify-between">
                <p class="text-xs text-zinc-500">"© 2026 Renzora. All rights reserved."</p>
                <div class="flex items-center gap-5">
                    <a href="/terms" class="text-xs text-zinc-600 hover:text-zinc-100 transition-colors">"Terms"</a>
                    <a href="/privacy" class="text-xs text-zinc-600 hover:text-zinc-100 transition-colors">"Privacy"</a>
                    <a href="https://github.com/renzora" target="_blank" rel="noopener noreferrer" class="flex items-center gap-1.5 text-xs text-zinc-600 hover:text-zinc-100 transition-colors">
                        <i class="ph ph-github-logo text-sm"></i>"GitHub"
                    </a>
                    <a href="https://discord.gg/9UHUGUyDJv" target="_blank" rel="noopener noreferrer" class="flex items-center gap-1.5 text-xs text-zinc-600 hover:text-[#5865F2] transition-colors">
                        <i class="ph ph-discord-logo text-sm"></i>"Discord"
                    </a>
                    <a href="https://youtube.com/@renzoragame" target="_blank" rel="noopener noreferrer" class="flex items-center gap-1.5 text-xs text-zinc-600 hover:text-[#FF0000] transition-colors">
                        <i class="ph ph-youtube-logo text-sm"></i>"YouTube"
                    </a>
                </div>
            </div>
        </footer>
    }
}
