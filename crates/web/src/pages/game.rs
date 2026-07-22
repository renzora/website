use leptos::prelude::*;
use leptos_meta::{Meta, Title};

#[component]
pub fn GamePage() -> impl IntoView {
    view! {
        <Title text="Renzora, Coming Soon" />
        <Meta name="description" content="Renzora, a cozy, cryptic mystery. Hazel wakes in a quiet forest with no memory of how she got there, and makes a home in a little valley that is not quite what it seems. Join the waiting list." />

        // ── Hero ──
        <section class="relative overflow-hidden px-4 py-10 min-h-[calc(100vh-60px)] flex flex-col justify-center">
            <canvas id="fireflies" class="absolute inset-0 w-full h-full pointer-events-none"></canvas>
            <div class="absolute inset-0 bg-gradient-to-b from-fuchsia-500/[0.05] via-purple-600/[0.03] to-transparent pointer-events-none"></div>
            <div class="absolute top-1/4 -left-24 w-96 h-96 bg-fuchsia-500/12 rounded-full blur-[130px] pointer-events-none"></div>
            <div class="absolute -bottom-10 -right-16 w-[28rem] h-72 bg-purple-600/10 rounded-full blur-[130px] pointer-events-none"></div>

            <div class="relative z-10 w-full max-w-5xl mx-auto">
                <div class="text-center mb-6">
                    <span class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-fuchsia-500/10 border border-fuchsia-500/25 text-fuchsia-300 text-xs font-medium">
                        <span class="w-1.5 h-1.5 rounded-full bg-fuchsia-400 animate-pulse"></span>
                        "A Renzora Studios Game"
                    </span>
                </div>
                <div id="waitlist" class="scroll-mt-20 grid md:grid-cols-2 items-stretch rounded-3xl overflow-hidden border border-white/[0.08] bg-[#100a1c] shadow-2xl shadow-black/50 ring-1 ring-fuchsia-500/10">
                    <div class="order-2 md:order-1 p-8 md:p-10 flex flex-col justify-center">
                        <span class="text-[11px] font-semibold uppercase tracking-[0.22em] text-fuchsia-400">"Season 1 · Coming Soon"</span>
                        <h1 class="mt-2 text-4xl md:text-5xl font-black tracking-wide bg-gradient-to-r from-fuchsia-400 via-fuchsia-300 to-purple-400 bg-clip-text text-transparent">"RENZORA"</h1>
                        <p class="mt-4 text-sm md:text-[15px] text-zinc-300 leading-relaxed">
                            "Hazel wakes in a place she has never seen before, and yet she has lived there her entire life."
                        </p>
                        <p class="mt-3 text-sm md:text-[15px] text-zinc-300 leading-relaxed">
                            "There is far more to this place than anyone can remember, and the truth beneath it has been buried for centuries."
                        </p>
                        <p class="mt-6 text-sm md:text-base font-semibold text-zinc-100">"A new open world adventure experience with endless immersive stories, online seasonal content and collectibles."</p>
                        <form id="wl-form" onsubmit="return wlJoin(event)" class="mt-4 flex gap-2">
                            <input id="wl-email" type="email" required placeholder="your email" autocomplete="email" class="flex-1 px-4 py-3 bg-white/[0.04] border border-white/[0.1] rounded-xl text-zinc-50 text-sm outline-none focus:border-fuchsia-500/50 transition-all placeholder:text-zinc-500" />
                            <button type="submit" id="wl-btn" class="px-5 py-3 rounded-xl text-sm font-semibold bg-fuchsia-600 text-white hover:bg-fuchsia-500 transition-colors shrink-0 whitespace-nowrap">"Join Waiting List"</button>
                        </form>
                        <p id="wl-msg" class="hidden mt-3 text-sm"></p>
                    </div>
                    <div class="order-1 md:order-2 relative aspect-[16/11] md:aspect-auto">
                        <picture>
                            <source type="image/avif" sizes="(max-width: 768px) 100vw, 512px" srcset="/assets/previews/game-scene-640.avif 640w, /assets/previews/game-scene.avif 718w" />
                            <source type="image/webp" sizes="(max-width: 768px) 100vw, 512px" srcset="/assets/previews/game-scene-640.webp 640w, /assets/previews/game-scene.webp 718w" />
                            <img src="/assets/previews/game-scene.webp" alt="Renzora" width="718" height="572" class="w-full h-full object-cover object-center md:absolute md:inset-0" fetchpriority="high" decoding="async" />
                        </picture>
                        <div class="hidden md:block absolute inset-y-0 left-0 w-24 bg-gradient-to-r from-[#100a1c] to-transparent pointer-events-none"></div>
                    </div>
                </div>
                <p class="mt-8 text-center text-xs text-zinc-500 flex items-center justify-center gap-1.5">
                    <i class="ph ph-cube text-sm text-fuchsia-400/80"></i>
                    "Built with "
                    <a href="/" class="text-fuchsia-400 hover:text-fuchsia-300 transition-colors font-medium">"Renzora Engine"</a>
                </p>
            </div>
        </section>

        <script>
            r#"
            // ── Twinkling stars ──
            (function(){
                var canvas = document.getElementById('fireflies');
                if(!canvas) return;
                var ctx = canvas.getContext('2d');
                var w, h, stars = [];
                function resize(){
                    var r = canvas.getBoundingClientRect();
                    w = canvas.width = Math.max(1, Math.floor(r.width));
                    h = canvas.height = Math.max(1, Math.floor(r.height));
                }
                resize();
                window.addEventListener('resize', resize);
                var palette = ['255,255,255','255,236,190','236,170,255'];
                var N = Math.min(110, Math.floor(w * h / 9000));
                for(var i=0;i<N;i++){
                    stars.push({
                        x: Math.random()*w, y: Math.random()*h,
                        r: Math.random()*1.1 + 0.35,
                        phase: Math.random()*Math.PI*2,
                        speed: Math.random()*0.05 + 0.015,
                        col: palette[Math.floor(Math.random()*palette.length)],
                        flare: Math.random() < 0.3
                    });
                }
                function draw(){
                    ctx.clearRect(0,0,w,h);
                    for(var i=0;i<stars.length;i++){
                        var s = stars[i];
                        s.phase += s.speed;
                        var t = (Math.sin(s.phase) + 1) / 2;
                        var tw = t * t * t;
                        var alpha = 0.05 + tw * 0.9;
                        var rad = s.r * (0.6 + tw * 0.9);
                        var col = s.col;
                        var g = ctx.createRadialGradient(s.x, s.y, 0, s.x, s.y, rad*4);
                        g.addColorStop(0, 'rgba('+col+','+(alpha*0.5)+')');
                        g.addColorStop(1, 'rgba('+col+',0)');
                        ctx.fillStyle = g;
                        ctx.beginPath(); ctx.arc(s.x, s.y, rad*4, 0, Math.PI*2); ctx.fill();
                        ctx.fillStyle = 'rgba('+col+','+Math.min(1, alpha)+')';
                        ctx.beginPath(); ctx.arc(s.x, s.y, rad*0.85, 0, Math.PI*2); ctx.fill();
                        if(s.flare && tw > 0.45){
                            var len = rad * (3 + tw * 5);
                            ctx.strokeStyle = 'rgba('+col+','+(tw*0.45)+')';
                            ctx.lineWidth = 0.6;
                            ctx.beginPath();
                            ctx.moveTo(s.x - len, s.y); ctx.lineTo(s.x + len, s.y);
                            ctx.moveTo(s.x, s.y - len); ctx.lineTo(s.x, s.y + len);
                            ctx.stroke();
                        }
                    }
                    requestAnimationFrame(draw);
                }
                draw();
            })();

            // ── Waitlist ──
            async function wlJoin(e){
                e.preventDefault();
                var email = document.getElementById('wl-email').value.trim();
                var msg = document.getElementById('wl-msg');
                var btn = document.getElementById('wl-btn');
                msg.classList.add('hidden');
                if(!email || email.indexOf('@') < 1){ msg.textContent='Please enter a valid email'; msg.className='mt-3 text-sm text-red-400'; msg.classList.remove('hidden'); return false; }
                btn.disabled = true; btn.textContent = 'Joining...';
                try {
                    var res = await fetch('/api/waitlist/join', { method:'POST', headers:{'Content-Type':'application/json'}, body: JSON.stringify({ email: email, source: 'game' }) });
                    var d = await res.json();
                    if(res.ok && d.ok){ msg.textContent = "You're on the list. See you in the valley."; msg.className='mt-3 text-sm text-green-400'; msg.classList.remove('hidden'); document.getElementById('wl-form').reset(); }
                    else { msg.textContent = d.error || 'Something went wrong. Please try again.'; msg.className='mt-3 text-sm text-red-400'; msg.classList.remove('hidden'); }
                } catch(err) { msg.textContent = 'Something went wrong. Please try again.'; msg.className='mt-3 text-sm text-red-400'; msg.classList.remove('hidden'); }
                btn.disabled = false; btn.textContent = 'Join Waiting List';
                return false;
            }
            "#
        </script>
    }
}
