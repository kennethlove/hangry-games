-- Add field to represent tribute status
ALTER TABLE tribute ADD COLUMN status VARCHAR(12) DEFAULT 'alive';
