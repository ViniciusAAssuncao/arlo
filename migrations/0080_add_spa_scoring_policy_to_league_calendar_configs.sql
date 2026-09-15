ALTER TABLE league_calendar_configs ADD COLUMN spa_win_weight REAL NOT NULL DEFAULT 3.0;
ALTER TABLE league_calendar_configs ADD COLUMN spa_draw_weight REAL NOT NULL DEFAULT 1.0;
ALTER TABLE league_calendar_configs ADD COLUMN spa_loss_weight REAL NOT NULL DEFAULT 0.25;
ALTER TABLE league_calendar_configs ADD COLUMN spa_feo_k_factor REAL NOT NULL DEFAULT 5.0;
