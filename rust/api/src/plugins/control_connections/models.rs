use crate::schema::control_connections;
use diesel::{Identifiable, Queryable};

#[derive(Queryable, Identifiable)]
#[diesel(primary_key(connection_id), belongs_to(Connection))]
pub struct ControlConnection {
    pub connection_id: i32,
    pub is_control: bool,
}
