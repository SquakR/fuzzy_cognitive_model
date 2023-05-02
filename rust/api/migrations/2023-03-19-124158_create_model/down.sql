-- This file should undo anything in `up.sql`
ALTER TABLE plugins DROP COLUMN connection_value_type;
ALTER TABLE plugins DROP COLUMN concept_value_type;
ALTER TABLE projects DROP COLUMN connection_value_type;
ALTER TABLE projects DROP COLUMN concept_value_type;
DROP TABLE connections;
DROP TYPE connection_value_type;
DROP TABLE concepts;
DROP TYPE concept_value_type;