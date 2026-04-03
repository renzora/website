-- Avatar customization system
CREATE TABLE avatar_parts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slot VARCHAR(32) NOT NULL,
    name VARCHAR(128) NOT NULL,
    slug VARCHAR(128) NOT NULL UNIQUE,
    part_data JSONB NOT NULL DEFAULT '{}',
    price_credits BIGINT NOT NULL DEFAULT 0,
    is_default BOOLEAN NOT NULL DEFAULT false,
    sort_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_avatar_parts_slot ON avatar_parts(slot);
CREATE INDEX idx_avatar_parts_default ON avatar_parts(is_default);

CREATE TABLE user_avatar_parts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    part_id UUID NOT NULL REFERENCES avatar_parts(id) ON DELETE CASCADE,
    purchased_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, part_id)
);

CREATE TABLE user_avatars (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    skin_color VARCHAR(7) NOT NULL DEFAULT '#C68642',
    eye_color VARCHAR(7) NOT NULL DEFAULT '#4A90D9',
    hair_color VARCHAR(7) NOT NULL DEFAULT '#3B2F2F',
    equipped_parts JSONB NOT NULL DEFAULT '{}',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed default parts (free, available to everyone)

-- Hair styles
INSERT INTO avatar_parts (slot, name, slug, part_data, is_default, sort_order) VALUES
('hair', 'Buzz Cut', 'hair-buzz', '{"geometries":[{"type":"sphere","args":[0.52,8,8],"position":[0,1.62,0],"scale":[1,0.3,1]}]}', true, 0),
('hair', 'Spiky', 'hair-spiky', '{"geometries":[{"type":"cone","args":[0.08,0.35,4],"position":[0,1.95,0]},{"type":"cone","args":[0.08,0.3,4],"position":[0.15,1.85,0.1],"rotation":[0,0,0.3]},{"type":"cone","args":[0.08,0.3,4],"position":[-0.15,1.85,0.1],"rotation":[0,0,-0.3]},{"type":"cone","args":[0.08,0.28,4],"position":[0,1.88,0.15],"rotation":[0.3,0,0]},{"type":"cone","args":[0.08,0.28,4],"position":[0.1,1.88,-0.1],"rotation":[-0.2,0,0.2]}]}', true, 1),
('hair', 'Side Sweep', 'hair-sidesweep', '{"geometries":[{"type":"sphere","args":[0.53,8,8],"position":[0,1.65,0],"scale":[1.1,0.35,1]},{"type":"sphere","args":[0.25,6,6],"position":[0.35,1.65,0.1],"scale":[1,0.6,0.8]}]}', true, 2),
('hair', 'Mohawk', 'hair-mohawk', '{"geometries":[{"type":"box","args":[0.1,0.4,0.6],"position":[0,1.9,0]},{"type":"box","args":[0.08,0.3,0.5],"position":[0,2.1,0]}]}', false, 3),
('hair', 'Puff', 'hair-puff', '{"geometries":[{"type":"sphere","args":[0.55,8,8],"position":[0,1.75,0],"scale":[1.1,0.8,1.1]}]}', false, 4);

-- Hats
INSERT INTO avatar_parts (slot, name, slug, part_data, price_credits, is_default, sort_order) VALUES
('hat', 'None', 'hat-none', '{"geometries":[]}', 0, true, 0),
('hat', 'Beanie', 'hat-beanie', '{"geometries":[{"type":"sphere","args":[0.54,8,8],"position":[0,1.72,0],"scale":[1,0.5,1],"color":"#2D3748"},{"type":"sphere","args":[0.08,6,6],"position":[0,1.98,0],"color":"#2D3748"}]}', 50, false, 1),
('hat', 'Top Hat', 'hat-tophat', '{"geometries":[{"type":"cylinder","args":[0.35,0.35,0.5,8],"position":[0,2.0,0],"color":"#1A1A2E"},{"type":"cylinder","args":[0.5,0.5,0.05,8],"position":[0,1.75,0],"color":"#1A1A2E"}]}', 100, false, 2),
('hat', 'Cap', 'hat-cap', '{"geometries":[{"type":"sphere","args":[0.54,8,8],"position":[0,1.7,0],"scale":[1,0.35,1],"color":"#E53E3E"},{"type":"box","args":[0.3,0.04,0.25],"position":[0,1.62,0.45],"color":"#E53E3E"}]}', 75, false, 3);

-- Tops
INSERT INTO avatar_parts (slot, name, slug, part_data, price_credits, is_default, sort_order) VALUES
('top', 'T-Shirt', 'top-tshirt', '{"geometries":[{"type":"cylinder","args":[0.35,0.3,0.7,8],"position":[0,0.65,0],"color":"#4A5568"}]}', 0, true, 0),
('top', 'Hoodie', 'top-hoodie', '{"geometries":[{"type":"cylinder","args":[0.38,0.32,0.75,8],"position":[0,0.65,0],"color":"#553C9A"},{"type":"sphere","args":[0.15,6,6],"position":[0,1.15,0.05],"color":"#553C9A"}]}', 80, false, 1),
('top', 'Tank Top', 'top-tank', '{"geometries":[{"type":"cylinder","args":[0.33,0.28,0.65,8],"position":[0,0.65,0],"color":"#E53E3E"}]}', 40, false, 2),
('top', 'Jacket', 'top-jacket', '{"geometries":[{"type":"cylinder","args":[0.4,0.34,0.78,8],"position":[0,0.65,0],"color":"#2D3748"},{"type":"box","args":[0.08,0.6,0.02],"position":[0,0.65,0.34],"color":"#A0AEC0"}]}', 120, false, 3);

-- Bottoms
INSERT INTO avatar_parts (slot, name, slug, part_data, price_credits, is_default, sort_order) VALUES
('bottom', 'Jeans', 'bottom-jeans', '{"geometries":[{"type":"cylinder","args":[0.15,0.13,0.55,6],"position":[-0.15,-0.2,0],"color":"#2B6CB0"},{"type":"cylinder","args":[0.15,0.13,0.55,6],"position":[0.15,-0.2,0],"color":"#2B6CB0"}]}', 0, true, 0),
('bottom', 'Shorts', 'bottom-shorts', '{"geometries":[{"type":"cylinder","args":[0.16,0.14,0.3,6],"position":[-0.15,-0.05,0],"color":"#718096"},{"type":"cylinder","args":[0.16,0.14,0.3,6],"position":[0.15,-0.05,0],"color":"#718096"}]}', 30, false, 1),
('bottom', 'Cargo', 'bottom-cargo', '{"geometries":[{"type":"cylinder","args":[0.17,0.15,0.55,6],"position":[-0.15,-0.2,0],"color":"#5F6B2D"},{"type":"cylinder","args":[0.17,0.15,0.55,6],"position":[0.15,-0.2,0],"color":"#5F6B2D"},{"type":"box","args":[0.1,0.15,0.08],"position":[-0.28,-0.15,0],"color":"#5F6B2D"},{"type":"box","args":[0.1,0.15,0.08],"position":[0.28,-0.15,0],"color":"#5F6B2D"}]}', 60, false, 2);

-- Shoes
INSERT INTO avatar_parts (slot, name, slug, part_data, price_credits, is_default, sort_order) VALUES
('shoes', 'Sneakers', 'shoes-sneakers', '{"geometries":[{"type":"box","args":[0.14,0.08,0.22],"position":[-0.15,-0.52,0.04],"color":"#E2E8F0"},{"type":"box","args":[0.14,0.08,0.22],"position":[0.15,-0.52,0.04],"color":"#E2E8F0"}]}', 0, true, 0),
('shoes', 'Boots', 'shoes-boots', '{"geometries":[{"type":"cylinder","args":[0.1,0.12,0.25,6],"position":[-0.15,-0.42,0],"color":"#5D3A1A"},{"type":"cylinder","args":[0.1,0.12,0.25,6],"position":[0.15,-0.42,0],"color":"#5D3A1A"}]}', 50, false, 1),
('shoes', 'Sandals', 'shoes-sandals', '{"geometries":[{"type":"box","args":[0.14,0.03,0.22],"position":[-0.15,-0.5,0.04],"color":"#A0522D"},{"type":"box","args":[0.14,0.03,0.22],"position":[0.15,-0.5,0.04],"color":"#A0522D"}]}', 25, false, 2);

-- Accessories
INSERT INTO avatar_parts (slot, name, slug, part_data, price_credits, is_default, sort_order) VALUES
('accessory', 'None', 'acc-none', '{"geometries":[]}', 0, true, 0),
('accessory', 'Glasses', 'acc-glasses', '{"geometries":[{"type":"torus","args":[0.08,0.015,6,12],"position":[-0.15,1.55,0.42],"color":"#1A1A1A"},{"type":"torus","args":[0.08,0.015,6,12],"position":[0.15,1.55,0.42],"color":"#1A1A1A"},{"type":"cylinder","args":[0.01,0.01,0.12,4],"position":[0,1.55,0.42],"rotation":[0,0,1.57],"color":"#1A1A1A"}]}', 60, false, 1),
('accessory', 'Backpack', 'acc-backpack', '{"geometries":[{"type":"box","args":[0.3,0.35,0.15],"position":[0,0.75,-0.35],"color":"#C53030"},{"type":"cylinder","args":[0.02,0.02,0.3,4],"position":[-0.12,0.85,-0.25],"color":"#C53030"},{"type":"cylinder","args":[0.02,0.02,0.3,4],"position":[0.12,0.85,-0.25],"color":"#C53030"}]}', 90, false, 2),
('accessory', 'Scarf', 'acc-scarf', '{"geometries":[{"type":"torus","args":[0.32,0.06,6,12],"position":[0,1.15,0],"rotation":[1.57,0,0],"color":"#D69E2E"},{"type":"box","args":[0.08,0.35,0.04],"position":[0.2,0.95,0.25],"color":"#D69E2E"}]}', 45, false, 3);

-- Eyes (just color variations, the geometry stays the same)
INSERT INTO avatar_parts (slot, name, slug, part_data, price_credits, is_default, sort_order) VALUES
('eyes', 'Round', 'eyes-round', '{"geometries":[{"type":"sphere","args":[0.06,6,6],"position":[-0.15,1.57,0.4]},{"type":"sphere","args":[0.06,6,6],"position":[0.15,1.57,0.4]}]}', 0, true, 0),
('eyes', 'Wide', 'eyes-wide', '{"geometries":[{"type":"sphere","args":[0.08,6,6],"position":[-0.15,1.57,0.38]},{"type":"sphere","args":[0.08,6,6],"position":[0.15,1.57,0.38]}]}', 0, true, 1),
('eyes', 'Narrow', 'eyes-narrow', '{"geometries":[{"type":"sphere","args":[0.06,6,6],"position":[-0.15,1.57,0.4],"scale":[1.3,0.5,1]},{"type":"sphere","args":[0.06,6,6],"position":[0.15,1.57,0.4],"scale":[1.3,0.5,1]}]}', 0, true, 2);
