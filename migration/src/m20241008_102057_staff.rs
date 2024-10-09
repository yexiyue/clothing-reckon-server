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
                    .table(Staff::Table)
                    .if_not_exists()
                    .col(pk_auto(Staff::Id))
                    .col(string(Staff::Name))
                    .col(string(Staff::PhoneNumber))
                    .col(string_null(Staff::Description))
                    .col(
                        timestamp_with_time_zone(Staff::CreateAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(integer(Staff::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Staff::Table, Staff::UserId)
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
            .drop_table(Table::drop().table(Staff::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Staff {
    Table,
    Id,
    UserId,
    Name,
    Description,
    PhoneNumber,
    CreateAt,
}
