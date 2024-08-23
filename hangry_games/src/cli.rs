use clap::{Parser, Subcommand};
use crate::tributes::actors::Tribute;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    AddTribute { name: String },
    AddArea { name: String },
}


pub fn parse() {
    let cli = Cli::parse();
    match cli.command {
        Commands::AddTribute { name } => {
            let tribute = Tribute::new(name);
            dbg!(&tribute);
        }
        Commands::AddArea { name } => {
            println!("Add area: {}", name);
        }
    }
}