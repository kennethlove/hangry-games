-- Create the action table.
CREATE TABLE action (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL
);

-- Insert the default actions.
INSERT INTO action (name, description) VALUES ('idle', 'Do nothing');
INSERT INTO action (name, description) VALUES ('move', 'Move to a new location');
INSERT INTO action (name, description) VALUES ('rest', 'Rest and recover');
INSERT INTO action (name, description) VALUES ('use-item', 'Use an item');
INSERT INTO action (name, description) VALUES ('attack', 'Attack another tribute');
INSERT INTO action (name, description) VALUES ('hide', 'Hide to avoid detection');
