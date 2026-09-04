DROP TABLE player_positions;
DROP TABLE positions;

CREATE TABLE player_positions (
    player_id TEXT NOT NULL REFERENCES players(id),
    position TEXT NOT NULL,
    proficiency INTEGER NOT NULL,
    PRIMARY KEY (player_id, position)
);