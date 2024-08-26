use crate::establish_connection;
use crate::models::{
    create_area, create_game, create_tribute, fill_tributes, get_action, get_all_tributes,
    get_area, get_areas, get_tribute, place_tribute_in_area, get_game, get_game_tributes, get_games,
};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    AddArea { name: String },
    ShowAreas,
    GetArea { name: String },
    AddTribute { name: String, game: String },
    ShowAllTributes,
    ShowTributes { game: String },
    FillTributes { game: String },
    PlaceTribute { tribute: String, area: String },
    ShowTributeActions { tribute: String },
    TributeTakesAction { tribute: String, action: String },
    AddGame,
    ShowGames,
}

pub fn parse() {
    let cli = Cli::parse();
    let connection = &mut establish_connection();
    match cli.command {
        // Areas
        Commands::AddArea { name } => {
            let area = create_area(connection, &name);
            dbg!(&area);
        }
        Commands::ShowAreas => {
            for area in get_areas(connection) {
                println!("{}", area.name);
            }
        }
        Commands::GetArea { name } => {
            let area = get_area(connection, &name);
            dbg!(&area);
        }

        // Tributes
        Commands::AddTribute { name, game } => {
            let game = get_game(connection, &game).expect("Game not found");
            let mut tribute = create_tribute(connection, &name);
            tribute.try_set_game(&game).expect("Error adding tribute to game");
            dbg!(&tribute);
        }
        Commands::ShowAllTributes => {
            for tribute in get_all_tributes(connection) {
                println!("{}, District {}", tribute.name, tribute.district);
            }
        }
        Commands::ShowTributes { game } => {
            let game = get_game(connection, &game).expect("Game not found");
            dbg!(get_game_tributes(connection, &game));
            for tribute in get_game_tributes(connection, &game) {
                println!("{}, District {}", tribute.name, tribute.district);
            }
        }
        Commands::FillTributes { game } => {
            let game = get_game(connection, &game).expect("Game not found");
            let count = fill_tributes(connection, game);
            println!("{} tributes created", count);
        }
        Commands::PlaceTribute {
            tribute: name,
            area,
        } => {
            let tribute = get_tribute(connection, &name);
            let current_area = tribute.area();
            let area = get_area(connection, &area);
            place_tribute_in_area(connection, &tribute, &area);
            if let Some(area) = current_area {
                println!(
                    "{} moves from {:?} to {:?}",
                    tribute.name, area.name, area.name
                );
            } else {
                println!("{} moves to {:?}", tribute.name, area.name);
            }
        }

        // Actions
        Commands::ShowTributeActions { tribute: name } => {
            let tribute = get_tribute(connection, &name);
            for action in tribute.actions() {
                println!("{}", action.name);
            }
        }
        Commands::TributeTakesAction { tribute, action } => {
            let tribute = get_tribute(connection, &tribute);
            let action = get_action(connection, &action);
            tribute.take_action(&action);
        }

        // Games
        Commands::AddGame => {
            let game = create_game(connection);
            dbg!(&game);
        }
        Commands::ShowGames => {
            for game in get_games(connection) {
                println!("{}", game.name);
            }
        }
    }
}
