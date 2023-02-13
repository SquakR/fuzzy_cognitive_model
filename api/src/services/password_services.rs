use crate::errors::AppError;
use crate::models::PasswordReset;
use crate::models::User;
use crate::schema::password_resets;
use crate::schema::users;
use crate::services::mailing_services;
use crate::services::session_services;
use crate::services::users_services;
use crate::types::{ChangePasswordType, ResetPasswordType};
use crate::utils;
use argon2::{password_hash::PasswordHash, Argon2, PasswordHasher, PasswordVerifier};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use jwt::{SignWithKey, VerifyWithKey};
use std::collections::BTreeMap;

pub fn change_user_password(
    connection: &mut PgConnection,
    user: &User,
    change_password: ChangePasswordType,
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
    set_password(connection, user.id, &new_password_hash)?;
    Ok(())
}

pub async fn request_password_reset(
    connection: &mut PgConnection,
    email: &str,
) -> Result<PasswordReset, AppError> {
    let user = users_services::find_user_by_email(connection, email)?;
    if !user.is_email_confirmed {
        return Err(AppError::ValidationError(String::from(
            "Resetting the password is not possible because email not confirmed.",
        )));
    }
    let password_reset = create_password_reset(connection, &user)?;
    send_password_reset_email(&user, &password_reset).await?;
    Ok(password_reset)
}

pub fn reset_password(
    connection: &mut PgConnection,
    reset_password: ResetPasswordType,
) -> Result<(), AppError> {
    let key = utils::get_jwt_key();
    let claims: BTreeMap<String, i32> = match reset_password.token.verify_with_key(&key) {
        Ok(claims) => claims,
        Err(_) => return Err(AppError::ValidationError(String::from("Invalid token."))),
    };
    let password_reset = find_password_reset_by_id(connection, claims["password_reset_id"])?;
    if !password_reset.is_valid {
        return Err(AppError::ValidationError(String::from(
            "The link is not active.",
        )));
    }
    let new_password_hash = hash_password(&reset_password.new_password);
    set_password(connection, password_reset.user_id, &new_password_hash)?;
    AppError::update_result(
        diesel::update(password_resets::table)
            .filter(password_resets::id.eq(password_reset.id))
            .set((
                password_resets::is_reset.eq(true),
                password_resets::is_valid.eq(false),
            ))
            .execute(connection),
    )?;
    session_services::deactivate_all_user_sessions(connection, password_reset.user_id)?;
    Ok(())
}

pub fn create_password_reset(
    connection: &mut PgConnection,
    user: &User,
) -> Result<PasswordReset, AppError> {
    AppError::update_result(
        diesel::update(password_resets::table)
            .filter(password_resets::user_id.eq(user.id))
            .filter(password_resets::is_valid.eq(true))
            .set(password_resets::is_valid.eq(false))
            .execute(connection),
    )?;
    AppError::update_result(
        diesel::insert_into(password_resets::table)
            .values(password_resets::user_id.eq(user.id))
            .get_result::<PasswordReset>(connection),
    )
}

pub async fn send_password_reset_email(
    user: &User,
    password_reset: &PasswordReset,
) -> Result<(), AppError> {
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

pub fn find_password_reset_by_id(
    connection: &mut PgConnection,
    id: i32,
) -> Result<PasswordReset, AppError> {
    AppError::update_result(
        password_resets::table
            .find(id)
            .first::<PasswordReset>(connection),
    )
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

fn set_password(
    connection: &mut PgConnection,
    user_id: i32,
    new_password_hash: &str,
) -> Result<usize, AppError> {
    AppError::update_result(
        diesel::update(users::table)
            .filter(users::id.eq(user_id))
            .set(users::password.eq(new_password_hash))
            .execute(connection),
    )
}