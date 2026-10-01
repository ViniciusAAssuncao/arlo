CREATE TABLE match_player_impulse_runs (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    run_index INTEGER NOT NULL,
    start_time_seconds REAL NOT NULL,
    end_time_seconds REAL NOT NULL,
    duration_seconds REAL NOT NULL,
    peak_value INTEGER NOT NULL,
    integrated_intensity REAL NOT NULL,
    shifts_count INTEGER NOT NULL,
    average_intensity REAL NOT NULL
);

CREATE UNIQUE INDEX idx_match_player_impulse_runs_match_player_seq ON match_player_impulse_runs(match_id, player_id, run_index);
