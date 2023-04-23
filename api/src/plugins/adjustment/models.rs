use crate::schema::concept_dynamic_models;
use diesel::{Identifiable, Queryable};
use rocket::serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(
    Debug,
    Clone,
    PartialEq,
    diesel_derive_enum::DbEnum,
    Serialize,
    Deserialize,
    JsonSchema,
    FromFormField,
)]
#[serde(rename_all = "snake_case")]
#[ExistingTypePath = "crate::schema::sql_types::DynamicModelType"]
pub enum DynamicModelType {
    DeltaDelta,
    DeltaValue,
    ValueDelta,
    ValueValue,
}

#[derive(Queryable, Identifiable)]
#[diesel(primary_key(concept_id), belongs_to(Concept))]
pub struct ConceptDynamicModel {
    pub concept_id: i32,
    pub dynamic_model_type: Option<DynamicModelType>,
}
