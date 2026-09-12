CREATE TABLE match_player_kick_fouls (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    kick_foul_takes INTEGER NOT NULL
);

CREATE TABLE match_player_kick_fouls_by_decision (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    decision_kind TEXT NOT NULL,
    takes_count INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_match_player_kick_fouls_match_player ON match_player_kick_fouls(match_id, player_id);
CREATE UNIQUE INDEX idx_match_player_kick_fouls_by_decision_match_player_dec ON match_player_kick_fouls_by_decision(match_id, player_id, decision_kind);
