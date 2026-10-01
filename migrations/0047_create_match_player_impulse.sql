CREATE TABLE match_player_impulse (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    baseline REAL NOT NULL,
    current_value INTEGER NOT NULL,
    initial_value INTEGER NOT NULL,
    min_value INTEGER NOT NULL,
    max_value INTEGER NOT NULL,
    average_value REAL NOT NULL,
    shifts_count INTEGER NOT NULL,
    positive_shifts INTEGER NOT NULL,
    negative_shifts INTEGER NOT NULL,
    time_below_baseline_seconds REAL NOT NULL,
    critical_reached_count INTEGER NOT NULL,
    runs_count INTEGER NOT NULL,
    longest_run_duration_seconds REAL NOT NULL,
    peak_run_value INTEGER NOT NULL,
    total_integrated_run_intensity REAL NOT NULL,
    average_run_duration_seconds REAL NOT NULL,
    average_run_intensity REAL NOT NULL
);

CREATE TABLE match_player_impulse_shifts_by_kind (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    event_kind TEXT NOT NULL,
    shifts_count INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_match_player_impulse_match_player ON match_player_impulse(match_id, player_id);
CREATE UNIQUE INDEX idx_match_player_impulse_shifts_by_kind_match_player_kind ON match_player_impulse_shifts_by_kind(match_id, player_id, event_kind);
