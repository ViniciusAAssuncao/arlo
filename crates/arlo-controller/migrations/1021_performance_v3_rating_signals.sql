ALTER TABLE match_player_performance
ADD COLUMN quality_signal REAL NOT NULL DEFAULT 0.0;

ALTER TABLE match_player_performance
ADD COLUMN confidence_evidence REAL NOT NULL DEFAULT 0.0;

ALTER TABLE match_player_performance
ADD COLUMN rating_latent REAL NOT NULL DEFAULT 0.0;

ALTER TABLE match_player_performance
ADD COLUMN impact_signal REAL NOT NULL DEFAULT 0.0;

ALTER TABLE match_player_performance
ADD COLUMN impact_adjustment REAL NOT NULL DEFAULT 0.0;
