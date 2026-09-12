CREATE TABLE match_player_fouls (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    fouls_committed INTEGER NOT NULL,
    fouls_drawn INTEGER NOT NULL,
    correct_calls_committed INTEGER NOT NULL,
    incorrect_calls_committed INTEGER NOT NULL
);

CREATE TABLE match_player_fouls_by_origin (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    origin TEXT NOT NULL,
    fouls_count INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_match_player_fouls_match_player ON match_player_fouls(match_id, player_id);
CREATE UNIQUE INDEX idx_match_player_fouls_by_origin_match_player_origin ON match_player_fouls_by_origin(match_id, player_id, origin);
