-- Your SQL goes here
ALTER TABLE items ADD COLUMN area_id INT;
ALTER TABLE items ADD COLUMN game_id INT;
ALTER TABLE items ADD CONSTRAINT fk_area_id FOREIGN KEY (area_id) REFERENCES area(id);
ALTER TABLE items ADD CONSTRAINT fk_game_id FOREIGN KEY (game_id) REFERENCES game(id);
