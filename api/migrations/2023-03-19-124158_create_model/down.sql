-- This file should undo anything in `up.sql`
ALTER TABLE plugins DROP COLUMN arc_value_type;
ALTER TABLE plugins DROP COLUMN vertex_value_type;
ALTER TABLE projects DROP COLUMN arc_value_type;
ALTER TABLE projects DROP COLUMN vertex_value_type;
DROP TABLE arcs;
DROP TYPE arc_value_type;
DROP TABLE vertices;
DROP TYPE vertex_value_type;