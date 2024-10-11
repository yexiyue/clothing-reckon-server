use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241008_102057_staff::Staff;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Production::Table)
                    .if_not_exists()
                    .col(pk_auto(Production::Id))
                    .col(string_null(Production::Description))
                    .col(
                        timestamp_with_time_zone(Production::CreateAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(integer(Production::StaffId))
                    .col(integer(Production::TotalSalary))
                    .col(boolean(Production::Settled).default(Expr::value(false)))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Production::Table, Production::StaffId)
                            .to(Staff::Table, Staff::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Production::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Production {
    Table,
    Id,
    StaffId,
    TotalSalary,
    Settled,
    CreateAt,
    Description,
}
