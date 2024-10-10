pub struct BossService;
use ::entity::boss::{ActiveModel, Column, Entity, Model};
use anyhow::Result;
use sea_orm::*;
use serde::{Deserialize, Serialize};

use crate::{ListQueryParams, ListResult};

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
    pub async fn create(db: &DbConn, user_id: i32, params: CreateBossParams) -> Result<Model> {
        let boss = ActiveModel {
            name: sea_orm::ActiveValue::Set(params.name),
            phone_number: sea_orm::ActiveValue::Set(params.phone_number),
            description: sea_orm::ActiveValue::Set(params.description),
            address: sea_orm::ActiveValue::Set(params.address),
            user_id: sea_orm::ActiveValue::Set(user_id),
            ..Default::default()
        };
        Ok(boss.save(db).await?.try_into_model()?)
    }

    pub async fn delete(db: &DbConn, user_id: i32, id: i32) -> Result<Model> {
        let boss = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Cannot find boss".into()))?;
        if boss.user_id != user_id {
            return Err(DbErr::RecordNotFound("Cannot find boss".into()).into());
        }
        let boss_clone = boss.clone();
        boss.delete(db).await?;
        Ok(boss_clone)
    }

    pub async fn update(
        db: &DbConn,
        user_id: i32,
        id: i32,
        params: UpdateBossParams,
    ) -> Result<Model> {
        let boss = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Cannot find boss".into()))?;
        if boss.user_id != user_id {
            return Err(DbErr::RecordNotFound("Cannot find boss".into()).into());
        }
        let mut boss = boss.into_active_model();
        if let Some(name) = params.name {
            boss.name = sea_orm::ActiveValue::Set(name);
        }
        if let Some(phone_number) = params.phone_number {
            boss.phone_number = sea_orm::ActiveValue::Set(phone_number);
        }
        boss.description = sea_orm::ActiveValue::Set(params.description);
        boss.address = sea_orm::ActiveValue::Set(params.address);

        Ok(boss.update(db).await?)
    }

    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Model> {
        let boss = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Cannot find boss".into()))?;

        Ok(boss)
    }

    // 查找用户关联的boss
    pub async fn find_by_user_id(
        db: &DbConn,
        user_id: i32,
        params: ListQueryParams,
    ) -> Result<ListResult<Model>> {
        let mut select = Entity::find().order_by_desc(Column::CreateAt);

        select = select.filter(Column::UserId.eq(user_id));

        if let Some(search) = params.search {
            select = select.filter(
                Column::Name
                    .contains(&search)
                    .or(Column::Description.contains(&search)),
            );
        }
        let total = select.clone().count(db).await?;
        let (page, page_size) = (params.page.unwrap_or(0), params.page_size.unwrap_or(10));
        let data = select.paginate(db, page_size).fetch_page(page).await?;

        Ok(ListResult { total, data })
    }
}
