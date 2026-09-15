CREATE TABLE IF NOT EXISTS calendar_week_days (
    id TEXT PRIMARY KEY NOT NULL,
    calendar_system_id TEXT NOT NULL REFERENCES calendar_systems(id),
    order_index INTEGER NOT NULL,
    name TEXT NOT NULL,
    UNIQUE(calendar_system_id, order_index)
);
