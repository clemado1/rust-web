#![allow(dead_code)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use std::env;
use warp::{self, body, get, path, path::end, post, Filter};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod invite_handler;
mod register_handler;
mod schema;
mod utils;
use crate::models::user;
use crate::session::POOL;

/// API will be:
///
/// - `GET /auth`: return a JSON Object of session user.
/// - `POST /auth`: login.
/// - `DELETE /auth`: delete a session user.

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // also can main=info
        env::set_var("RUST_LOG", "main=debug");
    }
    pretty_env_logger::init();

    let log = warp::log("debug");

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = path!("hello" / String).map(|name| format!("Hello, {}!", name));

    // GET /invitation =>
    let invite = path!("invite").map(|| "Hello!");

    // GET /auth =>
    let auth = path!("auth").map(|| "Hello!");

    // GET /register =>
    let register = path!("register" / String).map(|name| format!("Hello, {}!", name));

    let routes = warp::get().and(hello.or(invite).or(auth).or(register));

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
