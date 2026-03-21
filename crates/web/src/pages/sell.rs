use leptos::prelude::*;

#[component]
pub fn SellOnboardingPage() -> impl IntoView {
    view! {
        <section class="py-20 px-6">
            <div class="max-w-2xl mx-auto">
                <div id="onboard-loading" class="text-center py-12 text-zinc-500 text-sm">"Checking onboarding status..."</div>

                // Already onboarded — redirect message
                <div id="onboard-complete" class="hidden text-center py-12">
                    <div class="w-16 h-16 bg-green-500/10 rounded-full flex items-center justify-center mx-auto mb-4">
                        <i class="ph ph-check-circle text-3xl text-green-400"></i>
                    </div>
                    <h2 class="text-2xl font-bold mb-2">"You're all set!"</h2>
                    <p class="text-zinc-400 mb-6">"Your creator account is fully set up. You can start uploading assets."</p>
                    <a href="/marketplace/upload" class="inline-flex items-center gap-2 px-6 py-3 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                        <i class="ph ph-upload-simple text-lg"></i>"Upload Your First Asset"
                    </a>
                </div>

                // Onboarding steps
                <div id="onboard-steps" class="hidden">
                    <div class="text-center mb-10">
                        <h1 class="text-3xl font-bold mb-2">"Become a Creator"</h1>
                        <p class="text-zinc-400 text-sm">"Complete these steps to start selling on the Renzora Marketplace."</p>
                    </div>

                    // Progress indicator
                    <div class="flex items-center justify-center gap-2 mb-10">
                        <div id="step-dot-1" class="w-3 h-3 rounded-full bg-accent"></div>
                        <div class="w-16 h-0.5 bg-zinc-800"><div id="step-bar-1" class="h-full bg-accent transition-all duration-300" style="width:0%"></div></div>
                        <div id="step-dot-2" class="w-3 h-3 rounded-full bg-zinc-700"></div>
                        <div class="w-16 h-0.5 bg-zinc-800"><div id="step-bar-2" class="h-full bg-accent transition-all duration-300" style="width:0%"></div></div>
                        <div id="step-dot-3" class="w-3 h-3 rounded-full bg-zinc-700"></div>
                    </div>

                    // Step 1: Creator Policy
                    <div id="step-1" class="p-6 bg-surface-card border border-zinc-800 rounded-lg mb-6">
                        <div class="flex items-center gap-3 mb-4">
                            <div class="w-8 h-8 rounded-full bg-accent/10 flex items-center justify-center">
                                <span class="text-sm font-bold text-accent">"1"</span>
                            </div>
                            <h2 class="text-lg font-semibold">"Creator Policy"</h2>
                            <span id="step-1-badge" class="hidden ml-auto text-xs px-2 py-0.5 rounded-full bg-green-500/10 text-green-400">"Accepted"</span>
                        </div>

                        <div id="step-1-content" class="space-y-4">
                            <div class="bg-surface rounded-lg border border-zinc-800 p-5 max-h-80 overflow-y-auto text-sm text-zinc-300 space-y-4">
                                <h3 class="font-semibold text-zinc-50">"Renzora Marketplace Creator Agreement"</h3>

                                <div>
                                    <h4 class="font-medium text-zinc-200 mb-1">"1. Content Ownership & Rights"</h4>
                                    <p>"You confirm that you own or have the necessary rights and licenses to all content you upload. You grant Renzora a non-exclusive license to distribute, display, and promote your assets on the marketplace."</p>
                                </div>

                                <div>
                                    <h4 class="font-medium text-zinc-200 mb-1">"2. Revenue Share"</h4>
                                    <p>"Creators receive 80% of each sale. Renzora retains a 20% platform fee to cover hosting, payment processing, and marketplace operations. Revenue is credited to your account balance in real-time."</p>
                                </div>

                                <div>
                                    <h4 class="font-medium text-zinc-200 mb-1">"3. Payouts"</h4>
                                    <p>"Withdrawals require a connected Stripe account. The minimum withdrawal is 500 credits ($50 USD). Credits are converted at $0.10 per credit. Payouts are processed via Stripe Connect."</p>
                                </div>

                                <div>
                                    <h4 class="font-medium text-zinc-200 mb-1">"4. Prohibited Content"</h4>
                                    <p>"You may not upload content that is malicious, contains malware, violates intellectual property rights, or includes illegal, harmful, or discriminatory material. Renzora reserves the right to remove any content that violates these terms."</p>
                                </div>

                                <div>
                                    <h4 class="font-medium text-zinc-200 mb-1">"5. Quality Standards"</h4>
                                    <p>"Assets should be functional, reasonably documented, and match their descriptions. Misleading listings or non-functional assets may be removed and your account may be suspended."</p>
                                </div>

                                <div>
                                    <h4 class="font-medium text-zinc-200 mb-1">"6. Refunds & Disputes"</h4>
                                    <p>"Buyers may request refunds for non-functional or misrepresented assets. Refund disputes are reviewed by the Renzora team. Approved refunds are deducted from the creator's balance."</p>
                                </div>

                                <div>
                                    <h4 class="font-medium text-zinc-200 mb-1">"7. Account Termination"</h4>
                                    <p>"Renzora may suspend or terminate creator accounts that repeatedly violate these terms, engage in fraudulent activity, or receive excessive refund requests."</p>
                                </div>
                            </div>

                            <label class="flex items-start gap-3 cursor-pointer select-none">
                                <input type="checkbox" id="policy-checkbox" class="mt-1 accent-accent" />
                                <span class="text-sm text-zinc-300">"I have read and agree to the Renzora Marketplace Creator Agreement."</span>
                            </label>

                            <button onclick="acceptPolicy()" id="accept-btn" disabled
                                class="w-full inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors disabled:opacity-40 disabled:cursor-not-allowed">
                                <i class="ph ph-handshake text-lg"></i>"Accept & Continue"
                            </button>
                        </div>
                    </div>

                    // Step 2: Connect Stripe
                    <div id="step-2" class="p-6 bg-surface-card border border-zinc-800 rounded-lg mb-6 opacity-50">
                        <div class="flex items-center gap-3 mb-4">
                            <div class="w-8 h-8 rounded-full bg-zinc-800 flex items-center justify-center" id="step-2-circle">
                                <span class="text-sm font-bold text-zinc-500" id="step-2-num">"2"</span>
                            </div>
                            <h2 class="text-lg font-semibold">"Connect Payment Account"</h2>
                            <span id="step-2-badge" class="hidden ml-auto text-xs px-2 py-0.5 rounded-full bg-green-500/10 text-green-400">"Connected"</span>
                        </div>

                        <div id="step-2-content" class="hidden space-y-4">
                            <p class="text-sm text-zinc-400">"Connect your bank account through Stripe to receive payouts when your assets sell. This is required to sell paid assets."</p>
                            <div class="bg-surface rounded-lg border border-zinc-800 p-4 space-y-2">
                                <div class="flex items-center gap-2 text-sm text-zinc-300">
                                    <i class="ph ph-shield-check text-green-400"></i>"Secure payment processing by Stripe"
                                </div>
                                <div class="flex items-center gap-2 text-sm text-zinc-300">
                                    <i class="ph ph-bank text-accent"></i>"Direct deposits to your bank account"
                                </div>
                                <div class="flex items-center gap-2 text-sm text-zinc-300">
                                    <i class="ph ph-lightning text-amber-400"></i>"Fast payouts, typically 2-3 business days"
                                </div>
                            </div>
                            <button onclick="connectStripe()" id="stripe-btn"
                                class="w-full inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-[#635BFF] text-white hover:bg-[#5851db] transition-colors">
                                <i class="ph ph-stripe-logo text-lg"></i>"Connect with Stripe"
                            </button>
                            <button onclick="skipStripe()" class="w-full text-center text-xs text-zinc-500 hover:text-zinc-400 transition-colors cursor-pointer">
                                "Skip for now — you can set this up later in Settings (required for paid assets)"
                            </button>
                        </div>
                    </div>

                    // Step 3: Ready
                    <div id="step-3" class="p-6 bg-surface-card border border-zinc-800 rounded-lg opacity-50">
                        <div class="flex items-center gap-3 mb-4">
                            <div class="w-8 h-8 rounded-full bg-zinc-800 flex items-center justify-center" id="step-3-circle">
                                <span class="text-sm font-bold text-zinc-500" id="step-3-num">"3"</span>
                            </div>
                            <h2 class="text-lg font-semibold">"Start Selling"</h2>
                        </div>

                        <div id="step-3-content" class="hidden space-y-4">
                            <p class="text-sm text-zinc-400">"You're ready to upload your first asset to the Renzora Marketplace!"</p>
                            <a href="/marketplace/upload" class="w-full inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-colors">
                                <i class="ph ph-upload-simple text-lg"></i>"Upload Your First Asset"
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        <script>
            r#"
            function getToken() {
                return document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
            }

            let onboardState = { policy: false, stripe_connected: false, stripe_onboarded: false };

            // Enable accept button when checkbox is checked
            document.getElementById('policy-checkbox')?.addEventListener('change', function() {
                document.getElementById('accept-btn').disabled = !this.checked;
            });

            async function loadStatus() {
                const token = getToken();
                if (!token) { window.location.href = '/login'; return; }

                try {
                    const res = await fetch('/api/creator/onboard-status', { headers: { 'Authorization': 'Bearer ' + token } });
                    if (!res.ok) return;
                    const data = await res.json();
                    onboardState.policy = data.policy_accepted;
                    onboardState.stripe_connected = data.stripe_connected;
                    onboardState.stripe_onboarded = data.stripe_onboarded;

                    document.getElementById('onboard-loading').classList.add('hidden');

                    // If fully onboarded, show complete state
                    if (onboardState.policy && onboardState.stripe_onboarded) {
                        document.getElementById('onboard-complete').classList.remove('hidden');
                        return;
                    }

                    document.getElementById('onboard-steps').classList.remove('hidden');
                    updateSteps();
                } catch(e) {
                    console.error(e);
                }
            }

            function updateSteps() {
                // Step 1 - Policy
                if (onboardState.policy) {
                    document.getElementById('step-1-content').classList.add('hidden');
                    document.getElementById('step-1-badge').classList.remove('hidden');
                    document.getElementById('step-bar-1').style.width = '100%';
                    document.getElementById('step-dot-2').classList.remove('bg-zinc-700');
                    document.getElementById('step-dot-2').classList.add('bg-accent');

                    // Enable step 2
                    document.getElementById('step-2').classList.remove('opacity-50');
                    document.getElementById('step-2-content').classList.remove('hidden');
                    document.getElementById('step-2-circle').classList.remove('bg-zinc-800');
                    document.getElementById('step-2-circle').classList.add('bg-accent/10');
                    document.getElementById('step-2-num').classList.remove('text-zinc-500');
                    document.getElementById('step-2-num').classList.add('text-accent');
                }

                // Step 2 - Stripe
                if (onboardState.stripe_onboarded) {
                    document.getElementById('step-2-content').classList.add('hidden');
                    document.getElementById('step-2-badge').classList.remove('hidden');
                    document.getElementById('step-bar-2').style.width = '100%';
                    document.getElementById('step-dot-3').classList.remove('bg-zinc-700');
                    document.getElementById('step-dot-3').classList.add('bg-accent');

                    // Enable step 3
                    document.getElementById('step-3').classList.remove('opacity-50');
                    document.getElementById('step-3-content').classList.remove('hidden');
                    document.getElementById('step-3-circle').classList.remove('bg-zinc-800');
                    document.getElementById('step-3-circle').classList.add('bg-green-500/10');
                    document.getElementById('step-3-num').classList.remove('text-zinc-500');
                    document.getElementById('step-3-num').classList.add('text-green-400');
                    document.getElementById('step-3-num').innerHTML = '<i class="ph ph-check text-sm"></i>';
                } else if (onboardState.stripe_connected) {
                    // Partially connected
                    const btn = document.getElementById('stripe-btn');
                    if (btn) {
                        btn.textContent = 'Complete Stripe Setup';
                    }
                }
            }

            async function acceptPolicy() {
                const token = getToken();
                const btn = document.getElementById('accept-btn');
                btn.disabled = true;
                btn.innerHTML = '<i class="ph ph-spinner animate-spin"></i> Accepting...';

                try {
                    const res = await fetch('/api/creator/accept-policy', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    });
                    if (!res.ok) throw new Error('Failed');

                    onboardState.policy = true;
                    updateSteps();
                } catch(e) {
                    btn.disabled = false;
                    btn.innerHTML = '<i class="ph ph-handshake text-lg"></i> Accept & Continue';
                }
            }

            async function connectStripe() {
                const token = getToken();
                const btn = document.getElementById('stripe-btn');
                btn.disabled = true;
                btn.innerHTML = '<i class="ph ph-spinner animate-spin"></i> Redirecting to Stripe...';

                try {
                    const res = await fetch('/api/credits/connect/onboard', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    });
                    const data = await res.json();
                    if (res.ok && data.url) {
                        window.location.href = data.url;
                    } else {
                        throw new Error(data.error || 'Failed to start Stripe onboarding');
                    }
                } catch(e) {
                    btn.disabled = false;
                    btn.innerHTML = '<i class="ph ph-stripe-logo text-lg"></i> Connect with Stripe';
                    alert(e.message);
                }
            }

            function skipStripe() {
                // Skip to step 3 — they can only upload free assets
                document.getElementById('step-2-content').classList.add('hidden');
                document.getElementById('step-2-badge').innerHTML = 'Skipped';
                document.getElementById('step-2-badge').classList.remove('hidden', 'bg-green-500/10', 'text-green-400');
                document.getElementById('step-2-badge').classList.add('bg-amber-500/10', 'text-amber-400');
                document.getElementById('step-bar-2').style.width = '100%';
                document.getElementById('step-dot-3').classList.remove('bg-zinc-700');
                document.getElementById('step-dot-3').classList.add('bg-accent');

                document.getElementById('step-3').classList.remove('opacity-50');
                document.getElementById('step-3-content').classList.remove('hidden');
                document.getElementById('step-3-circle').classList.remove('bg-zinc-800');
                document.getElementById('step-3-circle').classList.add('bg-green-500/10');
                document.getElementById('step-3-num').classList.remove('text-zinc-500');
                document.getElementById('step-3-num').classList.add('text-green-400');
                document.getElementById('step-3-num').innerHTML = '<i class="ph ph-check text-sm"></i>';
            }

            // Check for Stripe return
            if (new URLSearchParams(window.location.search).get('connect') === 'success') {
                history.replaceState({}, '', '/marketplace/sell');
            }

            loadStatus();
            "#
        </script>
    }
}
