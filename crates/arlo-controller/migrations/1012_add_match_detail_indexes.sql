CREATE INDEX IF NOT EXISTS idx_matches_fixture ON matches(fixture_id);

CREATE INDEX IF NOT EXISTS idx_match_team_scores_match ON match_team_scores(match_id);
CREATE INDEX IF NOT EXISTS idx_match_team_lineup_usage_match ON match_team_lineup_usage(match_id);

CREATE INDEX IF NOT EXISTS idx_match_scoring_plays_match ON match_scoring_plays(match_id);
CREATE INDEX IF NOT EXISTS idx_match_fouls_match ON match_fouls(match_id);
CREATE INDEX IF NOT EXISTS idx_match_injuries_match ON match_injuries(match_id);
CREATE INDEX IF NOT EXISTS idx_match_substitutions_match ON match_substitutions(match_id);
CREATE INDEX IF NOT EXISTS idx_match_turnovers_match ON match_turnovers(match_id);

CREATE INDEX IF NOT EXISTS idx_match_kick_foul_awards_match ON match_kick_foul_awards(match_id);
CREATE INDEX IF NOT EXISTS idx_match_kick_foul_decisions_match ON match_kick_foul_decisions(match_id);

CREATE INDEX IF NOT EXISTS idx_match_time_calls_match ON match_time_calls(match_id);
CREATE INDEX IF NOT EXISTS idx_match_challenges_match ON match_challenges(match_id);
CREATE INDEX IF NOT EXISTS idx_match_tactical_profile_activations_match ON match_tactical_profile_activations(match_id);
CREATE INDEX IF NOT EXISTS idx_match_play_call_selections_match ON match_play_call_selections(match_id);

CREATE INDEX IF NOT EXISTS idx_match_availability_changes_match ON match_availability_changes(match_id);
CREATE INDEX IF NOT EXISTS idx_match_impulse_critical_events_match ON match_impulse_critical_events(match_id);
CREATE INDEX IF NOT EXISTS idx_match_added_time_match ON match_added_time(match_id);

CREATE INDEX IF NOT EXISTS idx_match_player_impulse_match ON match_player_impulse(match_id);
CREATE INDEX IF NOT EXISTS idx_match_player_impulse_runs_match ON match_player_impulse_runs(match_id);
CREATE INDEX IF NOT EXISTS idx_match_player_impulse_shifts_by_kind_match ON match_player_impulse_shifts_by_kind(match_id);

CREATE INDEX IF NOT EXISTS idx_match_player_availability_match ON match_player_availability(match_id);
CREATE INDEX IF NOT EXISTS idx_match_player_injuries_match ON match_player_injuries(match_id);
CREATE INDEX IF NOT EXISTS idx_match_player_injuries_by_body_region_match ON match_player_injuries_by_body_region(match_id);
CREATE INDEX IF NOT EXISTS idx_match_player_physical_match ON match_player_physical(match_id);

CREATE INDEX IF NOT EXISTS idx_match_team_impulse_match ON match_team_impulse(match_id);
CREATE INDEX IF NOT EXISTS idx_match_team_impulse_runs_match ON match_team_impulse_runs(match_id);
CREATE INDEX IF NOT EXISTS idx_match_team_possession_match ON match_team_possession(match_id);

CREATE INDEX IF NOT EXISTS idx_match_manager_decisions_match ON match_manager_decisions(match_id);
CREATE INDEX IF NOT EXISTS idx_match_manager_substitutions_by_reason_match ON match_manager_substitutions_by_reason(match_id);
CREATE INDEX IF NOT EXISTS idx_match_manager_play_calls_by_category_match ON match_manager_play_calls_by_category(match_id);
CREATE INDEX IF NOT EXISTS idx_match_play_call_outcomes_match ON match_play_call_outcomes(match_id);

CREATE INDEX IF NOT EXISTS idx_match_referee_performance_match ON match_referee_performance(match_id);