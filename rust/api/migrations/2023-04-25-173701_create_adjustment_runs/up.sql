-- Your SQL goes here
CREATE TABLE adjustment_runs (
  id SERIAL PRIMARY KEY,
  project_id INTEGER NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  model_copy_id INTEGER NOT NULL,
  FOREIGN KEY (model_copy_id) REFERENCES model_copies(id) ON DELETE CASCADE,
  name VARCHAR(255) NOT NULL,
  description TEXT NOT NULL,
  min_model_time INTEGER NOT NULL,
  max_model_time INTEGER NOT NULL,
  dynamic_model_type dynamic_model_type NOT NULL,
  generation_size INTEGER NOT NULL,
  generation_save_interval INTEGER NOT NULL,
  max_generations INTEGER NOT NULL,
  max_without_improvements INTEGER NOT NULL,
  error DOUBLE PRECISION NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
CREATE TABLE adjustment_generations (
  id SERIAL PRIMARY KEY,
  adjustment_run_id INTEGER NOT NULL,
  FOREIGN KEY (adjustment_run_id) REFERENCES adjustment_runs(id) ON DELETE CASCADE,
  number INTEGER NOT NULL,
  error DOUBLE PRECISION NOT NULL
);
CREATE TABLE adjustment_chromosomes (
  id SERIAL PRIMARY KEY,
  adjustment_generation_id INTEGER NOT NULL,
  FOREIGN KEY (adjustment_generation_id) REFERENCES adjustment_generations(id) ON DELETE CASCADE,
  number INTEGER NOT NULL,
  time INTEGER NOT NULL,
  error DOUBLE PRECISION NOT NULL
);
CREATE TABLE adjustment_concept_values (
  id SERIAL PRIMARY KEY,
  adjustment_chromosome_id INTEGER NOT NULL,
  FOREIGN KEY (adjustment_chromosome_id) REFERENCES adjustment_chromosomes(id) ON DELETE CASCADE,
  concept_id INTEGER NOT NULL,
  FOREIGN KEY (concept_id) REFERENCES concepts(id) ON DELETE CASCADE,
  value DOUBLE PRECISION NOT NULL
);
CREATE TABLE adjustment_connection_values (
  id SERIAL PRIMARY KEY,
  adjustment_chromosome_id INTEGER NOT NULL,
  FOREIGN KEY (adjustment_chromosome_id) REFERENCES adjustment_chromosomes(id) ON DELETE CASCADE,
  connection_id INTEGER NOT NULL,
  FOREIGN KEY (connection_id) REFERENCES connections(id) ON DELETE CASCADE,
  value DOUBLE PRECISION NOT NULL
);
ALTER TABLE adjustment_runs
ADD COLUMN result_chromosome_id INTEGER DEFAULT NULL;
ALTER TABLE adjustment_runs
ADD FOREIGN KEY (result_chromosome_id) REFERENCES adjustment_chromosomes(id) ON DELETE
SET NULL;