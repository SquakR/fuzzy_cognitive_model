pub mod adjustment;
pub mod control_concepts;

pub use adjustment::AdjustmentPlugin;
pub use control_concepts::ControlConceptsPlugin;

use crate::models::Project;
use crate::response::{ServiceResult, ToServiceResult};
use crate::services::plugin_services;
use crate::types::{ConceptOutType, ModelOutType};
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
    pub get_model_emitter: Mutex<GetModelEmitter>,
    pub add_concept_emitter: Mutex<AddConceptEmitter>,
}

impl Plugins {
    fn new() -> Self {
        let mut plugins = HashMap::<String, Arc<Mutex<Box<dyn Plugin + Sync + Send>>>>::new();
        plugins.insert(
            String::from("Control Concepts"),
            Arc::new(Mutex::new(Box::new(ControlConceptsPlugin))),
        );
        plugins.insert(
            String::from("Adjustment With Genetic Algorithms"),
            Arc::new(Mutex::new(Box::new(AdjustmentPlugin))),
        );
        Self {
            plugins,
            get_model_emitter: Mutex::new(GetModelEmitter::new()),
            add_concept_emitter: Mutex::new(AddConceptEmitter::new()),
        }
    }
}

pub struct GetModelEmitter {
    listeners: Vec<Mutex<Box<dyn Fn(ModelOutType) -> ServiceResult<ModelOutType> + Send>>>,
}

impl GetModelEmitter {
    fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }
    pub fn on(
        &mut self,
        callback: impl Fn(ModelOutType) -> ServiceResult<ModelOutType> + Send + 'static,
    ) -> () {
        self.listeners.push(Mutex::new(Box::new(callback)));
    }
    pub fn emit(&self, mut model_out: ModelOutType) -> ServiceResult<ModelOutType> {
        for callback in &self.listeners {
            model_out = callback.lock().unwrap()(model_out)?;
        }
        Ok(model_out)
    }
}

pub struct AddConceptEmitter {
    listeners:
        Vec<Mutex<Box<dyn Fn(Project, ConceptOutType) -> ServiceResult<ConceptOutType> + Send>>>,
}

impl AddConceptEmitter {
    fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }
    pub fn on(
        &mut self,
        callback: impl Fn(Project, ConceptOutType) -> ServiceResult<ConceptOutType> + Send + 'static,
    ) -> () {
        self.listeners.push(Mutex::new(Box::new(callback)));
    }
    pub fn emit(
        &self,
        project: Project,
        mut concept_out: ConceptOutType,
    ) -> ServiceResult<ConceptOutType> {
        for callback in &self.listeners {
            concept_out = callback.lock().unwrap()(project.clone(), concept_out)?;
        }
        Ok(concept_out)
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
