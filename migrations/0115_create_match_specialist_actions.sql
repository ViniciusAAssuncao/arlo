CREATE TABLE match_passer_contacts (
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    passer_id TEXT NOT NULL,
    defender_id TEXT NOT NULL,
    late INTEGER NOT NULL,
    rough INTEGER NOT NULL,
    violent INTEGER NOT NULL,
    PRIMARY KEY (match_id, sequence_number)
);

CREATE TABLE match_goalguard_recoveries (
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    goalguard_id TEXT NOT NULL,
    team_id TEXT NOT NULL,
    position_mirim REAL NOT NULL,
    zone TEXT NOT NULL,
    used_hands INTEGER NOT NULL,
    PRIMARY KEY (match_id, sequence_number)
);
