use crate::schema::concept_constraints;
use diesel::{Identifiable, Queryable};

#[derive(Queryable, Identifiable)]
#[diesel(primary_key(concept_id), belongs_to(Concept))]
pub struct ConceptConstraint {
    pub concept_id: i32,
    pub has_constraint: bool,
    pub min_value: f64,
    pub include_min_value: bool,
    pub max_value: f64,
    pub include_max_value: bool,
}
