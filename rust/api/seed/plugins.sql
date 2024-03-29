INSERT INTO plugins (
    name,
    description,
    concept_value_type,
    connection_value_type
  )
VALUES (
    'Control Concepts',
    'The plugin adds the ability to select control concepts.',
    null,
    null
  );
INSERT INTO plugins (
    name,
    description,
    concept_value_type,
    connection_value_type
  )
VALUES (
    'Target Concepts',
    'The plugin adds the ability to select target concepts.',
    'from_zero_to_one',
    null
  );
INSERT INTO plugins (
    name,
    description,
    concept_value_type,
    connection_value_type
  )
VALUES (
    'Control Connections',
    'The plugin adds the ability to select control connections.',
    null,
    null
  );
INSERT INTO plugins (
    name,
    description,
    concept_value_type,
    connection_value_type
  )
VALUES (
    'Concept Constraints',
    'The plugin adds the ability to select concept constraints.',
    'from_zero_to_one',
    null
  );
INSERT INTO plugins (
    name,
    description,
    concept_value_type,
    connection_value_type
  )
VALUES (
    'Connection Constraints',
    'The plugin adds the ability to select connection constraints.',
    null,
    'from_minus_one_to_one'
  );
INSERT INTO plugins (
    name,
    description,
    concept_value_type,
    connection_value_type
  )
VALUES (
    'Adjustment With Genetic Algorithms',
    'Structural and parametric adjustment of fuzzy cognitive models based on genetic algorithms.',
    'from_zero_to_one',
    'from_minus_one_to_one'
  );