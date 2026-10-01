CREATE TABLE match_team_lineup_usage (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    team_id TEXT NOT NULL REFERENCES teams(id),
    tactical_lineup_id TEXT NOT NULL REFERENCES tactical_lineups(id),
    formation_id TEXT NOT NULL REFERENCES formations(id),
    team_tactical_profile_id TEXT REFERENCES team_tactical_profiles(id),
    manager_id TEXT NOT NULL REFERENCES managers(id)
);

CREATE UNIQUE INDEX idx_match_team_lineup_usage_match_team ON match_team_lineup_usage(match_id, team_id);
