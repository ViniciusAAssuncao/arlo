CREATE TABLE continents (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE federations (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    scope TEXT NOT NULL,
    continent_id TEXT REFERENCES continents(id),
    parent_federation_id TEXT REFERENCES federations(id),
    prestige INTEGER NOT NULL
);