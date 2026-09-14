CREATE TABLE IF NOT EXISTS season_instances (
    id TEXT PRIMARY KEY NOT NULL,
    competition_id TEXT NOT NULL,
    reference_year INTEGER NOT NULL,
    current_stage_order_index INTEGER NOT NULL,
    status TEXT NOT NULL,
    created_at_unix_seconds INTEGER NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_season_instances_comp_year ON season_instances(competition_id, reference_year);
