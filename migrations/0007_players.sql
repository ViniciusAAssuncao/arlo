CREATE TABLE players (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    height_m REAL NOT NULL,
    birthdate_unix_seconds INTEGER NOT NULL,
    nationality_id TEXT NOT NULL REFERENCES countries(id),
    team_id TEXT REFERENCES teams(id),
    squad_number INTEGER
);