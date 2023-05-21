use super::super::Plugins;
use super::models::TargetConcept;
use super::types::{TargetConceptInChangeType, TargetConceptOutType};
use crate::db;
use crate::models::{Concept, ConceptValueType, Project, User};
use crate::plugins::control_concepts::services as control_concepts_services;
use crate::plugins::Plugin;
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::{concepts, projects, target_concepts};
use crate::services::{model_services, permission_services};
use crate::types::{ConceptOutType, ModelActionType};
use crate::validation_error;
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
            let target_concepts =
                find_project_target_concepts(conn, model_out.project.id).to_service_result()?;
            for concept_out in model_out.concepts.iter_mut() {
                let target_concept = target_concepts
                    .iter()
                    .find(|tc| tc.concept_id == concept_out.id)
                    .unwrap();
                add_target_concept(concept_out, &target_concept);
            }
            Ok(model_out)
        })
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
            let target_concept =
                create_target_concept(conn, &project, concept_out.id).to_service_result()?;
            add_target_concept(&mut concept_out, &target_concept);
            Ok(concept_out)
        });
}

pub fn create_project_target_concepts(
    conn: &mut PgConnection,
    project: &Project,
) -> ServiceResult<Vec<TargetConcept>> {
    let concepts = model_services::find_project_concepts(conn, project.id).to_service_result()?;
    diesel::insert_into(target_concepts::table)
        .values(
            &concepts
                .into_iter()
                .map(|concept| {
                    (
                        target_concepts::concept_id.eq(concept.id),
                        target_concepts::value.eq(match project.concept_value_type {
                            ConceptValueType::None => None,
                            ConceptValueType::FromZeroToOne => Some(0.0),
                        }),
                    )
                })
                .collect::<Vec<_>>(),
        )
        .get_results::<TargetConcept>(conn)
        .to_service_result()
}

pub fn delete_project_target_concepts(
    conn: &mut PgConnection,
    project_id: i32,
) -> ServiceResult<()> {
    let concepts = model_services::find_project_concepts(conn, project_id).to_service_result()?;
    diesel::delete(
        target_concepts::table.filter(
            target_concepts::concept_id.eq_any(
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

pub fn create_target_concept(
    conn: &mut PgConnection,
    project: &Project,
    concept_id: i32,
) -> QueryResult<TargetConcept> {
    diesel::insert_into(target_concepts::table)
        .values((
            target_concepts::concept_id.eq(concept_id),
            target_concepts::value.eq(match project.concept_value_type {
                ConceptValueType::None => None,
                ConceptValueType::FromZeroToOne => Some(0.0),
            }),
        ))
        .get_result::<TargetConcept>(conn)
}

pub async fn change_target_concept(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    concept_id: i32,
    target_concept_in: TargetConceptInChangeType,
) -> ServiceResult<ModelActionType<TargetConceptOutType>> {
    let project = model_services::find_project_by_concept_id(conn, concept_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    if control_concepts_services::is_control(conn, concept_id)? {
        return validation_error!("concept_is_control_error");
    }
    model_services::check_concept_value(&project, target_concept_in.value)?;
    let target_concept = find_target_concept_by_id(conn, concept_id)
        .to_service_result_find(String::from("target_concept_not_found_error"))?;
    let (target_concept, concept, project) = conn
        .transaction(|conn| {
            let target_concept = diesel::update(target_concepts::table)
                .filter(target_concepts::concept_id.eq(target_concept.concept_id))
                .set((
                    target_concepts::is_target.eq(target_concept_in.is_target),
                    target_concepts::value.eq(target_concept_in.value),
                ))
                .get_result::<TargetConcept>(conn)?;
            let (concept, project) =
                model_services::update_concept(conn, concept_id, project.id, Utc::now())?;
            Ok((target_concept, concept, project))
        })
        .to_service_result()?;
    let target_concept_out = TargetConceptOutType::from((target_concept, concept));
    let model_action = ModelActionType::new(
        &project,
        String::from("changeTargetConcept"),
        target_concept_out,
    );
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub fn find_project_target_concepts(
    conn: &mut PgConnection,
    project_id: i32,
) -> QueryResult<Vec<TargetConcept>> {
    projects::table
        .inner_join(concepts::table.inner_join(target_concepts::table))
        .select(target_concepts::all_columns)
        .filter(projects::id.eq(project_id))
        .get_results::<TargetConcept>(conn)
}

pub fn find_target_concept_by_id(
    conn: &mut PgConnection,
    concept_id: i32,
) -> QueryResult<TargetConcept> {
    target_concepts::table
        .filter(target_concepts::concept_id.eq(concept_id))
        .first::<TargetConcept>(conn)
}

pub fn is_target(conn: &mut PgConnection, concept_id: i32) -> ServiceResult<bool> {
    let target_concept = find_target_concept_by_id(conn, concept_id)
        .optional()
        .to_service_result()?;
    let is_target = match target_concept {
        Some(target_concept) => target_concept.is_target,
        None => false,
    };
    Ok(is_target)
}

fn add_target_concept(concept_out: &mut ConceptOutType, target_concept: &TargetConcept) -> () {
    let plugins_data = match &mut concept_out.plugins_data {
        Value::Object(plugins_data) => plugins_data,
        _ => unreachable!(),
    };
    plugins_data.entry("targetConcepts").or_insert(json!({
        "isTarget": target_concept.is_target,
        "value": target_concept.value
    }));
}

impl From<(TargetConcept, Concept)> for TargetConceptOutType {
    fn from((target_concept, concept): (TargetConcept, Concept)) -> Self {
        Self {
            concept_id: target_concept.concept_id,
            is_target: target_concept.is_target,
            value: target_concept.value,
            updated_at: concept.updated_at,
        }
    }
}
