CREATE TABLE leagues (
    competition_id TEXT PRIMARY KEY NOT NULL REFERENCES competitions(id),
    division_index INTEGER NOT NULL
);