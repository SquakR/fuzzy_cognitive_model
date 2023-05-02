-- Your SQL goes here
CREATE TABLE control_concepts (
  concept_id INTEGER NOT NULL PRIMARY KEY,
  FOREIGN KEY (concept_id) REFERENCES concepts(id) ON DELETE CASCADE,
  is_control BOOLEAN NOT NULL DEFAULT FALSE
);