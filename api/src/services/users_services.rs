use std::path::PathBuf;

use crate::errors::AppError;
use crate::models::{Session, User};
use crate::schema::users;
use crate::services::email_confirmation_services;
use crate::services::password_services;
use crate::services::session_services;
use crate::storage::Storage;
use crate::types::{CredentialsType, UserInChangeType, UserInCreateType, UserOutType};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use ipnetwork::IpNetwork;

pub async fn create_user(
    connection: &mut PgConnection,
    storage: &Storage,
    mut user_in: UserInCreateType<'_>,
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
    let user = AppError::update_result(
        diesel::insert_into(users::table)
            .values((
                users::username.eq(user_in.username),
                users::password.eq(password_services::hash_password(&user_in.password)),
                users::email.eq(user_in.email),
                users::first_name.eq(user_in.first_name),
                users::second_name.eq(user_in.second_name),
                users::last_name.eq(user_in.last_name),
                users::avatar.eq(avatar.and_then(|p| Some(p.to_str().unwrap().to_owned()))),
            ))
            .get_result::<User>(connection),
    )?;
    let email_confirmation =
        email_confirmation_services::create_email_confirmation(connection, &user)?;
    email_confirmation_services::send_email_confirmation_email(&email_confirmation).await?;
    Ok(user)
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

pub fn find_user_by_email(connection: &mut PgConnection, email: &str) -> Result<User, AppError> {
    AppError::update_result(
        users::table
            .filter(users::email.eq(email))
            .first::<User>(connection),
    )
}

pub fn find_user_by_session(connection: &mut PgConnection, session: &Session) -> User {
    users::table
        .filter(users::id.eq(session.user_id))
        .first::<User>(connection)
        .unwrap()
}

pub fn confirm_user_email(connection: &mut PgConnection, user: User) -> Result<User, AppError> {
    AppError::update_result(
        diesel::update(users::table)
            .filter(users::id.eq(user.id))
            .set(users::is_email_confirmed.eq(true))
            .get_result::<User>(connection),
    )
}

pub async fn change_user(
    connection: &mut PgConnection,
    storage: &Storage,
    user: User,
    mut user_in: UserInChangeType<'_>,
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
    let is_email_confirmed = if email.is_none() {
        user.is_email_confirmed
    } else {
        false
    };
    let user = AppError::update_result(
        diesel::update(users::table)
            .filter(users::id.eq(&user.id))
            .set((
                users::username.eq(&user_in.username),
                users::email.eq(&user_in.email),
                users::is_email_confirmed.eq(is_email_confirmed),
                users::first_name.eq(&user_in.first_name),
                users::second_name.eq(&user_in.second_name),
                users::last_name.eq(&user_in.last_name),
                users::avatar.eq(avatar.and_then(|p| Some(p.to_str().unwrap().to_owned()))),
            ))
            .get_result::<User>(connection),
    )?;
    if email.is_some() {
        let email_confirmation =
            email_confirmation_services::create_email_confirmation(connection, &user)?;
        email_confirmation_services::send_email_confirmation_email(&email_confirmation).await?;
    }
    Ok(user)
}

pub fn sign_in(
    connection: &mut PgConnection,
    credentials: CredentialsType,
    ip_address: &IpNetwork,
    user_agent: &str,
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
    if !password_services::verify_password(&credentials.password, &user.password) {
        return Err(AppError::ValidationError(String::from(
            "Incorrect username or password.",
        )));
    }
    session_services::create_session(connection, user.id, ip_address, user_agent)
}

pub fn sign_out(
    connection: &mut PgConnection,
    user: &User,
    session_id: i32,
) -> Result<Session, AppError> {
    session_services::deactivate_user_session(connection, user, session_id)
}

fn find_exist_user(
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

fn get_exist_user_app_error(exist_user: User, username: &str, email: &str) -> AppError {
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

impl From<User> for UserOutType {
    fn from(value: User) -> Self {
        UserOutType {
            id: value.id,
            username: value.username,
            email: value.email,
            is_email_confirmed: value.is_email_confirmed,
            first_name: value.first_name,
            second_name: value.second_name,
            last_name: value.last_name,
            avatar: value.avatar,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
