use crate::schema::connection_constraints;
use diesel::{Identifiable, Queryable};

#[derive(Queryable, Identifiable)]
#[diesel(primary_key(connection_id), belongs_to(Connection))]
pub struct ConnectionConstraint {
    pub connection_id: i32,
    pub has_constraint: bool,
    pub min_value: f64,
    pub include_min_value: bool,
    pub max_value: f64,
    pub include_max_value: bool,
}
