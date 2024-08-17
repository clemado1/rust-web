use axum::{http::StatusCode, Extension, Json};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};
use serde::{Deserialize, Serialize};

use crate::database::users::Entity as Users;
use crate::database::users::{self, Model};

#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseUser {
    id: i32,
    username: String,
    token: String,
}

pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let new_user = users::ActiveModel {
        username: Set(request_user.username),
        password: Set(request_user.password),
        token: Set(Some("temporary_token".to_owned())),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_op| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        id: new_user.id.unwrap(),
        username: new_user.username.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}

pub async fn login(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let db_user = Users::find()
        .filter(users::Column::Username.eq(request_user.username))
        .one(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let new_token = "temporary_new_token".to_owned();
    let mut user = db_user.into_active_model();

    user.token = Set(Some(new_token));
    let saved_user = user
        .save(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        id: saved_user.id.unwrap(),
        username: saved_user.username.unwrap(),
        token: saved_user.token.unwrap().unwrap(),
    }))
}

pub async fn logout(
    Extension(database): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<(), StatusCode> {
    let mut user = user.into_active_model();
    user.token = Set(None);

    user.save(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
