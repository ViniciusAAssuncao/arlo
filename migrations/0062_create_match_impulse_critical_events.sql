CREATE TABLE match_impulse_critical_events (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    player_id TEXT NOT NULL REFERENCES players(id),
    value INTEGER NOT NULL,
    duration_seconds REAL NOT NULL
);

CREATE INDEX idx_match_impulse_critical_events_match_seq ON match_impulse_critical_events(match_id, sequence_number);
