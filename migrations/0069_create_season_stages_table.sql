CREATE TABLE IF NOT EXISTS season_stages (
    id TEXT PRIMARY KEY NOT NULL,
    season_instance_id TEXT NOT NULL REFERENCES season_instances(id),
    stage_order_index INTEGER NOT NULL,
    stage_type TEXT NOT NULL,
    status TEXT NOT NULL
);