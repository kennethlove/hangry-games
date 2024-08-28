-- Create the tribute table.
CREATE TABLE tribute (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    health INTEGER NOT NULL DEFAULT 100,
    sanity INTEGER NOT NULL DEFAULT 100,
    movement INTEGER NOT NULL DEFAULT 100,
    is_alive BOOLEAN NOT NULL DEFAULT TRUE,
    district INTEGER NOT NULL
);
