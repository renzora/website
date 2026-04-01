use leptos::prelude::*;

#[component]
pub fn PrivacyPage() -> impl IntoView {
    view! {
        <section class="max-w-3xl mx-auto py-12 px-4">
            <h1 class="text-2xl font-bold text-zinc-100 mb-2">"Privacy Policy"</h1>
            <p class="text-xs text-zinc-500 mb-8">"Last updated: March 2026"</p>

            <div class="prose prose-invert prose-sm max-w-none space-y-6 text-zinc-400">
                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"1. Who We Are"</h2>
                    <p>"Renzora (\"we\", \"us\", \"our\") operates the renzora.com platform, the Renzora Engine, and the Renzora Launcher. This policy explains how we collect, use, and protect your personal data in accordance with the UK General Data Protection Regulation (UK GDPR) and the Data Protection Act 2018."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"2. Data We Collect"</h2>
                    <p class="font-medium text-zinc-300">"Account data:"</p>
                    <p>"Email address, username, password (hashed), profile information (bio, location, avatar), connected social accounts (Discord, Twitch, Steam, GitHub usernames and IDs)."</p>
                    <p class="font-medium text-zinc-300 mt-3">"Transaction data:"</p>
                    <p>"Credit purchases, marketplace transactions, withdrawal history, Stripe payment details (processed by Stripe, not stored by us)."</p>
                    <p class="font-medium text-zinc-300 mt-3">"Usage data:"</p>
                    <p>"IP addresses (for security and rate limiting), browser user agent, pages visited, API usage, launcher downloads."</p>
                    <p class="font-medium text-zinc-300 mt-3">"Content data:"</p>
                    <p>"Assets you upload, posts, comments, messages, forum threads, reviews."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"3. How We Use Your Data"</h2>
                    <p>"We use your data to: provide and improve the Platform; process transactions; send notifications; prevent fraud and abuse; comply with legal obligations; communicate service updates."</p>
                    <p>"Legal basis: contract performance (account services), legitimate interests (security, analytics), consent (marketing communications)."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"4. Data Sharing"</h2>
                    <p>"We share data with: Stripe (payment processing); Cloudflare (CDN and security); connected OAuth providers (only what you authorise). We do not sell your personal data. We may disclose data if required by law or to protect our legal rights."</p>
                    <p>"Your public profile information (username, avatar, bio, social links, badges) is visible to other users."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"5. Data Retention"</h2>
                    <p>"Account data is retained while your account is active. Transaction records are retained for 7 years for tax and legal compliance. You may request deletion of your account through Settings. Upon deletion, personal data is removed within 30 days, except where retention is required by law."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"6. Your Rights"</h2>
                    <p>"Under UK GDPR, you have the right to: access your data; rectify inaccurate data; erase your data (right to be forgotten); restrict processing; data portability; object to processing; withdraw consent. To exercise these rights, contact support@renzora.com."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"7. Cookies"</h2>
                    <p>"We use essential cookies for authentication (JWT tokens, session management). We do not use tracking cookies or third-party advertising cookies. Functional cookies store your preferences (theme, language)."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"8. Security"</h2>
                    <p>"We protect your data with: HTTPS encryption in transit; hashed passwords (Argon2); hashed API tokens (SHA-256); two-factor authentication (TOTP); role-based access controls. No system is 100% secure. If we discover a breach affecting your data, we will notify you and the ICO within 72 hours."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"9. Children"</h2>
                    <p>"The Platform is not intended for children under 13. We do not knowingly collect data from children under 13. If you believe a child has provided us with personal data, contact us and we will delete it."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"10. International Transfers"</h2>
                    <p>"Your data may be processed outside the UK where our service providers operate (e.g., Cloudflare, Stripe). We ensure adequate safeguards are in place for any international transfers."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"11. Changes"</h2>
                    <p>"We may update this policy. We will notify you of material changes via email or Platform announcement. Continued use after changes constitutes acceptance."</p>
                </div>

                <div>
                    <h2 class="text-lg font-semibold text-zinc-200 mb-2">"12. Contact"</h2>
                    <p>"Data Controller: Renzora"</p>
                    <p>"Email: support@renzora.com"</p>
                    <p>"You may also lodge a complaint with the Information Commissioner's Office (ICO) at ico.org.uk."</p>
                </div>
            </div>
        </section>
    }
}
