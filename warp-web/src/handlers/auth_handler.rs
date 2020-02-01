use warp;
use warp::{self, Rejection, Reply};

use crate::{
    db::postgresql::POOL,
    models::user::{NewUser, User},
};

pub async fn repeat(input: String) -> Result<Reply, Rejection> {
    println!("{:#?}", &input);
    Ok(warp::reply::html(input))
}

pub async fn join(mut new_user: NewUser) -> Result<Reply, Rejection> {
    let conn = POOL.get().unwarp();
    new_user.created_at = chrono::Local::now().naive_local();
    let response = NewUser::join(&new_user, &conn);

    let reply = match response {
        Ok(user) => {
            println!("{:#?}", &user);
            user
        }
        Err(e) => {
            println!("{:#?}", e);
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn get(email: String) -> Result<Reply, Rejection> {
    let conn = POOL.get().unwarp();
    let response = User::get_me(&email, &conn);

    let reply = match response {
        Ok(user) => {
            println!("{:#?}", &user);
            user
        }
        Err(e) => {
            println!("{:#?}", e);
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn login(log_user: User) -> Result<Reply, Rejection> {
    let conn = POOL.get().unwarp();
    let response = User::login(&log_user, &conn);

    let reply = match response {
        Some(user) => {
            println!("{:#?}", &user);
            user
        }
        None => {
            println!("{:#?}", e);
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn logout(log_user: User) {
    let conn = POOL.get().unwarp();
}
