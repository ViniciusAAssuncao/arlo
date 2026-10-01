CREATE TABLE match_player_drives (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    total_drives INTEGER NOT NULL,
    central_drives INTEGER NOT NULL,
    left_lateral_drives INTEGER NOT NULL,
    right_lateral_drives INTEGER NOT NULL,
    lateral_drives INTEGER NOT NULL,
    max_drives_in_series INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_match_player_drives_match_player ON match_player_drives(match_id, player_id);
