use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20241008_102110_clothing::Clothing, m20241008_102121_shipment::Shipment};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ShipmentItem::Table)
                    .if_not_exists()
                    .col(pk_auto(ShipmentItem::Id))
                    .col(integer(ShipmentItem::Amount))
                    .col(integer(ShipmentItem::ShipmentId))
                    .col(integer(ShipmentItem::ClothingId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(ShipmentItem::Table, ShipmentItem::ShipmentId)
                            .to(Shipment::Table, Shipment::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ShipmentItem::Table, ShipmentItem::ClothingId)
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
            .drop_table(Table::drop().table(ShipmentItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ShipmentItem {
    Table,
    Id,
    ShipmentId,
    ClothingId,
    Amount,
}
