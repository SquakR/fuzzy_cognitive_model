use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::JsonSchema;

/// Type of connection constraint to change
#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionConstraintInChangeType {
    /// Whether connection has constraint
    pub has_constraint: bool,
    /// Minimum connection constraint value
    pub min_value: f64,
    /// Whether to include minimum value in constraint
    pub include_min_value: bool,
    /// Maximum connection constraint value
    pub max_value: f64,
    /// Whether to include maximum value in constraint
    pub include_max_value: bool,
}

/// Type of connection constraint
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionConstraintOutType {
    /// Connection identifier
    pub connection_id: i32,
    /// Whether connection has constraint
    pub has_constraint: bool,
    /// Minimum connection constraint value
    pub min_value: f64,
    /// Whether to include minimum value in constraint
    pub include_min_value: bool,
    /// Maximum Connection constraint value
    pub max_value: f64,
    /// Whether to include maximum value in constraint
    pub include_max_value: bool,
    /// Connection update time
    pub updated_at: DateTime<Utc>,
}
