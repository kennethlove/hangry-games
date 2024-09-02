-- Add fields to the tribute table to track stats about each tribute
-- This will allow us to track how well each tribute does in the games
-- and use that information to make the games more interesting

-- How many tributes has this tribute killed?
ALTER TABLE tribute ADD COLUMN kills INTEGER DEFAULT 0;
-- How many tributes has this tribute defeated?
ALTER TABLE tribute ADD COLUMN wins INTEGER DEFAULT 0;
-- How many tributes has this tribute lost to?
ALTER TABLE tribute ADD COLUMN defeats INTEGER DEFAULT 0;
-- How many tributes has this tribute tied with?
ALTER TABLE tribute ADD COLUMN draws INTEGER DEFAULT 0;
-- How many games has this tribute participated in?
ALTER TABLE tribute ADD COLUMN games INTEGER DEFAULT 0;
-- How brave is this tribute, naturally?
ALTER TABLE tribute ADD COLUMN bravery INTEGER DEFAULT 100;
-- How loyal is this tribute to their group/district?
ALTER TABLE tribute ADD COLUMN loyalty INTEGER DEFAULT 100;
-- How fast is this tribute?
ALTER TABLE tribute ADD COLUMN speed INTEGER DEFAULT 100;
-- How smart is this tribute?
ALTER TABLE tribute ADD COLUMN intelligence INTEGER DEFAULT 100;
-- How persuasive is this tribute?
ALTER TABLE tribute ADD COLUMN persuasion INTEGER DEFAULT 100;
-- How lucky is this tribute?
ALTER TABLE tribute ADD COLUMN luck INTEGER DEFAULT 100;