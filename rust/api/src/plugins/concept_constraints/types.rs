use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

/// Type of concept constraint to change
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConceptConstraintInChangeType {
    /// Whether concept has constraint
    pub has_constraint: bool,
    /// Minimum concept constraint value
    pub min_value: f64,
    /// Whether to include minimum value in constraint
    pub include_min_value: bool,
    /// Maximum concept constraint value
    pub max_value: f64,
    /// Whether to include maximum value in constraint
    pub include_max_value: bool,
}

/// Type of concept constraint
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConceptConstraintOutType {
    /// Concept identifier
    pub concept_id: i32,
    /// Whether concept has constraint
    pub has_constraint: bool,
    /// Minimum concept constraint value
    pub min_value: f64,
    /// Whether to include minimum value in constraint
    pub include_min_value: bool,
    /// Maximum concept constraint value
    pub max_value: f64,
    /// Whether to include maximum value in constraint
    pub include_max_value: bool,
    /// Concept update time
    pub updated_at: DateTime<Utc>,
}
