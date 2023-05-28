use crate::schema::adjustment_runs;
use crate::schema::{
    adjustment_chromosomes, adjustment_concept_values, adjustment_connection_values,
    adjustment_generations, concept_dynamic_models,
};
use chrono::{DateTime, Utc};
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

#[derive(Queryable, Identifiable)]
#[diesel(belongs_to(Project))]
#[diesel(belongs_to(ModelCopy))]
pub struct AdjustmentRun {
    pub id: i32,
    pub project_id: i32,
    pub model_copy_id: i32,
    pub name: String,
    pub description: String,
    pub max_model_time: i32,
    pub dynamic_model_type: DynamicModelType,
    pub generation_size: i32,
    pub generation_save_interval: i32,
    pub max_generations: i32,
    pub max_without_improvements: i32,
    pub error: f64,
    pub created_at: DateTime<Utc>,
    pub result_chromosome_id: Option<i32>,
}

#[derive(Queryable, Identifiable)]
#[diesel(belongs_to(AdjustmentRun))]
pub struct AdjustmentGeneration {
    pub id: i32,
    pub adjustment_run_id: i32,
    pub number: i32,
    pub fitness: f64,
}

#[derive(Queryable, Identifiable)]
#[diesel(belongs_to(AdjustmentGeneration))]
pub struct AdjustmentChromosome {
    pub id: i32,
    pub adjustment_generation_id: i32,
    pub number: i32,
    pub fitness: f64,
}

#[derive(Queryable, Identifiable)]
#[diesel(belongs_to(AdjustmentChromosome))]
#[diesel(belongs_to(Concept))]
pub struct AdjustmentConceptValue {
    pub id: i32,
    pub adjustment_chromosome_id: i32,
    pub concept_id: i32,
    pub value: f64,
}

#[derive(Queryable, Identifiable)]
#[diesel(belongs_to(AdjustmentChromosome))]
#[diesel(belongs_to(Connection))]
pub struct AdjustmentConnectionValue {
    pub id: i32,
    pub adjustment_chromosome_id: i32,
    pub connection_id: i32,
    pub value: f64,
}
