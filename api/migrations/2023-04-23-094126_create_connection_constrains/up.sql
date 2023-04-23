-- Your SQL goes here
CREATE TABLE connection_constraints (
  connection_id INTEGER NOT NULL PRIMARY KEY,
  FOREIGN KEY (connection_id) REFERENCES connections(id) ON DELETE CASCADE,
  has_constraint BOOLEAN NOT NULL DEFAULT FALSE,
  min_value DOUBLE PRECISION NOT NULL,
  include_min_value BOOLEAN NOT NULL,
  max_value DOUBLE PRECISION NOT NULL,
  include_max_value BOOLEAN NOT NULL
);