-- Add the `is_alive` field back to the `tribute` table
ALTER TABLE tribute ADD COLUMN is_alive BOOLEAN NOT NULL DEFAULT TRUE;
