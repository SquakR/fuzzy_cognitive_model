use crate::errors::QueryError;
use crate::models::User;
use crate::schema::users;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn get_user(connection: &mut PgConnection, user_id: i32) -> Result<User, QueryError> {
    QueryError::update_result(users::table.find(user_id).first::<User>(connection))
}
