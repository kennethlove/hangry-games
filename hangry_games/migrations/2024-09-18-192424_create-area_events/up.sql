-- Creates the area_event table
CREATE TABLE area_event (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    area_id INTEGER NOT NULL,
    game_id INTEGER NOT NULL,
    FOREIGN KEY (area_id) REFERENCES area(id) ON DELETE CASCADE,
    FOREIGN KEY (game_id) REFERENCES game(id) ON DELETE CASCADE
);