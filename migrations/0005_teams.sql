CREATE TABLE teams (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    country_id TEXT NOT NULL REFERENCES countries(id),
    league_id TEXT REFERENCES leagues(competition_id),
    founded_at_unix_seconds INTEGER NOT NULL,
    prestige INTEGER NOT NULL,
    primary_color_hex TEXT,
    secondary_color_hex TEXT
);