CREATE TABLE match_manager_decisions (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    team_id TEXT NOT NULL REFERENCES teams(id),
    substitutions_made INTEGER NOT NULL,
    time_calls_used INTEGER NOT NULL,
    challenges_won INTEGER NOT NULL,
    challenges_lost INTEGER NOT NULL,
    tactical_profile_switches INTEGER NOT NULL
);

CREATE TABLE match_manager_substitutions_by_reason (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    team_id TEXT NOT NULL REFERENCES teams(id),
    reason TEXT NOT NULL,
    substitutions_count INTEGER NOT NULL
);

CREATE TABLE match_manager_play_calls_by_category (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    team_id TEXT NOT NULL REFERENCES teams(id),
    category TEXT NOT NULL,
    play_calls_count INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_match_manager_decisions_match_team ON match_manager_decisions(match_id, team_id);
CREATE UNIQUE INDEX idx_match_mgr_subs_match_team_reason ON match_manager_substitutions_by_reason(match_id, team_id, reason);
CREATE UNIQUE INDEX idx_match_mgr_calls_match_team_cat ON match_manager_play_calls_by_category(match_id, team_id, category);