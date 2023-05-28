use super::super::models::AdjustmentGeneration;
use super::super::types::AdjustmentRunOutType;
use super::adjustment_out_services;
use crate::plugins::adjustment::models::{AdjustmentChromosome, AdjustmentRun};
use crate::plugins::adjustment::types::{AdjustmentGenerationOutType, AdjustmentRunActionType};
use crate::response::{AppError, ServiceResult, ToServiceResult};
use crate::schema::{
    adjustment_chromosomes, adjustment_concept_values, adjustment_connection_values,
    adjustment_generations, adjustment_runs,
};
use crate::web_socket::WebSocketAdjustmentRunService;
use diesel::prelude::*;
use diesel::{Connection, PgConnection};
use fuzzy_cognitive_model_common::adjustment::{Chromosome, Generation, SaveResult};

pub struct SaveResultServer {
    pub conn: PgConnection,
    pub adjustment_run_id: i32,
    pub adjustment_run_service: WebSocketAdjustmentRunService,
}

#[rocket::async_trait]
impl SaveResult<(), AppError> for SaveResultServer {
    async fn save_result(&mut self, result_chromosome: &Chromosome) -> ServiceResult<()> {
        let project = adjustment_out_services::find_project_by_adjustment_run_id(
            &mut self.conn,
            self.adjustment_run_id,
        )
        .to_service_result_find(String::from("adjustment_run_not_found_error"))?;
        let adjustment_run = diesel::update(adjustment_runs::table)
            .filter(adjustment_runs::id.eq(self.adjustment_run_id))
            .set(adjustment_runs::result_chromosome_id.eq(result_chromosome.id.unwrap()))
            .get_result::<AdjustmentRun>(&mut self.conn)
            .to_service_result()?;
        let adjustment_run_out =
            AdjustmentRunOutType::from_adjustment_run(&mut self.conn, adjustment_run)?;
        let adjustment_run_action = AdjustmentRunActionType::new(
            project.id,
            self.adjustment_run_id,
            String::from("adjustmentResult"),
            adjustment_run_out,
        );
        self.adjustment_run_service
            .notify(adjustment_run_action.clone())
            .await;
        Ok(())
    }
    async fn save_generation(
        &mut self,
        generation: &mut Generation,
        number: i32,
    ) -> ServiceResult<()> {
        let project = adjustment_out_services::find_project_by_adjustment_run_id(
            &mut self.conn,
            self.adjustment_run_id,
        )
        .to_service_result_find(String::from("adjustment_run_not_found_error"))?;
        let adjustment_generation = self
            .conn
            .transaction(|conn| {
                let adjustment_generation = diesel::insert_into(adjustment_generations::table)
                    .values((
                        adjustment_generations::adjustment_run_id.eq(self.adjustment_run_id),
                        adjustment_generations::number.eq(number),
                        adjustment_generations::fitness.eq(generation.fitness),
                    ))
                    .get_result::<AdjustmentGeneration>(conn)?;
                for (i, chromosome) in generation.chromosomes.iter_mut().enumerate() {
                    let adjustment_chromosome = diesel::insert_into(adjustment_chromosomes::table)
                        .values((
                            adjustment_chromosomes::adjustment_generation_id
                                .eq(adjustment_generation.id),
                            adjustment_chromosomes::number.eq(i as i32 + 1),
                            adjustment_chromosomes::fitness.eq(chromosome.fitness),
                        ))
                        .get_result::<AdjustmentChromosome>(conn)?;
                    chromosome.id = Some(adjustment_chromosome.id);
                    for (concept_id, value) in &chromosome.concepts {
                        diesel::insert_into(adjustment_concept_values::table)
                            .values((
                                adjustment_concept_values::adjustment_chromosome_id
                                    .eq(adjustment_chromosome.id),
                                adjustment_concept_values::concept_id.eq(concept_id),
                                adjustment_concept_values::value.eq(value),
                            ))
                            .execute(conn)?;
                    }
                    for (connection_id, value) in &chromosome.connections {
                        diesel::insert_into(adjustment_connection_values::table)
                            .values((
                                adjustment_connection_values::adjustment_chromosome_id
                                    .eq(adjustment_chromosome.id),
                                adjustment_connection_values::connection_id.eq(connection_id),
                                adjustment_connection_values::value.eq(value),
                            ))
                            .execute(conn)?;
                    }
                }
                Ok(adjustment_generation)
            })
            .to_service_result()?;
        let adjustment_run_action = AdjustmentRunActionType::new(
            project.id,
            self.adjustment_run_id,
            String::from("adjustmentGeneration"),
            AdjustmentGenerationOutType::from(adjustment_generation),
        );
        self.adjustment_run_service
            .notify(adjustment_run_action.clone())
            .await;
        Ok(())
    }
}
