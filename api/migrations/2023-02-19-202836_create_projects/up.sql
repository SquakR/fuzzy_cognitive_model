-- Your SQL goes here
CREATE TABLE projects (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT NOT NULL,
  is_public BOOLEAN NOT NULL DEFAULT FALSE,
  is_archived BOOLEAN NOT NULL DEFAULT TRUE,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
CREATE TABLE project_users (
  id SERIAL PRIMARY KEY,
  project_id INTEGER NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  user_id INTEGER NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
CREATE TYPE project_user_status_value AS ENUM (
  'creator',
  'invited',
  'cancelled',
  'rejected',
  'member',
  'excluded',
  'left'
);
CREATE TABLE project_user_statuses (
  id SERIAL PRIMARY KEY,
  project_user_id INTEGER NOT NULL,
  FOREIGN KEY (project_user_id) REFERENCES project_users(id) ON DELETE CASCADE,
  status project_user_status_value NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
ALTER TABLE project_users
ADD last_status_id INTEGER;
ALTER TABLE project_users
ADD FOREIGN KEY (last_status_id) REFERENCES project_user_statuses(id) ON DELETE
SET NULL;
CREATE TABLE permissions (
  key VARCHAR(255) PRIMARY KEY,
  description TEXT NOT NULL
);
CREATE TABLE project_user_permissions (
  id SERIAL PRIMARY KEY,
  permission_key VARCHAR(255) NOT NULL,
  FOREIGN KEY (permission_key) REFERENCES permissions(key) ON DELETE CASCADE,
  project_user_id INTEGER NOT NULL,
  FOREIGN KEY (project_user_id) REFERENCES project_users(id) ON DELETE CASCADE
);
CREATE TABLE plugins (
  name VARCHAR(255) NOT NULL PRIMARY KEY,
  description TEXT NOT NULL
);
CREATE TABLE project_plugins (
  id SERIAL PRIMARY KEY,
  project_id INTEGER NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
  plugin_name VARCHAR(255) NOT NULL,
  FOREIGN KEY (plugin_name) REFERENCES plugins(name) ON DELETE CASCADE,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
SELECT diesel_manage_updated_at('projects');