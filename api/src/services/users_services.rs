use std::path::PathBuf;

use crate::errors::AppError;
use crate::models::{Session, User};
use crate::schema::users;
use crate::services::session_services;
use crate::storage::Storage;
use crate::types::{ChangePassword, Credentials, UserInChange, UserInCreate};
use crate::utils;
use argon2::{password_hash::PasswordHash, Argon2, PasswordHasher, PasswordVerifier};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

pub async fn create_user(
    connection: &mut PgConnection,
    storage: &Storage,
    mut user_in: UserInCreate<'_>,
) -> Result<User, AppError> {
    let exist_user = find_exist_user(connection, Some(&user_in.username), Some(&user_in.email))?;
    if let Some(exist_user) = exist_user {
        return Err(get_exist_user_app_error(
            exist_user,
            &user_in.username,
            &user_in.email,
        ));
    }
    let mut avatar = None;
    if let Some(avatar_file) = user_in.avatar.take() {
        if avatar_file.len() > 0 {
            avatar = Some(storage.add_user_avatar(avatar_file).await?);
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
                users::avatar.eq(avatar.and_then(|p| Some(p.to_str().unwrap().to_owned()))),
            ))
            .get_result::<User>(connection),
    )
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

pub async fn change_user(
    connection: &mut PgConnection,
    storage: &Storage,
    user: User,
    mut user_in: UserInChange<'_>,
) -> Result<User, AppError> {
    let username = if user_in.username != user.username {
        Some(user_in.username.as_str())
    } else {
        None
    };
    let email = if user_in.email != user.email {
        Some(user_in.email.as_str())
    } else {
        None
    };
    let exist_user = find_exist_user(connection, username, email)?;
    if let Some(exist_user) = exist_user {
        return Err(get_exist_user_app_error(
            exist_user,
            &user_in.username,
            &user_in.email,
        ));
    }
    let avatar = if user_in.reset_avatar {
        None
    } else {
        let mut avatar = if let Some(avatar) = user.avatar {
            Some(PathBuf::from(avatar))
        } else {
            None
        };
        if let Some(avatar_file) = user_in.avatar.take() {
            if avatar_file.len() > 0 {
                avatar = Some(storage.add_user_avatar(avatar_file).await?);
            }
        }
        avatar
    };
    AppError::update_result(
        diesel::update(users::table)
            .filter(users::id.eq(&user.id))
            .set((
                users::username.eq(user_in.username),
                users::email.eq(user_in.email),
                users::first_name.eq(user_in.first_name),
                users::second_name.eq(user_in.second_name),
                users::last_name.eq(user_in.last_name),
                users::avatar.eq(avatar.and_then(|p| Some(p.to_str().unwrap().to_owned()))),
            ))
            .get_result::<User>(connection),
    )
}

pub fn change_user_password(
    connection: &mut PgConnection,
    user: &User,
    change_password: ChangePassword,
) -> Result<(), AppError> {
    if !verify_password(&change_password.old_password, &user.password) {
        return Err(AppError::ValidationError(String::from(
            "Incorrect old password.",
        )));
    }
    let new_password_hash = hash_password(&change_password.new_password);
    if new_password_hash == user.password {
        return Err(AppError::ValidationError(String::from(
            "The new password must not be the same as the old one.",
        )));
    }
    AppError::update_result(
        diesel::update(users::table)
            .filter(users::id.eq(&user.id))
            .set(users::password.eq(new_password_hash))
            .execute(connection),
    )?;
    Ok(())
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

pub fn find_exist_user(
    connection: &mut PgConnection,
    username: Option<&str>,
    email: Option<&str>,
) -> Result<Option<User>, AppError> {
    if username.is_some() || email.is_some() {
        let mut query = users::table.into_boxed();
        if let Some(username) = username {
            query = query.or_filter(users::username.eq(username));
        }
        if let Some(email) = email {
            query = query.or_filter(users::email.eq(email));
        }
        return match query.first::<User>(connection) {
            Err(diesel_error) => {
                if diesel_error == DieselError::NotFound {
                    Ok(None)
                } else {
                    Err(AppError::from_diesel_error(diesel_error))
                }
            }
            Ok(exist_user) => Ok(Some(exist_user)),
        };
    }
    Ok(None)
}

pub fn get_exist_user_app_error(exist_user: User, username: &str, email: &str) -> AppError {
    if exist_user.username == username {
        return AppError::ValidationError(String::from(
            "A user with this username already exists.",
        ));
    }
    if exist_user.email == email {
        return AppError::ValidationError(String::from("A user with this email already exists."));
    }
    unreachable!();
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
