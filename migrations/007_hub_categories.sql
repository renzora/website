-- Clear and repopulate marketplace categories for multi-engine hub
DELETE FROM user_assets;
DELETE FROM transactions WHERE asset_id IS NOT NULL;
DELETE FROM assets;
DELETE FROM categories;
INSERT INTO categories (id, name, slug, description, icon, sort_order, max_file_size_mb, allowed_extensions) VALUES
    -- 3D Assets
    (gen_random_uuid(), '3D Models', '3d-models', 'Characters, props, vehicles, environments, and architectural models', 'ph-cube', 1, 100, '{zip,rar,7z,fbx,obj,gltf,glb,blend}'),
    (gen_random_uuid(), 'Animations', 'animations', 'Motion capture, keyframe animations, and animation packs', 'ph-person-arms-spread', 2, 100, '{zip,rar,7z,fbx,bvh}'),
    (gen_random_uuid(), 'Materials & Shaders', 'materials', 'PBR materials, shader graphs, and surface textures', 'ph-drop', 3, 50, '{zip,rar,material,wgsl,hlsl,glsl,shadergraph}'),
    -- 2D Assets
    (gen_random_uuid(), 'Textures & HDRIs', 'textures', 'PBR textures, HDR environments, skyboxes, and decals', 'ph-image', 4, 200, '{zip,rar,png,jpg,hdr,exr}'),
    (gen_random_uuid(), '2D Art & Sprites', '2d-art', 'Sprite sheets, pixel art, UI kits, icons, and illustrations', 'ph-paint-brush', 5, 50, '{zip,rar,png,svg,psd,aseprite}'),
    (gen_random_uuid(), 'Particle Effects', 'particles', 'VFX, particle systems, explosions, magic effects', 'ph-sparkle', 6, 50, '{zip,rar,7z}'),
    -- Audio
    (gen_random_uuid(), 'Sound Effects', 'sfx', 'Game SFX, UI sounds, foley, and ambient loops', 'ph-speaker-high', 7, 100, '{zip,rar,wav,ogg,mp3,flac}'),
    (gen_random_uuid(), 'Music', 'music', 'Game soundtracks, loops, stems, and adaptive music', 'ph-music-notes', 8, 200, '{zip,rar,wav,ogg,mp3,flac}'),
    -- Code & Plugins
    (gen_random_uuid(), 'Plugins & Tools', 'plugins', 'Engine plugins, editor tools, and pipeline utilities', 'ph-puzzle-piece', 9, 50, '{zip,rar,7z}'),
    (gen_random_uuid(), 'Scripts & Templates', 'scripts', 'Game templates, starter kits, and reusable script libraries', 'ph-code', 10, 50, '{zip,rar,lua,rhai,cs,gd,py}'),
    (gen_random_uuid(), 'Complete Projects', 'projects', 'Full game projects and example games ready to learn from', 'ph-folder-open', 11, 500, '{zip,rar,7z}'),
    -- Engine-Specific
    (gen_random_uuid(), 'Unreal Engine', 'unreal', 'Blueprints, C++ plugins, marketplace assets for UE4/UE5', 'ph-game-controller', 12, 200, '{zip,rar,7z}'),
    (gen_random_uuid(), 'Unity', 'unity', 'Unity packages, prefabs, shaders, and C# scripts', 'ph-unity-logo', 13, 200, '{zip,rar,unitypackage}'),
    (gen_random_uuid(), 'Godot', 'godot', 'Godot addons, GDScript tools, and scene templates', 'ph-robot', 14, 100, '{zip,rar,7z}'),
    (gen_random_uuid(), 'Renzora', 'renzora', 'Plugins, materials, and assets built for Renzora Engine', 'ph-rocket-launch', 15, 100, '{zip,rar,7z,material,rhai,lua}'),
    -- Other
    (gen_random_uuid(), 'Themes & UI', 'themes', 'Editor themes, UI kits, HUD designs, and menu templates', 'ph-palette', 16, 50, '{zip,rar,json,css}'),
    (gen_random_uuid(), 'Fonts', 'fonts', 'Game-ready fonts, pixel fonts, and icon sets', 'ph-text-aa', 17, 20, '{zip,rar,ttf,otf,woff,woff2}'),
    (gen_random_uuid(), 'Tutorials & Courses', 'tutorials', 'Video courses, written guides, and learning resources', 'ph-graduation-cap', 18, 500, '{zip,rar,pdf,mp4}')
ON CONFLICT DO NOTHING;

-- Clear and repopulate forum categories for game dev hub
DELETE FROM forum_posts;
DELETE FROM forum_threads;
DELETE FROM forum_categories;
INSERT INTO forum_categories (id, name, slug, description, icon, sort_order) VALUES
    -- General
    (gen_random_uuid(), 'Announcements', 'announcements', 'Official news and updates from Renzora', 'ph-megaphone', 1),
    (gen_random_uuid(), 'Introductions', 'introductions', 'Say hello and tell us about yourself', 'ph-hand-waving', 2),
    (gen_random_uuid(), 'General Discussion', 'general', 'Off-topic chat and general game dev discussion', 'ph-chat-circle', 3),
    -- Game Development
    (gen_random_uuid(), 'Game Design', 'game-design', 'Mechanics, level design, narrative, and player experience', 'ph-strategy', 4),
    (gen_random_uuid(), 'Programming', 'programming', 'Code, algorithms, architecture, and debugging help', 'ph-code', 5),
    (gen_random_uuid(), '3D Art & Modeling', '3d-art', 'Modeling, texturing, rigging, and 3D workflows', 'ph-cube', 6),
    (gen_random_uuid(), '2D Art & Pixel Art', '2d-art', 'Sprites, illustrations, UI design, and pixel art', 'ph-paint-brush', 7),
    (gen_random_uuid(), 'Audio & Music', 'audio', 'Sound design, music composition, and audio implementation', 'ph-music-notes', 8),
    (gen_random_uuid(), 'VFX & Shaders', 'vfx', 'Visual effects, shader programming, and post-processing', 'ph-sparkle', 9),
    -- Engine-Specific
    (gen_random_uuid(), 'Renzora Engine', 'renzora-engine', 'Discussion, help, and feedback for Renzora Engine', 'ph-rocket-launch', 10),
    (gen_random_uuid(), 'Unreal Engine', 'unreal-engine', 'UE4/UE5 development discussion', 'ph-game-controller', 11),
    (gen_random_uuid(), 'Unity', 'unity-engine', 'Unity development discussion', 'ph-circle-dashed', 12),
    (gen_random_uuid(), 'Godot', 'godot-engine', 'Godot development discussion', 'ph-robot', 13),
    (gen_random_uuid(), 'Other Engines', 'other-engines', 'Bevy, Fyrox, Defold, GameMaker, and more', 'ph-wrench', 14),
    -- Community
    (gen_random_uuid(), 'Showcase', 'showcase', 'Show off your games, prototypes, and experiments', 'ph-monitor-play', 15),
    (gen_random_uuid(), 'Work In Progress', 'wip', 'Share what you are working on and get feedback', 'ph-hammer', 16),
    (gen_random_uuid(), 'Tutorials & Resources', 'tutorials', 'Share and find community-made learning resources', 'ph-graduation-cap', 17),
    (gen_random_uuid(), 'Jobs & Collaboration', 'jobs', 'Find teammates, freelancers, or post job opportunities', 'ph-briefcase', 18),
    -- Meta
    (gen_random_uuid(), 'Marketplace Discussion', 'marketplace-discuss', 'Feedback on marketplace assets and seller support', 'ph-storefront', 19),
    (gen_random_uuid(), 'Feature Requests', 'feature-requests', 'Suggest features for the platform and engine', 'ph-lightbulb', 20),
    (gen_random_uuid(), 'Bug Reports', 'bug-reports', 'Report platform or engine bugs', 'ph-bug', 21)
ON CONFLICT DO NOTHING;
