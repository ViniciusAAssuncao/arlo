CREATE TABLE match_player_injuries (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    total_injuries INTEGER NOT NULL,
    contact_injuries INTEGER NOT NULL,
    non_contact_injuries INTEGER NOT NULL,
    grade_1_injuries INTEGER NOT NULL,
    grade_2_injuries INTEGER NOT NULL,
    grade_3_injuries INTEGER NOT NULL
);

CREATE TABLE match_player_injuries_by_body_region (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    body_region TEXT NOT NULL,
    injuries_count INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_match_player_injuries_match_player ON match_player_injuries(match_id, player_id);
CREATE UNIQUE INDEX idx_match_player_injuries_by_body_region_match_player_region ON match_player_injuries_by_body_region(match_id, player_id, body_region);
