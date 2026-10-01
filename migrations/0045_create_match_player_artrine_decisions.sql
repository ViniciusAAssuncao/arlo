CREATE TABLE match_player_artrine_decisions (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    total_decisions INTEGER NOT NULL,
    total_successful_decisions INTEGER NOT NULL,
    total_failed_decisions INTEGER NOT NULL,
    total_mirins_advanced REAL NOT NULL,
    total_points_generated INTEGER NOT NULL,
    goal_points_generated INTEGER NOT NULL,
    field_points_generated INTEGER NOT NULL,
    field_goals_generated INTEGER NOT NULL,
    success_rate REAL NOT NULL,
    average_mirins_per_decision REAL NOT NULL,
    average_points_per_decision REAL NOT NULL
);

CREATE TABLE match_player_artrine_decisions_by_kind (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    decision_kind TEXT NOT NULL,
    total INTEGER NOT NULL,
    successful INTEGER NOT NULL,
    failed INTEGER NOT NULL,
    mirins_advanced REAL NOT NULL,
    points_generated INTEGER NOT NULL,
    success_rate REAL NOT NULL,
    average_mirins_advanced REAL NOT NULL,
    average_points_generated REAL NOT NULL
);

CREATE UNIQUE INDEX idx_match_player_artrine_decisions_match_player ON match_player_artrine_decisions(match_id, player_id);
CREATE UNIQUE INDEX idx_match_player_artrine_decisions_by_kind_match_player_kind ON match_player_artrine_decisions_by_kind(match_id, player_id, decision_kind);
