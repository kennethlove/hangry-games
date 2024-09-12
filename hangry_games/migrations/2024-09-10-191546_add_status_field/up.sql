-- Adds status field to Tribute table
ALTER TABLE tribute ADD COLUMN status VARCHAR(255) NOT NULL DEFAULT 'alive';
