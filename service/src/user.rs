pub struct UserService;
use ::entity::user::{ActiveModel, Column, Entity, Model};
use anyhow::Result;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUserParams {
    pub username: String,
    pub password: String,
    pub phone_number: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateUserParams {
    pub username: Option<String>,
    pub password: Option<String>,
    pub phone_number: Option<String>,
}

impl UserService {
    pub async fn create(db: &DbConn, params: CreateUserParams) -> Result<Model> {
        let user = ActiveModel {
            username: sea_orm::ActiveValue::Set(params.username),
            password: sea_orm::ActiveValue::Set(params.password),
            phone_number: sea_orm::ActiveValue::Set(params.phone_number),
            ..Default::default()
        };
        Ok(user.save(db).await?.try_into_model()?)
    }

    pub async fn delete(db: &DbConn, id: i32) -> Result<Model> {
        let user = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Cannot find user".into()))?;

        let user_clone = user.clone();
        user.delete(db).await?;

        Ok(user_clone)
    }

    pub async fn update(db: &DbConn, id: i32, params: UpdateUserParams) -> Result<Model> {
        let user = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Cannot find user".into()))?;

        let mut user = user.into_active_model();

        if let Some(username) = params.username {
            user.username = sea_orm::ActiveValue::Set(username);
        }

        if let Some(password) = params.password {
            user.password = sea_orm::ActiveValue::Set(password);
        }

        if let Some(phone_number) = params.phone_number {
            user.phone_number = sea_orm::ActiveValue::Set(phone_number);
        }

        Ok(user.update(db).await?)
    }

    pub async fn find_by_phone_number(db: &DbConn, phone_number: String) -> Result<Model> {
        let user = Entity::find()
            .filter(Column::PhoneNumber.eq(phone_number))
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Cannot find user".into()))?;

        Ok(user)
    }
}
