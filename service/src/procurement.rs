use crate::{ListQueryParams, ListResult};
use ::entity::{procurement, procurement_item};
use futures::{stream, StreamExt};
use sea_orm::DbConn;
use sea_orm::*;
use serde::{Deserialize, Serialize};

pub struct ProcurementService;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateProcurementItem {
    amount: i32,
    clothing_id: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateProcurementParams {
    pub description: Option<String>,
    pub items: Vec<CreateProcurementItem>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateProcurementParams {
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Procurement {
    #[serde(flatten)]
    procurement: procurement::Model,
    items: Vec<procurement_item::Model>,
}

impl ProcurementService {
    async fn find_procurement_items(
        db: &DbConn,
        procurement_id: i32,
    ) -> Result<Vec<procurement_item::Model>, DbErr> {
        procurement_item::Entity::find()
            .filter(procurement_item::Column::ProcurementId.eq(procurement_id))
            .all(db)
            .await
    }

    pub async fn create(
        db: &DbConn,
        user_id: i32,
        params: CreateProcurementParams,
    ) -> Result<Procurement, DbErr> {
        let procurement = procurement::ActiveModel {
            description: Set(params.description),
            user_id: Set(user_id),
            ..Default::default()
        };
        let procurement = procurement.insert(db).await?;

        let procurement_items = params
            .items
            .into_iter()
            .map(|item| procurement_item::ActiveModel {
                amount: Set(item.amount),
                clothing_id: Set(item.clothing_id),
                procurement_id: Set(procurement.id),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        procurement_item::Entity::insert_many(procurement_items)
            .exec(db)
            .await?;

        let items = Self::find_procurement_items(db, procurement.id).await?;

        Ok(Procurement { procurement, items })
    }

    pub async fn find_by_id(db: &DbConn, user_id: i32, id: i32) -> Result<Procurement, DbErr> {
        let procurement = procurement::Entity::find_by_id(id)
            .filter(procurement::Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("procurement not found".into()))?;

        let items = Self::find_procurement_items(db, procurement.id).await?;

        Ok(Procurement { procurement, items })
    }

    pub async fn delete(db: &DbConn, user_id: i32, id: i32) -> Result<Procurement, DbErr> {
        let procurement = procurement::Entity::find_by_id(id)
            .filter(procurement::Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("procurement not found".into()))?;

        let items = Self::find_procurement_items(db, procurement.id).await?;

        procurement.clone().delete(db).await?;

        Ok(Procurement { procurement, items })
    }

    pub async fn update(
        db: &DbConn,
        user_id: i32,
        id: i32,
        params: UpdateProcurementParams,
    ) -> Result<Procurement, DbErr> {
        let procurement = procurement::Entity::find_by_id(id)
            .filter(procurement::Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("procurement not found".into()))?;

        let mut procurement = procurement.into_active_model();
        procurement.description = Set(params.description);

        let procurement = procurement.update(db).await?;
        let items = Self::find_procurement_items(db, procurement.id).await?;

        Ok(Procurement { procurement, items })
    }

    pub async fn find_by_user_id(
        db: &DbConn,
        user_id: i32,
        params: ListQueryParams,
    ) -> Result<ListResult<Procurement>, DbErr> {
        let mut select = procurement::Entity::find().order_by_desc(procurement::Column::CreateAt);

        select = select.filter(procurement::Column::UserId.eq(user_id));

        if let Some(search) = params.search {
            select = select.filter(procurement::Column::Description.contains(&search));
        }

        if let Some(start_time) = params.start_time {
            select = select.filter(procurement::Column::CreateAt.gt(start_time));
        }

        if let Some(end_time) = params.end_time {
            select = select.filter(procurement::Column::CreateAt.lt(end_time));
        }

        let total = select.clone().count(db).await?;
        let (page, page_size) = (params.page.unwrap(), params.page_size.unwrap());

        let data = stream::iter(select.paginate(db, page_size).fetch_page(page).await?)
            .filter_map(|procurement| async {
                if let Ok(items) = Self::find_procurement_items(db, procurement.id).await {
                    Some(Procurement { procurement, items })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .await;

        Ok(ListResult { total, data })
    }
}
