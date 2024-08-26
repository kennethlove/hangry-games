-- Add a `day` column to the `game` table, defaults to 0.
ALTER TABLE game ADD COLUMN day INT DEFAULT 0;
UPDATE game SET day = 0;
