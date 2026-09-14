ALTER TABLE league_calendar_configs ADD COLUMN standings_stage_order_index INTEGER NOT NULL DEFAULT 0;
ALTER TABLE league_calendar_configs ADD COLUMN promotion_rule_kind TEXT NOT NULL DEFAULT 'None';
ALTER TABLE league_calendar_configs ADD COLUMN promotion_count INTEGER;
ALTER TABLE league_calendar_configs ADD COLUMN promotion_playoff_stage_order_index INTEGER;
ALTER TABLE league_calendar_configs ADD COLUMN promotion_target_league_id TEXT REFERENCES leagues(competition_id);
ALTER TABLE league_calendar_configs ADD COLUMN relegation_rule_kind TEXT NOT NULL DEFAULT 'None';
ALTER TABLE league_calendar_configs ADD COLUMN relegation_count INTEGER;
ALTER TABLE league_calendar_configs ADD COLUMN relegation_playoff_stage_order_index INTEGER;
ALTER TABLE league_calendar_configs ADD COLUMN relegation_target_league_id TEXT REFERENCES leagues(competition_id);
