CREATE TABLE injury_recovery_profiles (
    injury_definition_id TEXT NOT NULL REFERENCES injury_definitions(id),
    severity_grade TEXT NOT NULL CHECK (severity_grade IN ('Grade1', 'Grade2', 'Grade3')),
    injury_extent TEXT CHECK (injury_extent IN ('Partial', 'Complete')),
    treatment_kind TEXT NOT NULL CHECK (treatment_kind IN ('Conservative', 'Surgical')),
    minimum_days INTEGER NOT NULL CHECK (minimum_days > 0),
    typical_days INTEGER NOT NULL CHECK (typical_days >= minimum_days),
    maximum_days INTEGER NOT NULL CHECK (maximum_days >= typical_days),
    mandatory_withdrawal BOOLEAN NOT NULL,
    PRIMARY KEY (injury_definition_id, severity_grade, treatment_kind)
);

INSERT INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, injury_extent, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT id, 'Grade2', 'Partial', 'Conservative', 90, 150, 240, 0
FROM injury_definitions WHERE code IN ('KNEE_ACL_TEAR_CONTACT', 'KNEE_ACL_TEAR_NONCONTACT');

INSERT INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, injury_extent, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT id, 'Grade2', 'Partial', 'Surgical', 180, 240, 300, 1
FROM injury_definitions WHERE code IN ('KNEE_ACL_TEAR_CONTACT', 'KNEE_ACL_TEAR_NONCONTACT');

INSERT INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, injury_extent, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT id, 'Grade3', 'Complete', 'Conservative', 270, 330, 450, 1
FROM injury_definitions WHERE code IN ('KNEE_ACL_TEAR_CONTACT', 'KNEE_ACL_TEAR_NONCONTACT');

INSERT INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, injury_extent, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT id, 'Grade3', 'Complete', 'Surgical', 270, 315, 365, 1
FROM injury_definitions WHERE code IN ('KNEE_ACL_TEAR_CONTACT', 'KNEE_ACL_TEAR_NONCONTACT');

INSERT INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT id, 'Grade1', 'Conservative', 10, 20, 35, 1
FROM injury_definitions WHERE code = 'HEAD_CONCUSSION_MILD';

INSERT INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT id, 'Grade2', 'Conservative', 21, 40, 75, 1
FROM injury_definitions WHERE code = 'HEAD_CONCUSSION_MODERATE';

INSERT INTO injury_recovery_profiles
    (injury_definition_id, severity_grade, treatment_kind, minimum_days, typical_days, maximum_days, mandatory_withdrawal)
SELECT id, 'Grade3', 'Conservative', 45, 75, 120, 1
FROM injury_definitions WHERE code = 'HEAD_CONCUSSION_SEVERE';
