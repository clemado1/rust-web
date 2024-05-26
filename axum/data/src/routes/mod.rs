mod hello_world;

use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new().route("/hello_world", get(hello_world::hello_world))
}
