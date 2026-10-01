CREATE INDEX IF NOT EXISTS idx_tactical_lineups_team_created
ON tactical_lineups(team_id, created_at_unix_seconds DESC);

ALTER TABLE play_calls ADD COLUMN ai_generated INTEGER NOT NULL DEFAULT 0;

CREATE INDEX IF NOT EXISTS idx_play_calls_lineup_origin_created
ON play_calls(tactical_lineup_id, ai_generated, created_at_unix_seconds);

CREATE INDEX IF NOT EXISTS idx_team_tactical_profiles_team_active
ON team_tactical_profiles(team_id, is_active);
