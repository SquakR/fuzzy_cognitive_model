-- This file should undo anything in `up.sql`
ALTER TABLE adjustment_runs DROP COLUMN result_individual_id;
DROP TABLE adjustment_connection_values;
DROP TABLE adjustment_concept_values;
DROP TABLE adjustment_individuals;
DROP TABLE adjustment_generations;
DROP TABLE adjustment_runs;