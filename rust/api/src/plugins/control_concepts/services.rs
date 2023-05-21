use super::super::Plugins;
use super::models::ControlConcept;
use super::types::ControlConceptOutType;
use crate::db;
use crate::models::{Concept, User};
use crate::plugins::concept_constraints::models::ConceptConstraint;
use crate::plugins::concept_constraints::services as concept_constraints_services;
use crate::plugins::target_concepts::services as target_concepts_services;
use crate::plugins::Plugin;
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::{concept_constraints, concepts, control_concepts, projects};
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
            let control_concepts =
                find_project_control_concepts(conn, model_out.project.id).to_service_result()?;
            for concept_out in model_out.concepts.iter_mut() {
                let control_concept = control_concepts
                    .iter()
                    .find(|cc| cc.concept_id == concept_out.id)
                    .unwrap();
                add_is_control(concept_out, &control_concept);
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
            let control_concept =
                create_control_concept(conn, concept_out.id).to_service_result()?;
            add_is_control(&mut concept_out, &control_concept);
            Ok(concept_out)
        });
}

pub fn create_project_control_concepts(
    conn: &mut PgConnection,
    project_id: i32,
) -> ServiceResult<Vec<ControlConcept>> {
    let concepts = model_services::find_project_concepts(conn, project_id).to_service_result()?;
    diesel::insert_into(control_concepts::table)
        .values(
            &concepts
                .into_iter()
                .map(|concept| control_concepts::concept_id.eq(concept.id))
                .collect::<Vec<_>>(),
        )
        .get_results::<ControlConcept>(conn)
        .to_service_result()
}

pub fn delete_project_control_concepts(
    conn: &mut PgConnection,
    project_id: i32,
) -> ServiceResult<()> {
    let concepts = model_services::find_project_concepts(conn, project_id).to_service_result()?;
    diesel::delete(
        control_concepts::table.filter(
            control_concepts::concept_id.eq_any(
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

pub fn create_control_concept(
    conn: &mut PgConnection,
    concept_id: i32,
) -> QueryResult<ControlConcept> {
    diesel::insert_into(control_concepts::table)
        .values(control_concepts::concept_id.eq(concept_id))
        .get_result::<ControlConcept>(conn)
}

pub async fn set_is_control(
    conn: &mut PgConnection,
    project_service: WebSocketProjectService,
    user: &User,
    concept_id: i32,
    is_control: bool,
) -> ServiceResult<ModelActionType<ControlConceptOutType>> {
    let project = model_services::find_project_by_concept_id(conn, concept_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    if target_concepts_services::is_target(conn, concept_id)? {
        return validation_error!("concept_is_target_error");
    }
    let control_concept = find_control_concept_by_id(conn, concept_id)
        .to_service_result_find(String::from("control_concept_not_found_error"))?;
    let concept_constraint =
        concept_constraints_services::find_concept_constraint_by_id(conn, concept_id)
            .optional()
            .to_service_result()?;
    let (control_concept, concept_constraint, concept, project) = conn
        .transaction(|conn| {
            let control_concept = diesel::update(control_concepts::table)
                .filter(control_concepts::concept_id.eq(control_concept.concept_id))
                .set(control_concepts::is_control.eq(is_control))
                .get_result::<ControlConcept>(conn)?;
            let concept_constraint = match concept_constraint {
                Some(concept_constraint) => {
                    if !is_control && concept_constraint.has_constraint {
                        let concept_constraint = diesel::update(concept_constraints::table)
                            .filter(
                                concept_constraints::concept_id.eq(concept_constraint.concept_id),
                            )
                            .set((concept_constraints::has_constraint.eq(false),))
                            .get_result::<ConceptConstraint>(conn)?;
                        Some(concept_constraint)
                    } else {
                        Some(concept_constraint)
                    }
                }
                None => None,
            };
            let (concept, project) =
                model_services::update_concept(conn, concept_id, project.id, Utc::now())?;
            Ok((control_concept, concept_constraint, concept, project))
        })
        .to_service_result()?;
    let control_concept_out =
        ControlConceptOutType::from((control_concept, concept_constraint, concept));
    let model_action = ModelActionType::new(
        &project,
        String::from("changeControlConcept"),
        control_concept_out,
    );
    project_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub fn find_project_control_concepts(
    conn: &mut PgConnection,
    project_id: i32,
) -> QueryResult<Vec<ControlConcept>> {
    projects::table
        .inner_join(concepts::table.inner_join(control_concepts::table))
        .select(control_concepts::all_columns)
        .filter(projects::id.eq(project_id))
        .get_results::<ControlConcept>(conn)
}

pub fn find_control_concept_by_id(
    conn: &mut PgConnection,
    concept_id: i32,
) -> QueryResult<ControlConcept> {
    control_concepts::table
        .filter(control_concepts::concept_id.eq(concept_id))
        .first::<ControlConcept>(conn)
}

pub fn is_control(conn: &mut PgConnection, concept_id: i32) -> ServiceResult<bool> {
    let control_concept = find_control_concept_by_id(conn, concept_id)
        .optional()
        .to_service_result()?;
    let is_control = match control_concept {
        Some(control_concept) => control_concept.is_control,
        None => false,
    };
    Ok(is_control)
}

fn add_is_control(concept_out: &mut ConceptOutType, control_concept: &ControlConcept) -> () {
    let plugins_data = match &mut concept_out.plugins_data {
        Value::Object(plugins_data) => plugins_data,
        _ => unreachable!(),
    };
    plugins_data
        .entry("controlConcepts")
        .or_insert(json!({ "isControl": control_concept.is_control }));
}

impl From<(ControlConcept, Option<ConceptConstraint>, Concept)> for ControlConceptOutType {
    fn from(
        (control_concept, concept_constraint, concept): (
            ControlConcept,
            Option<ConceptConstraint>,
            Concept,
        ),
    ) -> Self {
        Self {
            concept_id: control_concept.concept_id,
            is_control: control_concept.is_control,
            has_constraint: concept_constraint
                .map(|concept_constraint| concept_constraint.has_constraint),
            updated_at: concept.updated_at,
        }
    }
}
