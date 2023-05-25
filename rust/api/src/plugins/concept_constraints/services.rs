use super::super::Plugins;
use super::models::ConceptConstraint;
use super::types::{ConceptConstraintInChangeType, ConceptConstraintOutType};
use crate::db;
use crate::models::{Concept, User};
use crate::plugins::control_concepts::services as control_concepts_services;
use crate::plugins::Plugin;
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::{concept_constraints, concepts, projects};
use crate::services::{model_services, permission_services};
use crate::types::{ConceptOutType, ModelActionType};
use crate::validation_error;
use crate::web_socket::WebSocketModelService;
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
            let concept_constraints =
                find_project_concept_constraints(conn, model_out.project.id).to_service_result()?;
            for concept_out in model_out.concepts.iter_mut() {
                let concept_constraint = concept_constraints
                    .iter()
                    .find(|cc| cc.concept_id == concept_out.id)
                    .unwrap();
                add_concept_constraint(concept_out, &concept_constraint);
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
            let concept_constraint =
                create_concept_constraint(conn, concept_out.id).to_service_result()?;
            add_concept_constraint(&mut concept_out, &concept_constraint);
            Ok(concept_out)
        });
}

pub fn handle_change_concept_value(
    plugins: &Plugins,
    plugin: Arc<Mutex<Box<dyn Plugin + Sync + Send>>>,
) -> () {
    plugins
        .change_concept_value_emitter
        .lock()
        .unwrap()
        .on(move |value, extra| {
            let conn = &mut db::establish_connection();
            if !plugin.lock().unwrap().is_enabled(conn, extra.project.id)? {
                return Ok(value);
            }
            let concept_constraint = find_concept_constraint_by_id(conn, extra.concept_id)
                .to_service_result_find(String::from("concept_constraint_not_found_error"))?;
            check_constraint_value(
                value,
                &ConceptConstraintInChangeType {
                    has_constraint: concept_constraint.has_constraint,
                    min_value: concept_constraint.min_value,
                    include_min_value: concept_constraint.include_min_value,
                    max_value: concept_constraint.max_value,
                    include_max_value: concept_constraint.include_max_value,
                },
            )?;
            Ok(value)
        })
}

pub fn create_project_concept_constraints(
    conn: &mut PgConnection,
    project_id: i32,
) -> ServiceResult<Vec<ConceptConstraint>> {
    let concepts = model_services::find_project_concepts(conn, project_id).to_service_result()?;
    diesel::insert_into(concept_constraints::table)
        .values(
            &concepts
                .into_iter()
                .map(|concept| {
                    (
                        concept_constraints::concept_id.eq(concept.id),
                        concept_constraints::min_value.eq(0.0),
                        concept_constraints::include_min_value.eq(true),
                        concept_constraints::max_value.eq(1.0),
                        concept_constraints::include_max_value.eq(true),
                    )
                })
                .collect::<Vec<_>>(),
        )
        .get_results::<ConceptConstraint>(conn)
        .to_service_result()
}

pub fn delete_project_concept_constraints(
    conn: &mut PgConnection,
    project_id: i32,
) -> ServiceResult<()> {
    let concepts = model_services::find_project_concepts(conn, project_id).to_service_result()?;
    diesel::delete(
        concept_constraints::table.filter(
            concept_constraints::concept_id.eq_any(
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

pub fn create_concept_constraint(
    conn: &mut PgConnection,
    concept_id: i32,
) -> QueryResult<ConceptConstraint> {
    diesel::insert_into(concept_constraints::table)
        .values((
            concept_constraints::concept_id.eq(concept_id),
            concept_constraints::min_value.eq(0.0),
            concept_constraints::include_min_value.eq(true),
            concept_constraints::max_value.eq(1.0),
            concept_constraints::include_max_value.eq(true),
        ))
        .get_result::<ConceptConstraint>(conn)
}

pub async fn change_concept_constraint(
    conn: &mut PgConnection,
    model_service: WebSocketModelService,
    user: &User,
    concept_id: i32,
    concept_constraint_in: ConceptConstraintInChangeType,
) -> ServiceResult<ModelActionType<ConceptConstraintOutType>> {
    let project = model_services::find_project_by_concept_id(conn, concept_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_change_model(conn, &project, user.id)?;
    let control_concept_result =
        control_concepts_services::find_control_concept_by_id(conn, concept_id)
            .optional()
            .to_service_result()?;
    match control_concept_result {
        Some(control_concept) => {
            if !control_concept.is_control {
                return validation_error!("concept_not_control_error");
            }
        }
        None => return validation_error!("concept_not_control_error"),
    };
    let concept = model_services::find_concept_by_id(conn, concept_id)
        .to_service_result_find(String::from("concept_not_found_error"))?;
    check_constraint_value(concept.value, &concept_constraint_in)?;
    let concept_constraint = find_concept_constraint_by_id(conn, concept_id)
        .to_service_result_find(String::from("concept_constraint_not_found_error"))?;
    let (concept_constraint, concept, project) = conn
        .transaction(|conn| {
            let concept_constraint = diesel::update(concept_constraints::table)
                .filter(concept_constraints::concept_id.eq(concept_constraint.concept_id))
                .set((
                    concept_constraints::has_constraint.eq(concept_constraint_in.has_constraint),
                    concept_constraints::min_value.eq(concept_constraint_in.min_value),
                    concept_constraints::include_min_value
                        .eq(concept_constraint_in.include_min_value),
                    concept_constraints::max_value.eq(concept_constraint_in.max_value),
                    concept_constraints::include_max_value
                        .eq(concept_constraint_in.include_max_value),
                ))
                .get_result::<ConceptConstraint>(conn)?;
            let (concept, project) =
                model_services::update_concept(conn, concept_id, project.id, Utc::now())?;
            Ok((concept_constraint, concept, project))
        })
        .to_service_result()?;
    let concept_constraint_out = ConceptConstraintOutType::from((concept_constraint, concept));
    let model_action = ModelActionType::new(
        &project,
        String::from("changeConceptConstraint"),
        concept_constraint_out,
    );
    model_service.notify(model_action.clone()).await;
    Ok(model_action)
}

pub fn find_project_concept_constraints(
    conn: &mut PgConnection,
    project_id: i32,
) -> QueryResult<Vec<ConceptConstraint>> {
    projects::table
        .inner_join(concepts::table.inner_join(concept_constraints::table))
        .select(concept_constraints::all_columns)
        .filter(projects::id.eq(project_id))
        .get_results::<ConceptConstraint>(conn)
}

pub fn find_concept_constraint_by_id(
    conn: &mut PgConnection,
    concept_id: i32,
) -> QueryResult<ConceptConstraint> {
    concept_constraints::table
        .filter(concept_constraints::concept_id.eq(concept_id))
        .first::<ConceptConstraint>(conn)
}

fn add_concept_constraint(
    concept_out: &mut ConceptOutType,
    concept_constraint: &ConceptConstraint,
) -> () {
    let plugins_data = match &mut concept_out.plugins_data {
        Value::Object(plugins_data) => plugins_data,
        _ => unreachable!(),
    };
    plugins_data.entry("conceptConstraints").or_insert(json!({
        "hasConstraint": concept_constraint.has_constraint,
        "minValue": concept_constraint.min_value,
        "includeMinValue": concept_constraint.include_min_value,
        "maxValue": concept_constraint.max_value,
        "includeMaxValue": concept_constraint.include_max_value
    }));
}

fn check_constraint_value(
    value: Option<f64>,
    concept_constraint_in: &ConceptConstraintInChangeType,
) -> ServiceResult<()> {
    if !concept_constraint_in.has_constraint {
        return Ok(());
    }
    if concept_constraint_in.include_min_value {
        if value.unwrap() < concept_constraint_in.min_value {
            return generate_range_error(value, concept_constraint_in);
        }
    } else {
        if value.unwrap() <= concept_constraint_in.min_value {
            return generate_range_error(value, concept_constraint_in);
        }
    }
    if concept_constraint_in.include_max_value {
        if value.unwrap() > concept_constraint_in.max_value {
            return generate_range_error(value, concept_constraint_in);
        }
    } else {
        if value.unwrap() >= concept_constraint_in.max_value {
            return generate_range_error(value, concept_constraint_in);
        }
    }
    Ok(())
}

fn generate_range_error(
    value: Option<f64>,
    concept_constraint_in: &ConceptConstraintInChangeType,
) -> ServiceResult<()> {
    let range = format!(
        "{}{}; {}{}",
        if concept_constraint_in.include_min_value {
            "["
        } else {
            "("
        },
        concept_constraint_in.min_value,
        concept_constraint_in.max_value,
        if concept_constraint_in.include_max_value {
            "]"
        } else {
            ")"
        },
    );
    let value = value.unwrap();
    validation_error!(
        "concept_constraint_range_error",
        range = range,
        value = value
    )
}

impl From<(ConceptConstraint, Concept)> for ConceptConstraintOutType {
    fn from((concept_constraint, concept): (ConceptConstraint, Concept)) -> Self {
        Self {
            concept_id: concept_constraint.concept_id,
            has_constraint: concept_constraint.has_constraint,
            min_value: concept_constraint.min_value,
            include_min_value: concept_constraint.include_min_value,
            max_value: concept_constraint.max_value,
            include_max_value: concept_constraint.include_max_value,
            updated_at: concept.updated_at,
        }
    }
}
