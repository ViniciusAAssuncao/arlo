CREATE TABLE IF NOT EXISTS knockout_ties (
    id TEXT PRIMARY KEY NOT NULL,
    season_stage_id TEXT NOT NULL REFERENCES season_stages(id),
    round_index INTEGER NOT NULL,
    tie_index INTEGER NOT NULL,
    high_seed_team_id TEXT NOT NULL,
    high_seed_number INTEGER NOT NULL,
    low_seed_team_id TEXT NOT NULL,
    low_seed_number INTEGER NOT NULL,
    leg_one_fixture_id TEXT NOT NULL REFERENCES fixtures(id),
    leg_two_fixture_id TEXT REFERENCES fixtures(id),
    aggregate_winner_team_id TEXT
);

CREATE INDEX IF NOT EXISTS idx_knockout_ties_stage ON knockout_ties(season_stage_id);