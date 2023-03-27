use crate::models::{Arc, Project, User, Vertex, VertexValueType};
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::{arcs, projects, vertices};
use crate::services::{permission_services, project_services};
use crate::types::{
    ArcOutType, ModelActionType, ModelOutType, ProjectOutType, VertexInChangeDescriptionType,
    VertexInCreateType, VertexInMoveType, VertexOutChangeDescriptionType, VertexOutChangeValueType,
    VertexOutMoveType, VertexOutType,
};
use crate::validation_error;
use crate::web_socket::WebSocketProjectService;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use schemars::JsonSchema;
use serde::Serialize;

pub fn get_model(
    conn: &mut PgConnection,
    user: &User,
    project_id: i32,
) -> ServiceResult<ModelOutType> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_view_project(conn, &project, user)?;
    let project = ProjectOutType::from_project(conn, project)?;
    let vertices = find_project_vertices(conn, project_id)
        .to_service_result()?
        .into_iter()
        .map(VertexOutType::from)
        .collect();
    let arcs = find_project_arcs(conn, project_id)
        .to_service_result()?
        .into_iter()
        .map(ArcOutType::from)
        .collect();
    Ok(ModelOutType {
        project,
        vertices,
        arcs,
    })
}

pub fn find_project_vertices(conn: &mut PgConnection, project_id: i32) -> QueryResult<Vec<Vertex>> {
    projects::table
        .inner_join(vertices::table)
        .select(vertices::all_columns)
        .filter(projects::id.eq(project_id))
        .get_results::<Vertex>(conn)
}

pub fn find_project_arcs(conn: &mut PgConnection, project_id: i32) -> QueryResult<Vec<Arc>> {
    projects::table
        .inner_join(arcs::table)
        .select(arcs::all_columns)
        .filter(projects::id.eq(project_id))
        .get_results::<Arc>(conn)
}

pub async fn create_vertex(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    project_id: i32,
    vertex_in: VertexInCreateType,
) -> ServiceResult<ModelActionType<VertexOutType>> {
    let mut project = project_services::find_project_by_id(conn, project_id)
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
    project = project_services::update_project(conn, project_id, vertex.created_at)
        .to_service_result()?;
    let vertex_out = VertexOutType::from(vertex);
    let model_action = ModelActionType::new(&project, String::from("create_vertex"), vertex_out);
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub async fn change_vertex_description(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    project_id: i32,
    vertex_id: i32,
    vertex_in: VertexInChangeDescriptionType,
) -> ServiceResult<ModelActionType<VertexOutChangeDescriptionType>> {
    let mut project = project_services::find_project_by_id(conn, project_id)
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
    project = project_services::update_project(conn, project_id, vertex.updated_at)
        .to_service_result()?;
    let vertex_out = VertexOutChangeDescriptionType::from(vertex);
    let model_action = ModelActionType::new(
        &project,
        String::from("change_vertex_description"),
        vertex_out,
    );
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub async fn change_vertex_value(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    project_id: i32,
    vertex_id: i32,
    value: Option<f64>,
) -> ServiceResult<ModelActionType<VertexOutChangeValueType>> {
    let mut project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    check_vertex_value(&project, value.clone())?;
    let vertex = diesel::update(vertices::table)
        .filter(vertices::id.eq(vertex_id))
        .set((vertices::value.eq(value),))
        .get_result::<Vertex>(conn)
        .to_service_result()?;
    project = project_services::update_project(conn, project_id, vertex.updated_at)
        .to_service_result()?;
    let vertex_out = VertexOutChangeValueType::from(vertex);
    let model_action =
        ModelActionType::new(&project, String::from("change_vertex_value"), vertex_out);
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub async fn move_vertex(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    project_id: i32,
    vertex_id: i32,
    vertex_in: VertexInMoveType,
) -> ServiceResult<ModelActionType<VertexOutMoveType>> {
    let mut project = project_services::find_project_by_id(conn, project_id)
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
    project = project_services::update_project(conn, project_id, vertex.updated_at)
        .to_service_result()?;
    let vertex_out = VertexOutMoveType::from(vertex);
    let model_action = ModelActionType::new(&project, String::from("move_vertex"), vertex_out);
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub async fn delete_vertex(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    project_id: i32,
    vertex_id: i32,
) -> ServiceResult<ModelActionType<()>> {
    let mut project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let deleted_number = diesel::delete(vertices::table.filter(vertices::id.eq(vertex_id)))
        .execute(conn)
        .to_service_result()?;
    if deleted_number == 0 {
        return validation_error!("vertex_not_found_error");
    }
    project = project_services::update_project(conn, project_id, Utc::now()).to_service_result()?;
    let model_action = ModelActionType::new(&project, String::from("delete_vertex"), ());
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
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

impl<T> ModelActionType<T>
where
    T: Clone + Serialize + JsonSchema,
{
    fn new(project: &Project, name: String, data: T) -> Self {
        Self {
            project_id: project.id,
            project_updated_at: project.updated_at,
            name,
            data,
        }
    }
}

impl From<Vertex> for VertexOutType {
    fn from(vertex: Vertex) -> Self {
        Self {
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
        Self {
            id: vertex.id,
            name: vertex.name,
            description: vertex.description,
            updated_at: vertex.updated_at,
        }
    }
}

impl From<Vertex> for VertexOutChangeValueType {
    fn from(vertex: Vertex) -> Self {
        Self {
            id: vertex.id,
            value: vertex.value,
            updated_at: vertex.updated_at,
        }
    }
}

impl From<Vertex> for VertexOutMoveType {
    fn from(vertex: Vertex) -> Self {
        Self {
            id: vertex.id,
            x_position: vertex.x_position,
            y_position: vertex.y_position,
            updated_at: vertex.updated_at,
        }
    }
}

impl From<Arc> for ArcOutType {
    fn from(arc: Arc) -> Self {
        Self {
            id: arc.id,
            description: arc.description,
            value: arc.value,
            source_id: arc.source_id,
            target_id: arc.target_id,
            project_id: arc.project_id,
            created_at: arc.created_at,
            updated_at: arc.updated_at,
        }
    }
}
