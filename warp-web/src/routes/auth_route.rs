use warp::{filters::BoxedFilter, path, Filter};

use crate::models::user::{NewUser, User};

// Test
pub fn repeat() -> BoxedFilter<(String,)> {
    warp::get()
        .and(path!("repeat" / String))
        .and(warp::path::end())
        .boxed()
}

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "auth" / "v1" / ..).boxed()
}

pub fn join() -> BoxedFilter<(NewUser,)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .and(json_body)
        .boxed()
}

pub fn get() -> BoxedFilter<(String,)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::get()
        .and(path_prefix())
        .and(warp::path::param::<String>())
        .boxed()
}

pub fn login() -> BoxedFilter<(User,)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .and(json_body)
        .boxed()
}

pub fn logout() -> BoxedFilter<(String,)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::get()
        .and(path_prefix())
        .and(warp::path::param::<String>())
        .boxed()
}
