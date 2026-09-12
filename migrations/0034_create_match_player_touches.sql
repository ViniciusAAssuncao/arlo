CREATE TABLE match_player_touches (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    passes_attempted INTEGER NOT NULL,
    passes_received INTEGER NOT NULL,
    drives_recorded INTEGER NOT NULL,
    recoveries INTEGER NOT NULL,
    scoring_attempts INTEGER NOT NULL,
    total_touches INTEGER NOT NULL,
    turnovers_conceded INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_match_player_touches_match_player ON match_player_touches(match_id, player_id);
