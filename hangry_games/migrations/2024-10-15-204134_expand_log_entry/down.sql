-- Remove tribute_id, action_target_type, and action_target_id from log_entry
ALTER TABLE log_entry DROP COLUMN tribute_id;
ALTER TABLE log_entry DROP COLUMN action_target_type;
ALTER TABLE log_entry DROP COLUMN action_target_id;
