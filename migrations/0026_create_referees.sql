CREATE TABLE referees (
    id TEXT PRIMARY KEY NOT NULL REFERENCES persons(id),
    primary_league_id TEXT REFERENCES leagues(competition_id),
    tier TEXT NOT NULL
);

CREATE TABLE referee_attributes (
    referee_id TEXT NOT NULL REFERENCES referees(id),
    attribute_definition_id TEXT NOT NULL REFERENCES attribute_definitions(id),
    value INTEGER NOT NULL,
    PRIMARY KEY (referee_id, attribute_definition_id)
);