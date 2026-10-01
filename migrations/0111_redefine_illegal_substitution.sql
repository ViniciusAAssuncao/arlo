UPDATE fault_definitions
SET description = 'Designação de mais atletas do que o limite permitido para uma posição obrigatória ou função tática exclusiva',
    severity = 'Flagrant'
WHERE code = 'illegal_substitution';

DELETE FROM fault_punishment_options
WHERE fault_definition_id = (SELECT id FROM fault_definitions WHERE code = 'illegal_substitution');

INSERT INTO fault_punishment_options (id, fault_definition_id, kind, magnitude_min, magnitude_max)
SELECT '41595b07-a744-48f0-8c3c-ef07dacc37cc', id, 'Expulsion', NULL, NULL
FROM fault_definitions
WHERE code = 'illegal_substitution';
