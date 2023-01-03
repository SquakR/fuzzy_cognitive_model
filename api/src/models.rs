use crate::schema::sessions;
use crate::schema::users;
use diesel::{Identifiable, Queryable};
use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars;
use rocket_okapi::JsonSchema;

/// Fuzzy cognitive model user (expert or researcher)
#[derive(Queryable, Identifiable, Serialize, JsonSchema)]
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
    /// User second name or patronymic
    pub second_name: Option<String>,
    /// User last name
    pub last_name: String,
    /// User avatar
    pub avatar: Option<String>,
}

/// User session
#[derive(Queryable, Identifiable)]
#[diesel(belongs_to(User))]
pub struct Session {
    /// Session identifier
    pub id: i32,
    /// Is session active
    pub is_active: bool,
    /// User identifier
    pub user_id: i32,
}
