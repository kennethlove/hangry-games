pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_area_table;
mod m20240810_224544_add_areas;
mod m20240822_000021_create_tribute_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_area_table::Migration),
            Box::new(m20240810_224544_add_areas::Migration),
            Box::new(m20240822_000021_create_tribute_table::Migration),
        ]
    }
}
