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

pub struct AdjustmentPlugin;

impl Plugin for AdjustmentPlugin {
    fn get_name(&self) -> String {
        String::from("Adjustment With Genetic Algorithms")
    }
    fn install(&self, conn: &mut PgConnection, project: Project) -> ServiceResult<Project> {
        services::create_project_concept_dynamic_models(conn, project.id)?;
        Ok(project)
    }
    fn uninstall(&self, conn: &mut PgConnection, project: Project) -> ServiceResult<Project> {
        services::delete_project_concept_dynamic_models(conn, project.id)?;
        Ok(project)
    }
}

#[rocket::async_trait]
impl Fairing for AdjustmentPlugin {
    fn info(&self) -> Info {
        Info {
            name: "Adjustment With Genetic Algorithms",
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
