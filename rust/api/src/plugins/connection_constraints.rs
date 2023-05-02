pub mod models;
pub mod routes;
pub mod services;
pub mod types;

use super::Plugin;
use crate::models::Project;
use crate::plugins::Plugins;
use crate::response::ServiceResult;
use diesel::PgConnection;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request};
use std::sync::Arc;

pub struct ConnectionConstraintsPlugin;

impl Plugin for ConnectionConstraintsPlugin {
    fn get_name(&self) -> String {
        String::from("Connection Constraints")
    }
    fn install(&self, conn: &mut PgConnection, project: Project) -> ServiceResult<Project> {
        services::create_project_connection_constraints(conn, project.id)?;
        Ok(project)
    }
    fn uninstall(&self, conn: &mut PgConnection, project: Project) -> ServiceResult<Project> {
        services::delete_project_connection_constraints(conn, project.id)?;
        Ok(project)
    }
}

#[rocket::async_trait]
impl Fairing for ConnectionConstraintsPlugin {
    fn info(&self) -> Info {
        Info {
            name: "Connection Constraints",
            kind: Kind::Request,
        }
    }
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let plugins = request.local_cache::<Plugins, _>(|| unreachable!());
        let plugin = plugins.plugins.get(&self.get_name()).unwrap();
        services::handle_get_model(plugins, Arc::clone(plugin));
        services::handle_add_connection(plugins, Arc::clone(plugin));
        services::handle_change_connection_value(plugins, Arc::clone(plugin));
    }
}
