CREATE TABLE league_calendar_configs (
    id TEXT PRIMARY KEY NOT NULL,
    competition_id TEXT NOT NULL UNIQUE REFERENCES competitions(id),
    schedule_algorithm_kind TEXT NOT NULL,
    season_start_month_order_index INTEGER NOT NULL,
    season_start_day_of_month INTEGER NOT NULL,
    season_length_weeks INTEGER NOT NULL,
    max_games_per_team_per_week INTEGER NOT NULL,
    games_per_week_conflict_scope TEXT NOT NULL,
    postponement_strategy_kind TEXT NOT NULL,
    neutral_opener_enabled INTEGER NOT NULL,
    neutral_opener_selection_strategy TEXT,
    created_at_unix_seconds INTEGER NOT NULL
);
