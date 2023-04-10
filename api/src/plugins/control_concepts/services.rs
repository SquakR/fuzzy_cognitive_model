use super::super::Plugins;
use super::models::ControlConcept;
use super::types::ControlConceptOutType;
use crate::db;
use crate::models::User;
use crate::plugins::Plugin;
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::{concepts, control_concepts, projects};
use crate::services::{model_services, permission_services};
use crate::types::{ConceptOutType, ModelActionType};
use crate::validation_error;
use crate::web_socket::WebSocketProjectService;
use diesel::prelude::*;
use diesel::PgConnection;
use serde_json::{Map, Value};
use std::sync::{Arc, Mutex};

pub fn handle_get_model(
    plugins: &Plugins,
    plugin: Arc<Mutex<Box<dyn Plugin + Sync + Send>>>,
) -> () {
    plugins
        .get_model_emitter
        .lock()
        .unwrap()
        .on(move |mut model_out| {
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
                    .find(|cv| cv.concept_id == concept_out.id)
                    .unwrap();
                add_is_control(concept_out, control_concept.is_control);
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
        .on(move |project, mut concept_out| {
            let conn = &mut db::establish_connection();
            if !plugin.lock().unwrap().is_enabled(conn, project.id)? {
                return Ok(concept_out);
            }
            let control_concept = create_control_concept(conn, concept_out.id)?;
            add_is_control(&mut concept_out, control_concept.is_control);
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
) -> ServiceResult<ControlConcept> {
    diesel::insert_into(control_concepts::table)
        .values(control_concepts::concept_id.eq(concept_id))
        .get_result::<ControlConcept>(conn)
        .to_service_result()
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
    let mut control_concept = find_control_concept_by_id(conn, concept_id)
        .to_service_result_find(String::from("control_concept_not_found_error"))?;
    if control_concept.is_control && is_control {
        return validation_error!("concept_already_control_error");
    }
    if !control_concept.is_control && !is_control {
        return validation_error!("concept_not_control_error");
    }
    control_concept = diesel::update(control_concepts::table)
        .filter(control_concepts::concept_id.eq(control_concept.concept_id))
        .set(control_concepts::is_control.eq(is_control))
        .get_result::<ControlConcept>(conn)
        .to_service_result()?;
    let control_concept_out = ControlConceptOutType::from(control_concept);
    let model_action = ModelActionType::new(
        &project,
        String::from("change_concept_is_control"),
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

fn add_is_control(concept_out: &mut ConceptOutType, is_control: bool) -> () {
    let plugins_data = match &mut concept_out.plugins_data {
        Value::Object(plugins_data) => plugins_data,
        _ => unreachable!(),
    };
    let control_concepts_data = match plugins_data
        .entry("controlConcepts")
        .or_insert(Value::Object(Map::new()))
    {
        Value::Object(control_concepts_data) => control_concepts_data,
        _ => unreachable!(),
    };
    control_concepts_data
        .entry("isControl")
        .or_insert(Value::Bool(is_control));
}

impl From<ControlConcept> for ControlConceptOutType {
    fn from(control_concept: ControlConcept) -> Self {
        Self {
            concept_id: control_concept.concept_id,
            is_control: control_concept.is_control,
        }
    }
}
