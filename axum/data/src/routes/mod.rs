mod create_task;
mod custom_json_extractor;
mod get_tasks;
mod hello_world;
mod validate_with_serde;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use sea_orm::DatabaseConnection;

use create_task::create_task;
use custom_json_extractor::custom_json_extractor;
use get_tasks::{get_all_tasks, get_one_task};
use hello_world::hello_world;
use validate_with_serde::validate_with_serde;

pub fn create_routes(database: DatabaseConnection) -> Router {
    Router::new()
        .route("/hello_world", get(hello_world))
        .route("/validate_data", post(validate_with_serde))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/:task_id", get(get_one_task))
        .layer(Extension(database))
}
