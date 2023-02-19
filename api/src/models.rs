use crate::schema::email_confirmations;
use crate::schema::password_resets;
use crate::schema::permissions;
use crate::schema::plugins;
use crate::schema::project_permissions;
use crate::schema::project_plugins;
use crate::schema::projects;
use crate::schema::sessions;
use crate::schema::user_projects;
use crate::schema::users;
use chrono::{DateTime, Utc};
use diesel::{Identifiable, Queryable};
use ipnetwork::IpNetwork;

#[derive(Queryable, Identifiable, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub is_email_confirmed: bool,
    pub first_name: String,
    pub second_name: Option<String>,
    pub last_name: String,
    pub avatar: Option<String>,
    pub language: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Identifiable)]
#[diesel(belongs_to(User))]
pub struct EmailConfirmation {
    pub id: i32,
    pub user_id: i32,
    pub email: String,
    pub is_confirmed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Identifiable)]
#[diesel(belongs_to(User))]
pub struct PasswordReset {
    pub id: i32,
    pub user_id: i32,
    pub is_reset: bool,
    pub is_valid: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

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

#[derive(Queryable, Identifiable)]
#[diesel(belongs_to(User))]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_by_id: i32,
    pub is_public: bool,
    pub is_archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Queryable, Identifiable)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Project))]
pub struct UserProject {
    pub id: i32,
    pub user_id: i32,
    pub project_id: i32,
    pub is_confirmed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable, Identifiable)]
#[diesel(primary_key(key))]
pub struct Permission {
    pub key: String,
    pub name: String,
    pub description: String,
}

#[derive(Queryable, Identifiable)]
pub struct ProjectPermission {
    pub id: i32,
    pub permission_key: String,
    pub user_project_id: i32,
}

#[derive(Queryable, Identifiable)]
#[diesel(primary_key(name))]
pub struct Plugin {
    pub name: String,
    pub description: String,
}

#[derive(Queryable, Identifiable)]
#[diesel(belongs_to(Project))]
#[diesel(belongs_to(Plugin))]
pub struct ProjectPlugin {
    pub id: i32,
    pub project_id: i32,
    pub plugin_name: String,
    pub created_at: DateTime<Utc>,
}
