use crate::models::{Plugin, Project, ProjectPlugin, User};
use crate::plugins::Plugins;
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::{plugins, project_plugins, projects};
use crate::services::{permission_services, project_services};
use crate::types::PluginType;
use crate::validation_error;
use diesel::prelude::*;
use diesel::PgConnection;
use std::collections::HashSet;

pub fn set_project_plugins(
    conn: &mut PgConnection,
    plugins: &Plugins,
    user: &User,
    project_id: i32,
    new_plugins: Vec<String>,
) -> ServiceResult<Vec<String>> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_plugins(conn, &project, user.id)?;
    check_plugins(conn, &project, &new_plugins)?;
    install_and_uninstall_plugins(conn, project, plugins, &new_plugins)?;
    conn.transaction(|conn| {
        delete_project_plugins(conn, project_id)?;
        let mut insert_rows = vec![];
        for name in new_plugins {
            insert_rows.push((
                project_plugins::project_id.eq(project_id),
                project_plugins::plugin_name.eq(name),
            ));
        }
        let plugins = if insert_rows.len() > 0 {
            diesel::insert_into(project_plugins::table)
                .values(&insert_rows)
                .get_results::<ProjectPlugin>(conn)?
        } else {
            vec![]
        };
        Ok(plugins
            .into_iter()
            .map(|plugin| plugin.plugin_name)
            .collect())
    })
    .to_service_result()
}

pub fn delete_project_plugins(conn: &mut PgConnection, project_id: i32) -> QueryResult<usize> {
    diesel::delete(project_plugins::table.filter(project_plugins::project_id.eq(project_id)))
        .execute(conn)
}

pub fn get_plugins(conn: &mut PgConnection) -> QueryResult<Vec<Plugin>> {
    plugins::table.get_results::<Plugin>(conn)
}

pub fn find_project_plugins(conn: &mut PgConnection, project_id: i32) -> QueryResult<Vec<Plugin>> {
    projects::table
        .inner_join(project_plugins::table.inner_join(plugins::table))
        .select(plugins::all_columns)
        .filter(projects::id.eq(project_id))
        .get_results::<Plugin>(conn)
}

pub fn find_project_plugin_names(
    conn: &mut PgConnection,
    project_id: i32,
) -> QueryResult<Vec<String>> {
    Ok(find_project_plugins(conn, project_id)?
        .into_iter()
        .map(|plugin| plugin.name)
        .collect())
}

fn check_plugins(
    conn: &mut PgConnection,
    project: &Project,
    plugins: &[String],
) -> ServiceResult<()> {
    let all_plugins = get_plugins(conn).to_service_result()?;
    let all_plugin_names = all_plugins
        .iter()
        .map(|plugin| plugin.name.as_str())
        .collect::<Vec<&str>>();
    if let Some(index) = plugins
        .iter()
        .position(|plugin_name| !all_plugin_names.contains(&plugin_name.as_str()))
    {
        let plugin_name = plugins[index].to_owned();
        return validation_error!("invalid_plugin_error", plugin_name = &plugin_name);
    }
    for plugin_name in plugins {
        let plugin = all_plugins
            .iter()
            .find(|plugin| plugin.name == *plugin_name)
            .unwrap();
        let mut incompatible = false;
        if let Some(concept_value_type) = &plugin.concept_value_type {
            if project.concept_value_type != *concept_value_type {
                incompatible = true;
            }
        }
        if let Some(connection_value_type) = &plugin.connection_value_type {
            if project.connection_value_type != *connection_value_type {
                incompatible = true
            }
        }
        if incompatible {
            let plugin_name = plugin_name.to_owned();
            return validation_error!("incompatible_plugin_error", plugin_name = &plugin_name);
        }
    }
    Ok(())
}

fn install_and_uninstall_plugins(
    conn: &mut PgConnection,
    mut project: Project,
    plugins: &Plugins,
    new_plugins: &[String],
) -> ServiceResult<Project> {
    let current_plugins_set = find_project_plugin_names(conn, project.id)
        .to_service_result()?
        .into_iter()
        .collect::<HashSet<_>>();
    let new_plugins_set = new_plugins.iter().cloned().collect::<HashSet<_>>();
    for plugin_to_uninstall in current_plugins_set.difference(&new_plugins_set) {
        project = plugins.plugins[plugin_to_uninstall]
            .lock()
            .unwrap()
            .uninstall(conn, project)?;
    }
    for plugin_to_install in new_plugins_set.difference(&current_plugins_set) {
        project = plugins.plugins[plugin_to_install]
            .lock()
            .unwrap()
            .install(conn, project)?;
    }
    Ok(project)
}

impl From<Plugin> for PluginType {
    fn from(plugin: Plugin) -> Self {
        Self {
            name: plugin.name,
            description: plugin.description,
            concept_value_type: plugin.concept_value_type,
            connection_value_type: plugin.connection_value_type,
        }
    }
}
