CREATE INDEX IF NOT EXISTS idx_match_player_physical_player ON match_player_physical(player_id);
CREATE INDEX IF NOT EXISTS idx_match_player_impulse_player ON match_player_impulse(player_id);
CREATE INDEX IF NOT EXISTS idx_player_injury_history_player_status ON player_injury_history(player_id, status);