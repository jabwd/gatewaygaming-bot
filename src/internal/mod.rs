use diesel::{connection::Connection, PgConnection, prelude::*};
use dotenv::dotenv;

pub fn establish_db_connection() {
    // TODO: Connection pooling?
    let db_url = env::var("DATABASE_URL").expect("Unable to read database_url from ENV");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}
