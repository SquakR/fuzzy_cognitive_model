use crate::schema::sessions;
use crate::schema::users;
use chrono::{DateTime, Utc};
use diesel::{Identifiable, Queryable};
use ipnetwork::IpNetwork;
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
    /// User creation time
    pub created_at: DateTime<Utc>,
    /// User update time
    pub updated_at: DateTime<Utc>,
}

/// User session
#[derive(Queryable, Identifiable)]
#[diesel(belongs_to(User))]
pub struct Session {
    pub id: i32,
    pub is_active: bool,
    pub user_id: i32,
    pub ip_address: IpNetwork,
    pub user_agent: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
