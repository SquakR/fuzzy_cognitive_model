use std::path::PathBuf;

use crate::models::{Session, User};
use crate::pagination::Paginate;
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::users;
use crate::services::{email_confirmation_services, password_services};
use crate::storage::Storage;
use crate::types::{
    PaginationInType, PaginationOutType, UserInChangeType, UserInCreateType, UserOutType,
};
use diesel::dsl::sql;
use diesel::pg::{Pg, PgConnection};
use diesel::prelude::*;
use diesel::sql_types::{Bool, Text};

pub async fn create_user(
    conn: &mut PgConnection,
    storage: &Storage,
    mut user_in: UserInCreateType<'_>,
) -> ServiceResult<User> {
    let exist_user =
        find_exist_user(conn, Some(&user_in.username), Some(&user_in.email)).to_service_result()?;
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
    let (user, email_confirmation) = conn
        .transaction(|conn| {
            let user = diesel::insert_into(users::table)
                .values((
                    users::username.eq(user_in.username),
                    users::password.eq(password_services::hash_password(&user_in.password)),
                    users::email.eq(user_in.email),
                    users::first_name.eq(user_in.first_name),
                    users::second_name.eq(user_in.second_name),
                    users::last_name.eq(user_in.last_name),
                    users::avatar.eq(avatar.and_then(|p| Some(p.to_str().unwrap().to_owned()))),
                    users::language.eq(user_in.language.and_then(|l| {
                        if l == "" {
                            None
                        } else {
                            Some(l)
                        }
                    })),
                ))
                .get_result::<User>(conn)?;
            let email_confirmation =
                email_confirmation_services::create_email_confirmation(conn, &user)?;
            Ok((user, email_confirmation))
        })
        .to_service_result()?;
    email_confirmation_services::send_email_confirmation_email(&email_confirmation).await?;
    Ok(user)
}

pub fn find_user_by_id(conn: &mut PgConnection, user_id: i32) -> QueryResult<User> {
    users::table.find(user_id).first::<User>(conn)
}

pub fn find_user_by_username(conn: &mut PgConnection, username: &str) -> QueryResult<User> {
    users::table
        .filter(users::username.eq(username))
        .first::<User>(conn)
}

pub fn find_user_by_email(conn: &mut PgConnection, email: &str) -> QueryResult<User> {
    users::table
        .filter(users::email.eq(email))
        .first::<User>(conn)
}

pub fn find_user_by_session(conn: &mut PgConnection, session: &Session) -> User {
    users::table
        .filter(users::id.eq(session.user_id))
        .first::<User>(conn)
        .unwrap()
}

pub fn find_users_by_id(
    conn: &mut PgConnection,
    user_ids: impl Iterator<Item = i32>,
) -> QueryResult<Vec<User>> {
    users::table
        .filter(users::id.eq_any(user_ids))
        .get_results::<User>(conn)
}

pub fn filter_users<'a>(search: Option<String>) -> users::BoxedQuery<'a, Pg> {
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

pub fn paginate_users(
    conn: &mut PgConnection,
    search: Option<String>,
    pagination: PaginationInType,
) -> ServiceResult<PaginationOutType<UserOutType>> {
    let (users, total_pages) = filter_users(search)
        .paginate(pagination.page as i64)
        .per_page(pagination.per_page as i64)
        .load_and_count_pages::<User>(conn)
        .to_service_result()?;
    Ok(PaginationOutType {
        data: users
            .into_iter()
            .map(UserOutType::from)
            .collect::<Vec<UserOutType>>(),
        total_pages: total_pages as i32,
    })
}

pub fn confirm_user_email(conn: &mut PgConnection, user: User) -> QueryResult<User> {
    diesel::update(users::table)
        .filter(users::id.eq(user.id))
        .set(users::is_email_confirmed.eq(true))
        .get_result::<User>(conn)
}

pub async fn change_user(
    conn: &mut PgConnection,
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
    let exist_user = find_exist_user(conn, username, email).to_service_result()?;
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
    let (user, email_confirmation) = conn
        .transaction(|conn| {
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
                .get_result::<User>(conn)?;
            if email.is_some() {
                let email_confirmation =
                    email_confirmation_services::create_email_confirmation(conn, &user)?;
                Ok((user, Some(email_confirmation)))
            } else {
                Ok((user, None))
            }
        })
        .to_service_result()?;
    if let Some(email_confirmation) = email_confirmation {
        email_confirmation_services::send_email_confirmation_email(&email_confirmation).await?;
    }
    Ok(user)
}

pub fn change_user_language(
    conn: &mut PgConnection,
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
            .get_result::<User>(conn)
            .to_service_result()
    } else {
        Ok(user)
    }
}

fn find_exist_user(
    conn: &mut PgConnection,
    username: Option<&str>,
    email: Option<&str>,
) -> QueryResult<Option<User>> {
    if username.is_some() || email.is_some() {
        let mut query = users::table.into_boxed();
        if let Some(username) = username {
            query = query.or_filter(users::username.eq(username));
        }
        if let Some(email) = email {
            query = query.or_filter(users::email.eq(email));
        }
        return query.first::<User>(conn).optional();
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
