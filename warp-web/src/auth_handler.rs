/**
 * T0DO: cookie -> session
 */
use diesel::prelude::*;
use diesel::PgConnection;
use futures::Future;

use crate::models::{Pool, SlimUser, User};
use crate::utils::verify;

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

// we need the same data
// simple aliasing makes the intentions clear and its more readable
pub type LoggedUser = SlimUser;

pub fn logout() {}

pub fn login() {}

pub fn get_me(logged_user: LoggedUser) {}

/// Diesel query
fn query(
    auth_data: AuthData,
    pool: web::Data<Pool>,
) -> Result<SlimUser, Err> {
    use crate::schema::user_tb::dsl::{email, users};
    let conn: &PgConnection = &pool.get().unwrap();

    let mut items = users
        .filter(email.eq(&auth_data.email))
        .load::<User>(conn)?;

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.hash, &auth_data.password) {
            if matching {
                return Ok(user.into()); // convert into slimUser
            }
        }
    }
    Err()
}
