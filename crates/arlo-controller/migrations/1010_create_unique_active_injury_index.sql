CREATE UNIQUE INDEX IF NOT EXISTS idx_player_injury_history_unique_active
ON player_injury_history(player_id)
WHERE status != 'Resolved';
