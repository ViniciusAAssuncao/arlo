CREATE INDEX IF NOT EXISTS idx_fixtures_stage_id ON fixtures(season_stage_id);
CREATE INDEX IF NOT EXISTS idx_season_stages_season_instance ON season_stages(season_instance_id);

CREATE INDEX IF NOT EXISTS idx_match_squad_selections_player ON match_squad_selections(player_id);
CREATE INDEX IF NOT EXISTS idx_match_squad_selections_match ON match_squad_selections(match_id);

CREATE INDEX IF NOT EXISTS idx_match_player_touches_player ON match_player_touches(player_id);
CREATE INDEX IF NOT EXISTS idx_match_player_touches_match ON match_player_touches(match_id);

CREATE INDEX IF NOT EXISTS idx_match_player_duels_player ON match_player_duels(player_id);
CREATE INDEX IF NOT EXISTS idx_match_player_duels_match ON match_player_duels(match_id);

CREATE INDEX IF NOT EXISTS idx_match_player_duels_by_kind_player ON match_player_duels_by_kind(player_id);
CREATE INDEX IF NOT EXISTS idx_match_player_duels_by_kind_match ON match_player_duels_by_kind(match_id);

CREATE INDEX IF NOT EXISTS idx_match_player_scoring_attempts_player ON match_player_scoring_attempts(player_id);
CREATE INDEX IF NOT EXISTS idx_match_player_scoring_attempts_match ON match_player_scoring_attempts(match_id);

CREATE INDEX IF NOT EXISTS idx_match_player_receiving_player ON match_player_receiving(player_id);
CREATE INDEX IF NOT EXISTS idx_match_player_receiving_match ON match_player_receiving(match_id);

CREATE INDEX IF NOT EXISTS idx_match_player_drives_player ON match_player_drives(player_id);
CREATE INDEX IF NOT EXISTS idx_match_player_drives_match ON match_player_drives(match_id);

CREATE INDEX IF NOT EXISTS idx_match_player_fouls_player ON match_player_fouls(player_id);
CREATE INDEX IF NOT EXISTS idx_match_player_fouls_match ON match_player_fouls(match_id);

CREATE INDEX IF NOT EXISTS idx_match_player_kick_fouls_player ON match_player_kick_fouls(player_id);
CREATE INDEX IF NOT EXISTS idx_match_player_kick_fouls_match ON match_player_kick_fouls(match_id);

CREATE INDEX IF NOT EXISTS idx_match_player_kick_fouls_by_decision_player ON match_player_kick_fouls_by_decision(player_id);
CREATE INDEX IF NOT EXISTS idx_match_player_kick_fouls_by_decision_match ON match_player_kick_fouls_by_decision(match_id);

CREATE INDEX IF NOT EXISTS idx_match_player_artrine_decisions_player ON match_player_artrine_decisions(player_id);
CREATE INDEX IF NOT EXISTS idx_match_player_artrine_decisions_match ON match_player_artrine_decisions(match_id);

CREATE INDEX IF NOT EXISTS idx_match_player_assists_player ON match_player_assists(player_id);
CREATE INDEX IF NOT EXISTS idx_match_player_assists_match ON match_player_assists(match_id);

CREATE INDEX IF NOT EXISTS idx_match_player_punishments_player ON match_player_punishments(player_id);
CREATE INDEX IF NOT EXISTS idx_match_player_punishments_match ON match_player_punishments(match_id);