use crate::models::{PasswordReset, User};
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::{password_resets, users};
use crate::services::{mailing_services, session_services, user_services};
use crate::types::{ChangePasswordType, ResetPasswordType};
use crate::utils;
use argon2::password_hash::{PasswordHash, Salt};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use diesel::prelude::*;
use diesel::PgConnection;
use jwt::{SignWithKey, VerifyWithKey};
use std::collections::BTreeMap;

pub fn change_user_password(
    conn: &mut PgConnection,
    user: &User,
    change_password: ChangePasswordType,
) -> ServiceResult<()> {
    if !verify_password(&change_password.old_password, &user.password) {
        return Err(AppError::ValidationError(Box::new(|locale| {
            t!("incorrect_old_password_error", locale = locale)
        })));
    }
    let new_password_hash = hash_password(&change_password.new_password);
    if new_password_hash == user.password {
        return Err(AppError::ValidationError(Box::new(|locale| {
            t!("passwords_equal_error", locale = locale)
        })));
    }
    set_password(conn, user.id, &new_password_hash).to_service_result()?;
    Ok(())
}

pub async fn request_password_reset(
    conn: &mut PgConnection,
    email: &str,
) -> ServiceResult<PasswordReset> {
    let user = user_services::find_user_by_email(conn, email)
        .to_service_result_find(String::from("user_not_found_error"))?;
    if !user.is_email_confirmed {
        return Err(AppError::ValidationError(Box::new(|locale| {
            t!("reset_password_email_confirmation_error", locale = locale)
        })));
    }
    let password_reset = create_password_reset(conn, &user).to_service_result()?;
    send_password_reset_email(&user, &password_reset).await?;
    Ok(password_reset)
}

pub fn reset_password(
    conn: &mut PgConnection,
    reset_password: ResetPasswordType,
) -> ServiceResult<()> {
    let key = utils::get_jwt_key();
    let claims: BTreeMap<String, i32> = match reset_password.token.verify_with_key(&key) {
        Ok(claims) => claims,
        Err(_) => {
            return Err(AppError::ValidationError(Box::new(|locale| {
                t!("invalid_token_error", locale = locale)
            })));
        }
    };
    let password_reset = find_password_reset_by_id(conn, claims["password_reset_id"])
        .to_service_result_find(String::from("password_reset_not_found_error"))?;
    if !password_reset.is_valid {
        return Err(AppError::ValidationError(Box::new(|locale| {
            t!("link_is_not_active_error", locale = locale)
        })));
    }
    let new_password_hash = hash_password(&reset_password.new_password);
    conn.transaction(|conn| {
        set_password(conn, password_reset.user_id, &new_password_hash)?;
        diesel::update(password_resets::table)
            .filter(password_resets::id.eq(password_reset.id))
            .set((
                password_resets::is_reset.eq(true),
                password_resets::is_valid.eq(false),
            ))
            .execute(conn)?;
        session_services::deactivate_all_user_sessions(conn, password_reset.user_id)
    })
    .to_service_result()?;
    Ok(())
}

pub fn create_password_reset(conn: &mut PgConnection, user: &User) -> QueryResult<PasswordReset> {
    conn.transaction(|conn| {
        diesel::update(password_resets::table)
            .filter(password_resets::user_id.eq(user.id))
            .filter(password_resets::is_valid.eq(true))
            .set(password_resets::is_valid.eq(false))
            .execute(conn)?;
        diesel::insert_into(password_resets::table)
            .values(password_resets::user_id.eq(user.id))
            .get_result::<PasswordReset>(conn)
    })
}

pub async fn send_password_reset_email(
    user: &User,
    password_reset: &PasswordReset,
) -> ServiceResult<()> {
    let domain = utils::get_env("DOMAIN");
    let key = utils::get_jwt_key();
    let mut claims = BTreeMap::new();
    claims.insert("password_reset_id", password_reset.id);
    let token = claims.sign_with_key(&key).unwrap();
    let body = format!(
        "<html>
            <head></head>
            <body>
                <p>To reset the password for the site {} follow the link: <a href=\"{}/password_reset/{}\">reset</a></p>
            </body>
        </html>",
        domain, domain, token
    );
    mailing_services::send_message(&user.email, "Password reset", &body).await
}

pub fn find_password_reset_by_id(conn: &mut PgConnection, id: i32) -> QueryResult<PasswordReset> {
    password_resets::table.find(id).first::<PasswordReset>(conn)
}

pub fn hash_password(password: &str) -> String {
    let salt = utils::get_env("PASSWORD_SALT");
    Argon2::default()
        .hash_password(password.as_bytes(), Salt::from_b64(&salt).unwrap())
        .unwrap()
        .to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    Argon2::default()
        .verify_password(password.as_bytes(), &PasswordHash::new(hash).unwrap())
        .is_ok()
}

fn set_password(
    conn: &mut PgConnection,
    user_id: i32,
    new_password_hash: &str,
) -> QueryResult<usize> {
    diesel::update(users::table)
        .filter(users::id.eq(user_id))
        .set(users::password.eq(new_password_hash))
        .execute(conn)
}
