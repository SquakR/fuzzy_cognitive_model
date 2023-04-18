use super::super::Plugins;
use super::models::ControlConnection;
use super::types::ControlConnectionOutType;
use crate::db;
use crate::models::{Connection, User};
use crate::plugins::Plugin;
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::{connections, control_connections, projects};
use crate::services::{model_services, permission_services};
use crate::types::{ConnectionOutType, ModelActionType};
use crate::validation_error;
use crate::web_socket::WebSocketProjectService;
use chrono::Utc;
use diesel::prelude::*;
use diesel::Connection as DieselConnection;
use diesel::PgConnection;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

pub fn handle_get_model(
    plugins: &Plugins,
    plugin: Arc<Mutex<Box<dyn Plugin + Sync + Send>>>,
) -> () {
    plugins
        .get_model_emitter
        .lock()
        .unwrap()
        .on(move |mut model_out, _| {
            let conn = &mut db::establish_connection();
            if !plugin
                .lock()
                .unwrap()
                .is_enabled(conn, model_out.project.id)?
            {
                return Ok(model_out);
            }
            let control_connections =
                find_project_control_connections(conn, model_out.project.id).to_service_result()?;
            for connection_out in model_out.connections.iter_mut() {
                let control_connection = control_connections
                    .iter()
                    .find(|cc| cc.connection_id == connection_out.id)
                    .unwrap();
                add_is_control(connection_out, &control_connection);
            }
            Ok(model_out)
        })
}

pub fn handle_add_connection(
    plugins: &Plugins,
    plugin: Arc<Mutex<Box<dyn Plugin + Sync + Send>>>,
) -> () {
    plugins
        .add_connection_emitter
        .lock()
        .unwrap()
        .on(move |mut connection_out, project| {
            let conn = &mut db::establish_connection();
            if !plugin.lock().unwrap().is_enabled(conn, project.id)? {
                return Ok(connection_out);
            }
            let control_connection =
                create_control_connection(conn, connection_out.id).to_service_result()?;
            add_is_control(&mut connection_out, &control_connection);
            Ok(connection_out)
        });
}

pub fn create_project_control_connections(
    conn: &mut PgConnection,
    project_id: i32,
) -> ServiceResult<Vec<ControlConnection>> {
    let connections =
        model_services::find_project_connections(conn, project_id).to_service_result()?;
    diesel::insert_into(control_connections::table)
        .values(
            &connections
                .into_iter()
                .map(|connection| control_connections::connection_id.eq(connection.id))
                .collect::<Vec<_>>(),
        )
        .get_results::<ControlConnection>(conn)
        .to_service_result()
}

pub fn delete_project_control_connections(
    conn: &mut PgConnection,
    project_id: i32,
) -> ServiceResult<()> {
    let connections =
        model_services::find_project_connections(conn, project_id).to_service_result()?;
    diesel::delete(
        control_connections::table.filter(
            control_connections::connection_id.eq_any(
                connections
                    .into_iter()
                    .map(|connection| connection.id)
                    .collect::<Vec<_>>(),
            ),
        ),
    )
    .execute(conn)
    .to_service_result()?;
    Ok(())
}

pub fn create_control_connection(
    conn: &mut PgConnection,
    connection_id: i32,
) -> QueryResult<ControlConnection> {
    diesel::insert_into(control_connections::table)
        .values(control_connections::connection_id.eq(connection_id))
        .get_result::<ControlConnection>(conn)
}

pub async fn set_is_control(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    connection_id: i32,
    is_control: bool,
) -> ServiceResult<ModelActionType<ControlConnectionOutType>> {
    let project = model_services::find_project_by_connection_id(conn, connection_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let control_connection = find_control_connection_by_id(conn, connection_id)
        .to_service_result_find(String::from("control_connection_not_found_error"))?;
    if control_connection.is_control && is_control {
        return validation_error!("connection_already_control_error");
    }
    if !control_connection.is_control && !is_control {
        return validation_error!("connection_not_control_error");
    }
    let (control_connection, connection, project) = conn
        .transaction(|conn| {
            let control_connection = diesel::update(control_connections::table)
                .filter(control_connections::connection_id.eq(control_connection.connection_id))
                .set(control_connections::is_control.eq(is_control))
                .get_result::<ControlConnection>(conn)?;
            let (connection, project) =
                model_services::update_connection(conn, connection_id, project.id, Utc::now())?;
            Ok((control_connection, connection, project))
        })
        .to_service_result()?;
    let control_connection_out = ControlConnectionOutType::from((control_connection, connection));
    let model_action = ModelActionType::new(
        &project,
        String::from("change_control_connection"),
        control_connection_out,
    );
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub fn find_project_control_connections(
    conn: &mut PgConnection,
    project_id: i32,
) -> QueryResult<Vec<ControlConnection>> {
    projects::table
        .inner_join(connections::table.inner_join(control_connections::table))
        .select(control_connections::all_columns)
        .filter(projects::id.eq(project_id))
        .get_results::<ControlConnection>(conn)
}

pub fn find_control_connection_by_id(
    conn: &mut PgConnection,
    concept_id: i32,
) -> QueryResult<ControlConnection> {
    control_connections::table
        .filter(control_connections::connection_id.eq(concept_id))
        .first::<ControlConnection>(conn)
}

fn add_is_control(
    connection_out: &mut ConnectionOutType,
    control_connection: &ControlConnection,
) -> () {
    let plugins_data = match &mut connection_out.plugins_data {
        Value::Object(plugins_data) => plugins_data,
        _ => unreachable!(),
    };
    plugins_data
        .entry("controlConnections")
        .or_insert(json!({ "isControl": control_connection.is_control }));
}

impl From<(ControlConnection, Connection)> for ControlConnectionOutType {
    fn from((control_connection, connection): (ControlConnection, Connection)) -> Self {
        Self {
            connection_id: control_connection.connection_id,
            is_control: control_connection.is_control,
            updated_at: connection.updated_at,
        }
    }
}
