-- Add the area_id column to the tribute table and create a foreign key to an area.
ALTER TABLE tribute ADD COLUMN area_id INT;
ALTER TABLE tribute ADD CONSTRAINT fk_area_id FOREIGN KEY (area_id) REFERENCES area(id) ON DELETE SET NULL;
