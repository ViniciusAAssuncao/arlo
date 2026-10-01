WITH profiles(code, grade, minimum_days, typical_days, maximum_days, mandatory_withdrawal) AS (
    VALUES
        ('ANKLE_SINUS_TARSI_SYNDROME', 'Grade1', 7, 21, 42, 0),
        ('ANKLE_SYNOVITIS', 'Grade1', 5, 14, 30, 0),
        ('CALF_CALF_HEMATOMA', 'Grade1', 5, 14, 28, 0),
        ('CALF_CALF_MUSCLE_TEAR', 'Grade1', 14, 28, 45, 0),
        ('GROIN_ADDUCTOR_STRAIN', 'Grade1', 7, 14, 28, 0),
        ('GROIN_GRACILIS_STRAIN', 'Grade2', 21, 42, 70, 0),
        ('GROIN_TESTICULAR_TRAUMA', 'Grade1', 3, 7, 21, 0),
        ('HEAD_CORNEAL_ABRASION', 'Grade1', 2, 3, 7, 0),
        ('HEAD_DIFFUSE_AXONAL_INJURY', 'Grade1', 90, 180, 365, 1),
        ('HEAD_SCALP_LACERATION', 'Grade1', 3, 7, 14, 0),
        ('HEAD_TYMPANIC_RUPTURE', 'Grade1', 28, 49, 70, 0),
        ('HIP_HIP_POINTER', 'Grade2', 14, 28, 49, 0),
        ('HIP_ILIOPSOAS_STRAIN', 'Grade1', 7, 14, 28, 0),
        ('HIP_LABRAL_TEAR_TRAUMATIC', 'Grade1', 30, 60, 120, 0),
        ('KNEE_MCL_TEAR', 'Grade1', 14, 28, 42, 0),
        ('NECK_CERVICAL_STRAIN_WHIPLASH', 'Grade1', 7, 21, 42, 0),
        ('NECK_STINGER_BURNER', 'Grade2', 7, 21, 42, 0),
        ('SHOULDER_AC_JOINT_SPRAIN_GRADE3', 'Grade3', 56, 84, 120, 1),
        ('SHOULDER_DELTOID_STRAIN', 'Grade1', 7, 14, 28, 0),
        ('SHOULDER_SUBLUXATION', 'Grade1', 14, 28, 60, 0),
        ('THIGH_HAMSTRING_STRAIN_GRADE1', 'Grade1', 7, 14, 28, 0),
        ('THIGH_HAMSTRING_TEAR_TRAUMATIC', 'Grade1', 14, 28, 56, 0),
        ('THIGH_HEMATOMA', 'Grade1', 5, 14, 28, 0),
        ('TRUNK_COSTOCHONDRAL_SEPARATION', 'Grade1', 21, 42, 70, 0),
        ('TRUNK_DEHYDRATION', 'Grade1', 1, 2, 4, 0),
        ('TRUNK_LUMBAR_STRAIN', 'Grade1', 7, 14, 28, 0),
        ('TRUNK_LUMBAR_STRAIN', 'Grade2', 21, 42, 70, 0)
)
INSERT INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT d.id, p.grade, 'Conservative', p.minimum_days, p.typical_days,
    p.maximum_days, p.mandatory_withdrawal
FROM profiles p JOIN injury_definitions d ON d.code = p.code;
