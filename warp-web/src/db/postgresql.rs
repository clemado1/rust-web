use dotenv::dotenv;
use std::env;

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
lazy_static! {
    pub static ref POOL: Pool = {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set.");

        // create db connection pool
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        pool
    };
}
