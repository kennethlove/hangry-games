# hangry-games

## Description

A Hunger Games simulator.

## Setup

Hangry Games requires Rust to be installed. You can install Rust by following the instructions at
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Hangry Games also requires a PostgreSQL database to be running. It expects to connect on `localhost:5432` with username
and password `diesel`, and a database named `hangry_games`.

Once your dependencies are installed you should be able to `cargo build` with no issues.

If everything builds, install `dioxus-cli` and `diesel_cli` : `cargo install --locked dioxus-cli diesel_cli`.

Then run `diesel setup` to set up the database and `diesel migration run` to apply the migrations, if they weren't already.

Finally, you should be ready to run the application: `dx serve -- --bin hangry`.

## Running a game

1. `cargo run -- add-game`
    this will give you a game name, copy that.
2. `cargo run -- fill-tributes <game name>`
    this will fill the game with tributes.
3. `cargo run -- start-game <game name>`
    this will start the game by placing all tributes in the arena.

You can skip these first three steps by using `cargo run -- quick-start`.

4. `cargo run -- run-next-day <game name>`
    this will run a new day of the simulation.