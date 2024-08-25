-- Create the area table.
CREATE TABLE IF NOT EXISTS area (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
)
