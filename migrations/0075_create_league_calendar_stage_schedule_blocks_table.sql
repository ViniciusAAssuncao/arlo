CREATE TABLE IF NOT EXISTS league_calendar_stage_schedule_blocks (
    id TEXT PRIMARY KEY NOT NULL,
    league_calendar_stage_definition_id TEXT NOT NULL REFERENCES league_calendar_stage_definitions(id),
    block_order_index INTEGER NOT NULL,
    block_kind TEXT NOT NULL,
    group_a_id TEXT,
    group_b_id TEXT,
    mirrored BOOLEAN,
    pool_kind TEXT,
    rounds_count INTEGER,
    UNIQUE(league_calendar_stage_definition_id, block_order_index)
);
