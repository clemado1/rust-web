use crate::schema::user_tb;

use bcrypt;
use diesel::prelude::*;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use log::error;

use crate::schema::user_tb::dsl;
use crate::schema::user_tb::dsl::*;

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "user_tb"]
pub struct NewUser {
    pub email: String,
    pub passwd: String,
    pub username: String,
    pub nickname: String,
    pub created_at: chrono::NaiveDateTime,
}

impl NewUser {
    pub fn join(&self, connection: &PgConnection) -> Result<User, diesel::result::Error> {
        diesel::insert_into(user_tb::table)
            .values(self)
            .get_result(connection)
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub email: String,
    pub passwd: String,
    pub username: String,
    pub nickname: String,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
    pub fn get_me(
        user_email: &str,
        connection: &PgConnection,
    ) -> Result<User, diesel::result::Error> {
        user_tb::table.find(user_email).first(connection)
    }

    pub fn login(log_email: &str, log_passwd: &str, connection: &PgConnection) -> Option<Self> {
        let user: Result<User, diesel::result::Error> =
            user_tb::table.find(log_email).first(connection);

        match user {
            Ok(user) => match bcrypt::verify(&log_passwd, &user.passwd) {
                Ok(true) => return Some(user),
                Ok(false) => return None,
                Err(e) => {
                    error!("Verify failed for {:?}: {:?}", user, e);
                    return None;
                }
            },
            Err(e) => {
                println!("{:#?}", e);
                return None;
            }
        };
    }
}
