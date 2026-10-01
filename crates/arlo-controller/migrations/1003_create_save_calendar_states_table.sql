CREATE TABLE IF NOT EXISTS save_calendar_states (
    save_uuid TEXT PRIMARY KEY NOT NULL,
    calendar_system_id TEXT NOT NULL REFERENCES calendar_systems(id),
    current_year INTEGER NOT NULL,
    current_day_of_year INTEGER NOT NULL,
    assigned_at_unix_seconds INTEGER NOT NULL
);