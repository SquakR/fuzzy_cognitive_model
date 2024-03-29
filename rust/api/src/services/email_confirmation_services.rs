use crate::models::{EmailConfirmation, User};
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::email_confirmations;
use crate::services::{mailing_services, user_services};
use crate::utils;
use crate::validation_error;
use diesel::prelude::*;
use diesel::PgConnection;
use jwt::{SignWithKey, VerifyWithKey};
use std::collections::BTreeMap;

pub fn create_email_confirmation(
    conn: &mut PgConnection,
    user: &User,
) -> QueryResult<EmailConfirmation> {
    diesel::insert_into(email_confirmations::table)
        .values((
            email_confirmations::user_id.eq(user.id),
            email_confirmations::email.eq(&user.email),
        ))
        .get_result::<EmailConfirmation>(conn)
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

pub fn confirm_email(conn: &mut PgConnection, token: &str) -> ServiceResult<User> {
    let key = utils::get_jwt_key();
    let claims: BTreeMap<String, i32> = match token.verify_with_key(&key) {
        Ok(claims) => claims,
        Err(_) => {
            return validation_error!("invalid_token_error");
        }
    };
    let email_confirmation =
        find_email_confirmation_by_id(conn, claims["email_confirmation_id"])
            .to_service_result_find(String::from("email_confirmation_not_found_error"))?;
    if email_confirmation.is_confirmed {
        return validation_error!("link_is_not_active_error");
    }
    let user = user_services::find_user_by_id(conn, email_confirmation.user_id)
        .to_service_result_find(String::from("user_not_found_error"))?;
    if user.is_email_confirmed || user.email != email_confirmation.email {
        return validation_error!("link_is_not_active_error");
    }
    conn.transaction(|conn| {
        confirm_email_confirmation(conn, email_confirmation)?;
        user_services::confirm_user_email(conn, user)
    })
    .to_service_result()
}

fn find_email_confirmation_by_id(
    conn: &mut PgConnection,
    id: i32,
) -> QueryResult<EmailConfirmation> {
    email_confirmations::table
        .find(id)
        .first::<EmailConfirmation>(conn)
}

fn confirm_email_confirmation(
    conn: &mut PgConnection,
    email_confirmation: EmailConfirmation,
) -> QueryResult<EmailConfirmation> {
    diesel::update(email_confirmations::table)
        .filter(email_confirmations::id.eq(email_confirmation.id))
        .set(email_confirmations::is_confirmed.eq(true))
        .get_result::<EmailConfirmation>(conn)
}
