-- Add field for how strong a tribute is
ALTER TABLE tribute ADD COLUMN strength INTEGER DEFAULT 50;

-- Add field for how much damage a tribute can take
ALTER TABLE tribute ADD COLUMN defense INTEGER DEFAULT 0;
