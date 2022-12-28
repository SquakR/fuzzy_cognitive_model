use rocket::serde::Deserialize;
use rocket_okapi::JsonSchema;

/// Fuzzy cognitive model user (expert or researcher)
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserIn {
    /// User nickname
    pub username: String,
    /// User password
    pub password: String,
    /// User email
    pub email: String,
    /// User name
    pub first_name: String,
    /// User second name or patronymic
    pub second_name: Option<String>,
    /// User last name
    pub last_name: String,
}

/// User credentials
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    /// User nickname
    pub username: String,
    /// User password
    pub password: String,
}
