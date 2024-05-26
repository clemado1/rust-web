mod custom_json_extractor;
mod hello_world;
mod validate_with_serde;

use axum::{routing::{get, post}, Router};
use custom_json_extractor::custom_json_extractor;
use validate_with_serde::validate_with_serde;

pub fn create_routes() -> Router {
    Router::new().route("/hello_world", get(hello_world::hello_world))
    .route("/validate_data", post(validate_with_serde))
    .route("/custom_json_extractor", post(custom_json_extractor))
}
