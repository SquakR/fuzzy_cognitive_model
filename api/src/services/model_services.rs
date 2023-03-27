use crate::models::{Project, User, Vertex, VertexValueType};
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::vertices;
use crate::services::{permission_services, project_services};
use crate::types::{
    VertexInChangeDescriptionType, VertexInCreateType, VertexInMoveType,
    VertexOutChangeDescriptionType, VertexOutChangeValueType, VertexOutMoveType, VertexOutType,
};
use crate::validation_error;
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

pub async fn change_vertex_description(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    project_id: i32,
    vertex_id: i32,
    vertex_in: VertexInChangeDescriptionType,
) -> ServiceResult<VertexOutChangeDescriptionType> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let vertex = diesel::update(vertices::table)
        .filter(vertices::id.eq(vertex_id))
        .set((
            vertices::name.eq(vertex_in.name),
            vertices::description.eq(vertex_in.description),
        ))
        .get_result::<Vertex>(conn)
        .to_service_result()?;
    let vertex_out = VertexOutChangeDescriptionType::from(vertex);
    project_service
        .notify(
            project_id,
            String::from("change_vertex_description"),
            vertex_out.clone(),
        )
        .await;
    Ok(vertex_out)
}

pub async fn change_vertex_value(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    project_id: i32,
    vertex_id: i32,
    value: Option<f64>,
) -> ServiceResult<VertexOutChangeValueType> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    check_vertex_value(&project, value.clone())?;
    let vertex = diesel::update(vertices::table)
        .filter(vertices::id.eq(vertex_id))
        .set((vertices::value.eq(value),))
        .get_result::<Vertex>(conn)
        .to_service_result()?;
    let vertex_out = VertexOutChangeValueType::from(vertex);
    project_service
        .notify(
            project_id,
            String::from("change_vertex_value"),
            vertex_out.clone(),
        )
        .await;
    Ok(vertex_out)
}

pub async fn move_vertex(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    project_id: i32,
    vertex_id: i32,
    vertex_in: VertexInMoveType,
) -> ServiceResult<VertexOutMoveType> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let vertex = diesel::update(vertices::table)
        .filter(vertices::id.eq(vertex_id))
        .set((
            vertices::x_position.eq(vertex_in.x_position),
            vertices::y_position.eq(vertex_in.y_position),
        ))
        .get_result::<Vertex>(conn)
        .to_service_result()?;
    let vertex_out = VertexOutMoveType::from(vertex);
    project_service
        .notify(project_id, String::from("move_vertex"), vertex_out.clone())
        .await;
    Ok(vertex_out)
}

pub async fn delete_vertex(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    project_id: i32,
    vertex_id: i32,
) -> ServiceResult<()> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let deleted_number = diesel::delete(vertices::table.filter(vertices::id.eq(vertex_id)))
        .execute(conn)
        .to_service_result()?;
    if deleted_number == 0 {
        return validation_error!("vertex_not_found_error");
    }
    project_service
        .notify(project_id, String::from("delete_vertex"), vertex_id)
        .await;
    Ok(())
}

fn check_vertex_value(project: &Project, value: Option<f64>) -> ServiceResult<()> {
    match value {
        Some(value) => match project.vertex_value_type {
            VertexValueType::None => {
                validation_error!("invalid_vertex_value_error", expected = "null", got = value)
            }
            VertexValueType::FromZeroToOne => {
                if value >= 0.0 && value <= 1.0 {
                    Ok(())
                } else {
                    validation_error!(
                        "invalid_vertex_value_error",
                        expected = "[0.0; 1.0]",
                        got = value
                    )
                }
            }
        },
        None => match project.vertex_value_type {
            VertexValueType::None => Ok(()),
            VertexValueType::FromZeroToOne => {
                validation_error!(
                    "invalid_vertex_value_error",
                    expected = "[0.0; 1.0]",
                    got = "null"
                )
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

impl From<Vertex> for VertexOutChangeDescriptionType {
    fn from(vertex: Vertex) -> Self {
        VertexOutChangeDescriptionType {
            id: vertex.id,
            name: vertex.name,
            description: vertex.description,
            updated_at: vertex.updated_at,
        }
    }
}

impl From<Vertex> for VertexOutChangeValueType {
    fn from(vertex: Vertex) -> Self {
        VertexOutChangeValueType {
            id: vertex.id,
            value: vertex.value,
            updated_at: vertex.updated_at,
        }
    }
}

impl From<Vertex> for VertexOutMoveType {
    fn from(vertex: Vertex) -> Self {
        VertexOutMoveType {
            id: vertex.id,
            x_position: vertex.x_position,
            y_position: vertex.y_position,
            updated_at: vertex.updated_at,
        }
    }
}
