CREATE TABLE injury_definitions (
    id TEXT PRIMARY KEY NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    mechanism TEXT NOT NULL,
    body_region TEXT NOT NULL,
    relative_frequency REAL NOT NULL
);