pub struct BossService;
use crate::{ListQueryParams, ListResult};
use ::entity::boss::{ActiveModel, Column, Entity, Model};
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateBossParams {
    pub name: String,
    pub phone_number: String,
    pub description: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateBossParams {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub description: Option<String>,
    pub address: Option<String>,
}

impl BossService {
    pub async fn create(
        db: &DbConn,
        user_id: i32,
        params: CreateBossParams,
    ) -> Result<Model, DbErr> {
        let model = ActiveModel {
            name: sea_orm::ActiveValue::Set(params.name),
            phone_number: sea_orm::ActiveValue::Set(params.phone_number),
            description: sea_orm::ActiveValue::Set(params.description),
            address: sea_orm::ActiveValue::Set(params.address),
            user_id: sea_orm::ActiveValue::Set(user_id),
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

    pub async fn update(
        db: &DbConn,
        user_id: i32,
        id: i32,
        params: UpdateBossParams,
    ) -> Result<Model, DbErr> {
        let model = Self::find_by_id(db, user_id, id).await?;

        let mut model = model.into_active_model();
        if let Some(name) = params.name {
            model.name = sea_orm::ActiveValue::Set(name);
        }
        if let Some(phone_number) = params.phone_number {
            model.phone_number = sea_orm::ActiveValue::Set(phone_number);
        }
        model.description = sea_orm::ActiveValue::Set(params.description);
        model.address = sea_orm::ActiveValue::Set(params.address);

        model.update(db).await
    }

    pub async fn find_by_id(db: &DbConn, user_id: i32, id: i32) -> Result<Model, DbErr> {
        Entity::find_by_id(id)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Cannot find model".into()))
    }

    // 查找用户关联的model
    pub async fn find_by_user_id(
        db: &DbConn,
        user_id: i32,
        params: ListQueryParams,
    ) -> Result<ListResult<Model>, DbErr> {
        let mut select = Entity::find().order_by_desc(Column::CreateAt);

        select = select.filter(Column::UserId.eq(user_id));

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
