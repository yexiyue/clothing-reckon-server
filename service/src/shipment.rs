use crate::{ListQueryParams, ListResult};
use ::entity::{shipment, shipment_item};
use futures::{stream, StreamExt};
use sea_orm::DbConn;
use sea_orm::*;
use serde::{Deserialize, Serialize};

pub struct ShipmentService;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateShipmentItem {
    amount: i32,
    clothing_id: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateShipmentParams {
    pub description: Option<String>,
    pub items: Vec<CreateShipmentItem>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateShipmentParams {
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Shipment {
    #[serde(flatten)]
    shipment: shipment::Model,
    items: Vec<shipment_item::Model>,
}

impl ShipmentService {
    async fn find_shipment_items(
        db: &DbConn,
        shipment_id: i32,
    ) -> Result<Vec<shipment_item::Model>, DbErr> {
        shipment_item::Entity::find()
            .filter(shipment_item::Column::ShipmentId.eq(shipment_id))
            .all(db)
            .await
    }

    pub async fn create(
        db: &DbConn,
        user_id: i32,
        params: CreateShipmentParams,
    ) -> Result<Shipment, DbErr> {
        let shipment = shipment::ActiveModel {
            description: Set(params.description),
            user_id: Set(user_id),
            ..Default::default()
        };
        let shipment = shipment.insert(db).await?;

        let shipment_items = params
            .items
            .into_iter()
            .map(|item| shipment_item::ActiveModel {
                amount: Set(item.amount),
                clothing_id: Set(item.clothing_id),
                shipment_id: Set(shipment.id),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        shipment_item::Entity::insert_many(shipment_items)
            .exec(db)
            .await?;

        let items = Self::find_shipment_items(db, shipment.id).await?;

        Ok(Shipment {
            shipment,
            items,
        })
    }

    pub async fn find_by_id(db: &DbConn, user_id: i32, id: i32) -> Result<Shipment, DbErr> {
        let shipment = shipment::Entity::find_by_id(id)
            .filter(shipment::Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("shipment not found".into()))?;

        let items = Self::find_shipment_items(db, shipment.id).await?;

        Ok(Shipment {
            shipment,
            items,
        })
    }

    pub async fn delete(db: &DbConn, user_id: i32, id: i32) -> Result<Shipment, DbErr> {
        let shipment = shipment::Entity::find_by_id(id)
            .filter(shipment::Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("shipment not found".into()))?;

        let items = Self::find_shipment_items(db, shipment.id).await?;

        shipment.clone().delete(db).await?;

        Ok(Shipment {
            shipment,
            items,
        })
    }

    pub async fn update(
        db: &DbConn,
        user_id: i32,
        id: i32,
        params: UpdateShipmentParams,
    ) -> Result<Shipment, DbErr> {
        let shipment = shipment::Entity::find_by_id(id)
            .filter(shipment::Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("shipment not found".into()))?;

        let mut shipment = shipment.into_active_model();
        shipment.description = Set(params.description);

        let shipment = shipment.update(db).await?;
        let items = Self::find_shipment_items(db, shipment.id).await?;

        Ok(Shipment { shipment, items })
    }

    pub async fn find_by_user_id(
        db: &DbConn,
        user_id: i32,
        params: ListQueryParams,
    ) -> Result<ListResult<Shipment>, DbErr> {
        let mut select = shipment::Entity::find().order_by_desc(shipment::Column::CreateAt);

        select = select.filter(shipment::Column::UserId.eq(user_id));

        if let Some(search) = params.search {
            select = select.filter(shipment::Column::Description.contains(&search));
        }

        if let Some(start_time) = params.start_time {
            select = select.filter(shipment::Column::CreateAt.gt(start_time));
        }

        if let Some(end_time) = params.end_time {
            select = select.filter(shipment::Column::CreateAt.lt(end_time));
        }

        let total = select.clone().count(db).await?;
        let (page, page_size) = (params.page.unwrap(), params.page_size.unwrap());

        let data = stream::iter(select.paginate(db, page_size).fetch_page(page).await?)
            .filter_map(|shipment| async {
                if let Ok(items) = Self::find_shipment_items(db, shipment.id).await {
                    Some(Shipment { shipment, items })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .await;

        Ok(ListResult { total, data })
    }
}
