use leptos::prelude::*;
use leptos_meta::{Link, Meta, Title};
use renzora_common::ssr::PostSsr;

/// JSON-escape a string for safe embedding in a JSON-LD literal.
fn json_escape(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    o.push('"');
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\r' => o.push_str("\\r"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

/// `/community/post/:id` — a single public discussion, server-rendered so it's
/// its own indexable page (post body + comments + DiscussionForumPosting JSON-LD).
/// Replies/likes are handled by the client JS when signed in.
#[component]
pub fn PostDetailPage() -> impl IntoView {
    let ssr = use_context::<PostSsr>().filter(|p| p.found);
    let has_ssr = ssr.is_some();

    let head = ssr.clone().map(|p| {
        let snippet: String = p.body.chars().take(70).collect();
        let more = p.body.chars().count() > 70;
        let title = format!("{}{} — Renzora Community", snippet, if more { "…" } else { "" });
        let d: String = p.body.chars().take(155).collect();
        let desc = if p.body.chars().count() > 155 { format!("{d}…") } else { d };
        let canonical = format!("https://renzora.com/community/post/{}", p.id);
        let ld = format!(
            "{{\"@context\":\"https://schema.org\",\"@type\":\"DiscussionForumPosting\",\"headline\":{},\"articleBody\":{},\"author\":{{\"@type\":\"Person\",\"name\":{}}},\"datePublished\":\"{}\",\"url\":{},\"interactionStatistic\":[{{\"@type\":\"InteractionCounter\",\"interactionType\":\"https://schema.org/LikeAction\",\"userInteractionCount\":{}}},{{\"@type\":\"InteractionCounter\",\"interactionType\":\"https://schema.org/CommentAction\",\"userInteractionCount\":{}}}]}}",
            json_escape(&snippet), json_escape(&p.body), json_escape(&p.username), p.created_at, json_escape(&canonical), p.like_count, p.comment_count
        );
        view! {
            <Title text=title />
            <Meta name="description" content=desc />
            <Link rel="canonical" href=canonical />
            <script type="application/ld+json" inner_html=ld></script>
        }
    });

    let content = ssr.map(|p| {
        let channel_name = p.channel_name.clone().unwrap_or_default();
        let channel_crumb = p.channel_slug.clone().map(|s| {
            view! {
                " / "<a href=format!("/community/channel/{s}") class="hover:text-accent">{channel_name.clone()}</a>
            }
        });
        let comments = p.comments.iter().map(|c| {
            let (u, b, d) = (c.username.clone(), c.body.clone(), c.created_at.clone());
            view! {
                <div class="py-3 border-t border-white/[0.06] first:border-t-0">
                    <div class="flex items-center gap-2 text-xs mb-1">
                        <span class="font-semibold text-zinc-200">{u}</span>
                        <span class="text-zinc-600">{d}</span>
                    </div>
                    <p class="text-sm text-zinc-300 whitespace-pre-wrap break-words">{b}</p>
                </div>
            }
        }).collect_view();
        view! {
            <div class="max-w-[720px] mx-auto">
                <nav class="text-xs text-zinc-500 mb-4">
                    <a href="/community" class="hover:text-accent">"Community"</a>
                    {channel_crumb}
                </nav>
                <article class="rounded-2xl border border-white/[0.08] bg-white/[0.02] p-6">
                    <div class="flex items-center gap-2 mb-3">
                        <a href=format!("/profile/{}", p.username) class="text-sm font-semibold text-white hover:text-accent">{p.username.clone()}</a>
                        <span class="text-zinc-600 text-xs">"·"</span>
                        <span class="text-xs text-zinc-500">{p.created_at.clone()}</span>
                    </div>
                    <div class="text-zinc-100 whitespace-pre-wrap break-words leading-relaxed">{p.body.clone()}</div>
                    <div class="flex items-center gap-4 mt-4 text-xs text-zinc-500">
                        <span class="inline-flex items-center gap-1"><i class="ph ph-heart"></i>{p.like_count}</span>
                        <span class="inline-flex items-center gap-1"><i class="ph ph-chat-circle"></i>{p.comment_count}</span>
                    </div>
                </article>
                <h2 class="text-sm font-semibold text-white mt-6 mb-2">{p.comment_count}" comments"</h2>
                <div class="rounded-2xl border border-white/[0.08] bg-white/[0.02] px-5 py-1">
                    {comments}
                </div>
                <div id="pd-reply" class="mt-4"></div>
            </div>
        }
    });

    view! {
        {head}
        <section class="py-8 px-4 min-h-[70vh]">
            {content}
            {(!has_ssr).then(|| view! {
                <p class="text-center text-zinc-500 py-20">"Discussion not found."</p>
            })}
        </section>
        <script>
            r#"
            (function(){
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                var id = location.pathname.split('/').filter(Boolean).pop();
                var box = document.getElementById('pd-reply');
                if (!box) return;
                if (token) {
                    box.innerHTML = '<div class="rounded-2xl border border-white/[0.08] bg-white/[0.02] p-4"><textarea id="pd-input" rows="3" placeholder="Add a comment…" class="w-full bg-transparent text-sm text-zinc-100 placeholder-zinc-600 outline-none resize-none"></textarea><div class="flex justify-end mt-2"><button id="pd-send" class="px-4 py-1.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">Reply</button></div></div>';
                    document.getElementById('pd-send').addEventListener('click', async function(){
                        var v = document.getElementById('pd-input').value.trim();
                        if (!v) return;
                        try {
                            var r = await fetch('/api/feed/posts/'+id+'/comments', { method:'POST', headers:{'Authorization':'Bearer '+token,'Content-Type':'application/json'}, body: JSON.stringify({ body: v }) });
                            if (r.ok) location.reload();
                        } catch(e){}
                    });
                } else {
                    box.innerHTML = '<a href="/login?redirect='+encodeURIComponent(location.pathname)+'" class="block text-center rounded-2xl border border-white/[0.08] bg-white/[0.02] p-4 text-sm text-zinc-400 hover:text-white transition-colors">Sign in to join the discussion</a>';
                }
            })();
            "#
        </script>
    }
}
