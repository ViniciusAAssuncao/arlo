ALTER TABLE match_player_performance_category_contributions
ADD COLUMN rating_latent_contribution REAL NOT NULL DEFAULT 0.0;

ALTER TABLE match_player_performance_category_contributions
ADD COLUMN rating_opportunity_weight REAL NOT NULL DEFAULT 0.0;

ALTER TABLE match_player_performance_category_contributions
ADD COLUMN positional_relevance REAL NOT NULL DEFAULT 1.0;

ALTER TABLE match_player_performance_category_contributions
ADD COLUMN rating_high_impact REAL NOT NULL DEFAULT 0.0;
