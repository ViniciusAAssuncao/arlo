CREATE TABLE titles (
    id TEXT PRIMARY KEY NOT NULL,
    competition_id TEXT NOT NULL REFERENCES competitions(id),
    season_label TEXT NOT NULL,
    winner_team_id TEXT REFERENCES teams(id),
    winner_federation_id TEXT REFERENCES federations(id)
);

CREATE TABLE rules (
    id TEXT PRIMARY KEY NOT NULL,
    competition_id TEXT NOT NULL REFERENCES competitions(id),
    category TEXT NOT NULL,
    rule_key TEXT NOT NULL,
    value TEXT NOT NULL
);