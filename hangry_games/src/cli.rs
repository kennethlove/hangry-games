use crate::models::{create_area, create_game, create_tribute, get_action, get_all_tributes, get_area, get_areas, get_game, get_games, get_recently_dead_tributes, get_tribute, place_tribute_in_area};
use clap::{Parser, Subcommand};
use crate::models::game::{fill_tributes, get_all_living_tributes, get_dead_tributes, get_game_tributes};

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
    AddTribute { name: String, game_id: String },
    ShowAllTributes,
    ShowTributes { game_id: String },
    ShowTribute { game_id: String, tribute_id: String },
    FillTributes { game_id: String },
    PlaceTribute { tribute_id: String, area_id: String },
    ShowTributeActions { tribute_id: String },
    TributeTakesAction { tribute_id: String, action_id: String },
    AddGame,
    ShowGames,
    StartGame { game_id: String },
    RunNextDay { game_id: String },
    EndGame { game_id: String },
    GameStats { game_id: String },
    CloseArea { game_id: String, area_id: String },
    OpenArea { game_id: String, area_id: String },
    QuickStart,
    RunFullGame { game_id: String },
}

pub fn parse() {
    let cli = Cli::parse();
    match cli.command {
        // Areas
        Commands::AddArea { name } => {
            let area = create_area(&name);
            dbg!(&area);
        }
        Commands::ShowAreas => {
            for area in get_areas() {
                println!("{}", area.name);
            }
        }
        Commands::GetArea { name } => {
            let area = get_area(&name);
            dbg!(&area);
        }
        Commands::CloseArea { game_id: game, area_id: area } => {
            let mut game = get_game(&game).expect("Game not found");
            let area = get_area(&area);
            game.close_area(&area);
        }
        Commands::OpenArea { game_id: game, area_id: area } => {
            let mut game = get_game(&game).expect("Game not found");
            let area = get_area(&area);
            game.open_area(&area);
        }

        // Tributes
        Commands::AddTribute { name, game_id: game } => {
            let game = get_game(&game).expect("Game not found");
            let mut tribute = create_tribute(&name);
            tribute.try_set_game(&game).expect("Error adding tribute to game");
            dbg!(&tribute);
        }
        Commands::ShowAllTributes => {
            for tribute in get_all_tributes() {
                println!("{}, District {}", tribute.name, tribute.district);
            }
        }
        Commands::ShowTributes { game_id: game } => {
            let game = get_game(&game).expect("Game not found");
            for tribute in get_game_tributes(&game) {
                println!("{}, District {}", tribute.name, tribute.district);
            }
        }
        Commands::ShowTribute { game_id: game, tribute_id: tribute } => {
            let game = get_game(&game).expect("Game not found");
            let tribute = get_tribute(&tribute);
            if tribute.game_id != Some(game.id) {
                println!("Tribute is not in this game");
                return;
            }
            println!("{:?}", tribute);
        }
        Commands::FillTributes { game_id: game } => {
            let game = get_game(&game).expect("Game not found");
            let count = fill_tributes(&game);
            println!("{} tributes created", count);
        }
        Commands::PlaceTribute {
            tribute_id: name,
            area_id: area,
        } => {
            let tribute = get_tribute(&name);
            let current_area = tribute.area();
            let area = get_area(&area);
            place_tribute_in_area(&tribute, &area);
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
        Commands::ShowTributeActions { tribute_id: name } => {
            let tribute = get_tribute(&name);
            for (i, action) in tribute.actions().iter().enumerate() {
                println!("{}. {}", i, action.name);
            }
        }
        Commands::TributeTakesAction { tribute_id: tribute, action_id: action } => {
            let tribute = get_tribute(&tribute);
            let action = get_action(&action);
            tribute.take_action(&action);
        }

        // Games
        Commands::AddGame => {
            let game = create_game();
            println!("Game created: {}", game.name);
        }
        Commands::ShowGames => {
            for _game in get_games() {
                println!("{}, Day {}, Tributes {}/24 {}",
                    _game.name,
                    _game.day.unwrap_or(0),
                    get_all_living_tributes(&_game).len(),
                    if _game.ended_at.is_some() { "Closed" } else { "" }
                );
            }
        }
        Commands::StartGame { game_id } => {
            let game = get_game(&game_id).expect("Game not found");
            game.start();
        }
        Commands::RunNextDay { game_id } => {
            let mut game = get_game(&game_id).expect("Game not found");
            if game.ended_at.is_some() {
                println!("Game is already over");
                return;
            }

            game.run_next_day();
        }
        Commands::EndGame { game_id } => {
            let game = get_game(&game_id).expect("Game not found");
            game.end();
        }
        Commands::GameStats { game_id } => {
            let game = get_game(&game_id).expect("Game not found");
            let living_tributes = get_all_living_tributes(&game);
            let dead_tributes = get_dead_tributes(&game).into_iter().filter(|t| t.day_killed.is_some()).collect::<Vec<_>>();
            let recently_dead_tributes = get_recently_dead_tributes(&game).into_iter().collect::<Vec<_>>();
            println!("Day {}", game.day.unwrap_or(0));
            println!("{} tributes left", living_tributes.len());
            for area in get_areas() {
                let tributes = living_tributes.iter().filter(|t| t.area().is_some() && t.area().unwrap().id == area.id).collect::<Vec<_>>();
                println!("{} tributes in {}", tributes.len(), area.name);
            }
            println!("Deaths");
            for tribute in dead_tributes {
                println!("{} died on day {}, killed by {}", tribute.name, tribute.day_killed.unwrap_or(-1), tribute.killed_by.unwrap_or("Unknown".to_string()));
            }
            println!("Recently Dead");
            for tribute in recently_dead_tributes {
                println!("{} died today, killed by {}", tribute.name, tribute.killed_by.unwrap_or("Unknown".to_string()));
            }
            println!("Statuses");
            for tribute in living_tributes {
                let actions = tribute.tribute_actions();
                let last_actions = actions.last_chunk::<2>();
                let last_action = last_actions.unwrap().last().unwrap();
                let next_to_last_action = last_actions.unwrap().first().unwrap();

                println!("{} is {}, {}/100, {}/100, in {}, ({}, {:?}) -> ({}, {:?})",
                    tribute.name,
                    tribute.status,
                    tribute.health,
                    tribute.sanity,
                    tribute.area().expect("No area?").id,
                    next_to_last_action.action_id,
                    next_to_last_action.target,
                    last_action.action_id,
                    last_action.target,
                );
            }
            println!("Closed areas: {:?}", game.closed_areas);
        }
        Commands::QuickStart => {
            let game = create_game();
            println!("Game created: {}", game.name);
            let count = fill_tributes(&game);
            println!("{} tributes created", count);
            game.start();
        }
        Commands::RunFullGame { game_id } => {
            let mut game = get_game(&game_id).expect("Game not found");
            game.start();
            while game.living_tributes().len() > 1 {
                game.run_next_day();
            }
            game.end();
        }
    }
}
