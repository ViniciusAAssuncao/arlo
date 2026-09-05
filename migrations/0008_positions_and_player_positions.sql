CREATE TABLE positions (
    id TEXT PRIMARY KEY NOT NULL,
    code TEXT NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE player_positions (
    player_id TEXT NOT NULL REFERENCES players(id),
    position_id TEXT NOT NULL REFERENCES positions(id),
    proficiency INTEGER NOT NULL,
    PRIMARY KEY (player_id, position_id)
);