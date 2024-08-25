-- Create the tribute->action relationship table.
CREATE TABLE tribute_action (
    id SERIAL PRIMARY KEY,
    tribute_id INT NOT NULL,
    action_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (tribute_id) REFERENCES tribute(id),
    FOREIGN KEY (action_id) REFERENCES action(id)
);
