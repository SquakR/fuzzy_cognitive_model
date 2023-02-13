use crate::errors::AppError;
use crate::models::{EmailConfirmation, User};
use crate::schema::email_confirmations;
use crate::services::mailing_services;
use crate::services::users_services;
use crate::utils;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn create_email_confirmation(
    connection: &mut PgConnection,
    user: &User,
) -> Result<EmailConfirmation, AppError> {
    AppError::update_result(
        diesel::insert_into(email_confirmations::table)
            .values((
                email_confirmations::user_id.eq(user.id),
                email_confirmations::email.eq(&user.email),
            ))
            .get_result::<EmailConfirmation>(connection),
    )
}

pub async fn send_email_confirmation_email(
    email_confirmation: &EmailConfirmation,
) -> Result<(), AppError> {
    let domain = utils::get_env("DOMAIN");
    let key = get_key();
    let mut claims = BTreeMap::new();
    claims.insert("email_confirmation_id", email_confirmation.id);
    let token = claims.sign_with_key(&key).unwrap();
    let body = format!(
        "<html>
            <head></head>
            <body>
                <p>To confirm the email for the site {} follow the link: <a href=\"{}/email_confirmation/{}\">confirm</a></p>
            </body>
        </html>",
        domain, domain, token
    );
    mailing_services::send_message(&email_confirmation.email, "Email confirmation", &body).await
}

pub fn confirm_email(connection: &mut PgConnection, token: &str) -> Result<User, AppError> {
    let key = get_key();
    let claims: BTreeMap<String, i32> = match token.verify_with_key(&key) {
        Ok(claims) => claims,
        Err(_) => return Err(AppError::ValidationError(String::from("Invalid token."))),
    };
    let email_confirmation =
        find_email_confirmation_by_id(connection, claims["email_confirmation_id"])?;
    if email_confirmation.is_confirmed {
        return Err(AppError::ValidationError(String::from(
            "The link is not active.",
        )));
    }
    let user = users_services::find_user_by_id(connection, email_confirmation.user_id)?;
    if user.is_email_confirmed {
        return Err(AppError::ValidationError(String::from(
            "The link is not active.",
        )));
    }
    if user.email != email_confirmation.email {
        return Err(AppError::ValidationError(String::from(
            "The link is not active.",
        )));
    }
    confirm_email_confirmation(connection, email_confirmation)?;
    users_services::confirm_user_email(connection, user)
}

fn find_email_confirmation_by_id(
    connection: &mut PgConnection,
    id: i32,
) -> Result<EmailConfirmation, AppError> {
    AppError::update_result(
        email_confirmations::table
            .find(id)
            .first::<EmailConfirmation>(connection),
    )
}

fn confirm_email_confirmation(
    connection: &mut PgConnection,
    email_confirmation: EmailConfirmation,
) -> Result<EmailConfirmation, AppError> {
    AppError::update_result(
        diesel::update(email_confirmations::table)
            .filter(email_confirmations::id.eq(email_confirmation.id))
            .set(email_confirmations::is_confirmed.eq(true))
            .get_result::<EmailConfirmation>(connection),
    )
}

fn get_key() -> Hmac<Sha256> {
    let secret_key = utils::get_env("SECRET_KEY");
    Hmac::new_from_slice(secret_key.as_bytes()).unwrap()
}
