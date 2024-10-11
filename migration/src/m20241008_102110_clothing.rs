use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241008_102048_boss::Boss;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Clothing::Table)
                    .if_not_exists()
                    .col(pk_auto(Clothing::Id))
                    .col(string(Clothing::Name))
                    .col(string_null(Clothing::Description))
                    .col(integer(Clothing::Price))
                    .col(string_null(Clothing::Image))
                    .col(
                        timestamp_with_time_zone(Clothing::CreateAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(integer(Clothing::BossId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Clothing::Table, Clothing::BossId)
                            .to(Boss::Table, Boss::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Clothing::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Clothing {
    Table,
    Id,
    BossId,
    Name,
    Description,
    Price,
    Image,
    CreateAt,
}
