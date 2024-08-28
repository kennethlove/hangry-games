# hangry-games

## Description

A Hunger Games simulator.

## Running a game

1. `cargo run -- add-game`
    this will give you a game id, copy that.
2. `cargo run -- fill-tributes <game id>`
    this will fill the game with tributes.
3. `cargo run -- start-game <game id>`
    this will start the game by placing all tributes in the arena.
4. `cargo run -- run-next-day <game id>`
    this will run a new day of the simulation.