-- Your SQL goes here
CREATE TABLE target_concepts (
  concept_id INTEGER NOT NULL PRIMARY KEY,
  FOREIGN KEY (concept_id) REFERENCES concepts(id) ON DELETE CASCADE,
  is_target BOOLEAN NOT NULL DEFAULT FALSE,
  min_value DOUBLE PRECISION NOT NULL,
  include_min_value BOOLEAN NOT NULL,
  max_value DOUBLE PRECISION NOT NULL,
  include_max_value BOOLEAN NOT NULL
);