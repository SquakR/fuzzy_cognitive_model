use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

/// Type of target concept to change
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TargetConceptInChangeType {
    /// Whether concept is target
    pub is_target: bool,
    /// Minimum target concept desired value
    pub min_value: f64,
    /// Whether to include minimum value in desired value
    pub include_min_value: bool,
    /// Maximum target concept desired value
    pub max_value: f64,
    /// Whether to include maximum value in desired value
    pub include_max_value: bool,
}

/// Type of target concept
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TargetConceptOutType {
    /// Concept identifier
    pub concept_id: i32,
    /// Whether concept is target
    pub is_target: bool,
    /// Minimum target concept desired value
    pub min_value: f64,
    /// Whether to include minimum value in desired value
    pub include_min_value: bool,
    /// Maximum target concept desired value
    pub max_value: f64,
    /// Whether to include maximum value in desired value
    pub include_max_value: bool,
    /// Concept update time
    pub updated_at: DateTime<Utc>,
}
