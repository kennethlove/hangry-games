use futures::executor::block_on;

use dotenvy::dotenv;

use hangry_games::cli::parse;

async fn run() -> Result<(), std::io::Error> {
    parse();

    Ok(())
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
