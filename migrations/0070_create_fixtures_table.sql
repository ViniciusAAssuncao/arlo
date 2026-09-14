CREATE TABLE IF NOT EXISTS fixtures (
    id TEXT PRIMARY KEY NOT NULL,
    season_stage_id TEXT NOT NULL REFERENCES season_stages(id),
    round_index INTEGER NOT NULL,
    home_team_id TEXT NOT NULL,
    away_team_id TEXT NOT NULL,
    is_neutral_venue BOOLEAN NOT NULL,
    scheduled_year INTEGER NOT NULL,
    scheduled_day_of_year INTEGER NOT NULL,
    status TEXT NOT NULL,
    home_score INTEGER,
    away_score INTEGER
);
