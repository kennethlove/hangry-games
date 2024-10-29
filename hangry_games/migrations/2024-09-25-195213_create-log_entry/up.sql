-- Create log_entry table
CREATE TABLE log_entry (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    game_id INTEGER NOT NULL,
    day INTEGER NOT NULL,
    message TEXT NOT NULL,
    area_id INTEGER,
    FOREIGN KEY (game_id) REFERENCES game(id) ON DELETE CASCADE,
    FOREIGN KEY (area_id) REFERENCES area(id) ON DELETE CASCADE
);
