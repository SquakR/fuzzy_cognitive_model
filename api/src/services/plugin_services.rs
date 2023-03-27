use crate::models::{Plugin, Project, ProjectPlugin, User};
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::{plugins, project_plugins, projects};
use crate::services::{permission_services, project_services};
use crate::types::PluginType;
use diesel::prelude::*;
use diesel::PgConnection;

pub fn set_project_plugins(
    conn: &mut PgConnection,
    user: &User,
    project_id: i32,
    plugins: Vec<String>,
) -> ServiceResult<Vec<String>> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_plugins(conn, &project, user.id)?;
    check_plugins(conn, &project, &plugins)?;
    conn.transaction(|conn| {
        delete_project_plugins(conn, project_id)?;
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

pub fn get_plugin_names(conn: &mut PgConnection) -> ServiceResult<Vec<String>> {
    Ok(get_plugins(conn)
        .to_service_result()?
        .into_iter()
        .map(|plugin| plugin.name)
        .collect::<Vec<String>>())
}

pub fn find_project_plugins(conn: &mut PgConnection, project_id: i32) -> QueryResult<Vec<Plugin>> {
    projects::table
        .inner_join(project_plugins::table.inner_join(plugins::table))
        .select(plugins::all_columns)
        .filter(projects::id.eq(project_id))
        .get_results::<Plugin>(conn)
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
        return Err(AppError::ValidationError(Box::new(move |locale| {
            t!(
                "invalid_plugin_error",
                locale = locale,
                plugin_name = &plugin_name
            )
        })));
    }
    for plugin_name in plugins {
        let plugin = all_plugins
            .iter()
            .find(|plugin| plugin.name == *plugin_name)
            .unwrap();
        let mut incompatible = false;
        if let Some(vertex_value_type) = &plugin.vertex_value_type {
            if project.vertex_value_type != *vertex_value_type {
                incompatible = true;
            }
        }
        if let Some(arc_value_type) = &plugin.arc_value_type {
            if project.arc_value_type != *arc_value_type {
                incompatible = true
            }
        }
        if incompatible {
            let plugin_name = plugin_name.to_owned();
            return Err(AppError::ValidationError(Box::new(move |locale| {
                t!(
                    "incompatible_plugin_error",
                    locale = locale,
                    plugin_name = &plugin_name
                )
            })));
        }
    }
    Ok(())
}

impl From<Plugin> for PluginType {
    fn from(plugin: Plugin) -> Self {
        PluginType {
            name: plugin.name,
            description: plugin.description,
            vertex_value_type: plugin.vertex_value_type,
            arc_value_type: plugin.arc_value_type,
        }
    }
}
