-- Your SQL goes here
CREATE TYPE dynamic_model_type AS ENUM (
  'delta_delta',
  'delta_value',
  'value_delta',
  'value_value'
);
CREATE TABLE concept_dynamic_models (
  concept_id INTEGER NOT NULL PRIMARY KEY,
  FOREIGN KEY (concept_id) REFERENCES concepts(id) ON DELETE CASCADE,
  dynamic_model_type dynamic_model_type DEFAULT NULL
);