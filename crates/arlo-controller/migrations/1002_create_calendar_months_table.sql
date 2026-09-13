CREATE TABLE IF NOT EXISTS calendar_months (
    id TEXT PRIMARY KEY NOT NULL,
    calendar_system_id TEXT NOT NULL REFERENCES calendar_systems(id),
    order_index INTEGER NOT NULL,
    name TEXT NOT NULL,
    day_count INTEGER NOT NULL,
    UNIQUE(calendar_system_id, order_index)
);