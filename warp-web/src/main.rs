#![allow(dead_code)]

use warp::{self, Filter};

use console::Style;

mod handlers;
mod routes;
use self::{
    handlers::{auth_handler, invite_handler},
    routes::{auth_route, invite_route},
};

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use diesel::prelude::*;

mod db;
mod models;
mod schema;

mod api;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    let target: String = "0.0.0.0:8000".parse().unwrap();
    let blue = Style::new().blue();

    let main_api = invite_user!()
        .or(authenticate_user!())
        .or(join_user!())
        .or(get_user!())
        .or(login_user!())
        .or(logout_user!());

    let end = main_api.with(warp::log("main_api"));

    println!("\nRust Warp Server ready at {}", blue.apply_to(&target));
    warp::serve(end).run(([0, 0, 0, 0], 8000)).await;
}
