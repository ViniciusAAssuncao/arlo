INSERT OR IGNORE INTO outside_match_injury_definitions
    (injury_definition_id, daily_weight, severity_grade)
SELECT id, 0, 'Grade1'
FROM injury_definitions
WHERE code IN ('GROIN_INGUINAL_HERNIA', 'GROIN_SPORTS_HERNIA');

INSERT INTO match_injury_grade_floors (injury_definition_id, minimum_grade)
SELECT id, 'Grade2'
FROM injury_definitions
WHERE code IN (
    'HEAD_SUBDURAL_HEMATOMA_ACUTE',
    'NECK_CERVICAL_DISC_HERNIATION_TRAUMATIC',
    'TRUNK_LUMBAR_DISC_HERNIATION'
)
ON CONFLICT(injury_definition_id) DO UPDATE SET minimum_grade = excluded.minimum_grade;

WITH profiles(code, grade, minimum_days, typical_days, maximum_days, mandatory_withdrawal) AS (
    VALUES
        ('ANKLE_MEDIAL_SPRAIN_CONTACT', 'Grade1', 7, 14, 28, 0),
        ('CALF_CALF_CRAMPS', 'Grade1', 1, 2, 4, 0),
        ('CALF_CALF_HEMATOMA', 'Grade2', 14, 28, 49, 0),
        ('CALF_CALF_STRAIN_TENNIS_LEG', 'Grade2', 21, 42, 70, 0),
        ('FOOT_FOOT_CRAMPS', 'Grade1', 1, 2, 4, 0),
        ('FOOT_TURF_TOE_TRAUMATIC', 'Grade2', 28, 56, 90, 0),
        ('GROIN_INGUINAL_HERNIA', 'Grade1', 14, 30, 60, 0),
        ('GROIN_SPORTS_HERNIA', 'Grade1', 21, 45, 90, 0),
        ('HAND_WRIST_SPRAIN', 'Grade1', 5, 10, 21, 0),
        ('HEAD_DENTAL_AVULSION', 'Grade1', 7, 14, 28, 1),
        ('HEAD_SUBDURAL_HEMATOMA_ACUTE', 'Grade1', 21, 45, 90, 1),
        ('HEAD_SUBDURAL_HEMATOMA_ACUTE', 'Grade2', 45, 90, 180, 1),
        ('HEAD_SUBDURAL_HEMATOMA_ACUTE', 'Grade3', 120, 240, 450, 1),
        ('HEAD_TONGUE_LACERATION', 'Grade1', 3, 7, 14, 1),
        ('HIP_FLEXOR_STRAIN', 'Grade1', 7, 14, 28, 0),
        ('KNEE_EFFUSION', 'Grade1', 5, 14, 28, 0),
        ('KNEE_MENISCUS_TEAR', 'Grade1', 21, 42, 70, 0),
        ('KNEE_MENISCUS_TEAR', 'Grade2', 42, 84, 150, 0),
        ('NECK_CERVICAL_DISC_HERNIATION_TRAUMATIC', 'Grade1', 21, 45, 90, 1),
        ('NECK_CERVICAL_DISC_HERNIATION_TRAUMATIC', 'Grade2', 45, 90, 180, 1),
        ('NECK_CERVICAL_DISC_HERNIATION_TRAUMATIC', 'Grade3', 120, 210, 365, 1),
        ('NECK_CERVICAL_STRAIN_WHIPLASH', 'Grade2', 21, 42, 70, 0),
        ('NECK_STINGER_BURNER', 'Grade1', 1, 3, 10, 1),
        ('SHOULDER_BANKART_LESION', 'Grade2', 60, 120, 210, 0),
        ('SHOULDER_SLAP_TEAR', 'Grade2', 45, 90, 180, 0),
        ('SHOULDER_SUBLUXATION', 'Grade2', 28, 56, 100, 0),
        ('THIGH_ADDUCTOR_STRAIN_THIGH', 'Grade1', 7, 14, 28, 0),
        ('THIGH_MUSCLE_CRAMPS', 'Grade1', 1, 2, 4, 0),
        ('TRUNK_ABDOMINAL_MUSCLE_STRAIN', 'Grade1', 7, 14, 28, 0),
        ('TRUNK_HEMOTHORAX', 'Grade2', 30, 60, 120, 1),
        ('TRUNK_HEMOTHORAX', 'Grade3', 90, 180, 300, 1),
        ('TRUNK_LUMBAR_DISC_HERNIATION', 'Grade1', 21, 45, 90, 1),
        ('TRUNK_LUMBAR_DISC_HERNIATION', 'Grade2', 30, 75, 150, 1),
        ('TRUNK_LUMBAR_DISC_HERNIATION', 'Grade3', 90, 180, 365, 1),
        ('TRUNK_RECTUS_ABDOMINIS_STRAIN', 'Grade1', 7, 14, 28, 0)
)
INSERT OR IGNORE INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT d.id, p.grade, 'Conservative', p.minimum_days, p.typical_days,
    p.maximum_days, p.mandatory_withdrawal
FROM profiles p JOIN injury_definitions d ON d.code = p.code;
