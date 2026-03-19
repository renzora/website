use leptos::prelude::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <section class="py-20 px-6">
            <div class="max-w-sm mx-auto">
                <div class="text-center mb-8">
                    <h1 class="text-2xl font-bold">"Sign In"</h1>
                    <p class="text-zinc-400 text-sm mt-2">"Sign in to your Renzora account."</p>
                </div>

                <div id="auth-error" class="hidden mb-4 p-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm"></div>

                <form id="login-form" class="flex flex-col gap-4" onsubmit="return handleLogin(event)">
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Email"</label>
                        <input
                            type="email"
                            name="email"
                            required
                            placeholder="you@example.com"
                            class="w-full px-3 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors"
                        />
                    </div>
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Password"</label>
                        <input
                            type="password"
                            name="password"
                            required
                            placeholder="Password"
                            class="w-full px-3 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors"
                        />
                    </div>
                    <button
                        type="submit"
                        id="login-btn"
                        class="w-full mt-2 inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors"
                    >
                        <i class="ph ph-sign-in text-lg"></i>"Sign In"
                    </button>
                </form>

                <p class="text-center text-sm text-zinc-400 mt-6">
                    "Don't have an account? "
                    <a href="/register" class="text-accent hover:text-accent-hover">"Register"</a>
                </p>
            </div>
        </section>

        <script>
            r#"
            async function handleLogin(e) {
                e.preventDefault();
                const form = e.target;
                const btn = document.getElementById('login-btn');
                const err = document.getElementById('auth-error');
                err.classList.add('hidden');
                btn.disabled = true;
                btn.innerHTML = '<i class="ph ph-spinner text-lg animate-spin"></i> Signing in...';

                try {
                    const res = await fetch('/api/auth/login', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({
                            email: form.email.value,
                            password: form.password.value
                        })
                    });
                    const data = await res.json();
                    if (!res.ok) {
                        throw new Error(data.error || 'Login failed');
                    }
                    document.cookie = `token=${data.access_token};path=/;max-age=900;SameSite=Strict`;
                    document.cookie = `refresh_token=${data.refresh_token};path=/;max-age=604800;SameSite=Strict`;
                    document.cookie = `user=${encodeURIComponent(JSON.stringify(data.user))};path=/;max-age=604800;SameSite=Strict`;
                    window.location.href = '/dashboard';
                } catch (error) {
                    err.textContent = error.message;
                    err.classList.remove('hidden');
                    btn.disabled = false;
                    btn.innerHTML = '<i class="ph ph-sign-in text-lg"></i> Sign In';
                }
                return false;
            }
            "#
        </script>
    }
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    view! {
        <section class="py-20 px-6">
            <div class="max-w-sm mx-auto">
                <div class="text-center mb-8">
                    <h1 class="text-2xl font-bold">"Create Account"</h1>
                    <p class="text-zinc-400 text-sm mt-2">"Join the Renzora community."</p>
                </div>

                <div id="auth-error" class="hidden mb-4 p-3 rounded-lg bg-red-500/10 border border-red-500/20 text-red-400 text-sm"></div>

                <form id="register-form" class="flex flex-col gap-4" onsubmit="return handleRegister(event)">
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Username"</label>
                        <input
                            type="text"
                            name="username"
                            required
                            minlength="3"
                            maxlength="32"
                            placeholder="Username"
                            class="w-full px-3 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors"
                        />
                    </div>
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Email"</label>
                        <input
                            type="email"
                            name="email"
                            required
                            placeholder="you@example.com"
                            class="w-full px-3 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors"
                        />
                    </div>
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Password"</label>
                        <input
                            type="password"
                            name="password"
                            required
                            minlength="8"
                            placeholder="At least 8 characters"
                            class="w-full px-3 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors"
                        />
                    </div>
                    <div>
                        <label class="block text-xs text-zinc-500 mb-1.5">"Confirm Password"</label>
                        <input
                            type="password"
                            name="confirm_password"
                            required
                            placeholder="Confirm password"
                            class="w-full px-3 py-2.5 bg-surface-card border border-zinc-800 rounded-lg text-zinc-50 text-sm outline-none focus:border-accent transition-colors"
                        />
                    </div>
                    <button
                        type="submit"
                        id="register-btn"
                        class="w-full mt-2 inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors"
                    >
                        <i class="ph ph-user-plus text-lg"></i>"Create Account"
                    </button>
                </form>

                <p class="text-center text-sm text-zinc-400 mt-6">
                    "Already have an account? "
                    <a href="/login" class="text-accent hover:text-accent-hover">"Sign In"</a>
                </p>
            </div>
        </section>

        <script>
            r#"
            async function handleRegister(e) {
                e.preventDefault();
                const form = e.target;
                const btn = document.getElementById('register-btn');
                const err = document.getElementById('auth-error');
                err.classList.add('hidden');

                if (form.password.value !== form.confirm_password.value) {
                    err.textContent = 'Passwords do not match';
                    err.classList.remove('hidden');
                    return false;
                }

                btn.disabled = true;
                btn.innerHTML = '<i class="ph ph-spinner text-lg animate-spin"></i> Creating account...';

                try {
                    const res = await fetch('/api/auth/register', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({
                            username: form.username.value,
                            email: form.email.value,
                            password: form.password.value
                        })
                    });
                    const data = await res.json();
                    if (!res.ok) {
                        throw new Error(data.error || 'Registration failed');
                    }
                    document.cookie = `token=${data.access_token};path=/;max-age=900;SameSite=Strict`;
                    document.cookie = `refresh_token=${data.refresh_token};path=/;max-age=604800;SameSite=Strict`;
                    document.cookie = `user=${encodeURIComponent(JSON.stringify(data.user))};path=/;max-age=604800;SameSite=Strict`;
                    window.location.href = '/dashboard';
                } catch (error) {
                    err.textContent = error.message;
                    err.classList.remove('hidden');
                    btn.disabled = false;
                    btn.innerHTML = '<i class="ph ph-user-plus text-lg"></i> Create Account';
                }
                return false;
            }
            "#
        </script>
    }
}
