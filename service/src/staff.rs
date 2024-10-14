use ::entity::staff::{ActiveModel, Column, Entity, Model};
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
    pub async fn create(
        db: &DbConn,
        user_id: i32,
        params: CreateStaffParams,
    ) -> Result<Model, DbErr> {
        let staff = ActiveModel {
            name: sea_orm::ActiveValue::Set(params.name),
            phone_number: sea_orm::ActiveValue::Set(params.phone_number),
            description: sea_orm::ActiveValue::Set(params.description),
            user_id: sea_orm::ActiveValue::Set(user_id),
            ..Default::default()
        };
        staff.insert(db).await
    }

    pub async fn delete(db: &DbConn, user_id: i32, id: i32) -> Result<Model, DbErr> {
        let staff = Self::find_by_id(db, user_id, id).await?;

        let staff_clone = staff.clone();
        staff.delete(db).await?;
        Ok(staff_clone)
    }

    pub async fn update(
        db: &DbConn,
        user_id: i32,
        id: i32,
        params: UpdateStaffParams,
    ) -> Result<Model, DbErr> {
        let staff = Self::find_by_id(db, user_id, id).await?;

        let mut staff = staff.into_active_model();
        if let Some(name) = params.name {
            staff.name = sea_orm::ActiveValue::Set(name);
        }

        if let Some(phone_number) = params.phone_number {
            staff.phone_number = sea_orm::ActiveValue::Set(phone_number);
        }

        staff.description = sea_orm::ActiveValue::Set(params.description);

        staff.update(db).await
    }

    pub async fn find_by_id(db: &DbConn, user_id: i32, id: i32) -> Result<Model, DbErr> {
        Entity::find_by_id(id)
            .filter(Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Cannot find staff".into()))
    }

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
