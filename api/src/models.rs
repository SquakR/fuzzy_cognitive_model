use crate::schema::{
    concepts, connections, email_confirmations, password_resets, permissions, plugins,
    project_plugins, project_user_permissions, project_user_statuses, project_users, projects,
    sessions, users,
};
use chrono::{DateTime, Utc};
use diesel::{Associations, Identifiable, Queryable};
use ipnetwork::IpNetwork;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

#[derive(Queryable, Identifiable, Clone)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub is_archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub concept_value_type: ConceptValueType,
    pub connection_value_type: ConnectionValueType,
}

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Project))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(ProjectUserStatus, foreign_key = last_status_id))]
pub struct ProjectUser {
    pub id: i32,
    pub project_id: i32,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub last_status_id: i32,
}

#[derive(Debug, PartialEq, diesel_derive_enum::DbEnum, Serialize, JsonSchema, FromFormField)]
#[serde(rename_all = "snake_case")]
#[ExistingTypePath = "crate::schema::sql_types::ProjectUserStatusValue"]
pub enum ProjectUserStatusValue {
    Creator,
    Invited,
    Cancelled,
    Rejected,
    Member,
    Excluded,
    Left,
}

#[derive(Queryable, Identifiable)]
#[diesel(table_name = project_user_statuses, belongs_to(ProjectUser))]
pub struct ProjectUserStatus {
    pub id: i32,
    pub project_user_id: i32,
    pub status: ProjectUserStatusValue,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable, Identifiable)]
#[diesel(primary_key(key))]
pub struct Permission {
    pub key: String,
    pub description: String,
}

#[derive(Queryable, Identifiable)]
pub struct ProjectUserPermission {
    pub id: i32,
    pub permission_key: String,
    pub project_user_id: i32,
}

#[derive(Queryable, Identifiable)]
#[diesel(primary_key(name))]
pub struct Plugin {
    pub name: String,
    pub description: String,
    pub concept_value_type: Option<ConceptValueType>,
    pub connection_value_type: Option<ConnectionValueType>,
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

#[derive(
    Debug, PartialEq, diesel_derive_enum::DbEnum, Clone, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
#[ExistingTypePath = "crate::schema::sql_types::ConceptValueType"]
pub enum ConceptValueType {
    None,
    FromZeroToOne,
}

#[derive(Queryable, Identifiable)]
#[diesel(table_name = concepts, belongs_to(Project))]
pub struct Concept {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub value: Option<f64>,
    pub project_id: i32,
    pub x_position: f64,
    pub y_position: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(
    Debug, PartialEq, diesel_derive_enum::DbEnum, Clone, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
#[ExistingTypePath = "crate::schema::sql_types::ConnectionValueType"]
pub enum ConnectionValueType {
    Symbolic,
    FromMinusOneToOne,
}

#[derive(Queryable, Identifiable)]
#[diesel(belongs_to(Project))]
#[diesel(belongs_to(Concept, foreign_key = source_id))]
#[diesel(belongs_to(Concept, foreign_key = target_id))]
pub struct Connection {
    pub id: i32,
    pub description: String,
    pub value: f64,
    pub source_id: i32,
    pub target_id: i32,
    pub project_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
