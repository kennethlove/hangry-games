use sea_orm_migration::{prelude::*, schema::*};
use sea_orm_migration::sea_orm::{entity::*, query::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(Area::Table)
            .columns(vec![Area::Name])
            .values_panic(vec!["Cornucopia".into()])
            .values_panic(vec!["NorthWest".into()])
            .values_panic(vec!["NorthEast".into()])
            .values_panic(vec!["SouthWest".into()])
            .values_panic(vec!["SouthEast".into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let remove = Query::delete()
            .from_table(Area::Table)
            .to_owned();

        manager.exec_stmt(remove).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Area {
    Table,
    Name,

}
