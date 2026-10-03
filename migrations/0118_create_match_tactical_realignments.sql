CREATE TABLE match_tactical_realignments (
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    team_id TEXT NOT NULL REFERENCES teams(id),
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    total_elapsed_seconds REAL NOT NULL,
    assignments_json TEXT NOT NULL,
    PRIMARY KEY (match_id, sequence_number)
);
