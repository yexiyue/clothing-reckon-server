use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20241008_102110_clothing::Clothing, m20241009_014954_production::Production};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProductionItem::Table)
                    .if_not_exists()
                    .col(pk_auto(ProductionItem::Id))
                    .col(float(ProductionItem::UintPrice))
                    .col(integer(ProductionItem::Count))
                    .col(integer(ProductionItem::ProductionId))
                    .col(integer(ProductionItem::ClothingId))
                    .col(float(ProductionItem::Salary))
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProductionItem::Table, ProductionItem::ProductionId)
                            .to(Production::Table, Production::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ProductionItem::Table, ProductionItem::ClothingId)
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
            .drop_table(Table::drop().table(ProductionItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ProductionItem {
    Table,
    Id,
    UintPrice,
    ClothingId,
    ProductionId,
    Salary,
    Count,
}
