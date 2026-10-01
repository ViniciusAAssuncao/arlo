WITH adjustments(code, grade, minimum_days, typical_days, maximum_days) AS (
    VALUES
        ('TRUNK_RIB_FRACTURE_SINGLE', 'Grade1', 35, 50, 70),
        ('TRUNK_RIB_FRACTURE_SINGLE', 'Grade2', 42, 70, 100),
        ('TRUNK_RIB_FRACTURE_SINGLE', 'Grade3', 70, 105, 150),
        ('TRUNK_STERNAL_FRACTURE', 'Grade1', 42, 60, 90),
        ('TRUNK_STERNAL_FRACTURE', 'Grade2', 65, 100, 140),
        ('TRUNK_STERNAL_FRACTURE', 'Grade3', 100, 150, 220),
        ('SHOULDER_CLAVICLE_FRACTURE', 'Grade1', 42, 65, 95),
        ('SHOULDER_CLAVICLE_FRACTURE', 'Grade2', 70, 105, 150),
        ('SHOULDER_CLAVICLE_FRACTURE', 'Grade3', 105, 160, 240)
)
UPDATE injury_recovery_profiles
SET minimum_days = adjustments.minimum_days,
    typical_days = adjustments.typical_days,
    maximum_days = adjustments.maximum_days
FROM adjustments, injury_definitions
WHERE injury_definitions.id = injury_recovery_profiles.injury_definition_id
  AND injury_definitions.code = adjustments.code
  AND injury_recovery_profiles.severity_grade = adjustments.grade;
