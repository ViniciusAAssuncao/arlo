UPDATE injury_definitions
SET relative_frequency = MIN(relative_frequency, 0.01)
WHERE code IN (
    'HEAD_CEREBRAL_CONTUSION',
    'HEAD_DIFFUSE_AXONAL_INJURY',
    'HEAD_SUBDURAL_HEMATOMA_ACUTE',
    'TRUNK_CARDIAC_CONTUSION',
    'TRUNK_HEMOTHORAX',
    'TRUNK_PULMONARY_CONTUSION'
);

UPDATE injury_definitions
SET relative_frequency = MIN(relative_frequency, 0.10)
WHERE code = 'HEAD_CONCUSSION_SEVERE';

UPDATE injury_recovery_profiles
SET minimum_days = CASE severity_grade WHEN 'Grade1' THEN 14 WHEN 'Grade2' THEN 45 ELSE 120 END,
    typical_days = CASE severity_grade WHEN 'Grade1' THEN 30 WHEN 'Grade2' THEN 90 ELSE 240 END,
    maximum_days = CASE severity_grade WHEN 'Grade1' THEN 60 WHEN 'Grade2' THEN 180 ELSE 450 END,
    mandatory_withdrawal = 1
WHERE injury_definition_id = (SELECT id FROM injury_definitions WHERE code = 'HEAD_CEREBRAL_CONTUSION');

UPDATE injury_recovery_profiles
SET minimum_days = CASE severity_grade WHEN 'Grade1' THEN 14 WHEN 'Grade2' THEN 30 ELSE 90 END,
    typical_days = CASE severity_grade WHEN 'Grade1' THEN 30 WHEN 'Grade2' THEN 60 ELSE 180 END,
    maximum_days = CASE severity_grade WHEN 'Grade1' THEN 60 WHEN 'Grade2' THEN 120 ELSE 365 END,
    mandatory_withdrawal = 1
WHERE injury_definition_id = (SELECT id FROM injury_definitions WHERE code = 'TRUNK_CARDIAC_CONTUSION');

UPDATE injury_recovery_profiles
SET minimum_days = CASE severity_grade WHEN 'Grade1' THEN 7 WHEN 'Grade2' THEN 21 ELSE 60 END,
    typical_days = CASE severity_grade WHEN 'Grade1' THEN 21 WHEN 'Grade2' THEN 45 ELSE 120 END,
    maximum_days = CASE severity_grade WHEN 'Grade1' THEN 45 WHEN 'Grade2' THEN 90 ELSE 240 END,
    mandatory_withdrawal = 1
WHERE injury_definition_id = (SELECT id FROM injury_definitions WHERE code = 'TRUNK_PULMONARY_CONTUSION');
