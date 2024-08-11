use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Extension,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::Deserialize;

use crate::database::tasks::{self, Entity as Tasks};

#[derive(Deserialize)]
pub struct QueryParams {
    soft: bool,
}

pub async fn delete_task(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Query(query_params): Query<QueryParams>,
) -> Result<(), StatusCode> {
    // select delete
    // let task = Tasks::find_by_id(task_id)
    //     .one(&database)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    //     .ok_or(StatusCode::NOT_FOUND)?
    //     .into_active_model();

    // Tasks::delete(task)
    //     .exec(&database)
    //     .await
    //     .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    // // delete by id
    // Tasks::delete_by_id(task_id)
    //     .exec(&database)
    //     .await
    //     .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    if query_params.soft {
        let mut task = Tasks::find_by_id(task_id)
            .one(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::NOT_FOUND)?
            .into_active_model();

        let now = chrono::Utc::now();
        task.deleted_at = Set(Some(now.into()));

        Tasks::update(task)
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        Tasks::delete_many()
            .filter(tasks::Column::Id.eq(task_id))
            .exec(&database)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(())
}
