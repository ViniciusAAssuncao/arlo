CREATE TABLE match_injuries (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    player_id TEXT NOT NULL REFERENCES players(id),
    team_id TEXT NOT NULL REFERENCES teams(id),
    mechanism TEXT NOT NULL,
    body_region TEXT NOT NULL,
    severity_grade TEXT NOT NULL,
    injury_definition_id TEXT NOT NULL REFERENCES injury_definitions(id),
    trigger_probability REAL NOT NULL
);

CREATE INDEX idx_match_injuries_match_seq ON match_injuries(match_id, sequence_number);
