use clap::{Parser, Subcommand};
use crate::establish_connection;
use crate::models::{create_area, create_tribute, get_area, get_areas, get_tributes};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    AddTribute { name: String },
    ShowTributes,
    AddArea { name: String },
    ShowAreas,
    GetArea { name: String },
}


pub fn parse() {
    let cli = Cli::parse();
    let connection = &mut establish_connection();
    match cli.command {
        Commands::AddTribute { name } => {
            let tribute = create_tribute(connection, &name);
            dbg!(&tribute);
        }
        Commands::AddArea { name } => {
            let area = create_area(connection, &name);
            dbg!(&area);
        }
        Commands::ShowTributes => {
            for tribute in get_tributes(connection) {
                println!("{}, District {}", tribute.name, tribute.district);
            }
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
    }
}