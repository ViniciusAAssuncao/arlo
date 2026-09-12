CREATE TABLE match_team_impulse (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    team_id TEXT NOT NULL REFERENCES teams(id),
    average_baseline REAL NOT NULL,
    current_average_value REAL NOT NULL,
    min_average_value REAL NOT NULL,
    max_average_value REAL NOT NULL,
    average_value REAL NOT NULL,
    time_below_baseline_seconds REAL NOT NULL,
    runs_count INTEGER NOT NULL,
    longest_run_duration_seconds REAL NOT NULL,
    peak_run_average_value REAL NOT NULL,
    total_integrated_run_intensity REAL NOT NULL,
    average_run_duration_seconds REAL NOT NULL,
    average_run_intensity REAL NOT NULL
);

CREATE TABLE match_team_impulse_runs (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    team_id TEXT NOT NULL REFERENCES teams(id),
    run_index INTEGER NOT NULL,
    start_time_seconds REAL NOT NULL,
    end_time_seconds REAL NOT NULL,
    duration_seconds REAL NOT NULL,
    peak_average_value REAL NOT NULL,
    integrated_intensity REAL NOT NULL,
    average_intensity REAL NOT NULL
);

CREATE UNIQUE INDEX idx_match_team_impulse_match_team ON match_team_impulse(match_id, team_id);
CREATE UNIQUE INDEX idx_match_team_impulse_runs_match_team_seq ON match_team_impulse_runs(match_id, team_id, run_index);