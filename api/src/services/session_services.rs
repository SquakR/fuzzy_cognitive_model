use crate::errors::AppError;
use crate::models::Session;
use crate::schema::sessions;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn create_session(connection: &mut PgConnection, user_id: i32) -> Result<Session, AppError> {
    AppError::update_result(
        diesel::insert_into(sessions::table)
            .values(sessions::user_id.eq(user_id))
            .get_result::<Session>(connection),
    )
}

pub fn find_session_by_id(
    connection: &mut PgConnection,
    session_id: i32,
) -> Result<Session, AppError> {
    AppError::update_result(
        sessions::table
            .find(session_id)
            .first::<Session>(connection),
    )
}
