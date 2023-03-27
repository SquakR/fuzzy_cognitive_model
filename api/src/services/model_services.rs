use crate::models::{Project, User, Vertex, VertexValueType};
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::vertices;
use crate::services::{permission_services, project_services};
use crate::types::{VertexInCreateType, VertexOutType};
use crate::web_socket::{WebSocketProjectService, WebSocketService};
use diesel::prelude::*;
use diesel::PgConnection;

pub async fn create_vertex(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    project_id: i32,
    vertex_in: VertexInCreateType,
) -> ServiceResult<VertexOutType> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    check_vertex_value(&project, vertex_in.value.clone())?;
    let vertex = diesel::insert_into(vertices::table)
        .values((
            vertices::project_id.eq(project_id),
            vertices::name.eq(vertex_in.name),
            vertices::description.eq(vertex_in.description),
            vertices::value.eq(vertex_in.value),
            vertices::x_position.eq(vertex_in.x_position),
            vertices::y_position.eq(vertex_in.y_position),
        ))
        .get_result::<Vertex>(conn)
        .to_service_result()?;
    let vertex_out = VertexOutType::from(vertex);
    project_service
        .notify(
            project_id,
            String::from("create_vertex"),
            vertex_out.clone(),
        )
        .await;
    Ok(vertex_out)
}

fn check_vertex_value(project: &Project, value: Option<f64>) -> ServiceResult<()> {
    match value {
        Some(value) => match project.vertex_value_type {
            VertexValueType::None => Err(AppError::ValidationError(Box::new(move |locale| {
                t!(
                    "invalid_vertex_value_error",
                    locale = locale,
                    expected = "null",
                    got = value
                )
            }))),
            VertexValueType::FromZeroToOne => {
                if value >= 0.0 && value <= 1.0 {
                    Ok(())
                } else {
                    Err(AppError::ValidationError(Box::new(move |locale| {
                        t!(
                            "invalid_vertex_value_error",
                            locale = locale,
                            expected = "[0.0; 1.0]",
                            got = value
                        )
                    })))
                }
            }
        },
        None => match project.vertex_value_type {
            VertexValueType::None => Ok(()),
            VertexValueType::FromZeroToOne => {
                Err(AppError::ValidationError(Box::new(move |locale| {
                    t!(
                        "invalid_vertex_value_error",
                        locale = locale,
                        expected = "[0.0; 1.0]",
                        got = "null"
                    )
                })))
            }
        },
    }
}

impl From<Vertex> for VertexOutType {
    fn from(vertex: Vertex) -> Self {
        VertexOutType {
            id: vertex.id,
            name: vertex.name,
            description: vertex.description,
            value: vertex.value,
            project_id: vertex.project_id,
            x_position: vertex.x_position,
            y_position: vertex.y_position,
            created_at: vertex.created_at,
            updated_at: vertex.updated_at,
        }
    }
}
