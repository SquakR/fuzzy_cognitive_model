use super::super::Plugins;
use super::models::ControlVertex;
use super::types::ControlVertexOutType;
use crate::db;
use crate::models::User;
use crate::plugins::Plugin;
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::{control_vertices, projects, vertices};
use crate::services::{model_services, permission_services};
use crate::types::{ModelActionType, VertexOutType};
use crate::validation_error;
use crate::web_socket::WebSocketProjectService;
use diesel::prelude::*;
use diesel::PgConnection;
use serde_json::{Map, Value};
use std::sync::{Arc, Mutex};

pub fn handle_get_model(
    plugins: &Plugins,
    plugin: Arc<Mutex<Box<dyn Plugin + Sync + Send>>>,
) -> () {
    plugins
        .get_model_emitter
        .lock()
        .unwrap()
        .on(move |mut model_out| {
            let conn = &mut db::establish_connection();
            if !plugin
                .lock()
                .unwrap()
                .is_enabled(conn, model_out.project.id)?
            {
                return Ok(model_out);
            }
            let control_vertices =
                find_project_control_vertices(conn, model_out.project.id).to_service_result()?;
            for vertex_out in model_out.vertices.iter_mut() {
                let control_vertex = control_vertices
                    .iter()
                    .find(|cv| cv.vertex_id == vertex_out.id)
                    .unwrap();
                add_is_control(vertex_out, control_vertex.is_control);
            }
            Ok(model_out)
        })
}

pub fn handle_add_vertex(
    plugins: &Plugins,
    plugin: Arc<Mutex<Box<dyn Plugin + Sync + Send>>>,
) -> () {
    plugins
        .add_vertex_emitter
        .lock()
        .unwrap()
        .on(move |project, mut vertex_out| {
            let conn = &mut db::establish_connection();
            if !plugin.lock().unwrap().is_enabled(conn, project.id)? {
                return Ok(vertex_out);
            }
            let control_vertex = create_control_vertex(conn, vertex_out.id)?;
            add_is_control(&mut vertex_out, control_vertex.is_control);
            Ok(vertex_out)
        });
}

pub fn create_project_control_vertices(
    conn: &mut PgConnection,
    project_id: i32,
) -> ServiceResult<Vec<ControlVertex>> {
    let vertices = model_services::find_project_vertices(conn, project_id).to_service_result()?;
    diesel::insert_into(control_vertices::table)
        .values(
            &vertices
                .into_iter()
                .map(|vertex| control_vertices::vertex_id.eq(vertex.id))
                .collect::<Vec<_>>(),
        )
        .get_results::<ControlVertex>(conn)
        .to_service_result()
}

pub fn delete_project_control_vertices(
    conn: &mut PgConnection,
    project_id: i32,
) -> ServiceResult<()> {
    let vertices = model_services::find_project_vertices(conn, project_id).to_service_result()?;
    diesel::delete(
        control_vertices::table.filter(
            control_vertices::vertex_id.eq_any(
                vertices
                    .into_iter()
                    .map(|vertex| vertex.id)
                    .collect::<Vec<_>>(),
            ),
        ),
    )
    .execute(conn)
    .to_service_result()?;
    Ok(())
}

pub fn create_control_vertex(
    conn: &mut PgConnection,
    vertex_id: i32,
) -> ServiceResult<ControlVertex> {
    diesel::insert_into(control_vertices::table)
        .values(control_vertices::vertex_id.eq(vertex_id))
        .get_result::<ControlVertex>(conn)
        .to_service_result()
}

pub async fn set_is_control(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    vertex_id: i32,
    is_control: bool,
) -> ServiceResult<ModelActionType<ControlVertexOutType>> {
    let project = model_services::find_project_by_vertex_id(conn, vertex_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let mut control_vertex = find_control_vertex_by_id(conn, vertex_id)
        .to_service_result_find(String::from("control_vertex_not_found_error"))?;
    if control_vertex.is_control && is_control {
        return validation_error!("vertex_already_control_error");
    }
    if !control_vertex.is_control && !is_control {
        return validation_error!("vertex_not_control_error");
    }
    control_vertex = diesel::update(control_vertices::table)
        .filter(control_vertices::vertex_id.eq(control_vertex.vertex_id))
        .set(control_vertices::is_control.eq(is_control))
        .get_result::<ControlVertex>(conn)
        .to_service_result()?;
    let control_vertex_out = ControlVertexOutType::from(control_vertex);
    let model_action = ModelActionType::new(
        &project,
        String::from("change_vertex_is_control"),
        control_vertex_out,
    );
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub fn find_project_control_vertices(
    conn: &mut PgConnection,
    project_id: i32,
) -> QueryResult<Vec<ControlVertex>> {
    projects::table
        .inner_join(vertices::table.inner_join(control_vertices::table))
        .select(control_vertices::all_columns)
        .filter(projects::id.eq(project_id))
        .get_results::<ControlVertex>(conn)
}

pub fn find_control_vertex_by_id(
    conn: &mut PgConnection,
    vertex_id: i32,
) -> QueryResult<ControlVertex> {
    control_vertices::table
        .filter(control_vertices::vertex_id.eq(vertex_id))
        .first::<ControlVertex>(conn)
}

fn add_is_control(vertex_out: &mut VertexOutType, is_control: bool) -> () {
    let plugins_data = match &mut vertex_out.plugins_data {
        Value::Object(plugins_data) => plugins_data,
        _ => unreachable!(),
    };
    let control_vertices_data = match plugins_data
        .entry("controlVertices")
        .or_insert(Value::Object(Map::new()))
    {
        Value::Object(control_vertices_data) => control_vertices_data,
        _ => unreachable!(),
    };
    control_vertices_data
        .entry("isControl")
        .or_insert(Value::Bool(is_control));
}

impl From<ControlVertex> for ControlVertexOutType {
    fn from(control_vertex: ControlVertex) -> Self {
        Self {
            vertex_id: control_vertex.vertex_id,
            is_control: control_vertex.is_control,
        }
    }
}
