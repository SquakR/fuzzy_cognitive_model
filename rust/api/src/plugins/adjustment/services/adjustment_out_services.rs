use super::super::models::{
    AdjustmentConceptValue, AdjustmentConnectionValue, AdjustmentGeneration, AdjustmentIndividual,
    AdjustmentRun,
};
use super::super::types::{
    AdjustmentConceptValueOutType, AdjustmentConnectionValueOutType, AdjustmentGenerationOutType,
    AdjustmentRunOutType, StopConditionType,
};
use crate::filter_date_time;
use crate::models::{Project, User};
use crate::pagination::Paginate;
use crate::plugins::adjustment::types::{
    AdjustmentIndividualGenerationOutType, AdjustmentIndividualOutType,
};
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::{
    adjustment_concept_values, adjustment_connection_values, adjustment_generations,
    adjustment_individuals, adjustment_runs, projects,
};
use crate::services::{permission_services, project_services};
use crate::types::{IntervalInType, PaginationInType, PaginationOutType};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;

pub fn get_adjustment_run(
    conn: &mut PgConnection,
    user: &User,
    adjustment_run_id: i32,
) -> ServiceResult<AdjustmentRunOutType> {
    let project = find_project_by_adjustment_run_id(conn, adjustment_run_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_view_project(conn, &project, user)?;
    let adjustment_run = find_adjustment_run_by_id(conn, adjustment_run_id).to_service_result()?;
    AdjustmentRunOutType::from_adjustment_run(conn, adjustment_run)
}

pub fn paginate_adjustment_runs(
    conn: &mut PgConnection,
    user: &User,
    project_id: i32,
    search: Option<String>,
    created_at: Option<IntervalInType<DateTime<Utc>>>,
    pagination_in: PaginationInType,
) -> ServiceResult<PaginationOutType<AdjustmentRunOutType>> {
    let project = project_services::find_project_by_id(conn, project_id)
        .to_service_result_find(String::from("project_not_found_error"))?;
    permission_services::can_view_project(conn, &project, user)?;
    let mut query = adjustment_runs::table
        .filter(adjustment_runs::project_id.eq(project_id))
        .order(adjustment_runs::created_at.desc())
        .into_boxed();
    if let Some(search) = search {
        let like_pattern = format!("{}%", search);
        query = query.filter(
            adjustment_runs::name
                .ilike(like_pattern.to_owned())
                .or(adjustment_runs::description.ilike(like_pattern)),
        );
    }
    if let Some(created_at) = created_at {
        filter_date_time!(adjustment_runs::created_at, created_at, query);
    }
    let (adjustment_runs, total_count, total_pages) = query
        .paginate(pagination_in.page as i64)
        .per_page(pagination_in.per_page as i64)
        .load_and_count_pages::<AdjustmentRun>(conn)
        .to_service_result()?;
    Ok(PaginationOutType {
        data: AdjustmentRunOutType::from_adjustment_runs(conn, adjustment_runs)?,
        total_count: total_count as i32,
        total_pages: total_pages as i32,
    })
}

pub fn get_adjustment_generation(
    conn: &mut PgConnection,
    user: &User,
    adjustment_generation_id: i32,
) -> ServiceResult<AdjustmentGenerationOutType> {
    let project = find_project_by_adjustment_generation_id(conn, adjustment_generation_id)
        .to_service_result_find(String::from("adjustment_generation_not_found_error"))?;
    permission_services::can_view_project(conn, &project, user)?;
    let adjustment_generation =
        find_adjustment_generation_by_id(conn, adjustment_generation_id).to_service_result()?;
    Ok(AdjustmentGenerationOutType::from(adjustment_generation))
}

pub fn paginate_adjustment_generations(
    conn: &mut PgConnection,
    user: &User,
    adjustment_run_id: i32,
    pagination_in: PaginationInType,
) -> ServiceResult<PaginationOutType<AdjustmentGenerationOutType>> {
    let project = find_project_by_adjustment_run_id(conn, adjustment_run_id)
        .to_service_result_find(String::from("adjustment_run_not_found_error"))?;
    permission_services::can_view_project(conn, &project, user)?;
    let (generations, total_count, total_pages) = adjustment_generations::table
        .filter(adjustment_generations::adjustment_run_id.eq(adjustment_run_id))
        .order(adjustment_generations::number.desc())
        .paginate(pagination_in.page as i64)
        .per_page(pagination_in.per_page as i64)
        .load_and_count_pages::<AdjustmentGeneration>(conn)
        .to_service_result()?;
    Ok(PaginationOutType {
        data: generations
            .into_iter()
            .map(AdjustmentGenerationOutType::from)
            .collect(),
        total_count: total_count as i32,
        total_pages: total_pages as i32,
    })
}

pub fn get_adjustment_individual(
    conn: &mut PgConnection,
    user: &User,
    adjustment_individual_id: i32,
) -> ServiceResult<AdjustmentIndividualOutType> {
    let project = find_project_by_adjustment_individual_id(conn, adjustment_individual_id)
        .to_service_result_find(String::from("adjustment_generation_not_found_error"))?;
    permission_services::can_view_project(conn, &project, user)?;
    let adjustment_individual =
        find_adjustment_individual_by_id(conn, adjustment_individual_id).to_service_result()?;
    AdjustmentIndividualOutType::from_individual(conn, adjustment_individual)
}

pub fn paginate_adjustment_individuals(
    conn: &mut PgConnection,
    user: &User,
    adjustment_generation_id: i32,
    pagination_in: PaginationInType,
) -> ServiceResult<PaginationOutType<AdjustmentIndividualOutType>> {
    let project = find_project_by_adjustment_generation_id(conn, adjustment_generation_id)
        .to_service_result_find(String::from("adjustment_generation_not_found_error"))?;
    permission_services::can_view_project(conn, &project, user)?;
    let (adjustment_individuals, total_count, total_pages) = adjustment_individuals::table
        .filter(adjustment_individuals::adjustment_generation_id.eq(adjustment_generation_id))
        .paginate(pagination_in.page as i64)
        .per_page(pagination_in.per_page as i64)
        .load_and_count_pages::<AdjustmentIndividual>(conn)
        .to_service_result()?;
    Ok(PaginationOutType {
        data: AdjustmentIndividualOutType::from_individuals(conn, adjustment_individuals)?,
        total_count: total_count as i32,
        total_pages: total_pages as i32,
    })
}

pub fn find_adjustment_run_by_id(
    conn: &mut PgConnection,
    adjustment_run_id: i32,
) -> QueryResult<AdjustmentRun> {
    adjustment_runs::table
        .filter(adjustment_runs::id.eq(adjustment_run_id))
        .get_result::<AdjustmentRun>(conn)
}

pub fn find_project_by_adjustment_run_id(
    conn: &mut PgConnection,
    adjustment_run_id: i32,
) -> QueryResult<Project> {
    adjustment_runs::table
        .inner_join(projects::table)
        .filter(adjustment_runs::id.eq(adjustment_run_id))
        .select(projects::all_columns)
        .get_result::<Project>(conn)
}

pub fn find_adjustment_generation_by_id(
    conn: &mut PgConnection,
    adjustment_generation_id: i32,
) -> QueryResult<AdjustmentGeneration> {
    adjustment_generations::table
        .filter(adjustment_generations::id.eq(adjustment_generation_id))
        .get_result::<AdjustmentGeneration>(conn)
}

pub fn find_project_by_adjustment_generation_id(
    conn: &mut PgConnection,
    adjustment_generation_id: i32,
) -> QueryResult<Project> {
    adjustment_generations::table
        .inner_join(adjustment_runs::table.inner_join(projects::table))
        .filter(adjustment_generations::id.eq(adjustment_generation_id))
        .select(projects::all_columns)
        .get_result::<Project>(conn)
}

pub fn find_adjustment_individual_by_id(
    conn: &mut PgConnection,
    adjustment_individual_id: i32,
) -> QueryResult<AdjustmentIndividual> {
    adjustment_individuals::table
        .filter(adjustment_individuals::id.eq(adjustment_individual_id))
        .get_result::<AdjustmentIndividual>(conn)
}

pub fn find_project_by_adjustment_individual_id(
    conn: &mut PgConnection,
    adjustment_individual_id: i32,
) -> QueryResult<Project> {
    adjustment_individuals::table
        .inner_join(
            adjustment_generations::table
                .inner_join(adjustment_runs::table.inner_join(projects::table)),
        )
        .filter(adjustment_individuals::id.eq(adjustment_individual_id))
        .select(projects::all_columns)
        .get_result::<Project>(conn)
}

fn find_concept_values(
    conn: &mut PgConnection,
    individual_ids: &[i32],
) -> QueryResult<Vec<AdjustmentConceptValue>> {
    adjustment_concept_values::table
        .filter(adjustment_concept_values::adjustment_individual_id.eq_any(individual_ids))
        .get_results::<AdjustmentConceptValue>(conn)
}

fn find_connection_values(
    conn: &mut PgConnection,
    individual_ids: &[i32],
) -> QueryResult<Vec<AdjustmentConnectionValue>> {
    adjustment_connection_values::table
        .filter(adjustment_connection_values::adjustment_individual_id.eq_any(individual_ids))
        .get_results::<AdjustmentConnectionValue>(conn)
}

impl AdjustmentRunOutType {
    pub fn from_adjustment_run(
        conn: &mut PgConnection,
        adjustment_run: AdjustmentRun,
    ) -> ServiceResult<Self> {
        let result_individual = match adjustment_run.result_individual_id {
            Some(result_individual_id) => {
                let (id, number, time, error, generation_id, generation_number, generation_error) =
                    adjustment_individuals::table
                        .inner_join(adjustment_generations::table)
                        .filter(adjustment_individuals::id.eq(result_individual_id))
                        .select((
                            adjustment_individuals::id,
                            adjustment_individuals::number,
                            adjustment_individuals::time,
                            adjustment_individuals::error,
                            adjustment_generations::id,
                            adjustment_generations::number,
                            adjustment_generations::error,
                        ))
                        .get_result::<(i32, i32, i32, f64, i32, i32, f64)>(conn)
                        .to_service_result()?;
                let concept_values = Self::get_concept_values(conn, result_individual_id)?;
                let connection_values = Self::get_connection_values(conn, result_individual_id)?;
                Some(AdjustmentIndividualGenerationOutType {
                    id,
                    number,
                    time,
                    error,
                    generation_id,
                    generation_number,
                    generation_error,
                    concept_values,
                    connection_values,
                })
            }
            None => None,
        };
        Ok(Self::from((adjustment_run, result_individual)))
    }
    pub fn from_adjustment_runs(
        conn: &mut PgConnection,
        adjustment_runs: Vec<AdjustmentRun>,
    ) -> ServiceResult<Vec<Self>> {
        let individual_ids = adjustment_runs
            .iter()
            .filter_map(|ar| match ar.result_individual_id {
                Some(id) => Some(id),
                None => None,
            })
            .collect::<Vec<_>>();
        let mut result_individuals =
            Self::find_individuals(conn, &individual_ids).to_service_result()?;
        let mut concept_values = find_concept_values(conn, &individual_ids).to_service_result()?;
        let mut connection_values =
            find_connection_values(conn, &individual_ids).to_service_result()?;
        let mut result = vec![];
        for adjustment_run in adjustment_runs {
            let result_individual = match &adjustment_run.result_individual_id {
                Some(result_individual_id) => {
                    let result_individual_index = result_individuals
                        .iter()
                        .enumerate()
                        .find(|(_, rc)| rc.0 == *result_individual_id)
                        .unwrap()
                        .0;
                    let (
                        id,
                        number,
                        time,
                        error,
                        generation_id,
                        generation_number,
                        generation_error,
                    ) = result_individuals.remove(result_individual_index);
                    let concept_value_indices = concept_values
                        .iter()
                        .enumerate()
                        .filter(|(_, cv)| cv.adjustment_individual_id == *result_individual_id)
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>();
                    let mut concept_out_values = vec![];
                    for index in concept_value_indices.into_iter().rev() {
                        concept_out_values.push(AdjustmentConceptValueOutType::from(
                            concept_values.remove(index),
                        ));
                    }
                    let connection_value_indices = connection_values
                        .iter()
                        .enumerate()
                        .filter(|(_, cv)| cv.adjustment_individual_id == *result_individual_id)
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>();
                    let mut connection_out_values = vec![];
                    for index in connection_value_indices.into_iter().rev() {
                        connection_out_values.push(AdjustmentConnectionValueOutType::from(
                            connection_values.remove(index),
                        ));
                    }
                    Some(AdjustmentIndividualGenerationOutType {
                        id,
                        number,
                        time,
                        error,
                        generation_id,
                        generation_number,
                        generation_error,
                        concept_values: concept_out_values,
                        connection_values: connection_out_values,
                    })
                }
                None => None,
            };
            result.push(Self::from((adjustment_run, result_individual)))
        }
        Ok(result)
    }
    fn find_individuals(
        conn: &mut PgConnection,
        individual_ids: &[i32],
    ) -> QueryResult<Vec<(i32, i32, i32, f64, i32, i32, f64)>> {
        adjustment_individuals::table
            .inner_join(adjustment_generations::table)
            .filter(adjustment_individuals::id.eq_any(individual_ids))
            .select((
                adjustment_individuals::id,
                adjustment_individuals::number,
                adjustment_individuals::time,
                adjustment_individuals::error,
                adjustment_generations::id,
                adjustment_generations::number,
                adjustment_generations::error,
            ))
            .get_results::<(i32, i32, i32, f64, i32, i32, f64)>(conn)
    }
    fn get_concept_values(
        conn: &mut PgConnection,
        individual_id: i32,
    ) -> ServiceResult<Vec<AdjustmentConceptValueOutType>> {
        let concept_values = adjustment_concept_values::table
            .select((
                adjustment_concept_values::id,
                adjustment_concept_values::concept_id,
                adjustment_concept_values::value,
            ))
            .filter(adjustment_concept_values::adjustment_individual_id.eq(individual_id))
            .get_results::<(i32, i32, f64)>(conn)
            .to_service_result()?
            .into_iter()
            .map(|(id, concept_id, value)| AdjustmentConceptValueOutType {
                id,
                concept_id,
                value,
            })
            .collect::<Vec<_>>();
        Ok(concept_values)
    }
    fn get_connection_values(
        conn: &mut PgConnection,
        individual_id: i32,
    ) -> ServiceResult<Vec<AdjustmentConnectionValueOutType>> {
        let connection_values = adjustment_connection_values::table
            .select((
                adjustment_connection_values::id,
                adjustment_connection_values::connection_id,
                adjustment_connection_values::value,
            ))
            .filter(adjustment_connection_values::adjustment_individual_id.eq(individual_id))
            .get_results::<(i32, i32, f64)>(conn)
            .to_service_result()?
            .into_iter()
            .map(
                |(id, connection_id, value)| AdjustmentConnectionValueOutType {
                    id,
                    connection_id,
                    value,
                },
            )
            .collect::<Vec<_>>();
        Ok(connection_values)
    }
}

impl From<(AdjustmentRun, Option<AdjustmentIndividualGenerationOutType>)> for AdjustmentRunOutType {
    fn from(
        (adjustment_run, result_individual): (
            AdjustmentRun,
            Option<AdjustmentIndividualGenerationOutType>,
        ),
    ) -> Self {
        Self {
            id: adjustment_run.id,
            model_copy_id: adjustment_run.model_copy_id,
            name: adjustment_run.name,
            description: adjustment_run.description,
            min_model_time: adjustment_run.min_model_time,
            max_model_time: adjustment_run.max_model_time,
            dynamic_model_type: adjustment_run.dynamic_model_type,
            generation_size: adjustment_run.generation_size,
            generation_save_interval: adjustment_run.generation_save_interval,
            stop_condition: StopConditionType {
                max_generations: adjustment_run.max_generations,
                max_without_improvements: adjustment_run.max_without_improvements,
                error: adjustment_run.error,
            },
            created_at: adjustment_run.created_at,
            result_individual,
        }
    }
}

impl From<AdjustmentGeneration> for AdjustmentGenerationOutType {
    fn from(adjustment_generation: AdjustmentGeneration) -> Self {
        Self {
            id: adjustment_generation.id,
            number: adjustment_generation.number,
            error: adjustment_generation.error,
        }
    }
}

impl AdjustmentIndividualOutType {
    fn from_individual(
        conn: &mut PgConnection,
        adjustment_individual: AdjustmentIndividual,
    ) -> ServiceResult<Self> {
        let concept_values =
            find_concept_values(conn, &vec![adjustment_individual.id]).to_service_result()?;
        let connection_values =
            find_connection_values(conn, &vec![adjustment_individual.id]).to_service_result()?;
        Ok(Self {
            id: adjustment_individual.id,
            number: adjustment_individual.number,
            time: adjustment_individual.time,
            error: adjustment_individual.error,
            concept_values: concept_values
                .into_iter()
                .map(AdjustmentConceptValueOutType::from)
                .collect(),
            connection_values: connection_values
                .into_iter()
                .map(AdjustmentConnectionValueOutType::from)
                .collect(),
        })
    }
    fn from_individuals(
        conn: &mut PgConnection,
        adjustment_individuals: Vec<AdjustmentIndividual>,
    ) -> ServiceResult<Vec<Self>> {
        let individual_ids = adjustment_individuals
            .iter()
            .map(|ac| ac.id)
            .collect::<Vec<_>>();
        let mut concept_values = find_concept_values(conn, &individual_ids).to_service_result()?;
        let mut connection_values =
            find_connection_values(conn, &individual_ids).to_service_result()?;
        let mut result = vec![];
        for adjustment_individual in adjustment_individuals {
            let concept_value_indices = concept_values
                .iter()
                .enumerate()
                .filter(|(_, cv)| cv.adjustment_individual_id == adjustment_individual.id)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            let mut concept_out_values = vec![];
            for index in concept_value_indices.into_iter().rev() {
                concept_out_values.push(AdjustmentConceptValueOutType::from(
                    concept_values.remove(index),
                ));
            }
            let connection_value_indices = connection_values
                .iter()
                .enumerate()
                .filter(|(_, cv)| cv.adjustment_individual_id == adjustment_individual.id)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            let mut connection_out_values = vec![];
            for index in connection_value_indices.into_iter().rev() {
                connection_out_values.push(AdjustmentConnectionValueOutType::from(
                    connection_values.remove(index),
                ));
            }
            result.push(Self {
                id: adjustment_individual.id,
                number: adjustment_individual.number,
                time: adjustment_individual.time,
                error: adjustment_individual.error,
                concept_values: concept_out_values,
                connection_values: connection_out_values,
            })
        }
        Ok(result)
    }
}

impl From<AdjustmentConceptValue> for AdjustmentConceptValueOutType {
    fn from(adjustment_concept_value: AdjustmentConceptValue) -> Self {
        Self {
            id: adjustment_concept_value.id,
            concept_id: adjustment_concept_value.concept_id,
            value: adjustment_concept_value.value,
        }
    }
}

impl From<AdjustmentConnectionValue> for AdjustmentConnectionValueOutType {
    fn from(adjustment_connection_value: AdjustmentConnectionValue) -> Self {
        Self {
            id: adjustment_connection_value.id,
            connection_id: adjustment_connection_value.connection_id,
            value: adjustment_connection_value.value,
        }
    }
}
