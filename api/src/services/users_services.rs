use crate::errors::AppError;
use crate::models::{User, UserIn};
use crate::schema::users;
use argon2::{password_hash::PasswordHash, Argon2, PasswordHasher, PasswordVerifier};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use std::env;

pub fn create_user(connection: &mut PgConnection, user_in: UserIn) -> Result<User, AppError> {
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
    AppError::update_result(
        diesel::insert_into(users::table)
            .values((
                users::username.eq(user_in.username),
                users::password.eq(hash_password(&user_in.password)),
                users::email.eq(user_in.email),
                users::first_name.eq(user_in.first_name),
                users::second_name.eq(user_in.second_name),
                users::last_name.eq(user_in.last_name),
            ))
            .get_result::<User>(connection),
    )
}

pub fn get_user(connection: &mut PgConnection, user_id: i32) -> Result<User, AppError> {
    AppError::update_result(users::table.find(user_id).first::<User>(connection))
}

pub fn authenticate(
    connection: &mut PgConnection,
    username: &str,
    password: &str,
) -> Result<User, AppError> {
    let user_result = users::table
        .filter(users::username.eq(username))
        .first::<User>(connection);
    let user = match user_result {
        Ok(user) => user,
        Err(_) => {
            return Err(AppError::ValidationError(String::from(
                "Incorrect username or password",
            )))
        }
    };
    if verify_password(password, &user.password) {
        return Err(AppError::ValidationError(String::from(
            "Incorrect username or password",
        )));
    }
    return Ok(user);
}

pub fn hash_password(password: &str) -> String {
    let salt = env::var("PASSWORD_SALT").expect("PASSWORD_SALT must be set");
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
