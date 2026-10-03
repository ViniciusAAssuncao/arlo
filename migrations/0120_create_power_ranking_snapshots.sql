CREATE TABLE power_ranking_snapshots (
    id TEXT PRIMARY KEY NOT NULL,
    season_instance_id TEXT NOT NULL REFERENCES season_instances(id),
    year INTEGER NOT NULL,
    day_of_year INTEGER NOT NULL,
    model_version INTEGER NOT NULL,
    UNIQUE (season_instance_id, year, day_of_year, model_version)
);

CREATE INDEX idx_power_ranking_snapshots_season_date
    ON power_ranking_snapshots (season_instance_id, model_version, year DESC, day_of_year DESC);

CREATE TABLE power_ranking_entries (
    snapshot_id TEXT NOT NULL REFERENCES power_ranking_snapshots(id) ON DELETE CASCADE,
    team_id TEXT NOT NULL,
    rank INTEGER NOT NULL,
    rating REAL NOT NULL,
    initial_rating REAL NOT NULL,
    games_rated INTEGER NOT NULL,
    PRIMARY KEY (snapshot_id, team_id),
    UNIQUE (snapshot_id, rank)
);
