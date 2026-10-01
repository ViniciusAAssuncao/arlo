CREATE TABLE IF NOT EXISTS player_injury_history (
    id TEXT PRIMARY KEY NOT NULL,
    player_id TEXT NOT NULL REFERENCES players(id),
    injury_definition_id TEXT NOT NULL REFERENCES injury_definitions(id),
    body_region TEXT NOT NULL,
    severity_grade TEXT NOT NULL,
    onset_year INTEGER NOT NULL,
    onset_day_of_year INTEGER NOT NULL,
    expected_recovery_days INTEGER NOT NULL,
    days_remaining INTEGER NOT NULL,
    observation_days_remaining INTEGER NOT NULL,
    status TEXT NOT NULL,
    is_relapse BOOLEAN NOT NULL,
    origin_record_id TEXT REFERENCES player_injury_history(id),
    resolved_at_unix_seconds INTEGER,
    created_at_unix_seconds INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_player_injury_history_player ON player_injury_history(player_id);
CREATE INDEX IF NOT EXISTS idx_player_injury_history_status ON player_injury_history(status);
