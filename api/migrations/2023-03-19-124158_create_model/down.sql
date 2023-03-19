-- This file should undo anything in `up.sql`
ALTER TABLE plugins DROP COLUMN arc_value_type;
ALTER TABLE plugins DROP COLUMN node_value_type;
ALTER TABLE projects DROP COLUMN arc_value_type;
ALTER TABLE projects DROP COLUMN node_value_type;
DROP TABLE arcs;
DROP TYPE arc_value_type;
DROP TABLE nodes;
DROP TYPE node_value_type;