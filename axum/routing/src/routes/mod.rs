mod always_errors;
mod get_json;
mod hello_world;
mod middleware_message;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_headers;
mod mirror_user_agent;
mod path_variables;
mod query_params;
mod read_middleware_custom_header;
mod set_middleware_custom_header;
mod returns_201;

use axum::{
    http::Method,
    middleware,
    routing::{get, post},
    Extension, Router,
};

use always_errors::always_errors;
use get_json::get_json;
use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_headers::mirror_custom_headers;
use mirror_user_agent::mirror_user_agent;
use path_variables::{hard_coded_path, path_variables};
use query_params::query_params;
use read_middleware_custom_header::read_middleware_custom_header;
use set_middleware_custom_header::set_middleware_custom_header;
use returns_201::returns_201;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from shared data".to_owned(),
    };

    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/middleware_message", get(middleware_message))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_headers", get(mirror_custom_headers))
        .route("/path_variables/15", get(hard_coded_path))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .layer(Extension(shared_data))
        .layer(cors)
        .route("/always_errors", get(always_errors))
        .route("/returns_201", post(returns_201))
        .route("/get_json", get(get_json))
}
