-- Clear and repopulate marketplace categories for Renzora Engine
DELETE FROM user_assets;
DELETE FROM transactions WHERE asset_id IS NOT NULL;
DELETE FROM assets;
DELETE FROM categories;
INSERT INTO categories (id, name, slug, description, icon, sort_order, max_file_size_mb, allowed_extensions) VALUES
    (gen_random_uuid(), '3D Models', '3d-models', 'Characters, props, vehicles, environments, and architectural models', 'ph-cube', 1, 100, '{zip,rar,7z,fbx,obj,gltf,glb,blend}'),
    (gen_random_uuid(), 'Animations', 'animations', 'Motion capture, keyframe animations, and animation packs', 'ph-person-arms-spread', 2, 100, '{zip,rar,7z,fbx,bvh}'),
    (gen_random_uuid(), 'Materials & Shaders', 'materials', 'PBR materials, shader graphs, and WGSL shaders', 'ph-drop', 3, 50, '{zip,rar,material,wgsl}'),
    (gen_random_uuid(), 'Textures & HDRIs', 'textures', 'PBR textures, HDR environments, skyboxes, and decals', 'ph-image', 4, 200, '{zip,rar,png,jpg,hdr,exr}'),
    (gen_random_uuid(), '2D Art & Sprites', '2d-art', 'Sprite sheets, pixel art, UI kits, icons, and illustrations', 'ph-paint-brush', 5, 50, '{zip,rar,png,svg,psd,aseprite}'),
    (gen_random_uuid(), 'Particle Effects', 'particles', 'VFX, particle systems, explosions, magic effects', 'ph-sparkle', 6, 50, '{zip,rar,7z}'),
    (gen_random_uuid(), 'Sound Effects', 'sfx', 'Game SFX, UI sounds, foley, and ambient loops', 'ph-speaker-high', 7, 100, '{zip,rar,wav,ogg,mp3,flac}'),
    (gen_random_uuid(), 'Music', 'music', 'Game soundtracks, loops, stems, and adaptive music', 'ph-music-notes', 8, 200, '{zip,rar,wav,ogg,mp3,flac}'),
    (gen_random_uuid(), 'Plugins', 'plugins', 'Engine plugins, editor extensions, and pipeline tools', 'ph-puzzle-piece', 9, 50, '{zip,rar,7z}'),
    (gen_random_uuid(), 'Scripts', 'scripts', 'Lua, Rhai scripts, and reusable code libraries', 'ph-code', 10, 50, '{zip,rar,lua,rhai}'),
    (gen_random_uuid(), 'Blueprints', 'blueprints', 'Visual scripting blueprints and node graph templates', 'ph-tree-structure', 11, 50, '{zip,rar,blueprint}'),
    (gen_random_uuid(), 'Complete Projects', 'projects', 'Full game projects and example games', 'ph-folder-open', 12, 500, '{zip,rar,7z}'),
    (gen_random_uuid(), 'Themes', 'themes', 'Editor themes and UI kits', 'ph-palette', 13, 20, '{zip,rar,json}'),
    (gen_random_uuid(), 'Fonts', 'fonts', 'Game-ready fonts, pixel fonts, and icon sets', 'ph-text-aa', 14, 20, '{zip,rar,ttf,otf,woff,woff2}')
ON CONFLICT DO NOTHING;

-- Clear and repopulate forum categories for Renzora Engine
DELETE FROM forum_posts;
DELETE FROM forum_threads;
DELETE FROM forum_categories;
INSERT INTO forum_categories (id, name, slug, description, icon, sort_order) VALUES
    (gen_random_uuid(), 'Announcements', 'announcements', 'Official news and updates', 'ph-megaphone', 1),
    (gen_random_uuid(), 'Introductions', 'introductions', 'Say hello and introduce yourself', 'ph-hand-waving', 2),
    (gen_random_uuid(), 'General Discussion', 'general', 'General chat about Renzora and game dev', 'ph-chat-circle', 3),
    (gen_random_uuid(), 'Help & Support', 'help', 'Ask questions and get help with the engine', 'ph-question', 4),
    (gen_random_uuid(), 'Scripting', 'scripting', 'Lua, Rhai, and visual blueprint discussion', 'ph-code', 5),
    (gen_random_uuid(), 'Editor & Tools', 'editor', 'Editor features, workflows, and panel tips', 'ph-cube', 6),
    (gen_random_uuid(), 'Materials & Shaders', 'shaders', 'Material graphs, WGSL shaders, and rendering', 'ph-drop', 7),
    (gen_random_uuid(), 'Networking', 'networking', 'Multiplayer, servers, and replication', 'ph-wifi-high', 8),
    (gen_random_uuid(), 'Showcase', 'showcase', 'Show off your games, prototypes, and experiments', 'ph-monitor-play', 9),
    (gen_random_uuid(), 'Work In Progress', 'wip', 'Share what you are working on', 'ph-hammer', 10),
    (gen_random_uuid(), 'Tutorials & Resources', 'tutorials', 'Community guides and learning resources', 'ph-graduation-cap', 11),
    (gen_random_uuid(), 'Marketplace Discussion', 'marketplace-discuss', 'Asset feedback and seller support', 'ph-storefront', 12),
    (gen_random_uuid(), 'Feature Requests', 'feature-requests', 'Suggest new engine features', 'ph-lightbulb', 13),
    (gen_random_uuid(), 'Bug Reports', 'bugs', 'Report engine bugs', 'ph-bug', 14)
ON CONFLICT DO NOTHING;
