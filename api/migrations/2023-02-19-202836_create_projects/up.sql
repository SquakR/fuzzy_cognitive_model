-- Your SQL goes here
CREATE TABLE projects (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT NOT NULL,
  created_by_id INTEGER NOT NULL,
  FOREIGN KEY (created_by_id) REFERENCES users(id),
  is_public BOOLEAN NOT NULL DEFAULT FALSE,
  is_archived BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
CREATE TABLE project_users (
  id SERIAL PRIMARY KEY,
  project_id INTEGER NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id),
  user_id INTEGER NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id),
  is_confirmed BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
CREATE TABLE permissions (
  key VARCHAR(255) PRIMARY KEY,
  description TEXT NOT NULL
);
CREATE TABLE user_permissions (
  id SERIAL PRIMARY KEY,
  permission_key VARCHAR(255) NOT NULL,
  FOREIGN KEY (permission_key) REFERENCES permissions(key),
  project_user_id INTEGER NOT NULL,
  FOREIGN KEY (project_user_id) REFERENCES project_users(id)
);
CREATE TABLE plugins (
  name VARCHAR(255) NOT NULL PRIMARY KEY,
  description TEXT NOT NULL
);
CREATE TABLE project_plugins (
  id SERIAL PRIMARY KEY,
  project_id INTEGER NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id),
  plugin_name VARCHAR(255) NOT NULL,
  FOREIGN KEY (plugin_name) REFERENCES plugins(name),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
SELECT diesel_manage_updated_at('projects');