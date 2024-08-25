use crate::establish_connection;
use crate::models::{
    create_area, create_tribute, fill_tributes, get_area, get_area_by_id, get_areas, get_tribute,
    get_tributes, place_tribute_in_area,
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
    PlaceTribute { name: String, area: String },
    ShowActions { name: String }
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
        Commands::AddTribute { name } => {
            let tribute = create_tribute(connection, &name);
            dbg!(&tribute);
        }
        Commands::ShowTributes => {
            for mut tribute in get_tributes(connection) {
                if let Some(area) = tribute.area() {
                    println!(
                        "{}, District {}, in {:?}",
                        tribute.name, tribute.district, area.name
                    );
                } else {
                    println!("{}, District {}", tribute.name, tribute.district,);
                }
            }
        }
        Commands::FillTributes => {
            fill_tributes(connection);
        }
        Commands::PlaceTribute { name, area } => {
            let tribute = get_tribute(connection, &name);
            let area = get_area(connection, &area);
            place_tribute_in_area(connection, &tribute, &area);
        }
        Commands::ShowActions { name } => {
            let tribute = get_tribute(connection, &name);
            for action in tribute.actions() {
                println!("{}", action.name);
            }
        }
    }
}
