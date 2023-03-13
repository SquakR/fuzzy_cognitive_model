use crate::models::{Plugin, ProjectPlugin, User};
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::{plugins, project_plugins, projects};
use crate::services::permission_services;
use crate::types::PluginType;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn set_project_plugins(
    connection: &mut PgConnection,
    user: &User,
    project_id: i32,
    plugins: Vec<String>,
) -> ServiceResult<Vec<String>> {
    if !permission_services::can_change_plugins(connection, project_id, user.id)? {
        return Err(AppError::ForbiddenError(String::from(
            "change_plugins_forbidden_error",
        )));
    }
    let all_plugins = get_plugins(connection)?
        .into_iter()
        .map(|plugin| plugin.name)
        .collect::<Vec<String>>();
    if let Some(index) = plugins
        .iter()
        .position(|plugin| !all_plugins.contains(plugin))
    {
        return Err(AppError::ValidationError(Box::new(move |locale| {
            t!(
                "invalid_plugin_error",
                locale = locale,
                plugin = &plugins[index]
            )
        })));
    }
    delete_project_plugins(connection, project_id)?;
    let mut insert_rows = vec![];
    for name in plugins {
        insert_rows.push((
            project_plugins::project_id.eq(project_id),
            project_plugins::plugin_name.eq(name),
        ));
    }
    let plugins = if insert_rows.len() > 0 {
        diesel::insert_into(project_plugins::table)
            .values(&insert_rows)
            .get_results::<ProjectPlugin>(connection)
            .to_service_result()?
    } else {
        vec![]
    };
    Ok(plugins
        .into_iter()
        .map(|plugin| plugin.plugin_name)
        .collect())
}

pub fn delete_project_plugins(
    connection: &mut PgConnection,
    project_id: i32,
) -> ServiceResult<usize> {
    diesel::delete(project_plugins::table.filter(project_plugins::project_id.eq(project_id)))
        .execute(connection)
        .to_service_result()
}

pub fn get_plugins(connection: &mut PgConnection) -> ServiceResult<Vec<Plugin>> {
    plugins::table
        .get_results::<Plugin>(connection)
        .to_service_result()
}

pub fn find_project_plugins(connection: &mut PgConnection, project_id: i32) -> Vec<Plugin> {
    projects::table
        .inner_join(project_plugins::table.inner_join(plugins::table))
        .select(plugins::all_columns)
        .filter(projects::id.eq(project_id))
        .get_results::<Plugin>(connection)
        .unwrap()
}

impl From<Plugin> for PluginType {
    fn from(plugin: Plugin) -> Self {
        PluginType {
            name: plugin.name,
            description: plugin.description,
        }
    }
}
