use warp::{filters::BoxedFilter, path, Filter};

use crate::models::invitation::Invitation;

// Test
pub fn repeat() -> BoxedFilter<(String,)> {
    warp::get()
        .and(path!("repeat" / String))
        .and(warp::path::end())
        .boxed()
}

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / "invite" / "v1" / ..).boxed()
}

pub fn invite() -> BoxedFilter<(Invitation,)> {
    let json_body = warp::body::content_length_limit(1024 * 16).and(warp::body::json());

    warp::get()
        .and(path_prefix())
        .and(warp::path::end())
        .and(json_body)
        .boxed()
}

pub fn authenticate() -> BoxedFilter<(uuid::Uuid,)> {
    warp::get()
        .and(path_prefix())
        .and(warp::path::param::<uuid::Uuid>())
        .boxed()
}
