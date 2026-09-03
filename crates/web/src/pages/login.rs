use leptos::prelude::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <section class="min-h-[80vh] flex items-center justify-center px-4 py-12">
            // Auth card
            <div id="lobby-auth" class="w-[380px] max-w-full bg-[rgba(8,8,14,0.75)] backdrop-blur-2xl border border-white/[0.1] rounded-3xl shadow-2xl shadow-black/60 p-8">
                <div class="text-center mb-6">
                    <img src="/assets/previews/hazel.webp" alt="Hazel" width="48" height="48" class="w-12 h-12 rounded-xl object-cover mx-auto mb-3" />
                    <h1 class="text-xl font-bold tracking-tight" id="lobby-title">"Welcome back"</h1>
                    <p class="text-zinc-500 text-sm mt-1" id="lobby-subtitle">"Sign in to continue"</p>
                </div>

                // Why join, revealed in register mode
                <div id="lobby-benefits" class="hidden mb-6 space-y-2.5 text-left">
                    <div class="flex items-center gap-2.5 text-sm text-zinc-300"><i class="ph ph-storefront text-teal-400 text-base"></i>"Free assets, models and plugins"</div>
                    <div class="flex items-center gap-2.5 text-sm text-zinc-300"><i class="ph ph-upload-simple text-accent text-base"></i>"Publish and sell your creations"</div>
                    <div class="flex items-center gap-2.5 text-sm text-zinc-300"><i class="ph ph-books text-sky-400 text-base"></i>"Your library, ready to re-download"</div>
                    <div class="flex items-center gap-2.5 text-sm text-zinc-300"><i class="ph ph-coin text-violet-400 text-base"></i>"Credits, gift cards and payouts"</div>
                    <div class="flex items-center gap-2.5 text-sm text-zinc-300"><i class="ph ph-trophy text-amber-400 text-base"></i>"Earn XP and level up"</div>
                </div>

                <div id="lobby-error" class="hidden mb-4 p-3 rounded-xl bg-red-500/10 border border-red-500/20 text-red-400 text-xs"></div>

                // Login form
                <form id="lobby-login" class="flex flex-col gap-3" onsubmit="return window._doLogin(event)">
                    <input type="email" name="email" required placeholder="Email" class="w-full px-4 py-2.5 bg-white/[0.04] border border-white/[0.06] rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all placeholder:text-zinc-600" />
                    <input type="password" name="password" required placeholder="Password" class="w-full px-4 py-2.5 bg-white/[0.04] border border-white/[0.06] rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all placeholder:text-zinc-600" />
                    <button type="submit" id="lobby-login-btn" class="w-full mt-1 py-2.5 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all">"Sign In"</button>
                </form>

                // Register form
                <form id="lobby-register" class="hidden flex flex-col gap-3" onsubmit="return window._doRegister(event)">
                    <input type="text" name="username" required minlength="3" maxlength="32" placeholder="Username" class="w-full px-4 py-2.5 bg-white/[0.04] border border-white/[0.06] rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all placeholder:text-zinc-600" />
                    <input type="email" name="email" required placeholder="Email" class="w-full px-4 py-2.5 bg-white/[0.04] border border-white/[0.06] rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all placeholder:text-zinc-600" />
                    <input type="password" name="password" required minlength="8" placeholder="Password" class="w-full px-4 py-2.5 bg-white/[0.04] border border-white/[0.06] rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all placeholder:text-zinc-600" />
                    <input type="password" name="confirm_password" required placeholder="Confirm password" class="w-full px-4 py-2.5 bg-white/[0.04] border border-white/[0.06] rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all placeholder:text-zinc-600" />
                    <button type="submit" id="lobby-register-btn" class="w-full mt-1 py-2.5 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all">"Create Account"</button>
                </form>

                <p class="text-center text-xs text-zinc-500 mt-5">
                    <span id="lobby-toggle-text">"Don't have an account? "</span>
                    <button onclick="window._toggleAuth()" class="text-accent hover:text-accent-hover" id="lobby-toggle-btn">"Register"</button>
                </p>
            </div>
        </section>

        <script>
            r##"
            // ── Check if already logged in ──
            const existingToken = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
            if (existingToken) {
                const redirect = new URLSearchParams(window.location.search).get('redirect') || '/';
                window.location.href = redirect;
            }

            // ── Auth logic ──
            let mode = 'login';
            // Check if URL is /register
            if (window.location.pathname === '/register') {
                mode = 'register';
                document.getElementById('lobby-login').classList.add('hidden');
                document.getElementById('lobby-register').classList.remove('hidden');
                document.getElementById('lobby-benefits').classList.remove('hidden');
                document.getElementById('lobby-title').textContent = 'Join Renzora';
                document.getElementById('lobby-subtitle').textContent = 'Create your free account, it only takes a moment';
                document.getElementById('lobby-toggle-text').textContent = 'Already have an account? ';
                document.getElementById('lobby-toggle-btn').textContent = 'Sign In';
            }

            function toggleAuth(){
                mode = mode==='login'?'register':'login';
                document.getElementById('lobby-login').classList.toggle('hidden', mode!=='login');
                document.getElementById('lobby-register').classList.toggle('hidden', mode!=='register');
                document.getElementById('lobby-benefits').classList.toggle('hidden', mode!=='register');
                document.getElementById('lobby-title').textContent = mode==='login'?'Welcome back':'Join Renzora';
                document.getElementById('lobby-subtitle').textContent = mode==='login'?'Sign in to continue':'Create your free account, it only takes a moment';
                document.getElementById('lobby-toggle-text').textContent = mode==='login'?"Don't have an account? ":'Already have an account? ';
                document.getElementById('lobby-toggle-btn').textContent = mode==='login'?'Register':'Sign In';
                document.getElementById('lobby-error').classList.add('hidden');
            }

            function setCookies(data){
                document.cookie = `token=${data.access_token};path=/;max-age=2592000;SameSite=Strict`;
                document.cookie = `refresh_token=${data.refresh_token};path=/;max-age=2592000;SameSite=Strict`;
                document.cookie = `user=${encodeURIComponent(JSON.stringify(data.user))};path=/;max-age=2592000;SameSite=Strict`;
            }

            async function doLogin(e){
                e.preventDefault();
                const form=e.target, btn=document.getElementById('lobby-login-btn'), err=document.getElementById('lobby-error');
                err.classList.add('hidden'); btn.disabled=true; btn.textContent='Signing in...';
                try{
                    const res=await fetch('/api/auth/login',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify({email:form.email.value,password:form.password.value})});
                    const data=await res.json();
                    if(!res.ok) throw new Error(data.error||'Login failed');
                    setCookies(data);
                    const redirect = new URLSearchParams(window.location.search).get('redirect') || '/';
                    window.location.href = redirect;
                }catch(error){err.textContent=error.message;err.classList.remove('hidden');btn.disabled=false;btn.textContent='Sign In';}
                return false;
            }

            async function doRegister(e){
                e.preventDefault();
                const form=e.target, btn=document.getElementById('lobby-register-btn'), err=document.getElementById('lobby-error');
                err.classList.add('hidden');
                if(form.password.value!==form.confirm_password.value){err.textContent='Passwords do not match';err.classList.remove('hidden');return false;}
                btn.disabled=true; btn.textContent='Creating account...';
                try{
                    const body={username:form.username.value,email:form.email.value,password:form.password.value};
                    const refCode=new URLSearchParams(window.location.search).get('ref')||'';
                    if(refCode)body.referral_code=refCode;
                    const res=await fetch('/api/auth/register',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(body)});
                    const data=await res.json();
                    if(!res.ok) throw new Error(data.error||'Registration failed');
                    setCookies(data);
                    // New user → straight to the marketplace
                    window.location.href = '/marketplace';
                }catch(error){err.textContent=error.message;err.classList.remove('hidden');btn.disabled=false;btn.textContent='Create Account';}
                return false;
            }

            window._toggleAuth=toggleAuth;
            window._doLogin=doLogin;
            window._doRegister=doRegister;
            "##
        </script>
    }
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    // Same page, the JS detects the /register path and shows the register form.
    view! {
        <LoginPage />
    }
}
