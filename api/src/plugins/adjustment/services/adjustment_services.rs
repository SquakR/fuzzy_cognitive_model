use super::super::models::{AdjustmentRun, DynamicModelType};
use super::super::types::{AdjustmentInType, AdjustmentRunOutType};
use super::adjustment_run_services::{AdjustmentModel, Concept, Connection, Constraint};
use super::adjustment_save_result_services::SaveResultServer;
use super::permission_services;
use crate::forbidden_error;
use crate::models::User;
use crate::plugins::Plugins;
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::{
    adjustment_runs, concept_constraints, concept_dynamic_models, concepts, connection_constraints,
    connections, control_concepts, control_connections, target_concepts,
};
use crate::services::{model_services, project_services};
use crate::types::ModelActionType;
use crate::web_socket::WebSocketProjectService;
use diesel::prelude::*;
use diesel::PgConnection;
use rocket::tokio::runtime::Handle;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

pub async fn adjust(
    mut conn: PgConnection,
    plugins: &Plugins,
    project_service: WebSocketProjectService,
    user: &User,
    project_id: i32,
    adjustment_in: AdjustmentInType,
) -> ServiceResult<ModelActionType<AdjustmentRunOutType>> {
    let project = project_services::find_project_by_id(&mut conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    if !plugins
        .plugins
        .get("Adjustment With Genetic Algorithms")
        .unwrap()
        .lock()
        .unwrap()
        .is_enabled(&mut conn, project_id)?
    {
        return forbidden_error!("adjustment_plugin_is_not_enabled_error");
    }
    permission_services::can_adjust(&mut conn, &project, user.id)?;
    let model_copy = model_services::save_model_copy(&mut conn, plugins, user, project_id)?;
    let adjustment_model = get_adjustment_model(&mut conn, project_id, adjustment_in)?;
    let adjustment_run = create_adjustment_run(
        &mut conn,
        project_id,
        model_copy.id,
        &adjustment_model.adjustment_in,
    )?;
    let adjustment_run_id = adjustment_run.id;
    let adjustment_run_out = AdjustmentRunOutType::from_adjustment_run(&mut conn, adjustment_run)?;
    let model_action = ModelActionType::new(&project, String::from("adjust"), adjustment_run_out);
    project_service.notify(model_action.clone()).await;
    let handle = Handle::current();
    thread::spawn(move || {
        handle.spawn(async move {
            adjustment_model
                .run(SaveResultServer {
                    conn,
                    project_id,
                    adjustment_run_id,
                    project_service,
                })
                .await;
        })
    });
    Ok(model_action)
}

fn get_adjustment_model(
    conn: &mut PgConnection,
    project_id: i32,
    adjustment_in: AdjustmentInType,
) -> ServiceResult<AdjustmentModel> {
    let concepts = get_concepts(conn, project_id)?;
    let concepts_map = HashMap::from_iter(
        concepts
            .iter()
            .map(|concept| (concept.id, Arc::clone(concept))),
    );
    let control_concepts = concepts
        .iter()
        .filter(|concept| concept.is_control)
        .cloned()
        .collect();
    let target_concepts = concepts
        .iter()
        .filter(|concept| concept.is_target)
        .cloned()
        .collect();
    let regular_concepts = concepts
        .iter()
        .filter(|concept| !concept.is_control && !concept.is_target)
        .cloned()
        .collect();
    let connections = get_connections(conn, project_id)?;
    let connections_map = HashMap::from_iter(
        connections
            .iter()
            .map(|connection| (connection.id, Arc::clone(connection))),
    );
    let control_connections = connections
        .iter()
        .filter(|connection| connection.is_control)
        .cloned()
        .collect();
    Ok(AdjustmentModel {
        adjustment_in,
        concepts_map,
        control_concepts,
        target_concepts,
        regular_concepts,
        connections_map,
        control_connections,
    })
}

fn create_adjustment_run(
    conn: &mut PgConnection,
    project_id: i32,
    model_copy_id: i32,
    adjustment_in: &AdjustmentInType,
) -> ServiceResult<AdjustmentRun> {
    diesel::insert_into(adjustment_runs::table)
        .values((
            adjustment_runs::project_id.eq(project_id),
            adjustment_runs::model_copy_id.eq(model_copy_id),
            adjustment_runs::name.eq(&adjustment_in.name),
            adjustment_runs::description.eq(&adjustment_in.description),
            adjustment_runs::max_model_time.eq(&adjustment_in.max_model_time),
            adjustment_runs::dynamic_model_type.eq(&adjustment_in.dynamic_model_type),
            adjustment_runs::generation_size.eq(&adjustment_in.generation_size),
            adjustment_runs::generation_save_interval.eq(&adjustment_in.generation_save_interval),
            adjustment_runs::max_generations.eq(&adjustment_in.stop_condition.max_generations),
            adjustment_runs::max_without_improvements
                .eq(&adjustment_in.stop_condition.max_without_improvements),
            adjustment_runs::error.eq(&adjustment_in.stop_condition.error),
        ))
        .get_result::<AdjustmentRun>(conn)
        .to_service_result()
}

fn get_concepts(conn: &mut PgConnection, project_id: i32) -> ServiceResult<Vec<Arc<Concept>>> {
    let concepts = concepts::table
        .inner_join(control_concepts::table)
        .inner_join(target_concepts::table)
        .inner_join(concept_constraints::table)
        .inner_join(concept_dynamic_models::table)
        .filter(concepts::project_id.eq(project_id))
        .select((
            concepts::id,
            concepts::value,
            control_concepts::is_control,
            target_concepts::is_target,
            target_concepts::value,
            concept_constraints::has_constraint,
            concept_constraints::min_value,
            concept_constraints::include_min_value,
            concept_constraints::max_value,
            concept_constraints::include_max_value,
            concept_dynamic_models::dynamic_model_type,
        ))
        .get_results::<(
            i32,
            Option<f64>,
            bool,
            bool,
            Option<f64>,
            bool,
            f64,
            bool,
            f64,
            bool,
            Option<DynamicModelType>,
        )>(conn)
        .to_service_result()?
        .into_iter()
        .map(
            |(
                id,
                value,
                is_control,
                is_target,
                target_value,
                has_constraint,
                min_value,
                include_min_value,
                max_value,
                include_max_value,
                dynamic_model_type,
            )| {
                let constraint = if has_constraint {
                    Some(Constraint {
                        min_value,
                        include_min_value,
                        max_value,
                        include_max_value,
                    })
                } else {
                    None
                };
                Arc::new(Concept {
                    id,
                    value: value.unwrap(),
                    is_control,
                    is_target,
                    target_value,
                    constraint,
                    dynamic_model_type,
                })
            },
        )
        .collect::<Vec<_>>();
    Ok(concepts)
}

fn get_connections(
    conn: &mut PgConnection,
    project_id: i32,
) -> ServiceResult<Vec<Arc<Connection>>> {
    let connections = connections::table
        .inner_join(control_connections::table)
        .inner_join(connection_constraints::table)
        .filter(connections::project_id.eq(project_id))
        .select((
            connections::id,
            connections::value,
            connections::source_id,
            connections::target_id,
            control_connections::is_control,
            connection_constraints::has_constraint,
            connection_constraints::min_value,
            connection_constraints::include_min_value,
            connection_constraints::max_value,
            connection_constraints::include_max_value,
        ))
        .get_results::<(i32, f64, i32, i32, bool, bool, f64, bool, f64, bool)>(conn)
        .to_service_result()?
        .into_iter()
        .map(
            |(
                id,
                value,
                source_id,
                target_id,
                is_control,
                has_constraint,
                min_value,
                include_min_value,
                max_value,
                include_max_value,
            )| {
                let constraint = if has_constraint {
                    Some(Constraint {
                        min_value,
                        include_min_value,
                        max_value,
                        include_max_value,
                    })
                } else {
                    None
                };
                Arc::new(Connection {
                    id,
                    value,
                    source_id,
                    target_id,
                    is_control,
                    constraint,
                })
            },
        )
        .collect::<Vec<_>>();
    Ok(connections)
}
