use crate::schema::control_vertices;
use diesel::{Identifiable, Queryable};

#[derive(Queryable, Identifiable)]
#[diesel(table_name = control_vertices, primary_key(vertex_id), belongs_to(Vertex))]
pub struct ControlVertex {
    pub vertex_id: i32,
    pub is_control: bool,
}
