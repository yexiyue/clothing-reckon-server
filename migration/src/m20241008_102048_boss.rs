use sea_orm_migration::{prelude::*, schema::*};

use crate::m20220101_000001_create_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Boss::Table)
                    .if_not_exists()
                    .col(pk_auto(Boss::Id))
                    .col(string(Boss::Name))
                    .col(string(Boss::PhoneNumber))
                    .col(string_null(Boss::Address))
                    .col(string_null(Boss::Description))
                    .col(
                        timestamp_with_time_zone(Boss::CreateAt).default(Expr::current_timestamp()),
                    )
                    .col(integer(Boss::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Boss::Table, Boss::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Boss::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Boss {
    Table,
    Id,
    Name,
    Address,
    UserId,
    PhoneNumber,
    Description,
    CreateAt,
}
