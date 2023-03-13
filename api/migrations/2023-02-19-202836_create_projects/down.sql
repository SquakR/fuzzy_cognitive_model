-- This file should undo anything in `up.sql`
DROP TABLE project_plugins;
DROP TABLE plugins;
DROP TABLE project_user_permissions;
DROP TABLE permissions;
ALTER TABLE project_users DROP COLUMN last_status_id;
DROP TABLE project_user_statuses;
DROP TYPE project_user_status_value;
DROP TABLE project_users;
DROP TABLE projects;