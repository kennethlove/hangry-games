CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    type VARCHAR NOT NULL,
    weight INT NOT NULL,
    strength_mod INT NOT NULL,
    defense_mod INT NOT NULL,
    health_mod INT NOT NULL,
    speed_mod INT NOT NULL,
    attack_mod INT NOT NULL
)-- Your SQL goes here
