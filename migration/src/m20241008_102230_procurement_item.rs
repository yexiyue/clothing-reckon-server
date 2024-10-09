use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20241008_102110_clothing::Clothing, m20241008_102132_procurement::Procurement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProcurementItem::Table)
                    .if_not_exists()
                    .col(pk_auto(ProcurementItem::Id))
                    .col(integer(ProcurementItem::Amount))
                    .col(integer(ProcurementItem::ProcurementId))
                    .col(integer(ProcurementItem::ClothingId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProcurementItem::Table, ProcurementItem::ProcurementId)
                            .to(Procurement::Table, Procurement::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProcurementItem::Table, ProcurementItem::ClothingId)
                            .to(Clothing::Table, Clothing::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProcurementItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ProcurementItem {
    Table,
    Id,
    ProcurementId,
    ClothingId,
    Amount,
}
