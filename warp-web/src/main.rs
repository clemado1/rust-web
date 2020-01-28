use std::env;
use warp::{self, body, get, path, path::end, post, Filter};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod auth_handler;
mod invite_handler;
mod models;
mod register_handler;
mod schema;
mod utils;

/// API will be:
///
/// - `GET /auth`: return a JSON Object of session user.
/// - `POST /auth`: login.
/// - `DELETE /auth`: delete a session user.

pub fn create_connection() -> models::Pool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Faild to create poll")
}

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // also can main=info
        env::set_var("RUST_LOG", "main=debug");
    }
    pretty_env_logger::init();

    let log = warp::log("debug");

    let pool: models::Pool = create_connection();
    let db = move || pool.clone();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello =
        path!("hello" / String).map(|name| format!("Hello, {}!", name));

    // GET /invitation =>
    let invite = path!("invite").map(|| "Hello!");

    // GET /auth =>
    let auth = warp::path!("auth")
        .and(warp::post())
        .and(body::form())
        .and(db)
        .and_then(auth_handler::login())
        .or(warp::path!("auth")
            .and(warp::get())
            .and(body::form())
            .and(db)
            .and_then(auth_handler::get_me()))
        .or(warp::path!("auth")
            .and(warp::delete())
            .and(body::form())
            .and(db)
            .and_then(auth_handler::logout()));

    // GET /register =>
    let register =
        path!("register" / String).map(|name| format!("Hello, {}!", name));

    let routes = warp::get().and(hello.or(invite).or(auth).or(register));

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
