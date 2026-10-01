CREATE TABLE match_play_invalidations (
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    first_invalidated_sequence INTEGER NOT NULL,
    last_invalidated_sequence INTEGER NOT NULL,
    PRIMARY KEY (match_id, sequence_number)
);
