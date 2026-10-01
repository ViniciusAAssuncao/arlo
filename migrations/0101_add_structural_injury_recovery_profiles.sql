WITH grades(grade, factor) AS (
    VALUES ('Grade1', 1), ('Grade2', 2), ('Grade3', 3)
), fractures AS (
    SELECT id, code,
        CASE
            WHEN code IN ('FOOT_TOE_FRACTURE', 'FOOT_PHALANGEAL_FRACTURE_FOOT', 'HAND_PHALANGEAL_FRACTURE', 'HAND_METACARPAL_FRACTURE_BOXER') THEN 'SmallBone'
            WHEN code LIKE 'HEAD_SKULL_FRACTURE%' OR code LIKE '%VERTEBRAL_FRACTURE%' OR code LIKE 'NECK_CERVICAL_FRACTURE%' OR code LIKE 'HIP_%FRACTURE%' OR code = 'THIGH_FEMUR_FRACTURE' THEN 'MajorBone'
            ELSE 'OtherBone'
        END AS category
    FROM injury_definitions
    WHERE code LIKE '%FRACTURE%'
      AND code NOT LIKE 'HEAD_DENTAL_FRACTURE%'
), base AS (
    SELECT id, grade,
        CASE category
            WHEN 'SmallBone' THEN CASE factor WHEN 1 THEN 35 WHEN 2 THEN 55 ELSE 80 END
            WHEN 'MajorBone' THEN CASE factor WHEN 1 THEN 90 WHEN 2 THEN 150 ELSE 240 END
            ELSE CASE factor WHEN 1 THEN 50 WHEN 2 THEN 85 ELSE 140 END
        END AS minimum_days,
        CASE category
            WHEN 'SmallBone' THEN CASE factor WHEN 1 THEN 48 WHEN 2 THEN 75 ELSE 110 END
            WHEN 'MajorBone' THEN CASE factor WHEN 1 THEN 135 WHEN 2 THEN 220 ELSE 330 END
            ELSE CASE factor WHEN 1 THEN 70 WHEN 2 THEN 125 ELSE 200 END
        END AS typical_days,
        CASE category
            WHEN 'SmallBone' THEN CASE factor WHEN 1 THEN 65 WHEN 2 THEN 105 ELSE 150 END
            WHEN 'MajorBone' THEN CASE factor WHEN 1 THEN 200 WHEN 2 THEN 320 ELSE 480 END
            ELSE CASE factor WHEN 1 THEN 100 WHEN 2 THEN 180 ELSE 300 END
        END AS maximum_days
    FROM fractures CROSS JOIN grades
)
INSERT INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT id, grade, 'Conservative', minimum_days, typical_days, maximum_days, 1 FROM base;

WITH grades(grade, factor) AS (
    VALUES ('Grade1', 1), ('Grade2', 2), ('Grade3', 3)
), dislocations AS (
    SELECT id,
        CASE
            WHEN code LIKE 'HAND_%' THEN 'SmallJoint'
            WHEN code LIKE 'KNEE_PATELLAR_%' THEN 'Patellar'
            ELSE 'MajorJoint'
        END AS category
    FROM injury_definitions WHERE code LIKE '%DISLOCATION%'
), base AS (
    SELECT id, grade,
        CASE category
            WHEN 'SmallJoint' THEN CASE factor WHEN 1 THEN 21 WHEN 2 THEN 40 ELSE 70 END
            WHEN 'Patellar' THEN CASE factor WHEN 1 THEN 70 WHEN 2 THEN 100 ELSE 150 END
            ELSE CASE factor WHEN 1 THEN 45 WHEN 2 THEN 80 ELSE 130 END
        END AS minimum_days,
        CASE category
            WHEN 'SmallJoint' THEN CASE factor WHEN 1 THEN 35 WHEN 2 THEN 60 ELSE 100 END
            WHEN 'Patellar' THEN CASE factor WHEN 1 THEN 105 WHEN 2 THEN 150 ELSE 210 END
            ELSE CASE factor WHEN 1 THEN 75 WHEN 2 THEN 120 ELSE 190 END
        END AS typical_days,
        CASE category
            WHEN 'SmallJoint' THEN CASE factor WHEN 1 THEN 55 WHEN 2 THEN 90 ELSE 150 END
            WHEN 'Patellar' THEN CASE factor WHEN 1 THEN 170 WHEN 2 THEN 220 ELSE 320 END
            ELSE CASE factor WHEN 1 THEN 120 WHEN 2 THEN 190 ELSE 290 END
        END AS maximum_days
    FROM dislocations CROSS JOIN grades
)
INSERT INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT id, grade, 'Conservative', minimum_days, typical_days, maximum_days, 1 FROM base;

WITH grades(grade, factor) AS (
    VALUES ('Grade1', 1), ('Grade2', 2), ('Grade3', 3)
), tendons AS (
    SELECT id FROM injury_definitions
    WHERE code LIKE '%TENDON_RUPTURE%'
       OR code LIKE '%BICEPS_RUPTURE%'
       OR code LIKE '%TRICEPS_RUPTURE%'
       OR code LIKE '%ACHILLES_RUPTURE%'
       OR code = 'SHOULDER_PECTORALIS_MAJOR_RUPTURE'
       OR code = 'THIGH_HAMSTRING_TENDON_AVULSION'
       OR code = 'SHOULDER_ROTATOR_CUFF_TEAR_TRAUMATIC'
)
INSERT INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, injury_extent, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT id, grade,
    CASE WHEN factor = 3 THEN 'Complete' ELSE 'Partial' END,
    'Conservative',
    CASE factor WHEN 1 THEN 45 WHEN 2 THEN 90 ELSE 180 END,
    CASE factor WHEN 1 THEN 75 WHEN 2 THEN 140 ELSE 270 END,
    CASE factor WHEN 1 THEN 120 WHEN 2 THEN 220 ELSE 390 END,
    1
FROM tendons CROSS JOIN grades;

WITH grades(grade, factor) AS (
    VALUES ('Grade2', 2), ('Grade3', 3)
), tendons AS (
    SELECT id FROM injury_definitions
    WHERE code LIKE '%TENDON_RUPTURE%'
       OR code LIKE '%BICEPS_RUPTURE%'
       OR code LIKE '%TRICEPS_RUPTURE%'
       OR code LIKE '%ACHILLES_RUPTURE%'
       OR code = 'SHOULDER_PECTORALIS_MAJOR_RUPTURE'
       OR code = 'THIGH_HAMSTRING_TENDON_AVULSION'
       OR code = 'SHOULDER_ROTATOR_CUFF_TEAR_TRAUMATIC'
)
INSERT INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, injury_extent, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT id, grade,
    CASE WHEN factor = 3 THEN 'Complete' ELSE 'Partial' END,
    'Surgical',
    CASE factor WHEN 2 THEN 150 ELSE 240 END,
    CASE factor WHEN 2 THEN 210 ELSE 330 END,
    CASE factor WHEN 2 THEN 300 ELSE 450 END,
    1
FROM tendons CROSS JOIN grades;
