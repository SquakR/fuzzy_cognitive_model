use super::models::DynamicModelType;
use chrono::{DateTime, Utc};
use rocket::serde::Serialize;
use rocket_okapi::JsonSchema;

/// Type of concept dynamics model
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConceptDynamicModelOutType {
    /// Concept identifier
    pub concept_id: i32,
    /// Concept dynamics model type
    pub dynamic_model_type: Option<DynamicModelType>,
    /// Concept update time
    pub updated_at: DateTime<Utc>,
}
