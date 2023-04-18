pub mod adjustment;
pub mod control_concepts;
pub mod control_connections;
pub mod target_concepts;

pub use adjustment::AdjustmentPlugin;
pub use control_concepts::ControlConceptsPlugin;
pub use control_connections::ControlConnectionsPlugin;
pub use target_concepts::TargetConceptsPlugin;

use crate::models::Project;
use crate::response::{ServiceResult, ToServiceResult};
use crate::services::plugin_services;
use crate::types::{ConceptOutType, ConnectionOutType, ModelOutType};
use diesel::PgConnection;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub trait Plugin {
    fn get_name(&self) -> String;
    fn is_enabled(&self, conn: &mut PgConnection, project_id: i32) -> ServiceResult<bool> {
        let plugin_names =
            plugin_services::find_project_plugin_names(conn, project_id).to_service_result()?;
        return Ok(plugin_names.contains(&self.get_name()));
    }
    fn install(&self, _: &mut PgConnection, project: Project) -> ServiceResult<Project> {
        Ok(project)
    }
    fn uninstall(&self, _: &mut PgConnection, project: Project) -> ServiceResult<Project> {
        Ok(project)
    }
}

pub struct Plugins {
    pub plugins: HashMap<String, Arc<Mutex<Box<dyn Plugin + Sync + Send>>>>,
    pub get_model_emitter: Mutex<Emitter<ModelOutType, ()>>,
    pub add_concept_emitter: Mutex<Emitter<ConceptOutType, Project>>,
    pub add_connection_emitter: Mutex<Emitter<ConnectionOutType, Project>>,
}

impl Plugins {
    fn new() -> Self {
        let mut plugins = HashMap::<String, Arc<Mutex<Box<dyn Plugin + Sync + Send>>>>::new();
        plugins.insert(
            String::from("Control Concepts"),
            Arc::new(Mutex::new(Box::new(ControlConceptsPlugin))),
        );
        plugins.insert(
            String::from("Target Concepts"),
            Arc::new(Mutex::new(Box::new(TargetConceptsPlugin))),
        );
        plugins.insert(
            String::from("Control Connections"),
            Arc::new(Mutex::new(Box::new(ControlConnectionsPlugin))),
        );
        plugins.insert(
            String::from("Adjustment With Genetic Algorithms"),
            Arc::new(Mutex::new(Box::new(AdjustmentPlugin))),
        );
        Self {
            plugins,
            get_model_emitter: Mutex::new(Emitter::new()),
            add_concept_emitter: Mutex::new(Emitter::new()),
            add_connection_emitter: Mutex::new(Emitter::new()),
        }
    }
}

pub struct Emitter<R, E> {
    listeners: Vec<Mutex<Box<dyn Fn(R, E) -> ServiceResult<R> + Send>>>,
}

impl<R, E> Emitter<R, E>
where
    E: Clone,
{
    fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }
    pub fn on(&mut self, callback: impl Fn(R, E) -> ServiceResult<R> + Send + 'static) -> () {
        self.listeners.push(Mutex::new(Box::new(callback)));
    }
    pub fn emit(&self, mut result: R, extra: E) -> ServiceResult<R> {
        for callback in &self.listeners {
            result = callback.lock().unwrap()(result, extra.clone())?;
        }
        Ok(result)
    }
}

pub struct PluginsFairing;

#[rocket::async_trait]
impl Fairing for PluginsFairing {
    fn info(&self) -> Info {
        Info {
            name: "Model Plugins",
            kind: Kind::Request,
        }
    }
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        request.local_cache(|| Plugins::new());
    }
}
