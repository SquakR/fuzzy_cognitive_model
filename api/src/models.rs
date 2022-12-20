use crate::schema::users;
use diesel::{Identifiable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars;
use rocket_okapi::JsonSchema;

/// Fuzzy cognitive model user (expert or researcher)
#[derive(Queryable, Identifiable, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// User identifier in the database
    pub id: i32,
    /// User nickname
    pub username: String,
    /// Hashed password
    #[serde(skip_serializing)]
    pub password: String,
    /// User email
    pub email: String,
    /// User name
    pub first_name: String,
    /// User last name
    pub last_name: String,
    /// User patronymic
    pub patronymic: Option<String>,
}
