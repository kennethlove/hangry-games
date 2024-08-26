use crate::establish_connection;
use crate::models::{
    create_area, create_game, create_tribute, fill_tributes, get_action, get_all_tributes,
    get_area, get_areas, get_tribute, place_tribute_in_area,
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
    AddTribute { name: String },
    ShowTributes,
    FillTributes,
    PlaceTribute { tribute: String, area: String },
    ShowTributeActions { tribute: String },
    TributeTakesAction { tribute: String, action: String },
    AddGame,
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
        // TODO: Add game to AddTribute
        Commands::AddTribute { name } => {
            let tribute = create_tribute(connection, &name);
            dbg!(&tribute);
        }
        Commands::ShowTributes => {
            for tribute in get_all_tributes(connection) {
                println!("{}, District {}", tribute.name, tribute.district);
            }
        }
        Commands::FillTributes => {
            let count = fill_tributes(connection);
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
    }
}
