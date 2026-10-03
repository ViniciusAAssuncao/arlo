ALTER TABLE match_player_performance
ADD COLUMN effective_opportunity_weight REAL NOT NULL DEFAULT 0.0;

ALTER TABLE match_player_performance
ADD COLUMN offensive_latent REAL NOT NULL DEFAULT 0.0;

ALTER TABLE match_player_performance
ADD COLUMN defensive_latent REAL NOT NULL DEFAULT 0.0;

ALTER TABLE match_player_performance
ADD COLUMN raw_latent REAL NOT NULL DEFAULT 0.0;

CREATE TABLE IF NOT EXISTS match_player_performance_category_contributions (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    team_id TEXT NOT NULL REFERENCES teams(id),
    category TEXT NOT NULL,
    latent_contribution REAL NOT NULL,
    observations INTEGER NOT NULL CHECK (observations >= 0 AND observations <= 4294967295),
    opportunity_weight REAL NOT NULL,
    execution_score REAL NOT NULL,
    production_score REAL NOT NULL,
    defense_score REAL NOT NULL,
    ball_security_score REAL NOT NULL,
    discipline_score REAL NOT NULL,
    high_impact_score REAL NOT NULL,
    model_version INTEGER NOT NULL CHECK (model_version >= 0 AND model_version <= 4294967295),
    UNIQUE(match_id, player_id, category)
);

CREATE INDEX IF NOT EXISTS idx_match_player_performance_category_match
ON match_player_performance_category_contributions(match_id);

CREATE INDEX IF NOT EXISTS idx_match_player_performance_category_player
ON match_player_performance_category_contributions(player_id);

CREATE INDEX IF NOT EXISTS idx_match_player_performance_category_match_player
ON match_player_performance_category_contributions(match_id, player_id);
