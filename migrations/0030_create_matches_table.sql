CREATE TABLE matches (
    id TEXT PRIMARY KEY NOT NULL,
    home_team_id TEXT NOT NULL REFERENCES teams(id),
    away_team_id TEXT NOT NULL REFERENCES teams(id),
    venue_id TEXT REFERENCES venues(id),
    pitch_length_mirim REAL NOT NULL,
    pitch_width_mirim REAL NOT NULL,
    match_seed INTEGER NOT NULL,
    format_regulation_periods INTEGER NOT NULL,
    format_regulation_period_duration_seconds INTEGER NOT NULL,
    format_allows_overtime INTEGER NOT NULL,
    format_overtime_periods INTEGER NOT NULL,
    format_overtime_period_duration_seconds INTEGER NOT NULL,
    head_referee_id TEXT NOT NULL REFERENCES referees(id),
    peace_referee_id TEXT NOT NULL REFERENCES referees(id),
    final_period INTEGER NOT NULL,
    went_to_overtime INTEGER NOT NULL,
    completed_at_unix_seconds INTEGER NOT NULL,
    created_at_unix_seconds INTEGER NOT NULL
);

--- migrations/0031_create_match_team_scores.sql ---
CREATE TABLE match_team_scores (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    team_id TEXT NOT NULL REFERENCES teams(id),
    is_home INTEGER NOT NULL,
    goal_points INTEGER NOT NULL,
    field_goals INTEGER NOT NULL,
    field_points INTEGER NOT NULL,
    total_points INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_match_team_scores_match_team ON match_team_scores(match_id, team_id);
