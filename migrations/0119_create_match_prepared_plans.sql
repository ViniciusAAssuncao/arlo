CREATE TABLE match_prepared_plans (
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    team_id TEXT NOT NULL REFERENCES teams(id),
    plan_id TEXT NOT NULL,
    plan_json TEXT NOT NULL,
    profile_json TEXT NOT NULL,
    PRIMARY KEY (match_id, team_id, plan_id)
);

CREATE TABLE match_tactical_plan_activations (
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    team_id TEXT NOT NULL REFERENCES teams(id),
    plan_id TEXT NOT NULL,
    plan_name TEXT NOT NULL,
    formation_id TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    total_elapsed_seconds REAL NOT NULL,
    assignments_json TEXT NOT NULL,
    PRIMARY KEY (match_id, sequence_number)
);
