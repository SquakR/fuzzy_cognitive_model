use super::super::models::AdjustmentRun;
use super::super::types::{
    AdjustmentConceptValueOutType, AdjustmentConnectionValueOutType, AdjustmentRunOutType,
    StopConditionType,
};
use crate::plugins::adjustment::types::AdjustmentChromosomeGenerationOutType;
use crate::response::{ServiceResult, ToServiceResult};
use crate::schema::{
    adjustment_chromosomes, adjustment_concept_values, adjustment_connection_values,
    adjustment_generations,
};
use diesel::prelude::*;
use diesel::PgConnection;

impl AdjustmentRunOutType {
    pub fn from_adjustment(
        conn: &mut PgConnection,
        adjustment_run: AdjustmentRun,
    ) -> ServiceResult<Self> {
        let result_chromosome = match adjustment_run.result_chromosome_id {
            Some(result_chromosome_id) => {
                let (id, fitness, generation_id, generation_number, generation_fitness) =
                    adjustment_chromosomes::table
                        .inner_join(adjustment_generations::table)
                        .filter(adjustment_chromosomes::id.eq(result_chromosome_id))
                        .select((
                            adjustment_chromosomes::id,
                            adjustment_chromosomes::fitness,
                            adjustment_generations::id,
                            adjustment_generations::number,
                            adjustment_generations::fitness,
                        ))
                        .get_result::<(i32, f64, i32, i32, f64)>(conn)
                        .to_service_result()?;
                let concept_values =
                    AdjustmentRunOutType::get_concept_values(conn, result_chromosome_id)?;
                let connection_values =
                    AdjustmentRunOutType::get_connection_values(conn, result_chromosome_id)?;
                Some(AdjustmentChromosomeGenerationOutType {
                    id,
                    fitness,
                    generation_id,
                    generation_number,
                    generation_fitness,
                    concept_values,
                    connection_values,
                })
            }
            None => None,
        };
        Ok(AdjustmentRunOutType {
            id: adjustment_run.id,
            model_copy_id: adjustment_run.model_copy_id,
            name: adjustment_run.name,
            description: adjustment_run.description,
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
            result_chromosome,
        })
    }
    fn get_concept_values(
        conn: &mut PgConnection,
        chromosome_id: i32,
    ) -> ServiceResult<Vec<AdjustmentConceptValueOutType>> {
        let concept_values = adjustment_concept_values::table
            .select((
                adjustment_concept_values::id,
                adjustment_concept_values::concept_id,
                adjustment_concept_values::value,
            ))
            .filter(adjustment_concept_values::adjustment_chromosome_id.eq(chromosome_id))
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
        chromosome_id: i32,
    ) -> ServiceResult<Vec<AdjustmentConnectionValueOutType>> {
        let connection_values = adjustment_connection_values::table
            .select((
                adjustment_connection_values::id,
                adjustment_connection_values::connection_id,
                adjustment_connection_values::value,
            ))
            .filter(adjustment_connection_values::adjustment_chromosome_id.eq(chromosome_id))
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
