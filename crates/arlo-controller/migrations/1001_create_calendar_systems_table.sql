CREATE TABLE IF NOT EXISTS calendar_systems (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    leap_units_per_cycle INTEGER NOT NULL,
    cycle_length_years INTEGER NOT NULL,
    cycle_reference_year INTEGER NOT NULL,
    days_per_occurrence INTEGER NOT NULL,
    intercalation_placement_kind TEXT NOT NULL,
    intercalation_placement_month_order_index INTEGER,
    created_at_unix_seconds INTEGER NOT NULL
);