pub mod models;
pub mod routes;
pub mod services;
pub mod types;

use super::Plugin;
use crate::models::Project;
use crate::plugins::Plugins;
use crate::response::ServiceResult;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request};
use std::sync::Arc;

pub struct ControlVerticesPlugin;

impl Plugin for ControlVerticesPlugin {
    fn get_name(&self) -> String {
        String::from("Control Vertices")
    }
    fn install(
        &self,
        conn: &mut diesel::PgConnection,
        project: crate::models::Project,
    ) -> ServiceResult<Project> {
        services::create_project_control_vertices(conn, project.id)?;
        Ok(project)
    }
    fn uninstall(
        &self,
        conn: &mut diesel::PgConnection,
        project: Project,
    ) -> ServiceResult<Project> {
        services::delete_project_control_vertices(conn, project.id)?;
        Ok(project)
    }
}

#[rocket::async_trait]
impl Fairing for ControlVerticesPlugin {
    fn info(&self) -> Info {
        Info {
            name: "Control Vertices",
            kind: Kind::Request,
        }
    }
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let plugins = request.local_cache::<Plugins, _>(|| unreachable!());
        let plugin = plugins.plugins.get(&self.get_name()).unwrap();
        services::handle_get_model(plugins, Arc::clone(plugin));
        services::handle_add_vertex(plugins, Arc::clone(plugin));
    }
}
