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
CREATE TABLE user_projects (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id),
  project_id INTEGER NOT NULL,
  FOREIGN KEY (project_id) REFERENCES projects(id),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
CREATE TABLE permissions (
  key VARCHAR(255) PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT NOT NULL
);
CREATE TABLE project_permissions (
  id SERIAL PRIMARY KEY,
  permission_key VARCHAR(255) NOT NULL,
  FOREIGN KEY (permission_key) REFERENCES permissions(key),
  user_project_id INTEGER NOT NULL,
  FOREIGN KEY (user_project_id) REFERENCES user_projects(id)
);
SELECT diesel_manage_updated_at('projects');