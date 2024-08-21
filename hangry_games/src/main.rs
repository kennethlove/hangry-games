use futures::executor::block_on;

use entity::sea_orm::{DbErr, EntityTrait};
use entity::area::Entity as Area;
use hangry_games::db::Database;
use dotenv::dotenv;

async fn run() -> Result<(), DbErr> {
    let db = Database::new().await;
    let areas = Area::find().all(&db.connection).await?;
    for area in areas {
        println!("Area: {}", area.name);
    }
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
