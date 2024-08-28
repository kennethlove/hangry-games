-- Remove the area foreign key.
ALTER TABLE tribute DROP CONSTRAINT fk_area_id;
ALTER TABLE tribute DROP COLUMN area_id;
