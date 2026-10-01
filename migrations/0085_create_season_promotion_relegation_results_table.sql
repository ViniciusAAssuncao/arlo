CREATE TABLE IF NOT EXISTS season_promotion_relegation_results (
    id TEXT PRIMARY KEY NOT NULL,
    season_instance_id TEXT NOT NULL REFERENCES season_instances(id),
    team_id TEXT NOT NULL REFERENCES teams(id),
    movement_kind TEXT NOT NULL,
    source_league_id TEXT NOT NULL REFERENCES leagues(competition_id),
    destination_league_id TEXT REFERENCES leagues(competition_id),
    final_standing_position INTEGER,
    created_at_unix_seconds INTEGER NOT NULL
);
