-- Your SQL goes here
ALTER TABLE tribute ADD COLUMN game_id INT;
ALTER TABLE tribute ADD CONSTRAINT fk_game_id FOREIGN KEY (game_id) REFERENCES game(id);
