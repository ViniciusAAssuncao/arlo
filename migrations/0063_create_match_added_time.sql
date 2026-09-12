CREATE TABLE match_added_time (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    added_time_seconds REAL NOT NULL,
    foul_count INTEGER NOT NULL,
    injury_count INTEGER NOT NULL,
    challenge_count INTEGER NOT NULL,
    time_call_count INTEGER NOT NULL,
    kick_foul_count INTEGER NOT NULL,
    scoring_count INTEGER NOT NULL,
    accumulated_dead_ball_seconds REAL NOT NULL
);

CREATE INDEX idx_match_added_time_match_seq ON match_added_time(match_id, sequence_number);
