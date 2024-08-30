use crate::models::{create_area, create_game, create_tribute, fill_tributes, get_action, get_all_tributes, get_area, get_areas, get_tribute, place_tribute_in_area, get_game, get_game_tributes, get_games, Tribute, get_all_living_tributes};
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
            let count = fill_tributes(game);
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
                println!("{}", _game.name);
            }
        }
        Commands::StartGame { game_id } => {
            let game = get_game(&game_id).expect("Game not found");
            game.start();
        }
        Commands::RunNextDay { game_id } => {
            let game = get_game(&game_id).expect("Game not found");
            let living_tributes = get_all_living_tributes(&game);

            if living_tributes.len() == 1 {
                println!("{} wins!", living_tributes[0].name);
                return;
            }

            game.set_day(game.day.unwrap_or(0) + 1);

            println!("Day {}", game.day.unwrap_or(0));
            println!("{} tributes left", living_tributes.len());

            let mut deaths: Vec<Tribute> = vec![];
            for mut tribute in get_all_living_tributes(&game) {
                println!("-----------------------------------");
                println!("{}, District {}, in {}", tribute.name, tribute.district, tribute.area().unwrap().name);
                tribute = tribute.do_day();
                if tribute.health <= 0 {
                    deaths.push(tribute.clone());
                }
            }
            // Kill tributes
            for tribute in &deaths {
                tribute.dies();
                println!("💀 {} dies", tribute.name);
            }
            println!("{} left alive", get_all_living_tributes(&game).len());
        }
        Commands::EndGame { game_id } => {
            let game = get_game(&game_id).expect("Game not found");
            game.end();
        }
        Commands::GameStats { game_id } => {
            let game = get_game(&game_id).expect("Game not found");
            let living_tributes = get_all_living_tributes(&game);
            println!("Day {}", game.day.unwrap_or(0));
            println!("{} tributes left", living_tributes.len());
            for area in get_areas() {
                let tributes = living_tributes.iter().filter(|t| t.area().unwrap().id == area.id).collect::<Vec<_>>();
                println!("{} tributes in {}", tributes.len(), area.name);
            }
        }
    }
}
