use leptos::prelude::*;

#[component]
pub fn DevelopersPage() -> impl IntoView {
    view! {
        <section class="py-12 px-6 min-h-screen">
            <div class="max-w-4xl mx-auto">
                <div class="mb-10">
                    <h1 class="text-3xl font-bold">"Developers"</h1>
                    <p class="text-zinc-400 mt-2">"Build integrations, automate uploads, and extend the Renzora ecosystem with our API."</p>
                </div>

                <div id="dev-content">
                    <div class="text-center py-12">
                        <div class="inline-block animate-spin w-6 h-6 border-2 border-zinc-700 border-t-accent rounded-full"></div>
                    </div>
                </div>
            </div>
        </section>
        <script>
            r##"
            (async function() {
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                const el = document.getElementById('dev-content');

                // Fetch existing tokens if logged in
                let tokens = [];
                if (token) {
                    try {
                        const res = await fetch('/api/api-tokens', { headers: { 'Authorization': 'Bearer ' + token } });
                        if (res.ok) tokens = await res.json();
                    } catch(e) {}
                }

                el.innerHTML = `
                    <!-- API Overview -->
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-10">
                        <div class="p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl">
                            <div class="w-10 h-10 rounded-lg bg-accent/10 flex items-center justify-center mb-3">
                                <i class="ph ph-game-controller text-accent text-lg"></i>
                            </div>
                            <h3 class="font-semibold mb-1">Game Services</h3>
                            <p class="text-sm text-zinc-500">Achievements, leaderboards, player stats, and friends for your game.</p>
                        </div>
                        <div class="p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl">
                            <div class="w-10 h-10 rounded-lg bg-accent/10 flex items-center justify-center mb-3">
                                <i class="ph ph-shield-check text-accent text-lg"></i>
                            </div>
                            <h3 class="font-semibold mb-1">Scoped Access</h3>
                            <p class="text-sm text-zinc-500">Users grant your app specific permissions. No scope, no access.</p>
                        </div>
                        <div class="p-5 bg-white/[0.02] border border-zinc-800/50 rounded-xl">
                            <div class="w-10 h-10 rounded-lg bg-accent/10 flex items-center justify-center mb-3">
                                <i class="ph ph-brackets-curly text-accent text-lg"></i>
                            </div>
                            <h3 class="font-semibold mb-1">REST API</h3>
                            <p class="text-sm text-zinc-500">Marketplace API, game services, and multipart uploads.</p>
                        </div>
                    </div>

                    <!-- Developer Apps Section -->
                    <div class="mb-10">
                        <div class="flex items-center justify-between mb-4">
                            <h2 class="text-xl font-semibold">My Apps</h2>
                            ${token ? '<button onclick="registerApp()" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all">Register App</button>' : ''}
                        </div>
                        ${!token ? '<p class="text-sm text-zinc-500"><a href="/login" class="text-accent hover:text-accent-hover">Sign in</a> to register developer apps.</p>' : `
                            <div id="app-list" class="space-y-2"></div>
                        `}
                    </div>

                    <!-- Scopes Reference -->
                    <div class="mb-10">
                        <h2 class="text-xl font-semibold mb-4">Permission Scopes</h2>
                        <p class="text-sm text-zinc-500 mb-3">When creating an app token, specify exactly which scopes it needs. Users must grant each scope before your app can use it.</p>
                        <div class="bg-white/[0.02] border border-zinc-800/50 rounded-xl overflow-hidden">
                            <table class="w-full text-sm">
                                <thead><tr class="border-b border-zinc-800/50">
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Scope</th>
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Description</th>
                                </tr></thead>
                                <tbody>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">profile:read</td><td class="px-4 py-2.5 text-zinc-400">Read username and avatar</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">friends:read</td><td class="px-4 py-2.5 text-zinc-400">Read player's friend list</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">friends:write</td><td class="px-4 py-2.5 text-zinc-400">Send and accept friend requests</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">achievements:read</td><td class="px-4 py-2.5 text-zinc-400">Read player achievements</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">achievements:write</td><td class="px-4 py-2.5 text-zinc-400">Unlock achievements for players</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">stats:read</td><td class="px-4 py-2.5 text-zinc-400">Read player stats (play time, kills, etc.)</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">stats:write</td><td class="px-4 py-2.5 text-zinc-400">Update player stats</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">leaderboards:read</td><td class="px-4 py-2.5 text-zinc-400">Read leaderboard scores</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">leaderboards:write</td><td class="px-4 py-2.5 text-zinc-400">Submit leaderboard scores</td></tr>
                                    <tr><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">inventory:read</td><td class="px-4 py-2.5 text-zinc-400">Read purchased assets and games</td></tr>
                                </tbody>
                            </table>
                        </div>
                    </div>

                    <!-- Game Services API Reference -->
                    <div class="mb-10">
                        <h2 class="text-xl font-semibold mb-4">Game Services API</h2>
                        <p class="text-sm text-zinc-500 mb-4">Use <code class="text-zinc-400">rza_</code> prefixed app tokens for all game service endpoints. The user must have granted the required scope to your app.</p>
                        <div class="space-y-3">
                            <h3 class="text-sm font-medium text-zinc-500 mt-4 mb-2">Player Data (requires user grant)</h3>
                            ${apiEndpoint('GET', '/api/gameservices/player/:user_id/profile', 'Get player profile (scope: profile:read)', null)}
                            ${apiEndpoint('GET', '/api/gameservices/player/:user_id/friends', 'Get player friends (scope: friends:read)', null)}
                            ${apiEndpoint('GET', '/api/gameservices/player/:user_id/achievements', 'Get player achievements (scope: achievements:read)', null)}
                            ${apiEndpoint('POST', '/api/gameservices/player/:user_id/achievements/unlock', 'Unlock achievement (scope: achievements:write)', '{ "achievement_key": "first_win" }')}
                            ${apiEndpoint('GET', '/api/gameservices/player/:user_id/stats', 'Get player stats (scope: stats:read)', null)}
                            ${apiEndpoint('POST', '/api/gameservices/player/:user_id/stats', 'Set player stat (scope: stats:write)', '{ "key": "total_kills", "value_int": 42 }')}
                            ${apiEndpoint('POST', '/api/gameservices/player/:user_id/stats/increment', 'Increment stat (scope: stats:write)', '{ "key": "games_played", "delta": 1 }')}

                            <h3 class="text-sm font-medium text-zinc-500 mt-6 mb-2">Leaderboards</h3>
                            ${apiEndpoint('GET', '/api/gameservices/leaderboard/:key/scores?limit=50', 'Get top scores (uses app token, no user grant needed)', null)}
                            ${apiEndpoint('POST', '/api/gameservices/leaderboard/:key/submit', 'Submit score (scope: leaderboards:write)', '{ "user_id": "...", "score": 9500 }')}

                            <h3 class="text-sm font-medium text-zinc-500 mt-6 mb-2">App Management (owner only, JWT or rz_ token)</h3>
                            ${apiEndpoint('POST', '/api/gameservices/apps', 'Register a new app', '{ "name": "My Game", "description": "...", "website_url": "..." }')}
                            ${apiEndpoint('GET', '/api/gameservices/apps', 'List your apps', null)}
                            ${apiEndpoint('POST', '/api/gameservices/apps/:id/tokens', 'Create scoped app token', '{ "name": "prod", "scopes": ["achievements:write", "stats:write"] }')}
                            ${apiEndpoint('POST', '/api/gameservices/apps/:id/achievements', 'Define achievement', '{ "key": "first_win", "name": "First Victory", "points": 10 }')}
                            ${apiEndpoint('POST', '/api/gameservices/apps/:id/leaderboards', 'Create leaderboard', '{ "key": "high_score", "name": "High Score", "sort_order": "desc" }')}

                            <h3 class="text-sm font-medium text-zinc-500 mt-6 mb-2">User Grants</h3>
                            ${apiEndpoint('POST', '/api/gameservices/grants', 'Grant permissions to an app (user-facing)', '{ "app_id": "...", "scopes": ["profile:read", "achievements:write"] }')}
                            ${apiEndpoint('GET', '/api/gameservices/grants', 'List connected apps (user-facing)', null)}
                            ${apiEndpoint('DELETE', '/api/gameservices/grants/:app_id', 'Revoke app access (user-facing)', null)}

                            <h3 class="text-sm font-medium text-zinc-500 mt-6 mb-2">Friends (user-facing, any auth)</h3>
                            ${apiEndpoint('GET', '/api/gameservices/friends', 'List friends', null)}
                            ${apiEndpoint('GET', '/api/gameservices/friends/requests', 'List incoming requests', null)}
                            ${apiEndpoint('POST', '/api/gameservices/friends/add', 'Send friend request', '{ "user_id": "..." }')}
                            ${apiEndpoint('POST', '/api/gameservices/friends/accept', 'Accept friend request', '{ "user_id": "..." }')}
                            ${apiEndpoint('POST', '/api/gameservices/friends/remove', 'Remove friend', '{ "user_id": "..." }')}
                        </div>
                    </div>

                    <!-- API Tokens Section -->
                    <div class="mb-10">
                        <div class="flex items-center justify-between mb-4">
                            <h2 class="text-xl font-semibold">API Tokens</h2>
                            ${token ? '<button onclick="createToken()" class="px-4 py-2 rounded-lg text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all">Create Token</button>' : ''}
                        </div>

                        ${!token ? '<p class="text-sm text-zinc-500"><a href="/login" class="text-accent hover:text-accent-hover">Sign in</a> to manage API tokens.</p>' : `
                            <div id="token-list" class="space-y-2">
                                ${tokens.length === 0 ? '<p class="text-sm text-zinc-500">No API tokens yet. Create one to get started.</p>' :
                                    tokens.map(t => tokenRow(t)).join('')}
                            </div>
                            <div id="new-token-banner" class="hidden mt-4 p-4 bg-green-950/30 border border-green-800/50 rounded-xl">
                                <div class="flex items-center gap-2 mb-2">
                                    <i class="ph ph-check-circle text-green-400"></i>
                                    <span class="text-sm font-medium text-green-400">Token created</span>
                                </div>
                                <p class="text-xs text-zinc-400 mb-2">Copy this token now. It will not be shown again.</p>
                                <div class="flex items-center gap-2">
                                    <code id="new-token-value" class="flex-1 px-3 py-2 bg-black/40 rounded-lg text-xs text-zinc-300 font-mono select-all overflow-x-auto"></code>
                                    <button onclick="copyToken()" class="px-3 py-2 rounded-lg text-xs bg-white/5 hover:bg-white/10 transition-colors">Copy</button>
                                </div>
                            </div>
                        `}
                    </div>

                    <!-- API Reference -->
                    <div class="mb-10">
                        <h2 class="text-xl font-semibold mb-4">Endpoints</h2>
                        <div class="space-y-3">
                            <h3 class="text-sm font-medium text-zinc-500 mt-4 mb-2">Tokens</h3>
                            ${apiEndpoint('POST', '/api/api-tokens', 'Create an API token', '{ "name": "my-bot", "expires_in_days": 90 }')}
                            ${apiEndpoint('GET', '/api/api-tokens', 'List your tokens', null)}
                            ${apiEndpoint('DELETE', '/api/api-tokens/:id', 'Revoke a token', null)}
                            <h3 class="text-sm font-medium text-zinc-500 mt-6 mb-2">Marketplace</h3>
                            ${apiEndpoint('POST', '/api/marketplace/upload', 'Upload an asset (multipart form)', null)}
                            ${apiEndpoint('PUT', '/api/marketplace/:id/update', 'Update asset metadata (JSON)', null)}
                            ${apiEndpoint('PUT', '/api/marketplace/:id/files', 'Update asset file or thumbnail (multipart)', null)}
                            ${apiEndpoint('DELETE', '/api/marketplace/:id/delete', 'Delete an asset and all files from storage', null)}
                            ${apiEndpoint('GET', '/api/marketplace/', 'List published assets (supports ?q=, ?category=, ?sort=, ?page=)', null)}
                            ${apiEndpoint('GET', '/api/marketplace/detail/:slug', 'Get full asset details', null)}
                            ${apiEndpoint('GET', '/api/marketplace/categories', 'List all categories', null)}
                            ${apiEndpoint('GET', '/api/marketplace/:id/download', 'Get download URL (requires ownership)', null)}
                        </div>
                    </div>

                    <!-- Upload Fields -->
                    <div class="mb-10">
                        <h2 class="text-xl font-semibold mb-4">Upload Fields</h2>
                        <p class="text-sm text-zinc-500 mb-4">The upload endpoint accepts multipart/form-data with the following fields:</p>

                        <h3 class="text-sm font-medium text-zinc-400 mb-3">Multipart Fields</h3>
                        <div class="bg-white/[0.02] border border-zinc-800/50 rounded-xl overflow-hidden mb-6">
                            <table class="w-full text-sm">
                                <thead><tr class="border-b border-zinc-800/50">
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Field</th>
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Type</th>
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Required</th>
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Limits</th>
                                </tr></thead>
                                <tbody>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">metadata</td><td class="px-4 py-2.5 text-zinc-400">JSON string</td><td class="px-4 py-2.5"><span class="text-red-400 text-xs">required</span></td><td class="px-4 py-2.5 text-zinc-500 text-xs">See metadata fields below</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">file</td><td class="px-4 py-2.5 text-zinc-400">Binary file</td><td class="px-4 py-2.5"><span class="text-red-400 text-xs">required</span></td><td class="px-4 py-2.5 text-zinc-500 text-xs">200MB max</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">thumbnail</td><td class="px-4 py-2.5 text-zinc-400">Image</td><td class="px-4 py-2.5 text-zinc-600 text-xs">optional</td><td class="px-4 py-2.5 text-zinc-500 text-xs">10MB, png/jpg/webp/gif</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">screenshot_0 .. screenshot_9</td><td class="px-4 py-2.5 text-zinc-400">Image</td><td class="px-4 py-2.5 text-zinc-600 text-xs">optional</td><td class="px-4 py-2.5 text-zinc-500 text-xs">10MB each, max 10, png/jpg/webp/gif</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">video</td><td class="px-4 py-2.5 text-zinc-400">Video</td><td class="px-4 py-2.5 text-zinc-600 text-xs">optional</td><td class="px-4 py-2.5 text-zinc-500 text-xs">100MB, mp4/webm/mov</td></tr>
                                    <tr><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">audio</td><td class="px-4 py-2.5 text-zinc-400">Audio</td><td class="px-4 py-2.5 text-zinc-600 text-xs">optional</td><td class="px-4 py-2.5 text-zinc-500 text-xs">50MB, mp3/wav/ogg/flac</td></tr>
                                </tbody>
                            </table>
                        </div>

                        <h3 class="text-sm font-medium text-zinc-400 mb-3">Metadata JSON Fields</h3>
                        <div class="bg-white/[0.02] border border-zinc-800/50 rounded-xl overflow-hidden mb-6">
                            <table class="w-full text-sm">
                                <thead><tr class="border-b border-zinc-800/50">
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Field</th>
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Type</th>
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Required</th>
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Validation</th>
                                </tr></thead>
                                <tbody>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">name</td><td class="px-4 py-2.5 text-zinc-400">string</td><td class="px-4 py-2.5"><span class="text-red-400 text-xs">required</span></td><td class="px-4 py-2.5 text-zinc-500 text-xs">1-128 characters</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">description</td><td class="px-4 py-2.5 text-zinc-400">string</td><td class="px-4 py-2.5"><span class="text-red-400 text-xs">required</span></td><td class="px-4 py-2.5 text-zinc-500 text-xs">1-5000 characters</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">category</td><td class="px-4 py-2.5 text-zinc-400">string</td><td class="px-4 py-2.5"><span class="text-red-400 text-xs">required</span></td><td class="px-4 py-2.5 text-zinc-500 text-xs">Must match a category slug (e.g. "materials", "3d-models", "particles")</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">price_credits</td><td class="px-4 py-2.5 text-zinc-400">integer</td><td class="px-4 py-2.5"><span class="text-red-400 text-xs">required</span></td><td class="px-4 py-2.5 text-zinc-500 text-xs">0 or higher (0 = free)</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">version</td><td class="px-4 py-2.5 text-zinc-400">string</td><td class="px-4 py-2.5"><span class="text-red-400 text-xs">required</span></td><td class="px-4 py-2.5 text-zinc-500 text-xs">1-32 characters (e.g. "1.0.0")</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">tags</td><td class="px-4 py-2.5 text-zinc-400">string[]</td><td class="px-4 py-2.5 text-zinc-600 text-xs">optional</td><td class="px-4 py-2.5 text-zinc-500 text-xs">Max 5 tags, each 1-32 chars, auto-lowercased</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">licence</td><td class="px-4 py-2.5 text-zinc-400">string</td><td class="px-4 py-2.5 text-zinc-600 text-xs">optional</td><td class="px-4 py-2.5 text-zinc-500 text-xs">standard, extended, cc0, mit, apache2, gpl3 (default: standard)</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">ai_generated</td><td class="px-4 py-2.5 text-zinc-400">boolean</td><td class="px-4 py-2.5 text-zinc-600 text-xs">optional</td><td class="px-4 py-2.5 text-zinc-500 text-xs">true if asset contains AI-generated content (default: false)</td></tr>
                                    <tr><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">metadata</td><td class="px-4 py-2.5 text-zinc-400">object</td><td class="px-4 py-2.5 text-zinc-600 text-xs">optional</td><td class="px-4 py-2.5 text-zinc-500 text-xs">See extended metadata below</td></tr>
                                </tbody>
                            </table>
                        </div>

                        <h3 class="text-sm font-medium text-zinc-400 mb-3">Extended Metadata Object</h3>
                        <p class="text-xs text-zinc-500 mb-3">The <code class="text-zinc-400">metadata</code> field accepts a JSON object with category-specific details:</p>
                        <div class="bg-white/[0.02] border border-zinc-800/50 rounded-xl overflow-hidden mb-6">
                            <table class="w-full text-sm">
                                <thead><tr class="border-b border-zinc-800/50">
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Key</th>
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Type</th>
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">For</th>
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Values</th>
                                </tr></thead>
                                <tbody>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">render_pipeline</td><td class="px-4 py-2.5 text-zinc-400">string</td><td class="px-4 py-2.5 text-zinc-500 text-xs">Materials & Shaders</td><td class="px-4 py-2.5 text-zinc-500 text-xs">pbr, unlit, custom, forward, deferred</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">texture_resolution</td><td class="px-4 py-2.5 text-zinc-400">string</td><td class="px-4 py-2.5 text-zinc-500 text-xs">Textures, Materials</td><td class="px-4 py-2.5 text-zinc-500 text-xs">WIDTHxHEIGHT (e.g. "2048x2048", "4096x4096")</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">poly_count</td><td class="px-4 py-2.5 text-zinc-400">integer</td><td class="px-4 py-2.5 text-zinc-500 text-xs">3D Models</td><td class="px-4 py-2.5 text-zinc-500 text-xs">Non-negative (e.g. 12500)</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">rigged</td><td class="px-4 py-2.5 text-zinc-400">boolean</td><td class="px-4 py-2.5 text-zinc-500 text-xs">3D Models</td><td class="px-4 py-2.5 text-zinc-500 text-xs">true if model has a skeleton rig</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">animated</td><td class="px-4 py-2.5 text-zinc-400">boolean</td><td class="px-4 py-2.5 text-zinc-500 text-xs">3D Models, Animations</td><td class="px-4 py-2.5 text-zinc-500 text-xs">true if contains animations</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">loopable</td><td class="px-4 py-2.5 text-zinc-400">boolean</td><td class="px-4 py-2.5 text-zinc-500 text-xs">Music, SFX, Animations</td><td class="px-4 py-2.5 text-zinc-500 text-xs">true if asset loops seamlessly</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">duration_seconds</td><td class="px-4 py-2.5 text-zinc-400">number</td><td class="px-4 py-2.5 text-zinc-500 text-xs">Music, SFX, Animations</td><td class="px-4 py-2.5 text-zinc-500 text-xs">Duration in seconds</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">bpm</td><td class="px-4 py-2.5 text-zinc-400">integer</td><td class="px-4 py-2.5 text-zinc-500 text-xs">Music</td><td class="px-4 py-2.5 text-zinc-500 text-xs">Beats per minute</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">tileable</td><td class="px-4 py-2.5 text-zinc-400">boolean</td><td class="px-4 py-2.5 text-zinc-500 text-xs">Textures, 2D Art</td><td class="px-4 py-2.5 text-zinc-500 text-xs">true if texture tiles seamlessly</td></tr>
                                    <tr><td class="px-4 py-2.5 text-zinc-200 font-mono text-xs">engine_version</td><td class="px-4 py-2.5 text-zinc-400">string</td><td class="px-4 py-2.5 text-zinc-500 text-xs">Plugins, Scripts, Blueprints</td><td class="px-4 py-2.5 text-zinc-500 text-xs">Minimum engine version (e.g. "0.2.0")</td></tr>
                                </tbody>
                            </table>
                        </div>

                        <h3 class="text-sm font-medium text-zinc-400 mb-3">Category Slugs</h3>
                        <div class="flex flex-wrap gap-1.5 mb-6">
                            ${['3d-models','animations','materials','textures','2d-art','particles','sfx','music','plugins','scripts','blueprints','projects','themes','fonts'].map(c =>
                                '<code class="px-2 py-1 rounded bg-white/[0.03] border border-zinc-800/50 text-xs text-zinc-400 font-mono">' + c + '</code>'
                            ).join('')}
                        </div>
                    </div>

                    <!-- Quick Start -->
                    <div class="mb-10">
                        <h2 class="text-xl font-semibold mb-4">Quick Start</h2>

                        <div class="space-y-4">
                            <div class="bg-white/[0.02] border border-zinc-800/50 rounded-xl p-5">
                                <p class="text-sm text-zinc-400 mb-3">Upload a shader with all fields:</p>
                                <pre class="bg-black/40 rounded-lg p-4 text-xs text-zinc-300 font-mono overflow-x-auto whitespace-pre">curl -X POST ${window.location.origin}/api/marketplace/upload \\
  -H "Authorization: Bearer rz_your_token_here" \\
  -F 'metadata={
    "name": "Hologram Effect",
    "description": "Animated holographic scan line shader with customizable parameters.",
    "category": "materials",
    "price_credits": 0,
    "version": "1.0.0",
    "tags": ["hologram", "sci-fi", "animated", "shader"],
    "licence": "cc0",
    "ai_generated": false,
    "metadata": {
      "render_pipeline": "pbr"
    }
  }' \\
  -F "file=@hologram.wgsl" \\
  -F "thumbnail=@hologram_preview.png" \\
  -F "screenshot_0=@screenshot1.png" \\
  -F "screenshot_1=@screenshot2.png"</pre>
                            </div>

                            <div class="bg-white/[0.02] border border-zinc-800/50 rounded-xl p-5">
                                <p class="text-sm text-zinc-400 mb-3">Upload a 3D model with metadata:</p>
                                <pre class="bg-black/40 rounded-lg p-4 text-xs text-zinc-300 font-mono overflow-x-auto whitespace-pre">curl -X POST ${window.location.origin}/api/marketplace/upload \\
  -H "Authorization: Bearer rz_your_token_here" \\
  -F 'metadata={
    "name": "Medieval Sword",
    "description": "Low-poly medieval sword with PBR textures.",
    "category": "3d-models",
    "price_credits": 500,
    "version": "1.0.0",
    "tags": ["medieval", "weapon", "low-poly", "pbr"],
    "licence": "standard",
    "metadata": {
      "poly_count": 2400,
      "rigged": false,
      "texture_resolution": "2048x2048"
    }
  }' \\
  -F "file=@medieval_sword.glb" \\
  -F "thumbnail=@sword_thumb.png" \\
  -F "video=@sword_turntable.mp4"</pre>
                            </div>

                            <div class="bg-white/[0.02] border border-zinc-800/50 rounded-xl p-5">
                                <p class="text-sm text-zinc-400 mb-3">Update an existing asset:</p>
                                <pre class="bg-black/40 rounded-lg p-4 text-xs text-zinc-300 font-mono overflow-x-auto whitespace-pre">curl -X PUT ${window.location.origin}/api/marketplace/ASSET_ID/update \\
  -H "Authorization: Bearer rz_your_token_here" \\
  -H "Content-Type: application/json" \\
  -d '{
    "description": "Updated description",
    "tags": ["hologram", "sci-fi", "vfx"],
    "licence": "mit",
    "metadata": { "render_pipeline": "pbr" }
  }'</pre>
                            </div>

                            <div class="bg-white/[0.02] border border-zinc-800/50 rounded-xl p-5">
                                <p class="text-sm text-zinc-400 mb-3">Delete an asset (removes files from storage):</p>
                                <pre class="bg-black/40 rounded-lg p-4 text-xs text-zinc-300 font-mono overflow-x-auto whitespace-pre">curl -X DELETE ${window.location.origin}/api/marketplace/ASSET_ID/delete \\
  -H "Authorization: Bearer rz_your_token_here"</pre>
                            </div>
                        </div>
                    </div>

                    <!-- Rate Limits -->
                    <div>
                        <h2 class="text-xl font-semibold mb-4">Rate Limits</h2>
                        <div class="bg-white/[0.02] border border-zinc-800/50 rounded-xl overflow-hidden">
                            <table class="w-full text-sm">
                                <thead><tr class="border-b border-zinc-800/50">
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Endpoint</th>
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Rate</th>
                                    <th class="text-left px-4 py-3 text-zinc-500 font-medium">Burst</th>
                                </tr></thead>
                                <tbody>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-300">General API</td><td class="px-4 py-2.5 text-zinc-400">30 req/s</td><td class="px-4 py-2.5 text-zinc-400">50</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-300">Authentication</td><td class="px-4 py-2.5 text-zinc-400">5 req/s</td><td class="px-4 py-2.5 text-zinc-400">10</td></tr>
                                    <tr class="border-b border-zinc-800/30"><td class="px-4 py-2.5 text-zinc-300">Uploads</td><td class="px-4 py-2.5 text-zinc-400">2 req/s</td><td class="px-4 py-2.5 text-zinc-400">5</td></tr>
                                    <tr><td class="px-4 py-2.5 text-zinc-300">API Tokens</td><td class="px-4 py-2.5 text-zinc-400">10 req/s</td><td class="px-4 py-2.5 text-zinc-400">5</td></tr>
                                </tbody>
                            </table>
                        </div>
                    </div>
                `;
            })();

            // ── App management ──
            async function loadApps() {
                if (!token) return;
                try {
                    const res = await fetch('/api/gameservices/apps', { headers: { 'Authorization': 'Bearer ' + token } });
                    if (!res.ok) return;
                    const apps = await res.json();
                    const el = document.getElementById('app-list');
                    if (!el) return;
                    if (apps.length === 0) { el.innerHTML = '<p class="text-sm text-zinc-500">No apps registered. Create one to get started with Game Services.</p>'; return; }
                    el.innerHTML = apps.map(a => `
                        <div class="p-4 bg-white/[0.02] border border-zinc-800/50 rounded-xl">
                            <div class="flex items-center justify-between mb-2">
                                <div class="flex items-center gap-3">
                                    <div class="w-10 h-10 rounded-lg bg-accent/10 flex items-center justify-center"><i class="ph ph-game-controller text-accent"></i></div>
                                    <div>
                                        <div class="font-medium">${a.name}</div>
                                        <div class="text-xs text-zinc-500">Client ID: <code class="text-zinc-400">${a.client_id}</code></div>
                                    </div>
                                </div>
                                <div class="flex items-center gap-2">
                                    <button onclick="manageAppTokens('${a.id}', '${a.name}')" class="px-3 py-1.5 rounded-lg text-xs text-accent hover:bg-accent/10 border border-transparent hover:border-accent/20 transition-all">Tokens</button>
                                    <button onclick="deleteApp('${a.id}', this)" class="px-3 py-1.5 rounded-lg text-xs text-red-400 hover:bg-red-950/30 border border-transparent hover:border-red-900/50 transition-all">Delete</button>
                                </div>
                            </div>
                            ${a.description ? '<p class="text-xs text-zinc-500 ml-13">' + a.description + '</p>' : ''}
                        </div>
                    `).join('');
                } catch(e) {}
            }
            loadApps();

            async function registerApp() {
                const name = prompt('App name (e.g. "My Game"):');
                if (!name) return;
                const desc = prompt('Short description:') || '';
                const website = prompt('Website URL (optional):') || '';
                try {
                    const res = await fetch('/api/gameservices/apps', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({ name, description: desc, website_url: website })
                    });
                    const data = await res.json();
                    if (!res.ok) { alert(data.error || 'Failed to register app'); return; }
                    alert('App registered!\\n\\nClient ID: ' + data.client_id + '\\nClient Secret: ' + data.client_secret + '\\n\\nSave the client secret — it will not be shown again.');
                    loadApps();
                } catch(e) { alert('Error: ' + e.message); }
            }

            async function deleteApp(id, btn) {
                if (!confirm('Delete this app? All tokens, achievements, and leaderboards will be removed.')) return;
                const res = await fetch('/api/gameservices/apps/' + id, { method: 'DELETE', headers: { 'Authorization': 'Bearer ' + token } });
                if (res.ok) { loadApps(); } else { alert('Failed to delete app'); }
            }

            async function manageAppTokens(appId, appName) {
                try {
                    const res = await fetch('/api/gameservices/apps/' + appId + '/tokens', { headers: { 'Authorization': 'Bearer ' + token } });
                    const tokens = await res.json();
                    const scopeList = ['profile:read','friends:read','friends:write','achievements:read','achievements:write','stats:read','stats:write','leaderboards:read','leaderboards:write','inventory:read'];
                    let msg = 'Tokens for ' + appName + ':\\n\\n';
                    if (tokens.length === 0) msg += '(no tokens)\\n';
                    else tokens.forEach(t => { msg += t.name + ' (' + t.prefix + '...) — ' + t.scopes.join(', ') + '\\n'; });
                    msg += '\\nCreate a new token? Enter a name (or Cancel):';
                    const tokenName = prompt(msg);
                    if (!tokenName) return;
                    const scopeInput = prompt('Enter scopes (comma separated):\\n' + scopeList.join(', '));
                    if (!scopeInput) return;
                    const scopes = scopeInput.split(',').map(s => s.trim()).filter(Boolean);
                    const createRes = await fetch('/api/gameservices/apps/' + appId + '/tokens', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({ name: tokenName, scopes })
                    });
                    const data = await createRes.json();
                    if (!createRes.ok) { alert(data.error || 'Failed to create token'); return; }
                    alert('Token created!\\n\\n' + data.token + '\\n\\nScopes: ' + data.scopes.join(', ') + '\\n\\nSave this — it will not be shown again.');
                } catch(e) { alert('Error: ' + e.message); }
            }

            function tokenRow(t) {
                return '<div class="flex items-center justify-between p-3 bg-white/[0.02] border border-zinc-800/50 rounded-lg">' +
                    '<div class="flex items-center gap-3">' +
                        '<div class="w-8 h-8 rounded-lg bg-accent/10 flex items-center justify-center"><i class="ph ph-key text-accent text-sm"></i></div>' +
                        '<div>' +
                            '<div class="text-sm font-medium">' + t.name + '</div>' +
                            '<div class="text-xs text-zinc-600">' + t.prefix + '... · Created ' + new Date(t.created_at).toLocaleDateString() +
                            (t.last_used_at ? ' · Last used ' + new Date(t.last_used_at).toLocaleDateString() : ' · Never used') +
                            (t.expires_at ? ' · Expires ' + new Date(t.expires_at).toLocaleDateString() : '') + '</div>' +
                        '</div>' +
                    '</div>' +
                    '<button onclick="revokeToken(\'' + t.id + '\', this)" class="px-3 py-1.5 rounded-lg text-xs text-red-400 hover:bg-red-950/30 hover:text-red-300 border border-transparent hover:border-red-900/50 transition-all">Revoke</button>' +
                '</div>';
            }

            function apiEndpoint(method, path, desc, body) {
                var methodColor = method === 'GET' ? 'text-green-400 bg-green-950/30' :
                    method === 'POST' ? 'text-blue-400 bg-blue-950/30' :
                    method === 'PUT' ? 'text-amber-400 bg-amber-950/30' :
                    'text-red-400 bg-red-950/30';
                return '<div class="p-4 bg-white/[0.02] border border-zinc-800/50 rounded-xl">' +
                    '<div class="flex items-center gap-3 mb-1">' +
                        '<span class="px-2 py-0.5 rounded text-xs font-mono font-bold ' + methodColor + '">' + method + '</span>' +
                        '<code class="text-sm text-zinc-300 font-mono">' + path + '</code>' +
                    '</div>' +
                    '<p class="text-xs text-zinc-500 ml-14">' + desc + '</p>' +
                    (body ? '<pre class="mt-2 ml-14 text-xs text-zinc-600 font-mono">' + body + '</pre>' : '') +
                '</div>';
            }

            async function createToken() {
                var name = prompt('Token name (e.g. "upload-bot"):');
                if (!name) return;
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                try {
                    var res = await fetch('/api/api-tokens', {
                        method: 'POST',
                        headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                        body: JSON.stringify({ name: name })
                    });
                    var data = await res.json();
                    if (!res.ok) { alert(data.message || 'Failed to create token'); return; }
                    document.getElementById('new-token-value').textContent = data.token;
                    document.getElementById('new-token-banner').classList.remove('hidden');
                    // Reload token list
                    window.location.reload();
                } catch(e) { alert('Error: ' + e.message); }
            }

            async function revokeToken(id, btn) {
                if (!confirm('Revoke this API token? Any integrations using it will stop working.')) return;
                var token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) return;
                var res = await fetch('/api/api-tokens/' + id, {
                    method: 'DELETE',
                    headers: { 'Authorization': 'Bearer ' + token }
                });
                if (res.ok) { btn.closest('[class*="flex items-center justify-between"]').remove(); }
                else { alert('Failed to revoke token'); }
            }

            function copyToken() {
                var val = document.getElementById('new-token-value').textContent;
                navigator.clipboard.writeText(val);
            }
            "##
        </script>
    }
}
