use chrono::{DateTime, Utc};
use rocket::serde::Serialize;
use rocket_okapi::JsonSchema;
use serde::Deserialize;

/// Type of target concept to change
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TargetConceptInChangeType {
    /// Whether concept is target
    pub is_target: bool,
    /// Target concept desired value
    pub value: Option<f64>,
}

/// Type of target concept
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TargetConceptOutType {
    /// Concept identifier
    pub concept_id: i32,
    /// Whether concept is target
    pub is_target: bool,
    /// Target concept desired value
    pub value: Option<f64>,
    /// Vertex update time
    pub updated_at: DateTime<Utc>,
}
