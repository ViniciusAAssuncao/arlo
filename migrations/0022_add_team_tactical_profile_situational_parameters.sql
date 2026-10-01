CREATE TABLE team_tactical_profile_situational_parameters (
    id TEXT PRIMARY KEY NOT NULL,
    team_tactical_profile_id TEXT NOT NULL REFERENCES team_tactical_profiles(id) ON DELETE CASCADE,
    parameter_key TEXT NOT NULL,
    value REAL NOT NULL
);

CREATE UNIQUE INDEX idx_team_tactical_profile_situational_parameters_profile_key ON team_tactical_profile_situational_parameters(team_tactical_profile_id, parameter_key);
