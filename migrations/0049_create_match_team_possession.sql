CREATE TABLE match_team_possession (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    team_id TEXT NOT NULL REFERENCES teams(id),
    total_possession_seconds REAL NOT NULL
);

CREATE UNIQUE INDEX idx_match_team_possession_match_team ON match_team_possession(match_id, team_id);