-- Your SQL goes here
CREATE TABLE control_vertices (
  vertex_id INTEGER NOT NULL PRIMARY KEY,
  FOREIGN KEY (vertex_id) REFERENCES vertices(id) ON DELETE CASCADE,
  is_control BOOLEAN NOT NULL DEFAULT FALSE
);