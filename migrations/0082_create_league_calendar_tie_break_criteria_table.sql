CREATE TABLE IF NOT EXISTS league_calendar_tie_break_criteria (
    id TEXT PRIMARY KEY NOT NULL,
    league_calendar_config_id TEXT NOT NULL REFERENCES league_calendar_configs(id),
    order_index INTEGER NOT NULL,
    criterion_kind TEXT NOT NULL,
    UNIQUE(league_calendar_config_id, order_index)
);
