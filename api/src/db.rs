use crate::utils;
use diesel::prelude::*;
use diesel::PgConnection;

pub fn establish_connection() -> PgConnection {
    let database_url = utils::get_env("DATABASE_URL");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
