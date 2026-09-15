CREATE TABLE league_calendar_stage_definitions (
    id TEXT PRIMARY KEY NOT NULL,
    league_calendar_config_id TEXT NOT NULL REFERENCES league_calendar_configs(id) ON DELETE CASCADE,
    stage_order_index INTEGER NOT NULL,
    stage_type TEXT NOT NULL,
    entry_rule_kind TEXT NOT NULL,
    entry_rule_count INTEGER,
    leg_format TEXT
);

CREATE UNIQUE INDEX idx_league_calendar_stages_config_stage ON league_calendar_stage_definitions(league_calendar_config_id, stage_order_index);
