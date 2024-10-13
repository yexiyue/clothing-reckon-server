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
    pub price: i32,
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

        let staff = ActiveModel {
            name: sea_orm::ActiveValue::Set(params.name),
            price: sea_orm::ActiveValue::Set(params.price),
            description: sea_orm::ActiveValue::Set(params.description),
            boss_id: sea_orm::ActiveValue::Set(params.boss_id),
            image: sea_orm::ActiveValue::Set(params.image),
            ..Default::default()
        };
        Ok(staff.save(db).await?.try_into_model()?)
    }

    async fn check_permission(db: &DbConn, staff: &Model, user_id: i32) -> Result<(), DbErr> {
        if let Some(boss) = staff.find_related(::entity::boss::Entity).one(db).await? {
            if boss.user_id != user_id {
                return Err(DbErr::RecordNotFound("Cannot delete boss".into()).into());
            }
        }
        Ok(())
    }

    pub async fn delete(db: &DbConn, user_id: i32, id: i32) -> Result<Model, DbErr> {
        let staff = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Cannot find staff".into()))?;

        let staff_clone = staff.clone();

        Self::check_permission(db, &staff, user_id).await?;

        staff.delete(db).await?;
        Ok(staff_clone)
    }

    // 只能修改一些基本信息不能修改价格
    pub async fn update(
        db: &DbConn,
        user_id: i32,
        id: i32,
        params: UpdateClothingParams,
    ) -> Result<Model, DbErr> {
        let staff = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Cannot find staff".into()))?;

        Self::check_permission(db, &staff, user_id).await?;

        let mut staff = staff.into_active_model();

        if let Some(name) = params.name {
            staff.name = sea_orm::ActiveValue::Set(name);
        }

        staff.description = sea_orm::ActiveValue::Set(params.description);
        staff.image = sea_orm::ActiveValue::Set(params.image);

        Ok(staff.update(db).await?)
    }

    pub async fn find_by_id(db: &DbConn, user_id: i32, id: i32) -> Result<Model, DbErr> {
        let staff = Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("Cannot find staff".into()))?;

        Self::check_permission(db, &staff, user_id).await?;

        Ok(staff)
    }

    pub async fn find_by_user_id(
        db: &DbConn,
        user_id: i32,
        params: ClothingListQueryParams,
    ) -> Result<ListResult<Model>, DbErr> {
        let mut select = Entity::find().order_by_desc(Column::CreateAt);

        select = select.filter(
            Column::BossId.in_subquery(
                Query::select()
                    .column(boss::Column::Id)
                    .and_where(boss::Column::UserId.eq(user_id))
                    .and_where_option(params.boss_ids.map(|ids| boss::Column::Id.is_in(ids)))
                    .from(boss::Entity)
                    .to_owned(),
            ),
        );

        if let Some(search) = params.list_query.search {
            select = select.filter(
                Column::Name
                    .contains(&search)
                    .or(Column::Description.contains(&search)),
            );
        }

        if let Some(start_time) = params.list_query.start_time {
            select = select.filter(Column::CreateAt.gt(start_time));
        }

        if let Some(end_time) = params.list_query.end_time {
            select = select.filter(Column::CreateAt.lt(end_time));
        }

        let total = select.clone().count(db).await?;
        let (page, page_size) = (
            params.list_query.page.unwrap(),
            params.list_query.page_size.unwrap(),
        );
        let data = select.paginate(db, page_size).fetch_page(page).await?;

        Ok(ListResult { total, data })
    }
}
