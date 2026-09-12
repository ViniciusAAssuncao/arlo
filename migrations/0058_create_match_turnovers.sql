CREATE TABLE match_turnovers (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    previous_offense_team_id TEXT NOT NULL REFERENCES teams(id),
    new_offense_team_id TEXT NOT NULL REFERENCES teams(id),
    recovering_player_id TEXT REFERENCES players(id),
    lost_by_player_id TEXT REFERENCES players(id),
    in_live_play INTEGER NOT NULL,
    point_x REAL NOT NULL,
    point_y REAL NOT NULL
);

CREATE INDEX idx_match_turnovers_match_seq ON match_turnovers(match_id, sequence_number);
