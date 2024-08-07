use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{
    prelude::DateTimeWithTimeZone, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};
use serde::Deserialize;

use crate::database::tasks::{self, ActiveModel, Entity as Tasks};

#[derive(Deserialize)]
pub struct RequestTask {
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub description: Option<Option<String>>,
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub deleted_at: Option<Option<DateTimeWithTimeZone>>,
}

macro_rules! set_if_some {
    ($db_task:expr, $field:ident, $value:expr) => {
        if let Some(value) = $value {
            $db_task.$field = Set(value);
        }
    };
}

impl RequestTask {
    pub fn apply_to_db_task(self, db_task: &mut ActiveModel) {
        set_if_some!(db_task, priority, self.priority);
        set_if_some!(db_task, title, self.title);
        set_if_some!(db_task, completed_at, self.completed_at);
        set_if_some!(db_task, description, self.description);
        set_if_some!(db_task, deleted_at, self.deleted_at);
    }
}

pub async fn partial_update(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), StatusCode> {
    let mut db_task = Tasks::find_by_id(task_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?
        .into_active_model();

    request_task.apply_to_db_task(&mut db_task);

    Tasks::update(db_task)
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
