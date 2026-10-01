CREATE TABLE IF NOT EXISTS schedule_block_pool_groups (
    id TEXT PRIMARY KEY NOT NULL,
    schedule_block_id TEXT NOT NULL REFERENCES league_calendar_stage_schedule_blocks(id),
    group_id TEXT NOT NULL REFERENCES competition_groups(id),
    UNIQUE(schedule_block_id, group_id)
);
