-- Remove the game foreign key.
ALTER TABLE tribute DROP CONSTRAINT fk_game_id;
ALTER TABLE tribute DROP COLUMN game_id;
