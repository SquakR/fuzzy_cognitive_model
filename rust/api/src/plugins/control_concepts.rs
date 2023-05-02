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

pub struct ControlConceptsPlugin;

impl Plugin for ControlConceptsPlugin {
    fn get_name(&self) -> String {
        String::from("Control Concepts")
    }
    fn install(
        &self,
        conn: &mut PgConnection,
        project: crate::models::Project,
    ) -> ServiceResult<Project> {
        services::create_project_control_concepts(conn, project.id)?;
        Ok(project)
    }
    fn uninstall(&self, conn: &mut PgConnection, project: Project) -> ServiceResult<Project> {
        services::delete_project_control_concepts(conn, project.id)?;
        Ok(project)
    }
}

#[rocket::async_trait]
impl Fairing for ControlConceptsPlugin {
    fn info(&self) -> Info {
        Info {
            name: "Control Concepts",
            kind: Kind::Request,
        }
    }
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        let plugins = request.local_cache::<Plugins, _>(|| unreachable!());
        let plugin = plugins.plugins.get(&self.get_name()).unwrap();
        services::handle_get_model(plugins, Arc::clone(plugin));
        services::handle_add_concept(plugins, Arc::clone(plugin));
    }
}
