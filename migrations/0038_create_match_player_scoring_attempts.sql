CREATE TABLE match_player_scoring_attempts (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    attempts INTEGER NOT NULL,
    converted INTEGER NOT NULL,
    missed INTEGER NOT NULL,
    conversion_rate REAL NOT NULL,
    miss_rate REAL NOT NULL,
    goal_points_scored INTEGER NOT NULL,
    field_points_scored INTEGER NOT NULL,
    field_goals_scored INTEGER NOT NULL,
    total_points_scored INTEGER NOT NULL
);

CREATE TABLE match_player_scoring_attempts_by_post (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    scoring_post TEXT NOT NULL,
    attempts INTEGER NOT NULL,
    converted INTEGER NOT NULL,
    missed INTEGER NOT NULL,
    conversion_rate REAL NOT NULL
);

CREATE UNIQUE INDEX idx_match_player_scoring_attempts_match_player ON match_player_scoring_attempts(match_id, player_id);
CREATE UNIQUE INDEX idx_match_player_scoring_attempts_by_post_match_player_post ON match_player_scoring_attempts_by_post(match_id, player_id, scoring_post);
