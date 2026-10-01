CREATE TABLE league_calendar_matchday_weekdays (
    id TEXT PRIMARY KEY NOT NULL,
    league_calendar_config_id TEXT NOT NULL REFERENCES league_calendar_configs(id) ON DELETE CASCADE,
    weekday_order_index INTEGER NOT NULL
);

CREATE INDEX idx_league_calendar_matchday_weekdays_config ON league_calendar_matchday_weekdays(league_calendar_config_id);
