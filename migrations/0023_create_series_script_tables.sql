CREATE TABLE series_scripts (
    id TEXT PRIMARY KEY NOT NULL,
    team_id TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at_unix_seconds INTEGER NOT NULL
);

CREATE TABLE series_script_entries (
    id TEXT PRIMARY KEY NOT NULL,
    series_script_id TEXT NOT NULL REFERENCES series_scripts(id) ON DELETE CASCADE,
    sequence_index INTEGER NOT NULL,
    play_call_id TEXT NOT NULL REFERENCES play_calls(id)
);

CREATE UNIQUE INDEX idx_series_script_entries_script_seq ON series_script_entries(series_script_id, sequence_index);
