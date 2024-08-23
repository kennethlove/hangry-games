-- This file should undo anything in `up.sql`
ALTER TABLE tribute DROP CONSTRAINT fk_area_id;
ALTER TABLE tribute DROP COLUMN area_id;
