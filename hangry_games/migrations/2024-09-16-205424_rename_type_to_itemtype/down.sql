-- This file should undo anything in `up.sql`
ALTER TABLE items RENAME COLUMN itemtype TO type;