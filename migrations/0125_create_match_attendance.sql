CREATE TABLE match_attendance (
    match_id TEXT PRIMARY KEY NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    total INTEGER NOT NULL CHECK (total >= 0),
    home_supporters INTEGER NOT NULL CHECK (home_supporters >= 0),
    away_supporters INTEGER NOT NULL CHECK (away_supporters >= 0),
    unaffiliated_spectators INTEGER NOT NULL CHECK (unaffiliated_spectators >= 0),
    match_appeal REAL NOT NULL CHECK (match_appeal >= 0 AND match_appeal <= 1),
    home_popularity REAL NOT NULL CHECK (home_popularity >= 0 AND home_popularity <= 1),
    away_popularity REAL NOT NULL CHECK (away_popularity >= 0 AND away_popularity <= 1),
    model_version TEXT NOT NULL,
    CHECK (total = home_supporters + away_supporters + unaffiliated_spectators)
);
