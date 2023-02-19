use std::path::PathBuf;

use crate::models::{Session, User};
use crate::response::{AppError, ServiceResult, ToAppError, ToServiceResult};
use crate::schema::users;
use crate::services::email_confirmation_services;
use crate::services::password_services;
use crate::services::session_services;
use crate::storage::Storage;
use crate::types::{
    CredentialsType, PaginationInType, PaginationOutType, UserInChangeType, UserInCreateType,
    UserOutType,
};
use diesel::dsl::sql;
use diesel::pg::{Pg, PgConnection};
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::sql_types::{Bool, Text};
use ipnetwork::IpNetwork;

pub async fn create_user(
    connection: &mut PgConnection,
    storage: &Storage,
    mut user_in: UserInCreateType<'_>,
) -> ServiceResult<User> {
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
    let user = diesel::insert_into(users::table)
        .values((
            users::username.eq(user_in.username),
            users::password.eq(password_services::hash_password(&user_in.password)),
            users::email.eq(user_in.email),
            users::first_name.eq(user_in.first_name),
            users::second_name.eq(user_in.second_name),
            users::last_name.eq(user_in.last_name),
            users::avatar.eq(avatar.and_then(|p| Some(p.to_str().unwrap().to_owned()))),
            users::language.eq(user_in
                .language
                .and_then(|l| if l == "" { None } else { Some(l) })),
        ))
        .get_result::<User>(connection)
        .to_service_result()?;
    let email_confirmation =
        email_confirmation_services::create_email_confirmation(connection, &user)?;
    email_confirmation_services::send_email_confirmation_email(&email_confirmation).await?;
    Ok(user)
}

pub fn find_user_by_id(connection: &mut PgConnection, user_id: i32) -> ServiceResult<User> {
    users::table
        .find(user_id)
        .first::<User>(connection)
        .to_service_result_find(String::from("user_not_found_error"))
}

pub fn find_user_by_username(connection: &mut PgConnection, username: &str) -> ServiceResult<User> {
    users::table
        .filter(users::username.eq(username))
        .first::<User>(connection)
        .to_service_result_find(String::from("user_not_found_error"))
}

pub fn find_user_by_email(connection: &mut PgConnection, email: &str) -> ServiceResult<User> {
    users::table
        .filter(users::email.eq(email))
        .first::<User>(connection)
        .to_service_result_find(String::from("user_not_found_error"))
}

pub fn find_user_by_session(connection: &mut PgConnection, session: &Session) -> User {
    users::table
        .filter(users::id.eq(session.user_id))
        .first::<User>(connection)
        .unwrap()
}

pub fn paginate_users(
    connection: &mut PgConnection,
    pagination: PaginationInType,
) -> ServiceResult<PaginationOutType<UserOutType>> {
    let total_count = filter_users(&pagination.search)
        .count()
        .get_result::<i64>(connection)
        .to_service_result()?;
    let users = filter_users(&pagination.search)
        .limit(pagination.limit.unwrap_or(15).into())
        .offset(pagination.offset.unwrap_or(0).into())
        .get_results::<User>(connection)
        .to_service_result()?
        .into_iter()
        .map(UserOutType::from)
        .collect::<Vec<UserOutType>>();
    Ok(PaginationOutType {
        data: users,
        total_count,
    })
}

pub fn confirm_user_email(connection: &mut PgConnection, user: User) -> ServiceResult<User> {
    diesel::update(users::table)
        .filter(users::id.eq(user.id))
        .set(users::is_email_confirmed.eq(true))
        .get_result::<User>(connection)
        .to_service_result()
}

pub async fn change_user(
    connection: &mut PgConnection,
    storage: &Storage,
    user: User,
    mut user_in: UserInChangeType<'_>,
) -> ServiceResult<User> {
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
    let user = diesel::update(users::table)
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
        .get_result::<User>(connection)
        .to_service_result()?;
    if email.is_some() {
        let email_confirmation =
            email_confirmation_services::create_email_confirmation(connection, &user)?;
        email_confirmation_services::send_email_confirmation_email(&email_confirmation).await?;
    }
    Ok(user)
}

pub fn change_user_language(
    connection: &mut PgConnection,
    user: User,
    language: Option<&str>,
) -> ServiceResult<User> {
    let mut create = true;
    if let Some(user_language) = &user.language {
        if let Some(new_language) = language {
            if user_language == new_language {
                create = false;
            }
        }
    }
    if create {
        diesel::update(users::table)
            .filter(users::id.eq(&user.id))
            .set(users::language.eq(language))
            .get_result::<User>(connection)
            .to_service_result()
    } else {
        Ok(user)
    }
}

pub fn sign_in(
    connection: &mut PgConnection,
    credentials: CredentialsType,
    ip_address: &IpNetwork,
    user_agent: &str,
) -> ServiceResult<Session> {
    let user_result = find_user_by_username(connection, &credentials.username);
    let user = match user_result {
        Ok(user) => user,
        Err(_) => {
            return Err(AppError::ValidationError(Box::new(|locale| {
                t!("sign_in_credentials_error", locale = locale)
            })));
        }
    };
    if !password_services::verify_password(&credentials.password, &user.password) {
        return Err(AppError::ValidationError(Box::new(|locale| {
            t!("sign_in_credentials_error", locale = locale)
        })));
    }
    session_services::create_session(connection, user.id, ip_address, user_agent)
}

pub fn sign_out(
    connection: &mut PgConnection,
    user: &User,
    session_id: i32,
) -> ServiceResult<Session> {
    session_services::deactivate_user_session(connection, user, session_id)
}

fn find_exist_user(
    connection: &mut PgConnection,
    username: Option<&str>,
    email: Option<&str>,
) -> ServiceResult<Option<User>> {
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
                    Err(diesel_error.to_app_error(None))
                }
            }
            Ok(exist_user) => Ok(Some(exist_user)),
        };
    }
    Ok(None)
}

fn get_exist_user_app_error(exist_user: User, username: &str, email: &str) -> AppError {
    if exist_user.username == username {
        return AppError::ValidationError(Box::new(|locale| {
            t!("user_with_username_already_exists_error", locale = locale)
        }));
    }
    if exist_user.email == email {
        return AppError::ValidationError(Box::new(|locale| {
            t!("user_with_email_already_exists_error", locale = locale)
        }));
    }
    unreachable!();
}

fn filter_users<'a>(search: &Option<String>) -> users::BoxedQuery<'a, Pg> {
    match search {
        Some(search) => {
            let like_pattern = format!("{}%", search);
            users::table
                .or_filter(users::username.ilike(like_pattern.to_owned()))
                .or_filter(users::email.ilike(like_pattern.to_owned()))
                .or_filter(users::first_name.ilike(like_pattern.to_owned()))
                .or_filter(users::second_name.ilike(like_pattern.to_owned()))
                .or_filter(users::last_name.ilike(like_pattern.to_owned()))
                .or_filter(
                    sql::<Bool>("CONCAT(first_name, ' ', last_name) ILIKE ").bind::<Text, _>(like_pattern.to_owned()),
                )
                .or_filter(
                    sql::<Bool>("CONCAT(last_name, ' ', first_name) ILIKE ").bind::<Text, _>(like_pattern.to_owned()),
                )
                .or_filter(
                    sql::<Bool>("CONCAT(first_name, ' ', second_name, ' ', last_name) ILIKE ")
                        .bind::<Text, _>(like_pattern.to_owned()),
                )
                .or_filter(
                    sql::<Bool>("CONCAT(last_name, ' ', first_name, ' ', second_name) ILIKE ")
                        .bind::<Text, _>(like_pattern.to_owned()),
                )
                .or_filter(
                    sql::<Bool>("CONCAT(last_name, ' ', SUBSTR(first_name, 1, 1), ' ', SUBSTR(second_name, 1, 1)) ILIKE ")
                        .bind::<Text, _>(like_pattern.to_owned()),
                ).into_boxed()
        }
        None => users::table.into_boxed(),
    }
}

impl From<User> for UserOutType {
    fn from(user: User) -> Self {
        UserOutType {
            id: user.id,
            username: user.username,
            email: user.email,
            is_email_confirmed: user.is_email_confirmed,
            first_name: user.first_name,
            second_name: user.second_name,
            last_name: user.last_name,
            avatar: user.avatar,
            language: user.language,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}