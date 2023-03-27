-- Your SQL goes here
CREATE TYPE vertex_value_type AS ENUM ('none', 'from_zero_to_one');
CREATE TABLE vertices (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT NOT NULL,
  value DOUBLE PRECISION,
  project_id INTEGER NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  x_position DOUBLE PRECISION NOT NULL,
  y_position DOUBLE PRECISION NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
CREATE TYPE arc_value_type AS ENUM ('symbolic', 'from_minus_one_to_one');
CREATE TABLE arcs (
  id SERIAL PRIMARY KEY,
  description TEXT NOT NULL,
  value VARCHAR(255) NOT NULL,
  source_id INTEGER NOT NULL,
  FOREIGN KEY (source_id) REFERENCES vertices(id) ON DELETE CASCADE,
  target_id INTEGER NOT NULL,
  FOREIGN KEY (target_id) REFERENCES vertices(id) ON DELETE CASCADE,
  project_id INTEGER NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
ALTER TABLE projects
ADD COLUMN vertex_value_type vertex_value_type NOT NULL DEFAULT 'none';
ALTER TABLE projects
ADD COLUMN arc_value_type arc_value_type NOT NULL DEFAULT 'symbolic';
ALTER TABLE plugins
ADD COLUMN vertex_value_type vertex_value_type;
ALTER TABLE plugins
ADD COLUMN arc_value_type arc_value_type;
SELECT diesel_manage_updated_at('vertices');
SELECT diesel_manage_updated_at('arcs');