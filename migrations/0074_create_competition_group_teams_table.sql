CREATE TABLE IF NOT EXISTS competition_group_teams (
    id TEXT PRIMARY KEY NOT NULL,
    competition_group_id TEXT NOT NULL REFERENCES competition_groups(id),
    team_id TEXT NOT NULL REFERENCES teams(id),
    UNIQUE(competition_group_id, team_id)
);