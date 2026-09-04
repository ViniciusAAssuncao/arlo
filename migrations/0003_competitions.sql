CREATE TABLE competitions (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    federation_id TEXT NOT NULL REFERENCES federations(id),
    country_id TEXT REFERENCES countries(id),
    scope TEXT NOT NULL,
    kind TEXT NOT NULL,
    prestige INTEGER NOT NULL
);