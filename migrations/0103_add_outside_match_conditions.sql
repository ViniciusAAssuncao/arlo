CREATE TABLE outside_match_injury_definitions (
    injury_definition_id TEXT PRIMARY KEY REFERENCES injury_definitions(id),
    daily_weight REAL NOT NULL CHECK (daily_weight >= 0),
    severity_grade TEXT NOT NULL CHECK (severity_grade IN ('Grade1', 'Grade2', 'Grade3'))
);

INSERT INTO outside_match_injury_definitions (injury_definition_id, daily_weight, severity_grade)
SELECT id,
    CASE code
        WHEN 'TRUNK_INFLUENZA' THEN 30
        WHEN 'TRUNK_GASTROENTERITIS' THEN 25
        WHEN 'TRUNK_COVID19' THEN 15
        WHEN 'TRUNK_BRONCHITIS' THEN 12
        WHEN 'HEAD_SINUSITIS' THEN 8
        WHEN 'HEAD_OTITIS_MEDIA' THEN 4
        WHEN 'HEAD_OTITIS_EXTERNA' THEN 3
        WHEN 'TRUNK_PNEUMONIA' THEN 1
        WHEN 'TRUNK_INFECTIOUS_MONONUCLEOSIS' THEN 0.5
        ELSE 0
    END,
    CASE WHEN code IN ('TRUNK_PNEUMONIA', 'TRUNK_INFECTIOUS_MONONUCLEOSIS') THEN 'Grade2' ELSE 'Grade1' END
FROM injury_definitions
WHERE code IN (
    'TRUNK_INFLUENZA', 'TRUNK_GASTROENTERITIS', 'TRUNK_COVID19',
    'TRUNK_BRONCHITIS', 'HEAD_SINUSITIS', 'HEAD_OTITIS_MEDIA',
    'HEAD_OTITIS_EXTERNA', 'TRUNK_PNEUMONIA', 'TRUNK_INFECTIOUS_MONONUCLEOSIS',
    'HEAD_BACTERIAL_MENINGITIS', 'HEAD_VIRAL_MENINGITIS', 'HEAD_ENCEPHALITIS',
    'TRUNK_HEPATITIS_A', 'TRUNK_HEPATITIS_B', 'TRUNK_APPENDICITIS',
    'FOOT_ATHLETES_FOOT', 'FOOT_PLANTAR_WARTS'
);

INSERT OR IGNORE INTO outside_match_injury_definitions
    (injury_definition_id, daily_weight, severity_grade)
SELECT id, 0, 'Grade1'
FROM injury_definitions
WHERE code LIKE '%OSTEOARTHRITIS%'
   OR code LIKE '%TENDINOPATHY%'
   OR code LIKE '%BURSITIS%'
   OR code LIKE '%DEGENERATIVE%'
   OR code LIKE '%CHRONIC%'
   OR code LIKE '%DOMS%'
   OR code LIKE '%INGROWN_TOENAIL%'
   OR code LIKE '%HALLUX_VALGUS%'
   OR code LIKE '%HAMMER_TOE%'
   OR code LIKE '%FLAT_FEET%'
   OR code LIKE '%MIGRAINE%'
   OR code LIKE '%TENSION_HEADACHE%'
   OR code LIKE '%SCOLIOSIS%'
   OR code LIKE '%CONTRACTURE%';

WITH profiles(code, grade, minimum_days, typical_days, maximum_days) AS (
    VALUES
        ('TRUNK_INFLUENZA', 'Grade1', 4, 7, 12),
        ('TRUNK_GASTROENTERITIS', 'Grade1', 2, 4, 8),
        ('TRUNK_COVID19', 'Grade1', 5, 9, 16),
        ('TRUNK_BRONCHITIS', 'Grade1', 7, 14, 24),
        ('HEAD_SINUSITIS', 'Grade1', 5, 10, 18),
        ('HEAD_OTITIS_MEDIA', 'Grade1', 5, 9, 16),
        ('HEAD_OTITIS_EXTERNA', 'Grade1', 4, 7, 14),
        ('TRUNK_PNEUMONIA', 'Grade2', 15, 25, 45),
        ('TRUNK_INFECTIOUS_MONONUCLEOSIS', 'Grade2', 21, 35, 60)
)
INSERT INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT d.id, p.grade, 'Conservative', p.minimum_days, p.typical_days, p.maximum_days, 0
FROM profiles p JOIN injury_definitions d ON d.code = p.code;
