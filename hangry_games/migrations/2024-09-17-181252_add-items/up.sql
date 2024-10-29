-- Adds table for items that can be found in the game world.
CREATE TABLE item (
    id SERIAL PRIMARY KEY,
    -- Specific item name.
    name VARCHAR(255) NOT NULL,
    -- Type of item. eg. "Crossbow" or "Health Kit"
    item_type VARCHAR(255) NOT NULL,
    game_id INT,
    area_id INT,
    tribute_id INT,
    -- Quantity of an item, e.g. 20 arrows or 1 ax.
    quantity INT NOT NULL DEFAULT 1,
    -- Attribute affected by item
    attribute VARCHAR(255) NOT NULL,
    effect INT NOT NULL,

    CONSTRAINT fk_game_id FOREIGN KEY (game_id) REFERENCES game(id) ON DELETE CASCADE,
    CONSTRAINT fk_area_id FOREIGN KEY (area_id) REFERENCES area(id) ON DELETE CASCADE,
    CONSTRAINT fk_tribute_id FOREIGN KEY (tribute_id) REFERENCES tribute(id) ON DELETE CASCADE
);