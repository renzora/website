use leptos::prelude::*;

#[component]
pub fn UploadPage() -> impl IntoView {
    view! {
        <section class="py-12 px-6 min-h-[80vh]">
            <div class="max-w-3xl mx-auto">

                // ── Auth gate ──
                <div id="auth-required" class="hidden text-center py-20">
                    <div class="w-16 h-16 bg-zinc-800/50 rounded-full flex items-center justify-center mx-auto mb-4">
                        <i class="ph ph-lock text-2xl text-zinc-500"></i>
                    </div>
                    <p class="text-zinc-400 mb-4">"Sign in to publish content"</p>
                    <a href="/login" class="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all">"Sign in"</a>
                </div>

                // ── Wizard container ──
                <div id="wizard" class="hidden">

                    // Header
                    <div class="mb-8">
                        <a href="/dashboard" class="inline-flex items-center gap-1.5 text-sm text-zinc-500 hover:text-zinc-300 transition-colors mb-4">
                            <i class="ph ph-arrow-left"></i>" Back to Dashboard"
                        </a>
                        <h1 class="text-3xl font-bold">"Publish Content"</h1>
                        <p class="text-zinc-400 text-sm mt-2">"Share your creation with the Renzora community."</p>
                    </div>

                    // Progress indicator
                    <div class="flex items-center justify-center gap-2 mb-10">
                        <div id="dot-1" class="w-3 h-3 rounded-full bg-accent transition-colors"></div>
                        <div class="w-12 h-0.5 bg-zinc-800"><div id="bar-1" class="h-full bg-accent transition-all duration-300" style="width:0%"></div></div>
                        <div id="dot-2" class="w-3 h-3 rounded-full bg-zinc-700 transition-colors"></div>
                        <div class="w-12 h-0.5 bg-zinc-800"><div id="bar-2" class="h-full bg-accent transition-all duration-300" style="width:0%"></div></div>
                        <div id="dot-3" class="w-3 h-3 rounded-full bg-zinc-700 transition-colors"></div>
                        <div class="w-12 h-0.5 bg-zinc-800"><div id="bar-3" class="h-full bg-accent transition-all duration-300" style="width:0%"></div></div>
                        <div id="dot-4" class="w-3 h-3 rounded-full bg-zinc-700 transition-colors"></div>
                        <div class="w-12 h-0.5 bg-zinc-800"><div id="bar-4" class="h-full bg-accent transition-all duration-300" style="width:0%"></div></div>
                        <div id="dot-5" class="w-3 h-3 rounded-full bg-zinc-700 transition-colors"></div>
                        <div class="w-12 h-0.5 bg-zinc-800"><div id="bar-5" class="h-full bg-accent transition-all duration-300" style="width:0%"></div></div>
                        <div id="dot-6" class="w-3 h-3 rounded-full bg-zinc-700 transition-colors"></div>
                    </div>

                    // Step label
                    <div class="text-center mb-6">
                        <p id="step-label" class="text-xs text-zinc-500 uppercase tracking-wider font-medium">"Step 1 of 6 — Content Type"</p>
                    </div>

                    // Error / success
                    <div id="wizard-error" class="hidden mb-6 p-4 rounded-xl bg-red-500/10 border border-red-500/20 text-red-400 text-sm flex items-center gap-2">
                        <i class="ph ph-warning-circle text-lg"></i>
                        <span id="wizard-error-text"></span>
                    </div>
                    <div id="wizard-success" class="hidden mb-6 p-4 rounded-xl bg-green-500/10 border border-green-500/20 text-green-400 text-sm flex items-center gap-2">
                        <i class="ph ph-check-circle text-lg"></i>
                        <span id="wizard-success-text"></span>
                    </div>

                    // ════════════════════════════════════════
                    // STEP 1 — Content Type
                    // ════════════════════════════════════════
                    <div id="step-1" class="wizard-step">
                        <div class="grid grid-cols-2 gap-4">
                            <button type="button" onclick="selectContentType('asset')"
                                class="group p-8 bg-white/[0.02] border border-zinc-800/50 rounded-2xl text-left hover:border-accent/40 hover:bg-accent/[0.03] transition-all">
                                <div class="w-12 h-12 bg-accent/10 rounded-xl flex items-center justify-center mb-4 group-hover:bg-accent/20 transition-colors">
                                    <i class="ph ph-package text-2xl text-accent"></i>
                                </div>
                                <h3 class="text-lg font-semibold mb-1">"Marketplace Asset"</h3>
                                <p class="text-sm text-zinc-500">"3D models, scripts, audio, textures, plugins, and more."</p>
                            </button>
                            <button type="button" onclick="selectContentType('game')"
                                class="group p-8 bg-white/[0.02] border border-zinc-800/50 rounded-2xl text-left hover:border-accent/40 hover:bg-accent/[0.03] transition-all">
                                <div class="w-12 h-12 bg-accent/10 rounded-xl flex items-center justify-center mb-4 group-hover:bg-accent/20 transition-colors">
                                    <i class="ph ph-game-controller text-2xl text-accent"></i>
                                </div>
                                <h3 class="text-lg font-semibold mb-1">"Game"</h3>
                                <p class="text-sm text-zinc-500">"Publish a playable game for the Renzora community."</p>
                            </button>
                        </div>
                    </div>

                    // ════════════════════════════════════════
                    // STEP 2 — Category
                    // ════════════════════════════════════════
                    <div id="step-2" class="wizard-step hidden">
                        <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl">
                            <h2 class="text-base font-semibold mb-1">"Choose a category"</h2>
                            <p class="text-sm text-zinc-500 mb-5">"This helps buyers find your content."</p>
                            <div id="category-grid" class="grid grid-cols-3 gap-3">
                                <div class="text-center text-sm text-zinc-500 col-span-3 py-8">"Loading categories..."</div>
                            </div>
                        </div>
                        <div class="flex gap-3 mt-6">
                            <button type="button" onclick="prevStep()" class="px-5 py-2.5 rounded-xl text-sm font-medium bg-white/[0.05] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200 transition-all">
                                <i class="ph ph-arrow-left mr-1"></i>"Back"
                            </button>
                        </div>
                    </div>

                    // ════════════════════════════════════════
                    // STEP 3 — Basic Info
                    // ════════════════════════════════════════
                    <div id="step-3" class="wizard-step hidden">
                        <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5">
                            <h2 class="text-base font-semibold flex items-center gap-2">
                                <i class="ph ph-info text-accent"></i>"Basic Information"
                            </h2>

                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">"Name" <span class="text-red-400">"*"</span></label>
                                <input type="text" id="w-name" required maxlength="128" placeholder="My Awesome Creation"
                                    class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                            </div>

                            <div>
                                <label class="block text-sm text-zinc-400 mb-1.5">"Description" <span class="text-red-400">"*"</span></label>
                                <textarea id="w-description" required rows="5" placeholder="Describe what this is, what's included, and how to use it..."
                                    class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all resize-y"></textarea>
                                <p class="text-xs text-zinc-600 mt-1">"Markdown is not supported. Keep it clear and concise."</p>
                            </div>

                            <div class="grid grid-cols-2 gap-4">
                                <div>
                                    <label class="block text-sm text-zinc-400 mb-1.5">"Version"</label>
                                    <input type="text" id="w-version" value="1.0.0" placeholder="1.0.0"
                                        class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                                </div>
                                <div>
                                    <label class="block text-sm text-zinc-400 mb-1.5">"Price (credits)"</label>
                                    <input type="number" id="w-price" min="0" value="0" oninput="updatePricePreview()"
                                        class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                                    <p class="text-xs text-zinc-600 mt-1" id="price-preview">"Free — anyone can download"</p>
                                    <p class="text-xs text-zinc-600 mt-0.5">"You earn 80% of each sale. 1 credit = $0.10 USD."</p>
                                </div>
                            </div>

                            // Tags — asset only
                            <div id="tags-field" class="hidden">
                                <label class="block text-sm text-zinc-400 mb-1.5">"Tags"</label>
                                <div class="relative">
                                    <div id="tags-pills" class="flex flex-wrap gap-1.5 mb-2"></div>
                                    <input type="text" id="w-tags-input" placeholder="Type to search tags..."
                                        autocomplete="off"
                                        class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                                    <input type="hidden" id="w-tags" />
                                    <div id="tags-dropdown" class="hidden absolute left-0 right-0 top-full mt-1 bg-zinc-900 border border-zinc-700 rounded-xl shadow-lg z-50 max-h-48 overflow-y-auto"></div>
                                </div>
                                <p class="text-xs text-zinc-600 mt-1">"Add up to 5 tags. Press comma or click a suggestion. New tags are submitted for review."</p>
                            </div>

                            // Download filename — asset only
                            <div id="filename-field" class="hidden">
                                <label class="block text-sm text-zinc-400 mb-1.5">"Download Filename"</label>
                                <input type="text" id="w-download-filename" placeholder="my-asset.zip"
                                    class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                                <p class="text-xs text-zinc-600 mt-1">"The filename users will see when downloading. Auto-populated from your uploaded file."</p>
                            </div>

                            // Credit / Attribution — asset only
                            <div id="credit-field" class="hidden">
                                <div class="p-4 bg-white/[0.01] border border-zinc-800/30 rounded-xl space-y-4">
                                    <div class="flex items-center gap-2">
                                        <i class="ph ph-heart text-accent"></i>
                                        <p class="text-xs text-zinc-500 uppercase tracking-wider font-medium">"Credit / Attribution"</p>
                                    </div>
                                    <p class="text-xs text-zinc-600">"If this asset is from another creator, credit them here. Credited assets are automatically free."</p>
                                    <div>
                                        <label class="block text-sm text-zinc-400 mb-1.5">"Original Creator Name"</label>
                                        <input type="text" id="w-credit-name" placeholder="e.g. KayKit, Kenney"
                                            oninput="updateCreditState()"
                                            class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                                    </div>
                                    <div>
                                        <label class="block text-sm text-zinc-400 mb-1.5">"Creator Website / Source Link"</label>
                                        <input type="text" id="w-credit-url" placeholder="https://kaykit.itch.io"
                                            class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                                    </div>
                                    <div id="credit-free-notice" class="hidden p-3 bg-green-500/5 border border-green-500/10 rounded-lg">
                                        <p class="text-xs text-green-400 flex items-center gap-1.5">
                                            <i class="ph ph-info"></i>
                                            "This asset will be published as free because it credits another creator."
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="flex gap-3 mt-6">
                            <button type="button" onclick="prevStep()" class="px-5 py-2.5 rounded-xl text-sm font-medium bg-white/[0.05] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200 transition-all">
                                <i class="ph ph-arrow-left mr-1"></i>"Back"
                            </button>
                            <button type="button" onclick="nextStep()" class="flex-1 px-5 py-2.5 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all">
                                "Continue"<i class="ph ph-arrow-right ml-1"></i>
                            </button>
                        </div>
                    </div>

                    // ════════════════════════════════════════
                    // STEP 4 — Type-Specific Details
                    // ════════════════════════════════════════
                    <div id="step-4" class="wizard-step hidden">
                        <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5">
                            <h2 class="text-base font-semibold flex items-center gap-2">
                                <i class="ph ph-sliders-horizontal text-accent"></i>"Additional Details"
                            </h2>

                            // ── Asset-only fields ──
                            <div data-show-for="asset" class="hidden space-y-5">
                                <div>
                                    <label class="flex items-start gap-3 cursor-pointer select-none">
                                        <input type="checkbox" id="w-ai-generated" class="mt-1 accent-accent w-4 h-4" />
                                        <div>
                                            <span class="text-sm text-zinc-300">"This asset was created with AI assistance"</span>
                                            <p class="text-xs text-zinc-600 mt-0.5">"Check this if AI tools were used to generate content in this asset."</p>
                                        </div>
                                    </label>
                                </div>

                                <div>
                                    <label class="block text-sm text-zinc-400 mb-1.5">"Supported Engine Versions"</label>
                                    <input type="text" id="w-engine-versions" placeholder="r1-alpha4+"
                                        class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                                </div>

                                <div>
                                    <label class="block text-sm text-zinc-400 mb-1.5">"License"</label>
                                    <select id="w-license" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all">
                                        <option value="standard">"Standard Marketplace License"</option>
                                        <option value="mit">"MIT"</option>
                                        <option value="apache2">"Apache 2.0"</option>
                                        <option value="gpl3">"GPL 3.0"</option>
                                        <option value="cc-by">"CC BY 4.0"</option>
                                        <option value="cc0">"CC0 (Public Domain)"</option>
                                    </select>
                                </div>
                            </div>

                            // ── Audio fields (sfx, music) ──
                            <div data-show-for-category="sfx,music" class="hidden space-y-5">
                                <div class="p-4 bg-white/[0.01] border border-zinc-800/30 rounded-xl space-y-4">
                                    <p class="text-xs text-zinc-500 uppercase tracking-wider font-medium">"Audio Details"</p>
                                    <div class="grid grid-cols-2 gap-4">
                                        <div>
                                            <label class="block text-sm text-zinc-400 mb-1.5">"BPM"</label>
                                            <input type="number" id="w-bpm" min="1" max="999" placeholder="120"
                                                class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                                        </div>
                                        <div>
                                            <label class="block text-sm text-zinc-400 mb-1.5">"Genre"</label>
                                            <select id="w-genre" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all">
                                                <option value="">"Select genre..."</option>
                                                <option value="ambient">"Ambient"</option>
                                                <option value="orchestral">"Orchestral"</option>
                                                <option value="electronic">"Electronic"</option>
                                                <option value="retro">"Retro / Chiptune"</option>
                                                <option value="rock">"Rock"</option>
                                                <option value="cinematic">"Cinematic"</option>
                                                <option value="other">"Other"</option>
                                            </select>
                                        </div>
                                    </div>
                                    <label class="flex items-center gap-3 cursor-pointer select-none">
                                        <input type="checkbox" id="w-loopable" class="accent-accent w-4 h-4" />
                                        <span class="text-sm text-zinc-300">"Loop-friendly (seamless loop)"</span>
                                    </label>
                                </div>
                            </div>

                            // ── Script/plugin fields ──
                            <div data-show-for-category="scripts,plugins,blueprints" class="hidden space-y-5">
                                <div class="p-4 bg-white/[0.01] border border-zinc-800/30 rounded-xl space-y-4">
                                    <p class="text-xs text-zinc-500 uppercase tracking-wider font-medium">"Script Details"</p>
                                    <div>
                                        <label class="block text-sm text-zinc-400 mb-1.5">"Scripting Language"</label>
                                        <select id="w-script-lang" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all">
                                            <option value="">"Select..."</option>
                                            <option value="lua">"Lua"</option>
                                            <option value="rhai">"Rhai"</option>
                                            <option value="wgsl">"WGSL (Shader)"</option>
                                            <option value="blueprint">"Visual Blueprint"</option>
                                            <option value="other">"Other"</option>
                                        </select>
                                    </div>
                                    <div>
                                        <label class="block text-sm text-zinc-400 mb-1.5">"Dependencies"</label>
                                        <input type="text" id="w-dependencies" placeholder="e.g. physics-plugin, networking-core"
                                            class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                                        <p class="text-xs text-zinc-600 mt-1">"Comma-separated list of required plugins or assets."</p>
                                    </div>
                                </div>
                            </div>

                            // ── 3D model fields ──
                            <div data-show-for-category="3d-models,animations" class="hidden space-y-5">
                                <div class="p-4 bg-white/[0.01] border border-zinc-800/30 rounded-xl space-y-4">
                                    <p class="text-xs text-zinc-500 uppercase tracking-wider font-medium">"3D Details"</p>
                                    <div class="grid grid-cols-2 gap-4">
                                        <div>
                                            <label class="block text-sm text-zinc-400 mb-1.5">"Polygon Count"</label>
                                            <input type="text" id="w-polycount" placeholder="e.g. 12,500 tris"
                                                class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                                        </div>
                                        <div>
                                            <label class="block text-sm text-zinc-400 mb-1.5">"Texture Resolution"</label>
                                            <select id="w-texres" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all">
                                                <option value="">"Select..."</option>
                                                <option value="512">"512x512"</option>
                                                <option value="1024">"1024x1024"</option>
                                                <option value="2048">"2048x2048"</option>
                                                <option value="4096">"4096x4096"</option>
                                            </select>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            // ── 2D art fields ──
                            <div data-show-for-category="2d-art,textures,particles" class="hidden space-y-5">
                                <div class="p-4 bg-white/[0.01] border border-zinc-800/30 rounded-xl space-y-4">
                                    <p class="text-xs text-zinc-500 uppercase tracking-wider font-medium">"2D / Texture Details"</p>
                                    <div class="grid grid-cols-2 gap-4">
                                        <div>
                                            <label class="block text-sm text-zinc-400 mb-1.5">"Resolution"</label>
                                            <input type="text" id="w-resolution" placeholder="e.g. 1024x1024"
                                                class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                                        </div>
                                        <div>
                                            <label class="block text-sm text-zinc-400 mb-1.5">"Tile-friendly"</label>
                                            <label class="flex items-center gap-3 cursor-pointer select-none mt-2">
                                                <input type="checkbox" id="w-tileable" class="accent-accent w-4 h-4" />
                                                <span class="text-sm text-zinc-300">"Seamlessly tileable"</span>
                                            </label>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            // ── Materials/shaders fields ──
                            <div data-show-for-category="materials" class="hidden space-y-5">
                                <div class="p-4 bg-white/[0.01] border border-zinc-800/30 rounded-xl space-y-4">
                                    <p class="text-xs text-zinc-500 uppercase tracking-wider font-medium">"Material Details"</p>
                                    <div>
                                        <label class="block text-sm text-zinc-400 mb-1.5">"Render Pipeline"</label>
                                        <select id="w-pipeline" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all">
                                            <option value="">"Select..."</option>
                                            <option value="pbr">"PBR (Physically Based)"</option>
                                            <option value="unlit">"Unlit"</option>
                                            <option value="toon">"Toon / Cel-Shaded"</option>
                                            <option value="custom">"Custom WGSL"</option>
                                        </select>
                                    </div>
                                    <div>
                                        <label class="block text-sm text-zinc-400 mb-1.5">"Texture Resolution"</label>
                                        <select id="w-mat-texres" class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all">
                                            <option value="">"Select..."</option>
                                            <option value="512">"512x512"</option>
                                            <option value="1024">"1024x1024"</option>
                                            <option value="2048">"2048x2048"</option>
                                            <option value="4096">"4096x4096"</option>
                                        </select>
                                    </div>
                                </div>
                            </div>

                            // ── Game-only fields ──
                            <div data-show-for="game" class="hidden space-y-5">
                                <div>
                                    <label class="block text-sm text-zinc-400 mb-1.5">"Platforms"</label>
                                    <div class="flex flex-wrap gap-3">
                                        <label class="flex items-center gap-2 cursor-pointer select-none">
                                            <input type="checkbox" id="w-platform-windows" class="accent-accent w-4 h-4" checked />
                                            <span class="text-sm text-zinc-300">"Windows"</span>
                                        </label>
                                        <label class="flex items-center gap-2 cursor-pointer select-none">
                                            <input type="checkbox" id="w-platform-mac" class="accent-accent w-4 h-4" />
                                            <span class="text-sm text-zinc-300">"macOS"</span>
                                        </label>
                                        <label class="flex items-center gap-2 cursor-pointer select-none">
                                            <input type="checkbox" id="w-platform-linux" class="accent-accent w-4 h-4" />
                                            <span class="text-sm text-zinc-300">"Linux"</span>
                                        </label>
                                        <label class="flex items-center gap-2 cursor-pointer select-none">
                                            <input type="checkbox" id="w-platform-web" class="accent-accent w-4 h-4" />
                                            <span class="text-sm text-zinc-300">"Web"</span>
                                        </label>
                                    </div>
                                </div>
                                <div>
                                    <label class="block text-sm text-zinc-400 mb-1.5">"Minimum System Requirements"</label>
                                    <textarea id="w-sysreq" rows="3" placeholder="OS: Windows 10+, RAM: 4GB, GPU: OpenGL 3.3+"
                                        class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all resize-y"></textarea>
                                </div>
                            </div>

                            // ── Empty state ──
                            <div id="step4-empty" class="hidden text-center py-6">
                                <p class="text-sm text-zinc-500">"No additional details needed for this category."</p>
                            </div>
                        </div>

                        <div class="flex gap-3 mt-6">
                            <button type="button" onclick="prevStep()" class="px-5 py-2.5 rounded-xl text-sm font-medium bg-white/[0.05] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200 transition-all">
                                <i class="ph ph-arrow-left mr-1"></i>"Back"
                            </button>
                            <button type="button" onclick="nextStep()" class="flex-1 px-5 py-2.5 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all">
                                "Continue"<i class="ph ph-arrow-right ml-1"></i>
                            </button>
                        </div>
                    </div>

                    // ════════════════════════════════════════
                    // STEP 5 — Files & Media
                    // ════════════════════════════════════════
                    <div id="step-5" class="wizard-step hidden">
                        <div class="space-y-6">
                            // Main file
                            <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5">
                                <h2 class="text-base font-semibold flex items-center gap-2">
                                    <i class="ph ph-file-arrow-up text-cyan-400"></i><span id="file-section-title">"Files"</span>
                                </h2>

                                <div>
                                    <label class="block text-sm text-zinc-400 mb-1.5" id="file-label-text">"File" <span class="text-red-400">"*"</span></label>
                                    <div id="file-dropzone" class="relative border-2 border-dashed border-zinc-800/50 rounded-xl p-8 text-center hover:border-accent/30 transition-all cursor-pointer"
                                        onclick="document.getElementById('w-file').click()">
                                        <i class="ph ph-file-arrow-up text-2xl text-zinc-600 mb-2"></i>
                                        <p id="file-drop-label" class="text-sm text-zinc-500">"Drop a file or click to browse"</p>
                                        <p id="file-hint" class="text-xs text-zinc-600 mt-2"></p>
                                        <input type="file" id="w-file" class="hidden" onchange="previewMainFile(this)" />
                                    </div>
                                </div>

                                <div>
                                    <label class="block text-sm text-zinc-400 mb-1.5">"Cover Image"</label>
                                    <p class="text-xs text-zinc-600 mb-2" id="thumb-hint">"Recommended: 1280x720 (16:9). PNG or JPG."</p>
                                    <div id="thumb-dropzone" class="relative border-2 border-dashed border-zinc-800/50 rounded-xl p-6 text-center hover:border-accent/30 transition-all cursor-pointer"
                                        onclick="document.getElementById('w-thumbnail').click()">
                                        <i class="ph ph-image text-2xl text-zinc-600 mb-2" id="thumb-icon"></i>
                                        <p id="thumb-label" class="text-sm text-zinc-500">"Drop an image or click to browse"</p>
                                        <img id="thumb-preview" class="hidden mt-3 max-h-40 mx-auto rounded-lg" />
                                        <input type="file" id="w-thumbnail" accept="image/*" class="hidden" onchange="previewThumb(this)" />
                                    </div>
                                </div>
                            </div>

                            // Screenshots
                            <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-5">
                                <h2 class="text-base font-semibold flex items-center gap-2">
                                    <i class="ph ph-images text-purple-400"></i>"Screenshots & Media"
                                </h2>
                                <p class="text-xs text-zinc-500 -mt-2">"Add up to 10 screenshots. These appear in the gallery."</p>

                                <div>
                                    <label class="block text-sm text-zinc-400 mb-1.5">"Screenshots"</label>
                                    <input type="file" id="w-screenshots" accept="image/*" multiple onchange="updateScreenshotCount(this)"
                                        class="w-full text-sm text-zinc-400 file:mr-4 file:py-2 file:px-4 file:rounded-xl file:border-0 file:text-sm file:font-medium file:bg-white/[0.05] file:text-zinc-300 hover:file:bg-white/[0.08] file:cursor-pointer file:transition-colors" />
                                    <p class="text-xs text-zinc-600 mt-1" id="screenshot-count">"Select multiple images at once. PNG or JPG."</p>
                                    <div id="screenshot-previews" class="flex gap-2 mt-3 flex-wrap"></div>
                                </div>

                                // Asset-only: video + audio
                                <div id="media-extras" class="hidden space-y-5">
                                    <div>
                                        <label class="block text-sm text-zinc-400 mb-1.5">"Video Preview URL (optional)"</label>
                                        <input type="text" id="w-video-url" placeholder="https://www.youtube.com/watch?v=... or direct .mp4 link"
                                            class="w-full px-4 py-3 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-zinc-50 text-sm outline-none focus:border-accent/50 transition-all" />
                                        <p class="text-xs text-zinc-600 mt-1">"YouTube links are automatically embedded."</p>
                                    </div>

                                    <div>
                                        <label class="block text-sm text-zinc-400 mb-1.5">"Audio Previews (optional)"</label>
                                        <input type="file" id="w-audio" accept="audio/mpeg,audio/wav,audio/ogg,audio/flac,.mp3,.wav,.ogg,.flac" multiple
                                            class="w-full text-sm text-zinc-400 file:mr-4 file:py-2 file:px-4 file:rounded-xl file:border-0 file:text-sm file:font-medium file:bg-white/[0.05] file:text-zinc-300 hover:file:bg-white/[0.08] file:cursor-pointer file:transition-colors" />
                                        <p class="text-xs text-zinc-600 mt-1">"Upload audio samples. MP3, WAV, OGG, or FLAC."</p>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="flex gap-3 mt-6">
                            <button type="button" onclick="prevStep()" class="px-5 py-2.5 rounded-xl text-sm font-medium bg-white/[0.05] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200 transition-all">
                                <i class="ph ph-arrow-left mr-1"></i>"Back"
                            </button>
                            <button type="button" onclick="nextStep()" class="flex-1 px-5 py-2.5 rounded-xl text-sm font-medium bg-accent text-white hover:bg-accent-hover transition-all">
                                "Continue"<i class="ph ph-arrow-right ml-1"></i>
                            </button>
                        </div>
                    </div>

                    // ════════════════════════════════════════
                    // STEP 6 — Review & Submit
                    // ════════════════════════════════════════
                    <div id="step-6" class="wizard-step hidden">
                        <div class="p-6 bg-white/[0.02] border border-zinc-800/50 rounded-2xl space-y-4">
                            <h2 class="text-base font-semibold flex items-center gap-2">
                                <i class="ph ph-check-circle text-green-400"></i>"Review & Publish"
                            </h2>
                            <p class="text-sm text-zinc-500">"Review your submission before uploading."</p>

                            <div id="review-summary" class="space-y-3 mt-4"></div>
                        </div>

                        <div class="flex gap-3 mt-6">
                            <button type="button" onclick="prevStep()" class="px-5 py-2.5 rounded-xl text-sm font-medium bg-white/[0.05] border border-zinc-800/50 text-zinc-400 hover:border-zinc-600 hover:text-zinc-200 transition-all">
                                <i class="ph ph-arrow-left mr-1"></i>"Back"
                            </button>
                            <button type="button" onclick="handleSubmit(true)" id="publish-btn"
                                class="flex-1 inline-flex items-center justify-center gap-2 px-5 py-3 rounded-xl text-sm font-semibold bg-accent text-white hover:bg-accent-hover transition-all hover:shadow-[0_0_20px_rgba(99,102,241,0.2)]">
                                <i class="ph ph-rocket-launch text-lg"></i>"Publish"
                            </button>
                        </div>

                        <p class="text-xs text-zinc-600 text-center mt-3">"By publishing, you agree to the Renzora "<a href="/docs/marketplace/guidelines" class="text-accent hover:text-accent-hover">"content guidelines"</a>"."</p>
                    </div>

                </div>
            </div>
        </section>

        <script>
        r##"
        // ──────────────────────────────────────
        // Wizard State
        // ──────────────────────────────────────
        const W = {
            step: 1,
            contentType: null,  // 'asset' or 'game'
            category: null,     // slug
            categoryName: null, // display name
        };

        async function safeJson(res) {
            const text = await res.text();
            if (!text) return {};
            try { return JSON.parse(text); }
            catch(e) { return { error: text.substring(0, 200) }; }
        }

        const STEP_LABELS = [
            'Content Type',
            'Category',
            'Basic Information',
            'Additional Details',
            'Files & Media',
            'Review & Publish'
        ];

        // ──────────────────────────────────────
        // Navigation
        // ──────────────────────────────────────
        function goToStep(n) {
            if (n < 1 || n > 6) return;

            // Hide current
            const cur = document.getElementById('step-' + W.step);
            if (cur) cur.classList.add('hidden');

            W.step = n;

            // Show target
            const target = document.getElementById('step-' + n);
            if (target) target.classList.remove('hidden');

            // Update progress dots and bars
            for (let i = 1; i <= 6; i++) {
                const dot = document.getElementById('dot-' + i);
                if (i <= n) {
                    dot.classList.remove('bg-zinc-700');
                    dot.classList.add('bg-accent');
                } else {
                    dot.classList.remove('bg-accent');
                    dot.classList.add('bg-zinc-700');
                }
            }
            for (let i = 1; i <= 5; i++) {
                const bar = document.getElementById('bar-' + i);
                bar.style.width = i < n ? '100%' : '0%';
            }

            // Update label
            document.getElementById('step-label').textContent = 'Step ' + n + ' of 6 — ' + STEP_LABELS[n - 1];

            // Hide errors
            document.getElementById('wizard-error').classList.add('hidden');

            // Step-specific setup
            if (n === 3) setupStep3();
            if (n === 4) setupStep4();
            if (n === 5) setupStep5();
            if (n === 6) setupStep6();

            window.scrollTo({ top: 0, behavior: 'smooth' });
        }

        function nextStep() {
            const err = validateStep(W.step);
            if (err) {
                showError(err);
                return;
            }
            goToStep(W.step + 1);
        }

        function prevStep() {
            goToStep(W.step - 1);
        }

        function validateStep(n) {
            if (n === 3) {
                const name = document.getElementById('w-name').value.trim();
                const desc = document.getElementById('w-description').value.trim();
                if (!name) return 'Name is required.';
                if (!desc) return 'Description is required.';
            }
            if (n === 5) {
                if (!document.getElementById('w-file').files[0]) return 'Please select a file to upload.';
            }
            return null;
        }

        function showError(msg) {
            const el = document.getElementById('wizard-error');
            document.getElementById('wizard-error-text').textContent = msg;
            el.classList.remove('hidden');
        }

        // ──────────────────────────────────────
        // Step 1 — Content Type
        // ──────────────────────────────────────
        function selectContentType(type) {
            W.contentType = type;
            loadCategories(type);
            goToStep(2);
        }

        async function loadCategories(type) {
            const grid = document.getElementById('category-grid');
            grid.innerHTML = '<div class="text-center text-sm text-zinc-500 col-span-3 py-8">Loading categories...</div>';

            const url = type === 'game' ? '/api/games/categories' : '/api/marketplace/categories';
            try {
                const res = await fetch(url);
                if (!res.ok) throw new Error('Failed to load');
                const cats = await safeJson(res);
                if (!Array.isArray(cats)) throw new Error('Invalid response');
                grid.innerHTML = '';
                cats.forEach(cat => {
                    const btn = document.createElement('button');
                    btn.type = 'button';
                    btn.className = 'p-4 bg-white/[0.02] border border-zinc-800/50 rounded-xl text-left hover:border-accent/40 hover:bg-accent/[0.03] transition-all';
                    btn.onclick = () => selectCategory(cat.slug, cat.name);
                    btn.innerHTML = `
                        <div class="flex items-center gap-3">
                            <i class="${cat.icon || 'ph ph-folder'} text-lg text-accent"></i>
                            <span class="text-sm font-medium">${cat.name}</span>
                        </div>
                    `;
                    grid.appendChild(btn);
                });
            } catch (e) {
                grid.innerHTML = '<div class="text-center text-sm text-red-400 col-span-3 py-8">Failed to load categories</div>';
            }
        }

        function selectCategory(slug, name) {
            W.category = slug;
            W.categoryName = name;
            goToStep(3);
        }

        // ──────────────────────────────────────
        // Step 3 — Basic Info setup
        // ──────────────────────────────────────
        function setupStep3() {
            const tagsField = document.getElementById('tags-field');
            const filenameField = document.getElementById('filename-field');
            const creditField = document.getElementById('credit-field');
            if (W.contentType === 'asset') {
                tagsField.classList.remove('hidden');
                filenameField.classList.remove('hidden');
                creditField.classList.remove('hidden');
            } else {
                tagsField.classList.add('hidden');
                filenameField.classList.add('hidden');
                creditField.classList.add('hidden');
            }
        }

        function updateCreditState() {
            const creditName = document.getElementById('w-credit-name').value.trim();
            const notice = document.getElementById('credit-free-notice');
            const priceInput = document.getElementById('w-price');
            if (creditName) {
                notice.classList.remove('hidden');
                priceInput.value = '0';
                priceInput.disabled = true;
                updatePricePreview();
            } else {
                notice.classList.add('hidden');
                priceInput.disabled = false;
            }
        }

        // ── Tag autocomplete system ──
        const selectedTags = [];
        let tagSearchTimeout = null;

        function renderTagPills() {
            const container = document.getElementById('tags-pills');
            const hidden = document.getElementById('w-tags');
            container.innerHTML = '';
            selectedTags.forEach((tag, i) => {
                const pill = document.createElement('span');
                pill.className = 'inline-flex items-center gap-1 px-2.5 py-1 bg-accent/15 text-accent text-xs font-medium rounded-lg';
                pill.innerHTML = escHtml(tag) + ' <button type="button" class="hover:text-white ml-0.5" onclick="removeTag(' + i + ')">&times;</button>';
                container.appendChild(pill);
            });
            hidden.value = selectedTags.join(',');
        }

        function addTag(name) {
            const clean = name.trim().toLowerCase();
            if (!clean || selectedTags.length >= 5 || selectedTags.includes(clean)) return;
            selectedTags.push(clean);
            renderTagPills();
            document.getElementById('w-tags-input').value = '';
            document.getElementById('tags-dropdown').classList.add('hidden');
        }

        function removeTag(index) {
            selectedTags.splice(index, 1);
            renderTagPills();
        }

        async function searchTags(query) {
            const dropdown = document.getElementById('tags-dropdown');
            if (!query || query.length < 1) { dropdown.classList.add('hidden'); return; }
            try {
                const res = await fetch('/api/marketplace/tags?q=' + encodeURIComponent(query));
                const tags = await safeJson(res);
                if (!Array.isArray(tags) || tags.length === 0) {
                    // Show "submit new tag" option
                    dropdown.innerHTML = '<div class="px-3 py-2 text-xs text-zinc-500">No matching tags</div>'
                        + '<button type="button" class="w-full px-3 py-2 text-left text-sm text-accent hover:bg-white/[0.05] transition-colors" onclick="submitNewTag(\'' + escHtml(query).replace(/'/g, "\\'") + '\')">+ Submit &quot;' + escHtml(query) + '&quot; as new tag</button>';
                    dropdown.classList.remove('hidden');
                    return;
                }
                dropdown.innerHTML = '';
                tags.forEach(t => {
                    if (selectedTags.includes(t.name)) return;
                    const btn = document.createElement('button');
                    btn.type = 'button';
                    btn.className = 'w-full px-3 py-2 text-left text-sm text-zinc-200 hover:bg-white/[0.05] transition-colors';
                    btn.textContent = t.name;
                    btn.onclick = () => addTag(t.name);
                    dropdown.appendChild(btn);
                });
                // Always offer to submit as new if not in results
                const names = tags.map(t => t.name.toLowerCase());
                if (!names.includes(query.toLowerCase())) {
                    const btn = document.createElement('button');
                    btn.type = 'button';
                    btn.className = 'w-full px-3 py-2 text-left text-sm text-accent hover:bg-white/[0.05] transition-colors border-t border-zinc-800';
                    btn.innerHTML = '+ Submit &quot;' + escHtml(query) + '&quot; as new tag';
                    btn.onclick = () => submitNewTag(query);
                    dropdown.appendChild(btn);
                }
                dropdown.classList.remove('hidden');
            } catch (e) {
                dropdown.classList.add('hidden');
            }
        }

        async function submitNewTag(name) {
            const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
            if (!token) return;
            try {
                const res = await fetch('/api/marketplace/tags/submit', {
                    method: 'POST',
                    headers: { 'Authorization': 'Bearer ' + token, 'Content-Type': 'application/json' },
                    body: JSON.stringify({ name: name.trim() })
                });
                const data = await safeJson(res);
                if (res.ok) {
                    addTag(data.name || name.trim());
                }
            } catch (e) {}
        }

        // Wire up tag input events
        document.getElementById('w-tags-input').addEventListener('input', function(e) {
            clearTimeout(tagSearchTimeout);
            const val = e.target.value;
            // Check for comma — add tag immediately
            if (val.includes(',')) {
                const parts = val.split(',');
                parts.forEach((p, i) => {
                    if (i < parts.length - 1 && p.trim()) addTag(p.trim());
                });
                e.target.value = parts[parts.length - 1];
                return;
            }
            tagSearchTimeout = setTimeout(() => searchTags(val.trim()), 200);
        });

        document.getElementById('w-tags-input').addEventListener('keydown', function(e) {
            if (e.key === 'Backspace' && !e.target.value && selectedTags.length > 0) {
                removeTag(selectedTags.length - 1);
            }
        });

        // Close dropdown on outside click
        document.addEventListener('click', function(e) {
            const dropdown = document.getElementById('tags-dropdown');
            const input = document.getElementById('w-tags-input');
            if (dropdown && !dropdown.contains(e.target) && e.target !== input) {
                dropdown.classList.add('hidden');
            }
        });

        function updatePricePreview() {
            const price = parseInt(document.getElementById('w-price').value) || 0;
            const el = document.getElementById('price-preview');
            if (price === 0) {
                el.textContent = 'Free \u2014 anyone can download';
            } else {
                const usd = (price * 0.10).toFixed(2);
                const earn = (price * 0.08).toFixed(2);
                el.textContent = price + ' credits ($' + usd + ') \u2014 you earn ' + Math.floor(price * 0.8) + ' credits ($' + earn + ')';
            }
        }

        // ──────────────────────────────────────
        // Step 4 — Adaptive detail fields
        // ──────────────────────────────────────
        function setupStep4() {
            let hasVisible = false;

            // Show/hide by content type
            document.querySelectorAll('[data-show-for]').forEach(el => {
                const types = el.dataset.showFor.split(',');
                if (types.includes(W.contentType)) {
                    el.classList.remove('hidden');
                    hasVisible = true;
                } else {
                    el.classList.add('hidden');
                }
            });

            // Show/hide by category
            document.querySelectorAll('[data-show-for-category]').forEach(el => {
                const cats = el.dataset.showForCategory.split(',');
                if (cats.includes(W.category)) {
                    el.classList.remove('hidden');
                    hasVisible = true;
                } else {
                    el.classList.add('hidden');
                }
            });

            // Show empty state if nothing visible
            const empty = document.getElementById('step4-empty');
            if (!hasVisible) {
                empty.classList.remove('hidden');
            } else {
                empty.classList.add('hidden');
            }
        }

        // ──────────────────────────────────────
        // Step 5 — Files setup
        // ──────────────────────────────────────
        function setupStep5() {
            const fileInput = document.getElementById('w-file');
            const hint = document.getElementById('file-hint');
            const thumbHint = document.getElementById('thumb-hint');
            const extras = document.getElementById('media-extras');
            const title = document.getElementById('file-section-title');

            if (W.contentType === 'game') {
                fileInput.accept = '.zip,.exe,.tar.gz,.dmg,.appimage';
                hint.textContent = 'ZIP, EXE, .tar.gz, .dmg, or .appimage \u2014 Max 2 GB';
                thumbHint.textContent = 'Recommended: 1920\u00d71080 (16:9). PNG or JPG.';
                title.textContent = 'Game Files';
                extras.classList.add('hidden');
            } else {
                fileInput.accept = '.zip,.rar,.7z,.lua,.rhai,.wgsl,.fbx,.obj,.gltf,.glb,.blend,.png,.jpg,.svg,.wav,.ogg,.mp3,.flac,.ttf,.otf';
                hint.textContent = 'Accepted formats vary by category \u2014 Max 50 MB';
                thumbHint.textContent = 'Recommended: 1280\u00d7720 (16:9). PNG or JPG.';
                title.textContent = 'Asset Files';
                extras.classList.remove('hidden');
            }
        }

        function previewMainFile(input) {
            const label = document.getElementById('file-drop-label');
            if (input.files[0]) {
                const f = input.files[0];
                const sizeMB = (f.size / 1024 / 1024).toFixed(1);
                label.innerHTML = '<strong>' + f.name + '</strong> <span class="text-zinc-600">(' + sizeMB + ' MB)</span>';
                const maxMB = W.contentType === 'game' ? 2048 : 50;
                if (f.size > maxMB * 1024 * 1024) {
                    label.innerHTML += ' <span class="text-red-400">\u2014 exceeds ' + maxMB + 'MB limit</span>';
                }
                // Auto-populate download filename
                const filenameInput = document.getElementById('w-download-filename');
                if (filenameInput && !filenameInput.value) {
                    filenameInput.value = f.name;
                }
            }
        }

        function previewThumb(input) {
            const preview = document.getElementById('thumb-preview');
            const icon = document.getElementById('thumb-icon');
            const label = document.getElementById('thumb-label');
            if (input.files && input.files[0]) {
                const url = URL.createObjectURL(input.files[0]);
                preview.src = url;
                preview.classList.remove('hidden');
                if (icon) icon.classList.add('hidden');
                if (label) label.classList.add('hidden');
            }
        }

        function updateScreenshotCount(input) {
            const el = document.getElementById('screenshot-count');
            const previews = document.getElementById('screenshot-previews');
            const count = input.files.length;
            el.textContent = count > 0 ? count + ' screenshot' + (count !== 1 ? 's' : '') + ' selected' : 'Select multiple images at once. PNG or JPG.';
            previews.innerHTML = '';
            for (let i = 0; i < Math.min(count, 10); i++) {
                const url = URL.createObjectURL(input.files[i]);
                previews.innerHTML += '<div class="w-20 h-14 rounded-lg overflow-hidden border border-zinc-800/50 shrink-0"><img src="' + url + '" class="w-full h-full object-cover" /></div>';
            }
            if (count > 10) {
                previews.innerHTML += '<div class="w-20 h-14 rounded-lg bg-zinc-800/50 flex items-center justify-center text-xs text-zinc-500">+' + (count - 10) + '</div>';
            }
        }

        // ──────────────────────────────────────
        // Step 6 — Review summary
        // ──────────────────────────────────────
        function setupStep6() {
            const summary = document.getElementById('review-summary');
            const name = document.getElementById('w-name').value.trim();
            const desc = document.getElementById('w-description').value.trim();
            const price = parseInt(document.getElementById('w-price').value) || 0;
            const version = document.getElementById('w-version').value.trim() || '1.0.0';
            const file = document.getElementById('w-file').files[0];
            const thumb = document.getElementById('w-thumbnail').files[0];
            const screenshots = document.getElementById('w-screenshots').files;

            const typeLabel = W.contentType === 'game' ? 'Game' : 'Marketplace Asset';
            const priceLabel = price === 0 ? 'Free' : price + ' credits ($' + (price * 0.10).toFixed(2) + ')';

            let html = '<div class="divide-y divide-zinc-800/50">';
            html += reviewRow('Type', typeLabel);
            html += reviewRow('Category', W.categoryName || W.category);
            html += reviewRow('Name', escHtml(name));
            html += reviewRow('Description', '<span class="text-zinc-400">' + escHtml(desc.substring(0, 120)) + (desc.length > 120 ? '...' : '') + '</span>');
            html += reviewRow('Version', version);
            html += reviewRow('Price', priceLabel);

            if (W.contentType === 'asset') {
                if (selectedTags.length > 0) html += reviewRow('Tags', selectedTags.map(t => escHtml(t)).join(', '));
                const dlFilename = document.getElementById('w-download-filename').value.trim();
                if (dlFilename) html += reviewRow('Download Filename', escHtml(dlFilename));
                const creditName = document.getElementById('w-credit-name').value.trim();
                const creditUrl = document.getElementById('w-credit-url').value.trim();
                if (creditName) {
                    let creditHtml = escHtml(creditName);
                    if (creditUrl) creditHtml += ' (<a href="' + escHtml(creditUrl) + '" class="text-accent underline" target="_blank">' + escHtml(creditUrl) + '</a>)';
                    html += reviewRow('Credit', creditHtml);
                    html += reviewRow('Price', '<span class="text-green-400">Free (credited asset)</span>');
                }
            }

            if (file) {
                const sizeMB = (file.size / 1024 / 1024).toFixed(1);
                html += reviewRow('File', escHtml(file.name) + ' (' + sizeMB + ' MB)');
            }
            if (thumb) html += reviewRow('Cover Image', escHtml(thumb.name));
            if (screenshots.length > 0) html += reviewRow('Screenshots', screenshots.length + ' image' + (screenshots.length !== 1 ? 's' : ''));

            html += '</div>';
            summary.innerHTML = html;

            // No draft mode — always publish directly
        }

        function reviewRow(label, value) {
            return '<div class="flex items-start justify-between py-2.5"><span class="text-sm text-zinc-500">' + label + '</span><span class="text-sm text-zinc-200 text-right max-w-[60%]">' + value + '</span></div>';
        }

        function escHtml(str) {
            const div = document.createElement('div');
            div.textContent = str;
            return div.innerHTML;
        }

        // ──────────────────────────────────────
        // Submit
        // ──────────────────────────────────────
        async function handleSubmit(publish) {
            const errEl = document.getElementById('wizard-error');
            const okEl = document.getElementById('wizard-success');
            errEl.classList.add('hidden');
            okEl.classList.add('hidden');

            const btn = document.getElementById('publish-btn');
            const originalHtml = btn.innerHTML;
            btn.innerHTML = '<div class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></div> Publishing...';
            btn.disabled = true;

            try {
                const name = document.getElementById('w-name').value.trim();
                const description = document.getElementById('w-description').value.trim();
                const version = document.getElementById('w-version').value.trim() || '1.0.0';
                const price_credits = parseInt(document.getElementById('w-price').value) || 0;
                const file = document.getElementById('w-file').files[0];
                const thumbnail = document.getElementById('w-thumbnail').files[0];
                const screenshots = document.getElementById('w-screenshots').files;

                if (!file) throw new Error('No file selected');

                const dlFilename = document.getElementById('w-download-filename')?.value?.trim() || '';
                const metaObj = {
                    name: name,
                    description: description,
                    category: W.category,
                    price_credits: price_credits,
                    version: version
                };
                if (W.contentType === 'asset') {
                    metaObj.tags = selectedTags;
                    metaObj.download_filename = dlFilename;
                    const creditName = document.getElementById('w-credit-name')?.value?.trim() || '';
                    const creditUrl = document.getElementById('w-credit-url')?.value?.trim() || '';
                    if (creditName) {
                        metaObj.credit_name = creditName;
                        metaObj.credit_url = creditUrl;
                        metaObj.price_credits = 0;
                    }
                }
                const metadata = JSON.stringify(metaObj);

                const fd = new FormData();
                fd.append('metadata', metadata);
                fd.append('file', file);
                if (thumbnail) fd.append('thumbnail', thumbnail);

                let itemId, itemSlug;

                // Auth — always use Bearer token
                const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
                if (!token) throw new Error('Please sign in first');
                const headers = { 'Authorization': 'Bearer ' + token };

                if (W.contentType === 'game') {
                    const res = await fetch('/api/games/upload', {
                        method: 'POST',
                        headers: headers,
                        body: fd
                    });
                    const data = await safeJson(res);
                    if (!res.ok) throw new Error(data.error || 'Upload failed');
                    itemId = data.id;
                    itemSlug = data.slug;

                    // Upload screenshots
                    for (let i = 0; i < Math.min(screenshots.length, 10); i++) {
                        const mfd = new FormData();
                        mfd.append('type', 'image');
                        mfd.append('sort_order', i.toString());
                        mfd.append('file', screenshots[i]);
                        await fetch('/api/games/' + itemId + '/media', {
                            method: 'POST',
                            headers: headers,
                            body: mfd
                        });
                    }

                    const link = '/games/' + itemSlug;
                    document.getElementById('wizard-success-text').innerHTML = 'Game published! <a href="' + link + '" class="underline">View your game <i class="ph ph-arrow-right"></i></a>';

                } else {
                    const res = await fetch('/api/marketplace/upload', {
                        method: 'POST',
                        headers: headers,
                        body: fd
                    });
                    const data = await safeJson(res);
                    if (!res.ok) throw new Error(data.error || 'Upload failed');
                    itemId = data.id;
                    itemSlug = data.slug;

                    // Upload screenshots
                    for (let i = 0; i < Math.min(screenshots.length, 10); i++) {
                        const mfd = new FormData();
                        mfd.append('media_type', 'image');
                        mfd.append('file', screenshots[i]);
                        await fetch('/api/marketplace/' + itemId + '/media', {
                            method: 'POST',
                            headers: headers,
                            body: mfd
                        });
                    }

                    // Video URL
                    const videoUrl = document.getElementById('w-video-url').value.trim();
                    if (videoUrl) {
                        const vfd = new FormData();
                        vfd.append('video_url', videoUrl);
                        await fetch('/api/marketplace/' + itemId + '/media', {
                            method: 'POST',
                            headers: headers,
                            body: vfd
                        });
                    }

                    // Audio previews
                    const audioFiles = document.getElementById('w-audio')?.files || [];
                    for (let i = 0; i < Math.min(audioFiles.length, 10); i++) {
                        const afd = new FormData();
                        afd.append('media_type', 'audio');
                        afd.append('file', audioFiles[i]);
                        await fetch('/api/marketplace/' + itemId + '/media', {
                            method: 'POST',
                            headers: headers,
                            body: afd
                        });
                    }

                    const assetLink = '/marketplace/asset/' + itemSlug;
                    document.getElementById('wizard-success-text').innerHTML = 'Asset published! <a href="' + assetLink + '" class="underline">View your asset <i class="ph ph-arrow-right"></i></a>';
                }

                okEl.classList.remove('hidden');
                window.scrollTo({ top: 0, behavior: 'smooth' });

            } catch (error) {
                showError(error.message);
            }

            btn.innerHTML = originalHtml;
            btn.disabled = false;
        }

        // ──────────────────────────────────────
        // Auth & Init
        // ──────────────────────────────────────
        (async function init() {
            const authGate = document.getElementById('auth-required');
            const wizard = document.getElementById('wizard');

            // Check auth via credentials
            let authed = false;
            try {
                const res = await fetch('/api/auth/me', { credentials: 'include' });
                if (res.ok) authed = true;
            } catch(e) {}

            // Also check token cookie
            const token = document.cookie.match('(^|;)\\s*token\\s*=\\s*([^;]+)')?.pop();
            if (token) authed = true;

            if (!authed) {
                authGate.classList.remove('hidden');
                return;
            }

            wizard.classList.remove('hidden');

            // Check for ?type= param to auto-select content type
            const params = new URLSearchParams(window.location.search);
            const typeParam = params.get('type');
            if (typeParam === 'game' || typeParam === 'asset') {
                selectContentType(typeParam);
            }
        })();
        "##
        </script>
    }
}
