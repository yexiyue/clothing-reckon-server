use ::entity::staff::{ActiveModel, Column, Entity, Model};
use anyhow::Result;
use sea_orm::*;
use serde::{Deserialize, Serialize};

use crate::{ListQueryParams, ListResult};
pub struct StaffService;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateStaffParams {
    pub name: String,
    pub phone_number: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateStaffParams {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub description: Option<String>,
}

impl StaffService {
    pub async fn create(db: &DbConn, user_id: i32, params: CreateStaffParams) -> Result<Model> {
        let staff = ActiveModel {
            name: sea_orm::ActiveValue::Set(params.name),
            phone_number: sea_orm::ActiveValue::Set(params.phone_number),
            description: sea_orm::ActiveValue::Set(params.description),
            user_id: sea_orm::ActiveValue::Set(user_id),
            ..Default::default()
        };
        Ok(staff.save(db).await?.try_into_model()?)
    }

    pub async fn delete(db: &DbConn, user_id: i32, id: i32) -> Result<Model> {
        let staff = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Cannot find staff".into()))?;

        if staff.user_id != user_id {
            return Err(DbErr::RecordNotFound("Cannot find staff".into()).into());
        }

        let staff_clone = staff.clone();
        staff.delete(db).await?;
        Ok(staff_clone)
    }

    pub async fn update(
        db: &DbConn,
        user_id: i32,
        id: i32,
        params: UpdateStaffParams,
    ) -> Result<Model> {
        let staff = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Cannot find staff".into()))?;

        if staff.user_id != user_id {
            return Err(DbErr::RecordNotFound("Cannot find staff".into()).into());
        }
        let mut staff = staff.into_active_model();
        if let Some(name) = params.name {
            staff.name = sea_orm::ActiveValue::Set(name);
        }

        if let Some(phone_number) = params.phone_number {
            staff.phone_number = sea_orm::ActiveValue::Set(phone_number);
        }

        staff.description = sea_orm::ActiveValue::Set(params.description);

        Ok(staff.update(db).await?)
    }

    pub async fn find_by_id(db: &DbConn, user_id: i32, id: i32) -> Result<Model> {
        Ok(Entity::find_by_id(id)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Cannot find staff".into()))?)
    }

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
