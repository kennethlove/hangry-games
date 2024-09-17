-- Add `target` field to `tribute_action` table to record
-- the target of the tribute action, like an area or item.
ALTER TABLE tribute_action ADD COLUMN target TEXT;