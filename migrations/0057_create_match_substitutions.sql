CREATE TABLE match_substitutions (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    team_id TEXT NOT NULL REFERENCES teams(id),
    player_out_id TEXT NOT NULL REFERENCES players(id),
    player_in_id TEXT NOT NULL REFERENCES players(id),
    reason TEXT NOT NULL
);

CREATE INDEX idx_match_substitutions_match_seq ON match_substitutions(match_id, sequence_number);