use crate::models::{EmailConfirmation, User};
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::email_confirmations;
use crate::services::mailing_services;
use crate::services::users_services;
use crate::utils;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use jwt::{SignWithKey, VerifyWithKey};
use rust_i18n::t;
use std::collections::BTreeMap;

pub fn create_email_confirmation(
    connection: &mut PgConnection,
    user: &User,
) -> ServiceResult<EmailConfirmation> {
    diesel::insert_into(email_confirmations::table)
        .values((
            email_confirmations::user_id.eq(user.id),
            email_confirmations::email.eq(&user.email),
        ))
        .get_result::<EmailConfirmation>(connection)
        .to_service_result()
}

pub async fn send_email_confirmation_email(
    email_confirmation: &EmailConfirmation,
) -> ServiceResult<()> {
    let domain = utils::get_env("DOMAIN");
    let key = utils::get_jwt_key();
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

pub fn confirm_email(connection: &mut PgConnection, token: &str) -> ServiceResult<User> {
    let key = utils::get_jwt_key();
    let claims: BTreeMap<String, i32> = match token.verify_with_key(&key) {
        Ok(claims) => claims,
        Err(_) => {
            return Err(AppError::ValidationError(Box::new(|locale| {
                t!("invalid_token_error", locale = locale)
            })));
        }
    };
    let email_confirmation =
        find_email_confirmation_by_id(connection, claims["email_confirmation_id"])?;
    if email_confirmation.is_confirmed {
        return Err(AppError::ValidationError(Box::new(|locale| {
            t!("link_is_not_active_error", locale = locale)
        })));
    }
    let user = users_services::find_user_by_id(connection, email_confirmation.user_id)?;
    if user.is_email_confirmed || user.email != email_confirmation.email {
        return Err(AppError::ValidationError(Box::new(|locale| {
            t!("link_is_not_active_error", locale = locale)
        })));
    }
    confirm_email_confirmation(connection, email_confirmation)?;
    users_services::confirm_user_email(connection, user)
}

fn find_email_confirmation_by_id(
    connection: &mut PgConnection,
    id: i32,
) -> ServiceResult<EmailConfirmation> {
    email_confirmations::table
        .find(id)
        .first::<EmailConfirmation>(connection)
        .to_service_result_find(String::from("email_confirmation_not_found_error"))
}

fn confirm_email_confirmation(
    connection: &mut PgConnection,
    email_confirmation: EmailConfirmation,
) -> Result<EmailConfirmation, AppError> {
    diesel::update(email_confirmations::table)
        .filter(email_confirmations::id.eq(email_confirmation.id))
        .set(email_confirmations::is_confirmed.eq(true))
        .get_result::<EmailConfirmation>(connection)
        .to_service_result()
}
