use crate::schema::control_concepts;
use diesel::{Identifiable, Queryable};

#[derive(Queryable, Identifiable)]
#[diesel(table_name = control_concepts, primary_key(concept_id), belongs_to(Concept))]
pub struct ControlConcept {
    pub concept_id: i32,
    pub is_control: bool,
}
