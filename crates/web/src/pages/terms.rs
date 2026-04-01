use leptos::prelude::*;

#[component]
pub fn TermsPage() -> impl IntoView {
    view! {
        <section class="max-w-3xl mx-auto py-12 px-4">
            <h1 class="text-2xl font-bold text-zinc-100 mb-2">"Terms of Service"</h1>
            <p class="text-xs text-zinc-500 mb-8">"Last updated: March 2026"</p>

            <div class="prose prose-invert prose-sm max-w-none space-y-6 text-zinc-400">
                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"1. Acceptance of Terms"</h2>
                    <p>"By accessing or using Renzora (the \"Platform\"), including the website at renzora.com, the Renzora Engine, Launcher, and related services, you agree to be bound by these Terms of Service. If you do not agree, do not use the Platform."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"2. Account Registration"</h2>
                    <p>"You must be at least 13 years old to create an account. You are responsible for maintaining the security of your account credentials. You must not share your account or API tokens with others. You are responsible for all activity under your account."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"3. Marketplace"</h2>
                    <p>"Renzora operates a marketplace where users can buy and sell digital assets including 3D models, textures, scripts, plugins, and games. Sellers retain ownership of their original content. By listing an asset, you grant Renzora a licence to distribute it through the Platform. Buyers receive a licence to use purchased assets according to the licence type selected by the seller."</p>
                    <p>"Renzora charges a commission on marketplace sales. Commission rates vary by subscription tier. Credits purchased on the Platform are non-refundable except where required by law."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"4. Credits and Payments"</h2>
                    <p>"Credits are a virtual currency used on the Platform. 1 credit = $0.10 USD. Credits can be purchased via Stripe. Credits are non-transferable except through Platform features (gift cards, purchases, donations). Renzora reserves the right to adjust credit balances in cases of fraud, chargebacks, or errors."</p>
                    <p>"Creator withdrawals are processed via Stripe Connect. Minimum withdrawal is 500 credits ($50 USD). Renzora is not responsible for delays caused by payment processors."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"5. User Content"</h2>
                    <p>"You retain ownership of content you create and upload. You must not upload content that infringes on others' intellectual property, contains malware, or violates any laws. Renzora may remove content that violates these terms without notice."</p>
                    <p>"Posts, comments, forum threads, and messages are your responsibility. You must not post spam, harassment, hate speech, or illegal content."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"6. API and Developer Access"</h2>
                    <p>"API access is subject to rate limits based on your subscription tier. You must not circumvent rate limits, scrape the Platform, or use the API in ways that degrade service for other users. API tokens are confidential and must be stored securely."</p>
                    <p>"Developer apps must accurately describe their purpose and only request necessary permission scopes."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"7. Subscriptions"</h2>
                    <p>"Paid subscriptions renew automatically unless cancelled. You may cancel at any time; access continues until the end of the current billing period. Renzora may change subscription pricing with 30 days notice."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"8. Prohibited Conduct"</h2>
                    <p>"You must not: create multiple accounts to bypass limits; manipulate reviews or ratings; engage in fraudulent transactions; attempt to access other users' accounts; reverse engineer the Platform; use the Platform for any illegal purpose."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"9. Termination"</h2>
                    <p>"Renzora may suspend or terminate your account for violation of these terms. You may delete your account at any time through Settings. Upon termination, your credits are forfeited except where withdrawal is available and you are eligible."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"10. Limitation of Liability"</h2>
                    <p>"The Platform is provided \"as is\" without warranties. Renzora is not liable for: loss of data, loss of revenue, service interruptions, or third-party content. Our total liability is limited to the amount you paid to Renzora in the 12 months preceding the claim."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"11. Governing Law"</h2>
                    <p>"These terms are governed by the laws of England and Wales. Any disputes shall be subject to the exclusive jurisdiction of the courts of England and Wales."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"12. Changes to Terms"</h2>
                    <p>"We may update these terms from time to time. Continued use of the Platform after changes constitutes acceptance. We will notify users of material changes via email or Platform announcement."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"13. Contact"</h2>
                    <p>"For questions about these terms, contact us at support@renzora.com."</p>
                </div>
            </div>
        </section>
    }
}
