CREATE TABLE IF NOT EXISTS player_club_history (
    id TEXT PRIMARY KEY NOT NULL,
    player_id TEXT NOT NULL REFERENCES players(id),
    team_id TEXT NOT NULL REFERENCES teams(id),
    joined_year INTEGER NOT NULL,
    left_year INTEGER,
    created_at_unix_seconds INTEGER NOT NULL
);
