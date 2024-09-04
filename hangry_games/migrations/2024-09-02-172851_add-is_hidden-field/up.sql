-- Add column to remember if a tribute is currently hidden or not
ALTER TABLE tribute ADD COLUMN is_hidden BOOLEAN DEFAULT FALSE;