CREATE TABLE match_availability_changes (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    player_id TEXT NOT NULL REFERENCES players(id),
    team_id TEXT NOT NULL REFERENCES teams(id),
    previous_status TEXT NOT NULL,
    new_status TEXT NOT NULL,
    remaining_seconds REAL
);

CREATE INDEX idx_match_availability_changes_match_seq ON match_availability_changes(match_id, sequence_number);
