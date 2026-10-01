CREATE TABLE countries (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    continent_id TEXT NOT NULL REFERENCES continents(id),
    federation_id TEXT REFERENCES federations(id)
);