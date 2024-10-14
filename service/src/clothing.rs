use ::entity::{
    boss,
    clothing::{ActiveModel, Column, Entity, Model},
};
use sea_orm::*;
use sea_query::Query;
use serde::{Deserialize, Serialize};

use crate::{boss::BossService, ListQueryParams, ListResult};
pub struct ClothingService;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateClothingParams {
    pub boss_id: i32,
    pub name: String,
    pub price: f32,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateClothingParams {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClothingListQueryParams {
    #[serde(deserialize_with = "crate::utils::parse_svc", default)]
    pub boss_ids: Option<Vec<i32>>,

    #[serde(flatten)]
    pub list_query: ListQueryParams,
}

impl ClothingService {
    pub async fn create(
        db: &DbConn,
        user_id: i32,
        params: CreateClothingParams,
    ) -> Result<Model, DbErr> {
        BossService::find_by_id(db, user_id, params.boss_id).await?;

        let model = ActiveModel {
            name: sea_orm::ActiveValue::Set(params.name),
            price: sea_orm::ActiveValue::Set(params.price),
            description: sea_orm::ActiveValue::Set(params.description),
            boss_id: sea_orm::ActiveValue::Set(params.boss_id),
            image: sea_orm::ActiveValue::Set(params.image),
            ..Default::default()
        };
        model.insert(db).await
    }

    pub async fn delete(db: &DbConn, user_id: i32, id: i32) -> Result<Model, DbErr> {
        let model = Self::find_by_id(db, user_id, id).await?;

        let model_clone = model.clone();

        model.delete(db).await?;
        Ok(model_clone)
    }

    // 只能修改一些基本信息不能修改价格
    pub async fn update(
        db: &DbConn,
        user_id: i32,
        id: i32,
        params: UpdateClothingParams,
    ) -> Result<Model, DbErr> {
        let model = Self::find_by_id(db, user_id, id).await?;

        let mut model = model.into_active_model();

        if let Some(name) = params.name {
            model.name = sea_orm::ActiveValue::Set(name);
        }

        model.description = sea_orm::ActiveValue::Set(params.description);
        model.image = sea_orm::ActiveValue::Set(params.image);

        model.update(db).await
    }

    pub async fn find_by_id(db: &DbConn, user_id: i32, id: i32) -> Result<Model, DbErr> {
        Entity::find_by_id(id)
            .filter(
                Column::BossId.in_subquery(
                    Query::select()
                        .column(boss::Column::Id)
                        .and_where(boss::Column::UserId.eq(user_id))
                        .from(boss::Entity)
                        .to_owned(),
                ),
            )
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Cannot find clothing".into()))
    }

    pub async fn find_by_user_id(
        db: &DbConn,
        user_id: i32,
        params: ListQueryParams,
        boss_ids: Option<Vec<i32>>,
    ) -> Result<ListResult<Model>, DbErr> {
        let mut select = Entity::find().order_by_desc(Column::CreateAt);

        select = select.filter(
            Column::BossId.in_subquery(
                Query::select()
                    .column(boss::Column::Id)
                    .and_where(boss::Column::UserId.eq(user_id))
                    .and_where_option(boss_ids.map(|ids| boss::Column::Id.is_in(ids)))
                    .from(boss::Entity)
                    .to_owned(),
            ),
        );

        if let Some(search) = params.search {
            select = select.filter(
                Column::Name
                    .contains(&search)
                    .or(Column::Description.contains(&search)),
            );
        }

        if let Some(start_time) = params.start_time {
            select = select.filter(Column::CreateAt.gt(start_time));
        }

        if let Some(end_time) = params.end_time {
            select = select.filter(Column::CreateAt.lt(end_time));
        }

        let total = select.clone().count(db).await?;
        let (page, page_size) = (params.page.unwrap(), params.page_size.unwrap());
        let data = select.paginate(db, page_size).fetch_page(page).await?;

        Ok(ListResult { total, data })
    }
}
