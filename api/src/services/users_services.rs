use crate::errors::AppError;
use crate::models::{Session, User};
use crate::schema::users;
use crate::services::session_services;
use crate::storage::Storage;
use crate::types::{Credentials, UserIn};
use crate::utils;
use argon2::{password_hash::PasswordHash, Argon2, PasswordHasher, PasswordVerifier};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

pub async fn create_user(
    connection: &mut PgConnection,
    storage: &Storage,
    mut user_in: UserIn<'_>,
) -> Result<User, AppError> {
    match users::table
        .filter(users::username.eq(&user_in.username))
        .or_filter(users::email.eq(&user_in.email))
        .first::<User>(connection)
    {
        Err(diesel_error) => {
            if diesel_error != DieselError::NotFound {
                return Err(AppError::from_diesel_error(diesel_error));
            }
        }
        Ok(exist_user) => {
            if exist_user.username == user_in.username {
                return Err(AppError::ValidationError(String::from(
                    "A user with this username already exists.",
                )));
            }
            if exist_user.email == user_in.email {
                return Err(AppError::ValidationError(String::from(
                    "A user with this email already exists.",
                )));
            }
        }
    }
    let mut avatar = None;
    if let Some(avatar_file) = user_in.avatar.take() {
        avatar = Some(storage.add_user_avatar(avatar_file).await?);
    }
    AppError::update_result(
        diesel::insert_into(users::table)
            .values((
                users::username.eq(user_in.username),
                users::password.eq(hash_password(&user_in.password)),
                users::email.eq(user_in.email),
                users::first_name.eq(user_in.first_name),
                users::second_name.eq(user_in.second_name),
                users::last_name.eq(user_in.last_name),
                users::avatar.eq(avatar.and_then(|p| Some(p.to_str().unwrap().to_owned()))),
            ))
            .get_result::<User>(connection),
    )
}

pub fn find_user_by_id(connection: &mut PgConnection, user_id: i32) -> Result<User, AppError> {
    AppError::update_result(users::table.find(user_id).first::<User>(connection))
}

pub fn find_user_by_username(
    connection: &mut PgConnection,
    username: &str,
) -> Result<User, AppError> {
    AppError::update_result(
        users::table
            .filter(users::username.eq(username))
            .first::<User>(connection),
    )
}

pub fn find_user_by_session(connection: &mut PgConnection, session: &Session) -> User {
    users::table
        .filter(users::id.eq(session.user_id))
        .first::<User>(connection)
        .unwrap()
}

pub fn sign_in(
    connection: &mut PgConnection,
    credentials: Credentials,
) -> Result<Session, AppError> {
    let user_result = find_user_by_username(connection, &credentials.username);
    let user = match user_result {
        Ok(user) => user,
        Err(_) => {
            return Err(AppError::ValidationError(String::from(
                "Incorrect username or password.",
            )))
        }
    };
    if !verify_password(&credentials.password, &user.password) {
        return Err(AppError::ValidationError(String::from(
            "Incorrect username or password.",
        )));
    }
    session_services::create_session(connection, user.id)
}

pub fn sign_out(
    connection: &mut PgConnection,
    user: &User,
    session_id: i32,
) -> Result<Session, AppError> {
    session_services::deactivate_user_session(connection, user, session_id)
}

pub fn hash_password(password: &str) -> String {
    let salt = utils::get_env("PASSWORD_SALT");
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    Argon2::default()
        .verify_password(password.as_bytes(), &PasswordHash::new(hash).unwrap())
        .is_ok()
}
