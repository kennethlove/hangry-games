-- Your SQL goes here
CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    -- Specific item name. Might just equal itemtype in practice.
    name VARCHAR NOT NULL,
    -- Type of item. eg. "Crossbow" or "Health Kit"
    itemtype VARCHAR NOT NULL,
    weight INT NOT NULL,
    strength_mod INT NOT NULL,
    defense_mod INT NOT NULL,
    health_mod INT NOT NULL,
    speed_mod INT NOT NULL,
    attack_mod INT NOT NULL,
    -- Tracks max item status. eg. 100 for dagger durability, 20 for a full quiver, 1 for grenade charges.
    max_durability INT NOT NULL,
    -- Tracks current item status
    durability INT NOT NULL,
    -- Should these be nullable?
    area_id INT,
    game_id INT,
    CONSTRAINT fk_area_id FOREIGN KEY (area_id) REFERENCES area(id),
    CONSTRAINT fk_game_id FOREIGN KEY (game_id) REFERENCES game(id)
)