use crate::schema::session_tb;

use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use crate::schema::session_tb::dsl;
use crate::schema::session_tb::dsl::*;

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "session_tb"]
pub struct Invitation {
    pub id: uuid::Uuid,
    pub email: String,
    pub expired_at: chrono::NaiveDateTime,
}

impl Invitation {
    pub fn invite(&self, connection: &PgConnection) -> Result<User, diesel::result::Error> {
        diesel::insert_into(session_tb::table)
            .values(self)
            .get_result(connection)
    }

    pub fn authenticate(
        session: Invitation,
        connection: &PgConnection,
    ) -> Result<User, diesel::result::Error> {
        invitations
            .filter(id.eq(invitation_id))
            .load::<Invitation>(connection)
            .and_then(|mut result| {
                if let Some(invitation) = result.pop() {
                    // if invitation is not expired
                    if invitation.expires_at > chrono::Local::now().naive_local() {
                        return Ok(invitation);
                    }
                }
                Err(diesel::result::Error::NotFound)
            })
    }
}
