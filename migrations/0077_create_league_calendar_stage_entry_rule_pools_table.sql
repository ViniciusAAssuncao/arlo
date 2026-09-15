CREATE TABLE IF NOT EXISTS league_calendar_stage_entry_rule_pools (
    id TEXT PRIMARY KEY NOT NULL,
    league_calendar_stage_definition_id TEXT NOT NULL REFERENCES league_calendar_stage_definitions(id),
    pool_order_index INTEGER NOT NULL,
    pool_kind TEXT NOT NULL,
    count INTEGER,
    position_index INTEGER,
    UNIQUE(league_calendar_stage_definition_id, pool_order_index)
);
