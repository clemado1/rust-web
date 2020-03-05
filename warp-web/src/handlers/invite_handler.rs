use warp;

use crate::{db::postgresql::POOL, models::invitation::Invitation};

pub async fn repeat(input: String) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:#?}", &input);
    Ok(warp::reply::html(input))
}

pub async fn invite(invitation: Invitation) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Invitation::invite(
        &Invitation {
            id: uuid::Uuid::new_v4(),
            email: invitation.email,
            expired_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
        },
        &conn,
    );

    let reply = match response {
        Ok(invitation) => {
            println!("{:#?}", &invitation);
            invitation
        }
        Err(e) => {
            println!("{:#?}", e);
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}

pub async fn authenticate(session: uuid::Uuid) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = POOL.get().unwrap();
    let response = Invitation::authenticate(session, &conn);

    let reply = match response {
        Ok(invitation) => {
            println!("{:#?}", &invitation);
            invitation
        }
        Err(e) => {
            println!("{:#?}", e);
            return Err(warp::reject::not_found());
        }
    };
    Ok(warp::reply::json(&reply))
}
