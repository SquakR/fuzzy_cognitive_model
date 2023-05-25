use super::models::DynamicModelType;
use super::services::{
    adjustment_out_services, adjustment_services, concept_dynamic_model_services,
};
use super::types::{
    AdjustmentChromosomeOutType, AdjustmentGenerationOutType, AdjustmentInType,
    AdjustmentRunActionType, AdjustmentRunOutType, AdjustmentRunsInType,
    ConceptDynamicModelOutType,
};
use crate::db;
use crate::locale::Locale;
use crate::models::User;
use crate::plugins::Plugins;
use crate::response::{PathResult, ToPathResult};
use crate::types::{IntervalInType, ModelActionType, PaginationInType, PaginationOutType};
use crate::web_socket::{WebSocketAdjustmentRunService, WebSocketModelService};
use rocket::serde::json::Json;
use rocket_okapi::openapi;

/// Change concept concept dynamic model type
#[openapi(tag = "adjustment")]
#[patch(
    "/concepts/<concept_id>/change_dynamic_model_type",
    format = "json",
    data = "<dynamic_model_type>"
)]
pub async fn change_dynamic_model_type(
    concept_id: i32,
    dynamic_model_type: Json<Option<DynamicModelType>>,
    user: User,
    model_service: WebSocketModelService,
) -> PathResult<ModelActionType<ConceptDynamicModelOutType>> {
    let conn = &mut db::establish_connection();
    concept_dynamic_model_services::change_dynamic_model_type(
        conn,
        model_service,
        &user,
        concept_id,
        dynamic_model_type.into_inner(),
    )
    .await
    .to_path_result()
}

/// Run genetic algorithm for the structural-parametric adjustment of fuzzy cognitive model
#[openapi(tag = "adjustment")]
#[post(
    "/projects/<project_id>/adjust",
    format = "json",
    data = "<adjustment_in>"
)]
pub async fn adjust(
    project_id: i32,
    adjustment_in: Json<AdjustmentInType>,
    user: User,
    locale: &Locale,
    plugins: &Plugins,
    adjustment_run_service: WebSocketAdjustmentRunService,
) -> PathResult<AdjustmentRunActionType<AdjustmentRunOutType>> {
    let conn = db::establish_connection();
    adjustment_services::adjust(
        conn,
        plugins,
        adjustment_run_service,
        &user,
        locale,
        project_id,
        adjustment_in.into_inner(),
    )
    .await
    .to_path_result()
}

/// Get adjustment runs
#[openapi(tag = "adjustment")]
#[get("/project/<project_id>/adjustment_runs?<adjustment_runs_in..>")]
pub fn get_adjustment_runs(
    project_id: i32,
    adjustment_runs_in: AdjustmentRunsInType,
    user: User,
) -> PathResult<PaginationOutType<AdjustmentRunOutType>> {
    let conn = &mut db::establish_connection();
    let created_at = if adjustment_runs_in.created_at_start.is_some()
        || adjustment_runs_in.created_at_end.is_some()
    {
        Some(IntervalInType {
            start: adjustment_runs_in.created_at_start.map(|c| c.0),
            include_start: adjustment_runs_in.created_at_include_start.unwrap_or(true),
            end: adjustment_runs_in.created_at_end.map(|c| c.0),
            include_end: adjustment_runs_in.created_at_include_end.unwrap_or(true),
        })
    } else {
        None
    };
    let pagination = PaginationInType {
        page: adjustment_runs_in.page.unwrap_or(1),
        per_page: adjustment_runs_in.per_page.unwrap_or(15),
    };
    adjustment_out_services::paginate_adjustment_runs(
        conn,
        &user,
        project_id,
        adjustment_runs_in.search,
        created_at,
        pagination,
    )
    .to_path_result()
}

/// Get adjustment generations
#[openapi(tag = "adjustment")]
#[get("/adjustment_runs/<adjustment_run_id>/adjustment_generations?<page>&<per_page>")]
pub fn get_adjustment_generations(
    adjustment_run_id: i32,
    page: Option<u16>,
    per_page: Option<u16>,
    user: User,
) -> PathResult<PaginationOutType<AdjustmentGenerationOutType>> {
    let conn = &mut db::establish_connection();
    let pagination_in = PaginationInType {
        page: page.unwrap_or(1),
        per_page: per_page.unwrap_or(15),
    };
    adjustment_out_services::paginate_adjustment_generations(
        conn,
        &user,
        adjustment_run_id,
        pagination_in,
    )
    .to_path_result()
}

/// Get adjustment chromosomes
#[openapi(tag = "adjustment")]
#[get(
    "/adjustment_generations/<adjustment_generation_id>/adjustment_chromosomes?<page>&<per_page>"
)]
pub fn get_adjustment_chromosomes(
    adjustment_generation_id: i32,
    page: Option<u16>,
    per_page: Option<u16>,
    user: User,
) -> PathResult<PaginationOutType<AdjustmentChromosomeOutType>> {
    let conn = &mut db::establish_connection();
    let pagination_in = PaginationInType {
        page: page.unwrap_or(1),
        per_page: per_page.unwrap_or(15),
    };
    adjustment_out_services::paginate_adjustment_chromosomes(
        conn,
        &user,
        adjustment_generation_id,
        pagination_in,
    )
    .to_path_result()
}
