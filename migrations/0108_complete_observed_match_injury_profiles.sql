WITH profiles(code, grade, minimum_days, typical_days, maximum_days, mandatory_withdrawal) AS (
    VALUES
        ('ARM_FOREARM_STRAIN', 'Grade1', 5, 10, 21, 0),
        ('ARM_LITTLE_LEAGUE_ELBOW', 'Grade2', 30, 60, 120, 0),
        ('ARM_MEDIAL_EPICONDYLITIS', 'Grade1', 14, 28, 60, 0),
        ('CALF_PERONEAL_NERVE_INJURY', 'Grade1', 14, 28, 60, 1),
        ('CALF_PLANTARIS_RUPTURE', 'Grade1', 14, 28, 56, 0),
        ('CALF_SOLEUS_STRAIN', 'Grade1', 7, 14, 28, 0),
        ('FOOT_FOOT_CRAMPS', 'Grade2', 2, 4, 7, 0),
        ('FOOT_TURF_TOE_TRAUMATIC', 'Grade1', 7, 14, 28, 0),
        ('GROIN_TESTICULAR_TRAUMA', 'Grade2', 14, 28, 60, 1),
        ('HEAD_AURICULAR_HEMATOMA', 'Grade1', 7, 14, 28, 1),
        ('HIP_ADDUCTOR_STRAIN_PROXIMAL', 'Grade1', 7, 14, 28, 0),
        ('HIP_HIP_POINTER', 'Grade1', 3, 7, 14, 0),
        ('KNEE_OSGOOD_SCHLATTER', 'Grade2', 28, 56, 90, 0),
        ('NECK_CERVICAL_RADICULOPATHY', 'Grade2', 28, 60, 120, 1),
        ('NECK_LEVATOR_SCAPULAE_STRAIN', 'Grade1', 5, 10, 21, 0),
        ('SHOULDER_AC_JOINT_SPRAIN_GRADE2', 'Grade2', 28, 56, 90, 1),
        ('SHOULDER_BANKART_LESION', 'Grade1', 21, 45, 90, 0),
        ('SHOULDER_CALCIFIC_TENDINITIS', 'Grade1', 14, 28, 60, 0),
        ('THIGH_MYOSITIS_OSSIFICANS', 'Grade3', 90, 180, 365, 1),
        ('THIGH_QUADRICEPS_STRAIN', 'Grade2', 21, 42, 75, 0),
        ('TRUNK_MUSCLE_SPASM_ERECTOR_SPINAE', 'Grade1', 3, 7, 14, 0)
)
INSERT INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT d.id, p.grade, 'Conservative', p.minimum_days, p.typical_days,
    p.maximum_days, p.mandatory_withdrawal
FROM profiles p JOIN injury_definitions d ON d.code = p.code
WHERE NOT EXISTS (
    SELECT 1 FROM injury_recovery_profiles existing
    WHERE existing.injury_definition_id = d.id
      AND existing.severity_grade = p.grade
      AND existing.treatment_kind = 'Conservative'
);
