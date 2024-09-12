-- Remove the `is_alive` field, as it's replaced by the `status` field.
ALTER TABLE tribute DROP COLUMN is_alive;
