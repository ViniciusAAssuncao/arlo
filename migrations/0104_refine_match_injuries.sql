INSERT OR IGNORE INTO outside_match_injury_definitions
    (injury_definition_id, daily_weight, severity_grade)
SELECT id, 0, 'Grade1'
FROM injury_definitions
WHERE code LIKE '%DEGENERATION%'
   OR code IN (
       'FOOT_MORTONS_NEUROMA',
       'FOOT_PLANTAR_FASCIITIS',
       'FOOT_METATARSALGIA',
       'CALF_MTSS_SHIN_SPLINTS',
       'HAND_INTERSECTION_SYNDROME',
       'KNEE_CHONDROMALACIA_PATELLAE',
       'KNEE_ITB_SYNDROME',
       'KNEE_PATELLOFEMORAL_PAIN_SYNDROME',
       'KNEE_RECURRENT_PATELLAR_INSTABILITY',
       'NECK_CERVICALGIA',
       'SHOULDER_SCAPULAR_DYSKINESIS',
       'SHOULDER_SUBACROMIAL_IMPINGEMENT'
   );

CREATE TABLE match_injury_grade_floors (
    injury_definition_id TEXT PRIMARY KEY REFERENCES injury_definitions(id),
    minimum_grade TEXT NOT NULL CHECK (minimum_grade IN ('Grade1', 'Grade2', 'Grade3'))
);

INSERT INTO match_injury_grade_floors (injury_definition_id, minimum_grade)
SELECT id,
    CASE code
        WHEN 'HEAD_DIFFUSE_AXONAL_INJURY' THEN 'Grade3'
        ELSE 'Grade2'
    END
FROM injury_definitions
WHERE code IN ('HEAD_DIFFUSE_AXONAL_INJURY', 'THIGH_FEMUR_FRACTURE');

WITH profiles(code, grade, minimum_days, typical_days, maximum_days) AS (
    VALUES
        ('HEAD_DIFFUSE_AXONAL_INJURY', 'Grade3', 180, 365, 730)
)
INSERT INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT d.id, p.grade, 'Conservative', p.minimum_days, p.typical_days, p.maximum_days, 1
FROM profiles p JOIN injury_definitions d ON d.code = p.code;

WITH profiles(grade, minimum_days, typical_days, maximum_days) AS (
    VALUES
        ('Grade1', 3, 7, 14),
        ('Grade2', 10, 21, 42),
        ('Grade3', 28, 56, 90)
)
INSERT OR IGNORE INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT d.id, p.grade, 'Conservative', p.minimum_days, p.typical_days, p.maximum_days, 0
FROM profiles p JOIN injury_definitions d ON d.code LIKE '%CONTUSION%';

WITH profiles(grade, minimum_days, typical_days, maximum_days) AS (
    VALUES
        ('Grade1', 7, 14, 28),
        ('Grade2', 21, 42, 75),
        ('Grade3', 60, 90, 150)
)
INSERT OR IGNORE INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT d.id, p.grade, 'Conservative', p.minimum_days, p.typical_days, p.maximum_days, 0
FROM profiles p JOIN injury_definitions d
    ON d.code IN ('ANKLE_LATERAL_SPRAIN', 'ANKLE_LATERAL_SPRAIN_CONTACT');

WITH profiles(grade, minimum_days, typical_days, maximum_days) AS (
    VALUES
        ('Grade1', 3, 7, 14),
        ('Grade2', 7, 14, 30),
        ('Grade3', 21, 42, 90)
)
INSERT OR IGNORE INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT d.id, p.grade, 'Conservative', p.minimum_days, p.typical_days, p.maximum_days, 0
FROM profiles p JOIN injury_definitions d ON d.code = 'HEAD_FACIAL_LACERATION';

INSERT OR IGNORE INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT id, 'Grade1', 'Conservative', 14, 28, 42, 0
FROM injury_definitions WHERE code = 'SHOULDER_AC_JOINT_SPRAIN_GRADE1';
