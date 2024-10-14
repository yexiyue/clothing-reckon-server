use ::entity::production;
use ::entity::production_item;

use futures::stream;
use futures::StreamExt;
use sea_orm::*;
use sea_query::Query;
use serde::{Deserialize, Serialize};

use crate::staff;
use crate::ListQueryParams;
use crate::ListResult;
pub struct ProductionService;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductionItem {
    pub uint_price: f32,
    pub count: i32,
    pub clothing_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductionParams {
    pub description: Option<String>,
    pub staff_id: i32,
    pub items: Vec<CreateProductionItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProductionParams {
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Production {
    #[serde(flatten)]
    pub production: production::Model,
    pub items: Vec<production_item::Model>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductionListQueryParams {
    #[serde(flatten)]
    pub list_query: ListQueryParams,

    #[serde(deserialize_with = "crate::utils::parse_svc", default)]
    pub staff_ids: Option<Vec<i32>>,
}

impl ProductionService {
    async fn find_production_items(
        db: &DbConn,
        production_id: i32,
    ) -> Result<Vec<production_item::Model>, DbErr> {
        production_item::Entity::find()
            .filter(production_item::Column::ProductionId.eq(production_id))
            .all(db)
            .await
    }

    async fn find_production(
        db: &DbConn,
        user_id: i32,
        id: i32,
    ) -> Result<production::Model, DbErr> {
        production::Entity::find_by_id(id)
            .filter(
                production::Column::StaffId.in_subquery(
                    Query::select()
                        .column(::entity::staff::Column::Id)
                        .and_where(::entity::staff::Column::UserId.eq(user_id))
                        .from(::entity::staff::Entity)
                        .to_owned(),
                ),
            )
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("production not found".to_string()))
    }

    pub async fn create(
        db: &DatabaseConnection,
        user_id: i32,
        params: CreateProductionParams,
    ) -> Result<Production, DbErr> {
        let staff = staff::StaffService::find_by_id(db, user_id, params.staff_id).await?;

        let total_salary = params
            .items
            .iter()
            .map(|item| item.uint_price * item.count as f32)
            .sum::<f32>();

        let production = production::ActiveModel {
            description: ActiveValue::Set(params.description),
            staff_id: ActiveValue::Set(staff.id),
            total_salary: ActiveValue::Set(total_salary),
            ..Default::default()
        };

        let production = production.insert(db).await?;

        let procurement_items = params
            .items
            .into_iter()
            .map(|item| production_item::ActiveModel {
                uint_price: Set(item.uint_price),
                count: Set(item.count),
                production_id: Set(production.id),
                salary: Set(item.uint_price * item.count as f32),
                clothing_id: Set(item.clothing_id),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        production_item::Entity::insert_many(procurement_items)
            .exec(db)
            .await?;

        let items = Self::find_production_items(db, production.id).await?;

        Ok(Production { production, items })
    }

    pub async fn delete(db: &DbConn, user_id: i32, id: i32) -> Result<Production, DbErr> {
        let production = Self::find_production(db, user_id, id).await?;

        let items = Self::find_production_items(db, production.id).await?;
        production.clone().delete(db).await?;

        Ok(Production { production, items })
    }

    pub async fn update(
        db: &DatabaseConnection,
        user_id: i32,
        id: i32,
        params: UpdateProductionParams,
    ) -> Result<Production, DbErr> {
        let production = Self::find_production(db, user_id, id).await?;

        let mut production = production.into_active_model();
        production.description = Set(params.description);

        let production = production.update(db).await?;

        let items = Self::find_production_items(db, production.id).await?;

        Ok(Production { production, items })
    }

    pub async fn settle(
        db: &DatabaseConnection,
        user_id: i32,
        id: i32,
    ) -> Result<Production, DbErr> {
        let production = Self::find_production(db, user_id, id).await?;

        let mut production = production.into_active_model();

        production.settled = Set(true);

        let production = production.update(db).await?;

        let items = Self::find_production_items(db, production.id).await?;

        Ok(Production { production, items })
    }

    pub async fn find_by_id(db: &DbConn, user_id: i32, id: i32) -> Result<Production, DbErr> {
        let production = Self::find_production(db, user_id, id).await?;

        let items = Self::find_production_items(db, production.id).await?;

        Ok(Production { production, items })
    }

    pub async fn find_by_user_id(
        db: &DbConn,
        user_id: i32,
        params: ListQueryParams,
        staff_ids: Option<Vec<i32>>,
    ) -> Result<ListResult<Production>, DbErr> {
        let mut select = production::Entity::find().order_by_desc(production::Column::CreateAt);

        select = select.filter(
            production::Column::StaffId.in_subquery(
                Query::select()
                    .column(::entity::staff::Column::Id)
                    .and_where(::entity::staff::Column::UserId.eq(user_id))
                    .and_where_option(staff_ids.map(|ids| ::entity::staff::Column::Id.is_in(ids)))
                    .from(::entity::staff::Entity)
                    .to_owned(),
            ),
        );

        if let Some(search) = params.search {
            select = select.filter(production::Column::Description.contains(&search));
        }

        if let Some(start_time) = params.start_time {
            select = select.filter(production::Column::CreateAt.gt(start_time));
        }

        if let Some(end_time) = params.end_time {
            select = select.filter(production::Column::CreateAt.lt(end_time));
        }

        let total = select.clone().count(db).await?;
        let (page, page_size) = (params.page.unwrap(), params.page_size.unwrap());

        let data = stream::iter(select.paginate(db, page_size).fetch_page(page).await?)
            .filter_map(|production| async {
                if let Ok(items) = Self::find_production_items(db, production.id).await {
                    Some(Production { production, items })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .await;

        Ok(ListResult { total, data })
    }
}
