CREATE TABLE match_player_assists (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    goalpoint_assists INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_match_player_assists_match_player ON match_player_assists(match_id, player_id);
