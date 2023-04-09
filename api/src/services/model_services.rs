use crate::models::{Arc, ArcValueType, Project, User, Vertex, VertexValueType};
use crate::plugins::Plugins;
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::{arcs, projects, vertices};
use crate::services::{permission_services, project_services};
use crate::types::{
    ArcInCreateType, ArcOutChangeDescriptionType, ArcOutChangeValueType, ArcOutType,
    ModelActionType, ModelOutType, ProjectOutType, VertexInChangeDescriptionType,
    VertexInCreateType, VertexInMoveType, VertexOutChangeDescriptionType, VertexOutChangeValueType,
    VertexOutMoveType, VertexOutType,
};
use crate::validation_error;
use crate::web_socket::WebSocketProjectService;
use chrono::DateTime;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use schemars::JsonSchema;
use serde::Serialize;
use serde_json::{Map, Value};

pub fn get_model(
    conn: &mut PgConnection,
    plugins: &Plugins,
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
    let mut model_out = ModelOutType {
        project,
        vertices,
        arcs,
    };
    model_out = plugins.get_model_emitter.lock().unwrap().emit(model_out)?;
    Ok(model_out)
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

pub fn find_vertex_by_id(conn: &mut PgConnection, vertex_id: i32) -> QueryResult<Vertex> {
    vertices::table
        .filter(vertices::id.eq(vertex_id))
        .get_result::<Vertex>(conn)
}

pub fn find_project_by_vertex_id(conn: &mut PgConnection, vertex_id: i32) -> QueryResult<Project> {
    vertices::table
        .inner_join(projects::table)
        .select(projects::all_columns)
        .filter(vertices::id.eq(vertex_id))
        .get_result::<Project>(conn)
}

pub fn find_project_by_arc_id(conn: &mut PgConnection, arc_id: i32) -> QueryResult<Project> {
    arcs::table
        .inner_join(projects::table)
        .select(projects::all_columns)
        .filter(arcs::id.eq(arc_id))
        .get_result::<Project>(conn)
}

pub async fn create_vertex(
    conn: &mut PgConnection,
    plugins: &Plugins,
    project_service: WebSocketProjectService,
    user: &User,
    project_id: i32,
    vertex_in: VertexInCreateType,
) -> ServiceResult<ModelActionType<VertexOutType>> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    check_vertex_value(&project, vertex_in.value.clone())?;
    let (vertex, project) = conn
        .transaction(|conn| {
            let vertex = diesel::insert_into(vertices::table)
                .values((
                    vertices::project_id.eq(project_id),
                    vertices::name.eq(vertex_in.name),
                    vertices::description.eq(vertex_in.description),
                    vertices::value.eq(vertex_in.value),
                    vertices::x_position.eq(vertex_in.x_position),
                    vertices::y_position.eq(vertex_in.y_position),
                ))
                .get_result::<Vertex>(conn)?;
            let project = project_services::update_project(conn, project_id, vertex.created_at)?;
            Ok((vertex, project))
        })
        .to_service_result()?;
    let vertex_out = plugins
        .add_vertex_emitter
        .lock()
        .unwrap()
        .emit(project.clone(), VertexOutType::from(vertex))?;
    let model_action = ModelActionType::new(&project, String::from("create_vertex"), vertex_out);
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub fn update_vertex(
    conn: &mut PgConnection,
    vertex_id: i32,
    project_id: i32,
    updated_at: DateTime<Utc>,
) -> QueryResult<(Vertex, Project)> {
    conn.transaction(|conn| {
        let vertex = diesel::update(vertices::table)
            .filter(vertices::id.eq(vertex_id))
            .set(vertices::updated_at.eq(updated_at))
            .get_result::<Vertex>(conn)?;
        let project = project_services::update_project(conn, project_id, updated_at)?;
        Ok((vertex, project))
    })
}

pub async fn change_vertex_description(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    vertex_id: i32,
    vertex_in: VertexInChangeDescriptionType,
) -> ServiceResult<ModelActionType<VertexOutChangeDescriptionType>> {
    let project = find_project_by_vertex_id(conn, vertex_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let (vertex, project) = conn
        .transaction(|conn| {
            let vertex = diesel::update(vertices::table)
                .filter(vertices::id.eq(vertex_id))
                .set((
                    vertices::name.eq(vertex_in.name),
                    vertices::description.eq(vertex_in.description),
                ))
                .get_result::<Vertex>(conn)?;
            let project = project_services::update_project(conn, project.id, vertex.updated_at)?;
            Ok((vertex, project))
        })
        .to_service_result_find(String::from("vertex_not_found_error"))?;
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
    vertex_id: i32,
    value: Option<f64>,
) -> ServiceResult<ModelActionType<VertexOutChangeValueType>> {
    let project = find_project_by_vertex_id(conn, vertex_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    check_vertex_value(&project, value.clone())?;
    let (vertex, project) = conn
        .transaction(|conn| {
            let vertex = diesel::update(vertices::table)
                .filter(vertices::id.eq(vertex_id))
                .set((vertices::value.eq(value),))
                .get_result::<Vertex>(conn)?;
            let project = project_services::update_project(conn, project.id, vertex.updated_at)?;
            Ok((vertex, project))
        })
        .to_service_result_find(String::from("vertex_not_found_error"))?;
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
    vertex_id: i32,
    vertex_in: VertexInMoveType,
) -> ServiceResult<ModelActionType<VertexOutMoveType>> {
    let project = find_project_by_vertex_id(conn, vertex_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let (vertex, project) = conn
        .transaction(|conn| {
            let vertex = diesel::update(vertices::table)
                .filter(vertices::id.eq(vertex_id))
                .set((
                    vertices::x_position.eq(vertex_in.x_position),
                    vertices::y_position.eq(vertex_in.y_position),
                ))
                .get_result::<Vertex>(conn)?;
            let project = project_services::update_project(conn, project.id, vertex.updated_at)?;
            Ok((vertex, project))
        })
        .to_service_result_find(String::from("vertex_not_found_error"))?;
    let vertex_out = VertexOutMoveType::from(vertex);
    let model_action = ModelActionType::new(&project, String::from("move_vertex"), vertex_out);
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub async fn delete_vertex(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    vertex_id: i32,
) -> ServiceResult<ModelActionType<i32>> {
    let project = find_project_by_vertex_id(conn, vertex_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let (deleted_number, project) = conn
        .transaction(|conn| {
            let deleted_number =
                diesel::delete(vertices::table.filter(vertices::id.eq(vertex_id))).execute(conn)?;
            let project = if deleted_number == 0 {
                project
            } else {
                project_services::update_project(conn, project.id, Utc::now())?
            };
            Ok((deleted_number, project))
        })
        .to_service_result()?;
    if deleted_number == 0 {
        return validation_error!("vertex_not_found_error");
    }
    let model_action = ModelActionType::new(&project, String::from("delete_vertex"), vertex_id);
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub async fn create_arc(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    project_id: i32,
    arc_in: ArcInCreateType,
) -> ServiceResult<ModelActionType<ArcOutType>> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    find_vertex_by_id(conn, arc_in.source_id)
        .to_service_result_find(String::from("source_vertex_not_found_error"))?;
    find_vertex_by_id(conn, arc_in.target_id)
        .to_service_result_find(String::from("target_vertex_not_found_error"))?;
    check_arc_value(&project, arc_in.value)?;
    let (arc, project) = conn
        .transaction(|conn| {
            let arc = diesel::insert_into(arcs::table)
                .values((
                    arcs::project_id.eq(project_id),
                    arcs::description.eq(arc_in.description),
                    arcs::value.eq(arc_in.value),
                    arcs::source_id.eq(arc_in.source_id),
                    arcs::target_id.eq(arc_in.target_id),
                ))
                .get_result::<Arc>(conn)?;
            let project = project_services::update_project(conn, project_id, arc.created_at)?;
            Ok((arc, project))
        })
        .to_service_result_unique(String::from("arc_duplication_error"))?;
    let arc_out = ArcOutType::from(arc);
    let model_action = ModelActionType::new(&project, String::from("create_arc"), arc_out);
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub async fn change_arc_description(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    arc_id: i32,
    description: String,
) -> ServiceResult<ModelActionType<ArcOutChangeDescriptionType>> {
    let project = find_project_by_arc_id(conn, arc_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let (arc, project) = conn
        .transaction(|conn| {
            let arc = diesel::update(arcs::table)
                .filter(arcs::id.eq(arc_id))
                .set(arcs::description.eq(description))
                .get_result::<Arc>(conn)?;
            let project = project_services::update_project(conn, project.id, arc.updated_at)?;
            Ok((arc, project))
        })
        .to_service_result_find(String::from("arc_not_found_error"))?;
    let arc_out = ArcOutChangeDescriptionType::from(arc);
    let model_action =
        ModelActionType::new(&project, String::from("change_arc_description"), arc_out);
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub async fn change_arc_value(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    arc_id: i32,
    value: f64,
) -> ServiceResult<ModelActionType<ArcOutChangeValueType>> {
    let project = find_project_by_arc_id(conn, arc_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    check_arc_value(&project, value)?;
    let (arc, project) = conn
        .transaction(|conn| {
            let arc = diesel::update(arcs::table)
                .filter(arcs::id.eq(arc_id))
                .set((arcs::value.eq(value),))
                .get_result::<Arc>(conn)?;
            let project = project_services::update_project(conn, project.id, arc.updated_at)?;
            Ok((arc, project))
        })
        .to_service_result_find(String::from("arc_not_found_error"))?;
    let arc_out = ArcOutChangeValueType::from(arc);
    let model_action = ModelActionType::new(&project, String::from("change_arc_value"), arc_out);
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub async fn delete_arc(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    arc_id: i32,
) -> ServiceResult<ModelActionType<i32>> {
    let project = find_project_by_arc_id(conn, arc_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let (deleted_number, project) = conn
        .transaction(|conn| {
            let deleted_number =
                diesel::delete(arcs::table.filter(arcs::id.eq(arc_id))).execute(conn)?;
            let project = if deleted_number == 0 {
                project
            } else {
                project_services::update_project(conn, project.id, Utc::now())?
            };
            Ok((deleted_number, project))
        })
        .to_service_result()?;
    if deleted_number == 0 {
        return validation_error!("arc_not_found_error");
    }
    let model_action = ModelActionType::new(&project, String::from("delete_arc"), arc_id);
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

fn check_arc_value(project: &Project, value: f64) -> ServiceResult<()> {
    match project.arc_value_type {
        ArcValueType::Symbolic => {
            if value == 0.0 || value == 1.0 {
                Ok(())
            } else {
                validation_error!("invalid_arc_value_symbolic_error", got = value)
            }
        }
        ArcValueType::FromMinusOneToOne => {
            if value >= -1.0 && value <= 1.0 {
                Ok(())
            } else {
                validation_error!(
                    "invalid_arc_value_error",
                    expected = "[-1.0; 1.0]",
                    got = value
                )
            }
        }
    }
}

impl<T> ModelActionType<T>
where
    T: Clone + Serialize + JsonSchema,
{
    pub fn new(project: &Project, name: String, data: T) -> Self {
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
            plugins_data: Value::Object(Map::new()),
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

impl From<Arc> for ArcOutChangeDescriptionType {
    fn from(arc: Arc) -> Self {
        Self {
            id: arc.id,
            description: arc.description,
            updated_at: arc.updated_at,
        }
    }
}

impl From<Arc> for ArcOutChangeValueType {
    fn from(arc: Arc) -> Self {
        Self {
            id: arc.id,
            value: arc.value,
            updated_at: arc.updated_at,
        }
    }
}
