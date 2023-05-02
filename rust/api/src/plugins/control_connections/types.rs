use chrono::{DateTime, Utc};
use rocket::serde::Serialize;
use rocket_okapi::JsonSchema;

/// Type of control connection
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ControlConnectionOutType {
    /// Connection identifier
    pub connection_id: i32,
    /// Whether connection is control
    pub is_control: bool,
    /// Connection update time
    pub updated_at: DateTime<Utc>,
}
