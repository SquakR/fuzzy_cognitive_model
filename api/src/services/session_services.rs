use crate::errors::AppError;
use crate::models::{Session, User};
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

pub fn get_user_active_sessions(
    connection: &mut PgConnection,
    user_id: i32,
) -> Result<Vec<Session>, AppError> {
    AppError::update_result(
        sessions::table
            .filter(sessions::user_id.eq(user_id))
            .filter(sessions::is_active.eq(true))
            .get_results::<Session>(connection),
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

pub fn deactivate_all_user_sessions(
    connection: &mut PgConnection,
    user: &User,
) -> Result<Vec<Session>, AppError> {
    AppError::update_result(
        diesel::update(sessions::table)
            .filter(sessions::user_id.eq(user.id))
            .filter(sessions::is_active.eq(true))
            .set(sessions::is_active.eq(false))
            .get_results::<Session>(connection),
    )
}

pub fn deactivate_user_session(
    connection: &mut PgConnection,
    user: &User,
    session_id: i32,
) -> Result<Session, AppError> {
    let session = find_session_by_id(connection, session_id)?;
    if !session.is_active {
        deactivate_all_user_sessions(connection, user)?;
        return Err(AppError::BadRequestError);
    }
    if session.user_id != user.id {
        return Err(AppError::BadRequestError);
    }
    AppError::update_result(
        diesel::update(sessions::table.filter(sessions::id.eq(session_id)))
            .set(sessions::is_active.eq(false))
            .get_result::<Session>(connection),
    )
}
