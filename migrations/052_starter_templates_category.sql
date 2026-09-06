-- Starter templates: the category a new project can be created *from*.
--
-- Why this is not one of the three categories that nearly fit:
--
--   prefabs   a piece you drop INTO a scene you already have. Ready-made
--             objects and entity hierarchies, assembled but not standalone.
--   projects  a finished game you open to study or ship. It has been made;
--             you are not starting from it.
--   asset-pack a bag of assets with no scene and no settings.
--
-- A starter is none of those. It is the state a project begins in: a scene plus
-- the `project.toml` that goes with it, and the engine installs it before any
-- project exists, so it cannot land in a project folder the way every category
-- above does. That different install destination is the practical reason this
-- needs a slug of its own rather than a tag on `prefabs` -- the editor routes on
-- the category, and routing on a tag inside one would mean two kinds of thing
-- sharing a slug and behaving differently.
--
-- Icons are `ph-*` tokens: the Docker build scans migrations as well as the Rust
-- source when subsetting the Phosphor font, so this is picked up automatically.

INSERT INTO categories (id, name, slug, description, icon, sort_order, max_file_size_mb, allowed_extensions) VALUES
    (gen_random_uuid(), 'Starter Templates', 'starters',
     'Complete starting points for a new project: a scene plus the project settings that go with it',
     'ph-blueprint', 21, 500, '{zip,rar,7z}')
ON CONFLICT DO NOTHING;
