use crate::schema::sessions;
use crate::schema::users;
use chrono::{DateTime, Utc};
use diesel::{Identifiable, Queryable};
use ipnetwork::IpNetwork;

/// Fuzzy cognitive model user (expert or researcher)
#[derive(Queryable, Identifiable)]
pub struct User {
    /// User identifier
    pub id: i32,
    /// User nickname
    pub username: String,
    /// Hashed password
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
