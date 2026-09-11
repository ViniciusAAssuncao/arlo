CREATE TABLE fault_definitions (
    id TEXT PRIMARY KEY NOT NULL,
    code TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    severity TEXT NOT NULL
);

CREATE TABLE fault_punishment_options (
    id TEXT PRIMARY KEY NOT NULL,
    fault_definition_id TEXT NOT NULL REFERENCES fault_definitions(id) ON DELETE CASCADE,
    kind TEXT NOT NULL,
    magnitude_min INTEGER,
    magnitude_max INTEGER
);