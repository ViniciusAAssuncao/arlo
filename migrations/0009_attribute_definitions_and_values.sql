CREATE TABLE attribute_definitions (
    id TEXT PRIMARY KEY NOT NULL,
    key TEXT NOT NULL,
    display_name TEXT NOT NULL,
    category TEXT NOT NULL,
    applies_to TEXT NOT NULL
);

CREATE TABLE player_attributes (
    player_id TEXT NOT NULL REFERENCES players(id),
    attribute_definition_id TEXT NOT NULL REFERENCES attribute_definitions(id),
    value INTEGER NOT NULL,
    PRIMARY KEY (player_id, attribute_definition_id)
);

CREATE TABLE manager_attributes (
    manager_id TEXT NOT NULL REFERENCES managers(id),
    attribute_definition_id TEXT NOT NULL REFERENCES attribute_definitions(id),
    value INTEGER NOT NULL,
    PRIMARY KEY (manager_id, attribute_definition_id)
);