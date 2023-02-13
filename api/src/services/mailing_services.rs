use crate::{errors::AppError, utils};
use lettre::message::header;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

pub async fn send_message(to_email: &str, subject: &str, body: &str) -> Result<(), AppError> {
    let smtp_host = utils::get_env("SMTP_HOST");
    let smtp_login = utils::get_env("SMTP_LOGIN");
    let smtp_password = utils::get_env("SMTP_PASSWORD");
    let smtp_name = utils::get_env("SMTP_NAME");

    let smtp_credentials = Credentials::new(smtp_login.clone(), smtp_password);
    let mailer_builder = match AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_host.as_str()) {
        Ok(builder) => builder,
        Err(_) => return Err(AppError::InternalServerError),
    };
    let mailer = mailer_builder.credentials(smtp_credentials).build();
    let send_result = send_email_smtp(
        &mailer,
        &format!("{} <{}>", smtp_name, smtp_login),
        to_email,
        subject,
        body,
    )
    .await;
    if send_result.is_err() {
        return Err(AppError::InternalServerError);
    }
    Ok(())
}

async fn send_email_smtp(
    mailer: &AsyncSmtpTransport<Tokio1Executor>,
    from: &str,
    to: &str,
    subject: &str,
    body: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .header(header::ContentType::TEXT_HTML)
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    mailer.send(email).await?;

    Ok(())
}
