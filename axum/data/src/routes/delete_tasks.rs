use axum::{extract::Path, http::StatusCode, Extension};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};

use crate::database::tasks::{self, Entity as Tasks};

pub async fn delete_task(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<(), StatusCode> {
    // select delete
    let task = Tasks::find_by_id(task_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?
        .into_active_model();

    Tasks::delete(task)
        .exec(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    // delete by id
    Tasks::delete_by_id(task_id)
        .exec(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    // delete all by filter
    Tasks::delete_many()
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
