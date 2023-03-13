use crate::models::Plugin;
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::{plugins, project_plugins, projects};
use crate::types::PluginType;
use diesel::pg::PgConnection;
use diesel::prelude::*;

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
