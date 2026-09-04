CREATE TABLE persons (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    height_m REAL NOT NULL,
    birthdate_unix_seconds INTEGER NOT NULL,
    nationality_id TEXT NOT NULL REFERENCES countries(id)
);

CREATE TABLE managers (
    id TEXT PRIMARY KEY NOT NULL REFERENCES persons(id),
    team_id TEXT REFERENCES teams(id)
);