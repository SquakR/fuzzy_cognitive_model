-- Your SQL goes here
CREATE TABLE model_copies (
  id SERIAL PRIMARY KEY,
  project_id INTEGER NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  model jsonb NOT NULL
);