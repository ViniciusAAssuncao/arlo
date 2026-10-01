CREATE TABLE IF NOT EXISTS competition_groups (
    id TEXT PRIMARY KEY NOT NULL,
    league_calendar_config_id TEXT NOT NULL REFERENCES league_calendar_configs(id),
    order_index INTEGER NOT NULL,
    name TEXT NOT NULL,
    UNIQUE(league_calendar_config_id, order_index)
);
