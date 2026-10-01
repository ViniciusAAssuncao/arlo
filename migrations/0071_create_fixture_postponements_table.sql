CREATE TABLE IF NOT EXISTS fixture_postponements (
    id TEXT PRIMARY KEY NOT NULL,
    fixture_id TEXT NOT NULL REFERENCES fixtures(id),
    original_year INTEGER NOT NULL,
    original_day_of_year INTEGER NOT NULL,
    new_year INTEGER NOT NULL,
    new_day_of_year INTEGER NOT NULL,
    reason TEXT NOT NULL,
    created_at_unix_seconds INTEGER NOT NULL
);
