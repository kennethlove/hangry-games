-- Your SQL goes here
ALTER TABLE tribute ADD COLUMN area_id INT;
ALTER TABLE tribute ADD CONSTRAINT fk_area_id FOREIGN KEY (area_id) REFERENCES area(id);

