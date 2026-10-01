ALTER TABLE league_calendar_configs ADD COLUMN qta_home_win_weight REAL NOT NULL DEFAULT 1.0;
ALTER TABLE league_calendar_configs ADD COLUMN qta_away_win_weight REAL NOT NULL DEFAULT 0.9;
ALTER TABLE league_calendar_configs ADD COLUMN qta_home_draw_weight REAL NOT NULL DEFAULT 0.6;
ALTER TABLE league_calendar_configs ADD COLUMN qta_away_draw_weight REAL NOT NULL DEFAULT 0.55;
ALTER TABLE league_calendar_configs ADD COLUMN qta_home_loss_weight REAL NOT NULL DEFAULT -0.2;
ALTER TABLE league_calendar_configs ADD COLUMN qta_away_loss_weight REAL NOT NULL DEFAULT -0.15;
