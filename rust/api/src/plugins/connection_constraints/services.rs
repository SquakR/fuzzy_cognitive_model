use super::super::Plugins;
use super::models::ConnectionConstraint;
use super::types::{ConnectionConstraintInChangeType, ConnectionConstraintOutType};
use crate::db;
use crate::models::{Connection, User};
use crate::plugins::control_connections::services as control_connections_services;
use crate::plugins::Plugin;
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::{connection_constraints, connections, projects};
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
            let connection_constraints =
                find_project_connection_constraints(conn, model_out.project.id)
                    .to_service_result()?;
            for connection_out in model_out.connections.iter_mut() {
                let connection_constraint = connection_constraints
                    .iter()
                    .find(|cc| cc.connection_id == connection_out.id)
                    .unwrap();
                add_connection_constraint(connection_out, &connection_constraint);
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
            let connection_constraint =
                create_connection_constraint(conn, connection_out.id).to_service_result()?;
            add_connection_constraint(&mut connection_out, &connection_constraint);
            Ok(connection_out)
        })
}

pub fn handle_change_connection_value(
    plugins: &Plugins,
    plugin: Arc<Mutex<Box<dyn Plugin + Sync + Send>>>,
) -> () {
    plugins
        .change_connection_value_emitter
        .lock()
        .unwrap()
        .on(move |value, extra| {
            let conn = &mut db::establish_connection();
            if !plugin.lock().unwrap().is_enabled(conn, extra.project.id)? {
                return Ok(value);
            }
            let connection_constraint = find_connection_constraint_by_id(conn, extra.connection_id)
                .to_service_result_find(String::from("connection_constraint_not_found_error"))?;
            check_constraint_value(
                value,
                &ConnectionConstraintInChangeType {
                    has_constraint: connection_constraint.has_constraint,
                    min_value: connection_constraint.min_value,
                    include_min_value: connection_constraint.include_min_value,
                    max_value: connection_constraint.max_value,
                    include_max_value: connection_constraint.include_max_value,
                },
            )?;
            Ok(value)
        });
}

pub fn create_project_connection_constraints(
    conn: &mut PgConnection,
    project_id: i32,
) -> ServiceResult<Vec<ConnectionConstraint>> {
    let connections =
        model_services::find_project_connections(conn, project_id).to_service_result()?;
    diesel::insert_into(connection_constraints::table)
        .values(
            &connections
                .into_iter()
                .map(|connection| {
                    (
                        connection_constraints::connection_id.eq(connection.id),
                        connection_constraints::min_value.eq(-1.0),
                        connection_constraints::include_min_value.eq(true),
                        connection_constraints::max_value.eq(1.0),
                        connection_constraints::include_max_value.eq(true),
                    )
                })
                .collect::<Vec<_>>(),
        )
        .get_results::<ConnectionConstraint>(conn)
        .to_service_result()
}

pub fn delete_project_connection_constraints(
    conn: &mut PgConnection,
    project_id: i32,
) -> ServiceResult<()> {
    let connections =
        model_services::find_project_connections(conn, project_id).to_service_result()?;
    diesel::delete(
        connection_constraints::table.filter(
            connection_constraints::connection_id.eq_any(
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

pub fn create_connection_constraint(
    conn: &mut PgConnection,
    connection_id: i32,
) -> QueryResult<ConnectionConstraint> {
    diesel::insert_into(connection_constraints::table)
        .values((
            connection_constraints::connection_id.eq(connection_id),
            connection_constraints::min_value.eq(-1.0),
            connection_constraints::include_min_value.eq(true),
            connection_constraints::max_value.eq(1.0),
            connection_constraints::include_max_value.eq(true),
        ))
        .get_result::<ConnectionConstraint>(conn)
}

pub async fn change_connection_constraint(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    connection_id: i32,
    connection_constraint_in: ConnectionConstraintInChangeType,
) -> ServiceResult<ModelActionType<ConnectionConstraintOutType>> {
    let project = model_services::find_project_by_connection_id(conn, connection_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let control_connection_result =
        control_connections_services::find_control_connection_by_id(conn, connection_id)
            .optional()
            .to_service_result()?;
    match control_connection_result {
        Some(control_connection) => {
            if !control_connection.is_control {
                return validation_error!("connection_not_control_error");
            }
        }
        None => return validation_error!("connection_not_control_error"),
    };
    let connection = model_services::find_connection_by_id(conn, connection_id)
        .to_service_result_find(String::from("connection_not_found_error"))?;
    check_constraint_value(connection.value, &connection_constraint_in)?;
    let connection_constraint = find_connection_constraint_by_id(conn, connection_id)
        .to_service_result_find(String::from("connection_constraint_not_found_error"))?;
    let (connection_constraint, connection, project) = conn
        .transaction(|conn| {
            let connection_constraint = diesel::update(connection_constraints::table)
                .filter(
                    connection_constraints::connection_id.eq(connection_constraint.connection_id),
                )
                .set((
                    connection_constraints::has_constraint
                        .eq(connection_constraint_in.has_constraint),
                    connection_constraints::min_value.eq(connection_constraint_in.min_value),
                    connection_constraints::include_min_value
                        .eq(connection_constraint_in.include_min_value),
                    connection_constraints::max_value.eq(connection_constraint_in.max_value),
                    connection_constraints::include_max_value
                        .eq(connection_constraint_in.include_max_value),
                ))
                .get_result::<ConnectionConstraint>(conn)?;
            let (connection, project) =
                model_services::update_connection(conn, connection_id, project.id, Utc::now())?;
            Ok((connection_constraint, connection, project))
        })
        .to_service_result()?;
    let connection_constraint_out =
        ConnectionConstraintOutType::from((connection_constraint, connection));
    let model_action = ModelActionType::new(
        &project,
        String::from("change_connection_constraint"),
        connection_constraint_out,
    );
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub fn find_project_connection_constraints(
    conn: &mut PgConnection,
    project_id: i32,
) -> QueryResult<Vec<ConnectionConstraint>> {
    projects::table
        .inner_join(connections::table.inner_join(connection_constraints::table))
        .select(connection_constraints::all_columns)
        .filter(projects::id.eq(project_id))
        .get_results::<ConnectionConstraint>(conn)
}

pub fn find_connection_constraint_by_id(
    conn: &mut PgConnection,
    connection_id: i32,
) -> QueryResult<ConnectionConstraint> {
    connection_constraints::table
        .filter(connection_constraints::connection_id.eq(connection_id))
        .first::<ConnectionConstraint>(conn)
}

fn add_connection_constraint(
    connection_out: &mut ConnectionOutType,
    connection_constraint: &ConnectionConstraint,
) -> () {
    let plugins_data = match &mut connection_out.plugins_data {
        Value::Object(plugins_data) => plugins_data,
        _ => unreachable!(),
    };
    plugins_data
        .entry("connectionConstraints")
        .or_insert(json!({
            "hasConstraint": connection_constraint.has_constraint,
            "minValue": connection_constraint.min_value,
            "includeMinValue": connection_constraint.include_min_value,
            "maxValue": connection_constraint.max_value,
            "includeMaxValue": connection_constraint.include_max_value
        }));
}

fn check_constraint_value(
    value: f64,
    connection_constraint_in: &ConnectionConstraintInChangeType,
) -> ServiceResult<()> {
    if !connection_constraint_in.has_constraint {
        return Ok(());
    }
    if connection_constraint_in.include_min_value {
        if value < connection_constraint_in.min_value {
            return generate_range_error(value, connection_constraint_in);
        }
    } else {
        if value <= connection_constraint_in.min_value {
            return generate_range_error(value, connection_constraint_in);
        }
    }
    if connection_constraint_in.include_max_value {
        if value > connection_constraint_in.max_value {
            return generate_range_error(value, connection_constraint_in);
        }
    } else {
        if value >= connection_constraint_in.max_value {
            return generate_range_error(value, connection_constraint_in);
        }
    }
    Ok(())
}

fn generate_range_error(
    value: f64,
    connection_constraint_in: &ConnectionConstraintInChangeType,
) -> ServiceResult<()> {
    let range = format!(
        "{}{}; {}{}",
        if connection_constraint_in.include_min_value {
            "["
        } else {
            "("
        },
        connection_constraint_in.min_value,
        connection_constraint_in.max_value,
        if connection_constraint_in.include_max_value {
            "]"
        } else {
            ")"
        },
    );
    validation_error!(
        "connection_constraint_range_error",
        range = range,
        value = value
    )
}

impl From<(ConnectionConstraint, Connection)> for ConnectionConstraintOutType {
    fn from((connection_constraint, connection): (ConnectionConstraint, Connection)) -> Self {
        Self {
            connection_id: connection_constraint.connection_id,
            has_constraint: connection_constraint.has_constraint,
            min_value: connection_constraint.min_value,
            include_min_value: connection_constraint.include_min_value,
            max_value: connection_constraint.max_value,
            include_max_value: connection_constraint.include_max_value,
            updated_at: connection.updated_at,
        }
    }
}
