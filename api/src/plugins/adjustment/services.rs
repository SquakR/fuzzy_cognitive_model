use super::super::Plugins;
use super::models::{ConceptDynamicModel, DynamicModelType};
use super::types::ConceptDynamicModelOutType;
use crate::db;
use crate::models::{Concept, User};
use crate::plugins::Plugin;
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::{concept_dynamic_models, concepts, projects};
use crate::services::{model_services, permission_services};
use crate::types::{ConceptOutType, ModelActionType};
use crate::web_socket::WebSocketProjectService;
use chrono::Utc;
use diesel::prelude::*;
use diesel::PgConnection;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

pub fn handle_get_model(
    plugins: &Plugins,
    plugin: Arc<Mutex<Box<dyn Plugin + Sync + Send>>>,
) -> () {
    plugins
        .get_model_emitter
        .lock()
        .unwrap()
        .on(move |mut model_out, _| {
            let conn = &mut db::establish_connection();
            if !plugin
                .lock()
                .unwrap()
                .is_enabled(conn, model_out.project.id)?
            {
                return Ok(model_out);
            }
            let concept_dynamic_models =
                find_project_concept_dynamic_models(conn, model_out.project.id)
                    .to_service_result()?;
            for concept_out in model_out.concepts.iter_mut() {
                let concept_dynamic_model = concept_dynamic_models
                    .iter()
                    .find(|dm| dm.concept_id == concept_out.id)
                    .unwrap();
                add_dynamic_model(concept_out, &concept_dynamic_model);
            }
            Ok(model_out)
        });
}

pub fn handle_add_concept(
    plugins: &Plugins,
    plugin: Arc<Mutex<Box<dyn Plugin + Sync + Send>>>,
) -> () {
    plugins
        .add_concept_emitter
        .lock()
        .unwrap()
        .on(move |mut concept_out, project| {
            let conn = &mut db::establish_connection();
            if !plugin.lock().unwrap().is_enabled(conn, project.id)? {
                return Ok(concept_out);
            }
            let concept_dynamic_model =
                create_concept_dynamic_model(conn, concept_out.id).to_service_result()?;
            add_dynamic_model(&mut concept_out, &concept_dynamic_model);
            Ok(concept_out)
        });
}

pub fn create_project_concept_dynamic_models(
    conn: &mut PgConnection,
    project_id: i32,
) -> ServiceResult<Vec<ConceptDynamicModel>> {
    let concepts = model_services::find_project_concepts(conn, project_id).to_service_result()?;
    diesel::insert_into(concept_dynamic_models::table)
        .values(
            &concepts
                .into_iter()
                .map(|concept| concept_dynamic_models::concept_id.eq(concept.id))
                .collect::<Vec<_>>(),
        )
        .get_results::<ConceptDynamicModel>(conn)
        .to_service_result()
}

pub fn delete_project_concept_dynamic_models(
    conn: &mut PgConnection,
    project_id: i32,
) -> ServiceResult<()> {
    let concepts = model_services::find_project_concepts(conn, project_id).to_service_result()?;
    diesel::delete(
        concept_dynamic_models::table.filter(
            concept_dynamic_models::concept_id.eq_any(
                concepts
                    .into_iter()
                    .map(|concept| concept.id)
                    .collect::<Vec<_>>(),
            ),
        ),
    )
    .execute(conn)
    .to_service_result()?;
    Ok(())
}

pub fn create_concept_dynamic_model(
    conn: &mut PgConnection,
    concept_id: i32,
) -> QueryResult<ConceptDynamicModel> {
    diesel::insert_into(concept_dynamic_models::table)
        .values(concept_dynamic_models::concept_id.eq(concept_id))
        .get_result::<ConceptDynamicModel>(conn)
}

pub async fn change_dynamic_model_type(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    concept_id: i32,
    dynamic_model_type: Option<DynamicModelType>,
) -> ServiceResult<ModelActionType<ConceptDynamicModelOutType>> {
    let project = model_services::find_project_by_concept_id(conn, concept_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let concept_dynamic_model = find_concept_dynamic_model_by_id(conn, concept_id)
        .to_service_result_find(String::from("concept_dynamic_model_not_found_error"))?;
    let (concept_dynamic_model, concept, project) = conn
        .transaction(|conn| {
            let concept_dynamic_model = diesel::update(concept_dynamic_models::table)
                .filter(concept_dynamic_models::concept_id.eq(concept_dynamic_model.concept_id))
                .set(concept_dynamic_models::dynamic_model_type.eq(dynamic_model_type))
                .get_result::<ConceptDynamicModel>(conn)?;
            let (concept, project) =
                model_services::update_concept(conn, concept_id, project.id, Utc::now())?;
            Ok((concept_dynamic_model, concept, project))
        })
        .to_service_result()?;
    let concept_dynamic_model_out =
        ConceptDynamicModelOutType::from((concept_dynamic_model, concept));
    let model_action = ModelActionType::new(
        &project,
        String::from("change_dynamic_model_type"),
        concept_dynamic_model_out,
    );
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub fn find_project_concept_dynamic_models(
    conn: &mut PgConnection,
    project_id: i32,
) -> QueryResult<Vec<ConceptDynamicModel>> {
    projects::table
        .inner_join(concepts::table.inner_join(concept_dynamic_models::table))
        .select(concept_dynamic_models::all_columns)
        .filter(projects::id.eq(project_id))
        .get_results::<ConceptDynamicModel>(conn)
}

pub fn find_concept_dynamic_model_by_id(
    conn: &mut PgConnection,
    concept_id: i32,
) -> QueryResult<ConceptDynamicModel> {
    concept_dynamic_models::table
        .filter(concept_dynamic_models::concept_id.eq(concept_id))
        .first::<ConceptDynamicModel>(conn)
}

fn add_dynamic_model(
    concept_out: &mut ConceptOutType,
    concept_dynamic_model: &ConceptDynamicModel,
) -> () {
    let plugins_data = match &mut concept_out.plugins_data {
        Value::Object(plugins_data) => plugins_data,
        _ => unreachable!(),
    };
    plugins_data
        .entry("adjustment")
        .or_insert(json!({ "dynamic_model_type": concept_dynamic_model.dynamic_model_type }));
}

impl From<(ConceptDynamicModel, Concept)> for ConceptDynamicModelOutType {
    fn from((concept_dynamic_model, concept): (ConceptDynamicModel, Concept)) -> Self {
        Self {
            concept_id: concept_dynamic_model.concept_id,
            dynamic_model_type: concept_dynamic_model.dynamic_model_type,
            updated_at: concept.updated_at,
        }
    }
}
