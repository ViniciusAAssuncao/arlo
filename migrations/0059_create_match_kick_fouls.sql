CREATE TABLE match_kick_foul_awards (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    awarded_team_id TEXT NOT NULL REFERENCES teams(id),
    offending_team_id TEXT NOT NULL REFERENCES teams(id),
    scoring_tier TEXT NOT NULL,
    spot_x_mirim REAL NOT NULL,
    spot_y_mirim REAL NOT NULL
);

CREATE TABLE match_kick_foul_decisions (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    taker_id TEXT NOT NULL REFERENCES players(id),
    decision TEXT NOT NULL
);

CREATE INDEX idx_match_kick_foul_awards_match_seq ON match_kick_foul_awards(match_id, sequence_number);
CREATE INDEX idx_match_kick_foul_decisions_match_seq ON match_kick_foul_decisions(match_id, sequence_number);
