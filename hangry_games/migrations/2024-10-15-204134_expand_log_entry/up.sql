-- Add tribute_id, action_target_type, and action_target_id to log_entry
ALTER TABLE log_entry ADD COLUMN tribute_id INT;
ALTER TABLE log_entry ADD COLUMN action_target_type VARCHAR(255);
ALTER TABLE log_entry ADD COLUMN action_target_id INT;
ALTER TABLE log_entry ADD FOREIGN KEY (tribute_id) REFERENCES tribute(id) ON DELETE CASCADE;
