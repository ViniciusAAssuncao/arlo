CREATE TABLE match_player_availability (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    total_suspended_seconds REAL NOT NULL,
    expulsion_count INTEGER NOT NULL,
    is_currently_expelled INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_match_player_availability_match_player ON match_player_availability(match_id, player_id);
