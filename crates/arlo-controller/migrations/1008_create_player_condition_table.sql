CREATE TABLE IF NOT EXISTS player_condition (
    player_id TEXT PRIMARY KEY NOT NULL REFERENCES players(id),
    energy_level REAL NOT NULL,
    anaerobic_reserve REAL NOT NULL,
    impulse_current_value INTEGER NOT NULL,
    impulse_baseline REAL NOT NULL,
    conditioning_score REAL NOT NULL,
    last_updated_year INTEGER NOT NULL,
    last_updated_day_of_year INTEGER NOT NULL,
    last_match_year INTEGER,
    last_match_day_of_year INTEGER
);