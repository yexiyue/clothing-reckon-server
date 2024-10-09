pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20241008_102048_boss;
mod m20241008_102057_staff;
mod m20241008_102110_clothing;
mod m20241008_102121_shipment;
mod m20241008_102132_procurement;
mod m20241008_102213_shipment_item;
mod m20241008_102230_procurement_item;
mod m20241009_014954_production;
mod m20241009_015500_production_item;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20241008_102048_boss::Migration),
            Box::new(m20241008_102057_staff::Migration),
            Box::new(m20241008_102110_clothing::Migration),
            Box::new(m20241008_102121_shipment::Migration),
            Box::new(m20241008_102132_procurement::Migration),
            Box::new(m20241008_102213_shipment_item::Migration),
            Box::new(m20241008_102230_procurement_item::Migration),
            Box::new(m20241009_014954_production::Migration),
            Box::new(m20241009_015500_production_item::Migration),
        ]
    }
}
