use warp;
use warp::{self, Rejection, Reply};

use crate::{db::postgresql::POOL, models::invitation::Invitation};

pub async fn repeat(input: String) -> Result<Reply, Rejection> {
    println!("{:#?}", &input);
    Ok(warp::reply::html(input))
}

pub async fn invite(email: String) -> Result<Reply, Rejection> {
    let conn = POOL.get().unwarp();
    let response = Invitation::invite(
        Invitation {
            id: uuid::Uuid::new_v4(),
            email: email,
            expired_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
        },
        &conn,
    );

    let reply = match response {
        Ok(invitation) => {
            println!("{:#?}", &user);
            invitation
        }
        Err(e) => {
            println!("{:#?}", e);
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn register(session: Invitation) -> Result<Reply, Rejection> {
    let conn = POOL.get().unwarp();
    let response = Invitation::register(session, &conn);

    let reply = match response {
        Ok(invitation) => {
            println!("{:#?}", &user);
            invitation
        }
        Err(e) => {
            println!("{:#?}", e);
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}
