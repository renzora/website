-- Marketplace category refresh.
--
-- Splits two combined categories and adds four new ones. The splits are
-- non-destructive: `assets.category` is a plain slug column, so existing rows
-- keep pointing at `materials` / `textures`, which stay put under their
-- narrowed names. The new halves (`shaders`, `hdris`) start empty and sellers
-- can re-file into them.
--
-- Icons are `ph-*` tokens: the Docker build scans migrations as well as the
-- Rust source when subsetting the Phosphor font, so these are picked up
-- automatically.

-- ── Narrow the two combined categories ──
UPDATE categories
   SET name = 'Materials',
       description = 'PBR materials and material graphs'
 WHERE slug = 'materials';

UPDATE categories
   SET name = 'Textures',
       description = 'PBR texture sets, tiling textures, and decals'
 WHERE slug = 'textures';

-- ── The other halves, plus the new categories ──
INSERT INTO categories (id, name, slug, description, icon, sort_order, max_file_size_mb, allowed_extensions) VALUES
    (gen_random_uuid(), 'Shaders', 'shaders',
     'WGSL shaders, shader graphs, and post-processing effects',
     'ph-flask', 15, 50, '{zip,rar,7z,wgsl,glsl,shader}'),

    (gen_random_uuid(), 'HDRIs', 'hdris',
     'HDR environment maps, skyboxes, and image-based lighting',
     'ph-sun', 16, 500, '{zip,rar,hdr,exr,png,jpg}'),

    (gen_random_uuid(), 'Prefabs', 'prefabs',
     'Ready-made scene objects and assembled entity hierarchies',
     'ph-stack', 17, 200, '{zip,rar,7z,prefab,scene,ron}'),

    (gen_random_uuid(), 'Media', 'media',
     'Stingers, transitions, video clips, and cutscene footage',
     'ph-film-strip', 18, 500, '{zip,rar,mp4,webm,mov,wav,ogg,mp3}'),

    (gen_random_uuid(), 'SVGs & Vectors', 'svg',
     'Scalable vector icons, logos, and line art',
     'ph-bezier-curve', 19, 20, '{zip,rar,svg,ai,eps}'),

    (gen_random_uuid(), 'UI Templates', 'ui-templates',
     'Menu layouts, HUD kits, and complete interface templates',
     'ph-layout', 20, 100, '{zip,rar,7z,png,svg,json,ron}')
ON CONFLICT DO NOTHING;
