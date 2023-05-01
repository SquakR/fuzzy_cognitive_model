use super::super::models::AdjustmentGeneration;
use super::super::types::AdjustmentRunOutType;
use super::adjustment_run_services::{Chromosome, Generation};
use crate::plugins::adjustment::models::{AdjustmentChromosome, AdjustmentRun};
use crate::schema::{
    adjustment_chromosomes, adjustment_concept_values, adjustment_connection_values,
    adjustment_generations, adjustment_runs,
};
use crate::services::project_services;
use crate::types::ModelActionType;
use crate::web_socket::WebSocketProjectService;
use diesel::prelude::*;
use diesel::{Connection, PgConnection};

#[rocket::async_trait]
pub trait SaveResult {
    async fn save_result(&mut self, result_chromosome: &Chromosome) -> ();
    async fn save_generation(&mut self, generation: &mut Generation, number: i32) -> ();
}

pub struct SaveResultServer {
    pub conn: PgConnection,
    pub project_id: i32,
    pub adjustment_run_id: i32,
    pub project_service: WebSocketProjectService,
}

#[rocket::async_trait]
impl SaveResult for SaveResultServer {
    async fn save_result(&mut self, result_chromosome: &Chromosome) -> () {
        let project =
            project_services::find_project_by_id(&mut self.conn, self.project_id).unwrap();
        let adjustment_run = diesel::update(adjustment_runs::table)
            .filter(adjustment_runs::id.eq(self.adjustment_run_id))
            .set(adjustment_runs::result_chromosome_id.eq(result_chromosome.id.unwrap()))
            .get_result::<AdjustmentRun>(&mut self.conn)
            .unwrap();
        let adjustment_run_out =
            match AdjustmentRunOutType::from_adjustment_run(&mut self.conn, adjustment_run) {
                Ok(adjustment_run_out) => adjustment_run_out,
                Err(_) => unreachable!(),
            };
        let model_action =
            ModelActionType::new(&project, String::from("adjust_result"), adjustment_run_out);
        self.project_service.notify(model_action.clone()).await;
    }
    async fn save_generation(&mut self, generation: &mut Generation, number: i32) -> () {
        self.conn
            .transaction::<(), diesel::result::Error, _>(|conn| {
                let adjustment_generation = diesel::insert_into(adjustment_generations::table)
                    .values((
                        adjustment_generations::adjustment_run_id.eq(self.adjustment_run_id),
                        adjustment_generations::number.eq(number),
                        adjustment_generations::fitness.eq(generation.fitness),
                    ))
                    .get_result::<AdjustmentGeneration>(conn)?;
                for chromosome in &mut generation.chromosomes {
                    let adjustment_chromosome = diesel::insert_into(adjustment_chromosomes::table)
                        .values((
                            adjustment_chromosomes::adjustment_generation_id
                                .eq(adjustment_generation.id),
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
                Ok(())
            })
            .unwrap();
    }
}
