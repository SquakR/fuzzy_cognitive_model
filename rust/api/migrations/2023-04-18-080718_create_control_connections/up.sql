-- Your SQL goes here
CREATE TABLE control_connections (
  connection_id INTEGER NOT NULL PRIMARY KEY,
  FOREIGN KEY (connection_id) REFERENCES connections(id) ON DELETE CASCADE,
  is_control BOOLEAN NOT NULL DEFAULT FALSE
);