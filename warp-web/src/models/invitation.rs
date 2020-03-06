use crate::schema::session_tb;

use diesel::prelude::*;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use crate::schema::session_tb::dsl::*;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "session_tb"]
pub struct Invitation {
    pub id: uuid::Uuid,
    pub email: String,
    pub expired_at: chrono::NaiveDateTime,
}

impl Invitation {
    pub fn invite(&self, connection: &PgConnection) -> Result<Invitation, diesel::result::Error> {
        diesel::insert_into(session_tb::table)
            .values(self)
            .get_result(connection)
    }

    pub fn authenticate(
        session: uuid::Uuid,
        connection: &PgConnection,
    ) -> Result<Invitation, diesel::result::Error> {
        session_tb
            .filter(id.eq(session))
            .load::<Invitation>(connection)
            .and_then(|mut result| {
                if let Some(invitation) = result.pop() {
                    // if invitation is not expired
                    if invitation.expired_at > chrono::Local::now().naive_local() {
                        return Ok(invitation);
                    }
                }
                Err(diesel::result::Error::NotFound)
            })
    }
}
