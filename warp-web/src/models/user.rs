use crate::schema::user_tb;

use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

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
    pub fn join(&self, connection: &PgConnection) -> Result<Post, deisel::result::Error> {
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
}

pub struct LogUser {
    pub email: String,
    pub passwd: String,
}

impl User {
    pub fn get_me(
        log_user: LogUser,
        connection: &PgConnection,
    ) -> Result<User, diesel::result::Error> {
        user_tb::table.find(log_user.email).first(connection);

        /**
         * TO-DO: 비밀번호 인증
         */
    }
}
