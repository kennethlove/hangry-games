-- Create log_entry table
CREATE TABLE log_entry (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    game_id INTEGER NOT NULL,
    day INTEGER NOT NULL,
    message TEXT NOT NULL,
    tribute_action_id INTEGER,
    area_id INTEGER,
    FOREIGN KEY (game_id) REFERENCES game(id),
    FOREIGN KEY (tribute_action_id) REFERENCES tribute_action(id),
    FOREIGN KEY (area_id) REFERENCES area(id)
);
