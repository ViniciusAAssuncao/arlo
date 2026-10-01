CREATE TABLE IF NOT EXISTS league_calendar_collective_agreements (
    id TEXT PRIMARY KEY NOT NULL,
    league_calendar_config_id TEXT NOT NULL REFERENCES league_calendar_configs(id) ON DELETE CASCADE,
    collective_agreement_id TEXT NOT NULL REFERENCES collective_agreements(id) ON DELETE CASCADE,
    UNIQUE(league_calendar_config_id, collective_agreement_id)
);

CREATE INDEX IF NOT EXISTS idx_league_calendar_collective_agreements_config ON league_calendar_collective_agreements(league_calendar_config_id);