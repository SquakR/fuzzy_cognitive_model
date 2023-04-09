use rocket::serde::Serialize;
use rocket_okapi::JsonSchema;

/// Type of control vertex
#[derive(Clone, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ControlVertexOutType {
    /// Vertex identifier
    pub vertex_id: i32,
    /// Whether vertex is control
    pub is_control: bool,
}
