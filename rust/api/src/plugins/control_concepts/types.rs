use chrono::{DateTime, Utc};
use rocket::serde::Serialize;
use rocket_okapi::JsonSchema;

/// Type of control concept
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ControlConceptOutType {
    /// Concept identifier
    pub concept_id: i32,
    /// Whether concept is control
    pub is_control: bool,
    /// Concept update time
    pub updated_at: DateTime<Utc>,
}
