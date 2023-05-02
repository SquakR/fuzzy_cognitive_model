-- Your SQL goes here
CREATE TABLE plugin_dependencies (
  id SERIAL PRIMARY KEY,
  dependent_plugin_name VARCHAR(255) NOT NULL,
  FOREIGN KEY (dependent_plugin_name) REFERENCES plugins(name) ON DELETE CASCADE,
  dependency_plugin_name VARCHAR(255) NOT NULL,
  FOREIGN KEY (dependency_plugin_name) REFERENCES plugins(name) ON DELETE CASCADE
);