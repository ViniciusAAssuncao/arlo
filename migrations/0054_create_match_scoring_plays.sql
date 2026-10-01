CREATE TABLE match_scoring_plays (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    team_id TEXT NOT NULL REFERENCES teams(id),
    scorer_id TEXT NOT NULL REFERENCES players(id),
    artrine_id TEXT REFERENCES players(id),
    assister_id TEXT REFERENCES players(id),
    play_type TEXT NOT NULL,
    points INTEGER NOT NULL,
    scoring_post TEXT NOT NULL,
    drives_completed INTEGER,
    territory_advance_mirim REAL
);

CREATE INDEX idx_match_scoring_plays_match_seq ON match_scoring_plays(match_id, sequence_number);
