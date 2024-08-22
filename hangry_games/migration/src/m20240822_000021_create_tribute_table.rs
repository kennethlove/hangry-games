use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .create_table(
                Table::create()
                    .table(Tribute::Table)
                    .if_not_exists()
                    .col(pk_auto(Tribute::Id))
                    .col(string(Tribute::Name))
                    .col(integer(Tribute::Health))
                    .col(integer(Tribute::Sanity))
                    .col(integer(Tribute::Movement))
                    .col(boolean(Tribute::IsAlive))
                    .col(integer(Tribute::District))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Tribute::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Tribute {
    Table,
    Id,
    Name,
    Health,
    Sanity,
    Movement,
    IsAlive,
    District,
}
