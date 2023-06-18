use crate::schema::target_concepts;
use diesel::{Identifiable, Queryable};

#[derive(Queryable, Identifiable)]
#[diesel(primary_key(concept_id), belongs_to(Concept))]
pub struct TargetConcept {
    pub concept_id: i32,
    pub is_target: bool,
    pub min_value: f64,
    pub include_min_value: bool,
    pub max_value: f64,
    pub include_max_value: bool,
}
