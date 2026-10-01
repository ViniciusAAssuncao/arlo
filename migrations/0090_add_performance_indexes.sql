CREATE INDEX IF NOT EXISTS idx_matches_fixture_id ON matches(fixture_id);
CREATE INDEX IF NOT EXISTS idx_matches_home_team ON matches(home_team_id);
CREATE INDEX IF NOT EXISTS idx_matches_away_team ON matches(away_team_id);

CREATE INDEX IF NOT EXISTS idx_fixtures_stage_id ON fixtures(season_stage_id);
CREATE INDEX IF NOT EXISTS idx_fixtures_home_team ON fixtures(home_team_id);
CREATE INDEX IF NOT EXISTS idx_fixtures_away_team ON fixtures(away_team_id);
CREATE INDEX IF NOT EXISTS idx_fixtures_scheduled_date ON fixtures(scheduled_year, scheduled_day_of_year);
CREATE INDEX IF NOT EXISTS idx_fixtures_status ON fixtures(status);
CREATE INDEX IF NOT EXISTS idx_fixtures_venue_id ON fixtures(venue_id);

CREATE INDEX IF NOT EXISTS idx_season_instances_competition ON season_instances(competition_id);
CREATE INDEX IF NOT EXISTS idx_season_instances_status ON season_instances(status);

CREATE INDEX IF NOT EXISTS idx_season_stages_season_instance ON season_stages(season_instance_id);
CREATE INDEX IF NOT EXISTS idx_season_stages_status ON season_stages(status);

CREATE INDEX IF NOT EXISTS idx_titles_competition ON titles(competition_id);
CREATE INDEX IF NOT EXISTS idx_titles_winner_team ON titles(winner_team_id);
CREATE INDEX IF NOT EXISTS idx_titles_winner_federation ON titles(winner_federation_id);
CREATE INDEX IF NOT EXISTS idx_titles_created_at ON titles(created_at_unix_seconds);

CREATE INDEX IF NOT EXISTS idx_venues_country ON venues(country_id);
CREATE INDEX IF NOT EXISTS idx_venues_owner_team ON venues(owner_team_id);

CREATE INDEX IF NOT EXISTS idx_teams_country ON teams(country_id);
CREATE INDEX IF NOT EXISTS idx_teams_league ON teams(league_id);

CREATE INDEX IF NOT EXISTS idx_competitions_federation ON competitions(federation_id);
CREATE INDEX IF NOT EXISTS idx_competitions_country ON competitions(country_id);

CREATE INDEX IF NOT EXISTS idx_entry_rule_pools_external_comp ON league_calendar_stage_entry_rule_pools(external_competition_id);
CREATE INDEX IF NOT EXISTS idx_fixture_postponements_fixture ON fixture_postponements(fixture_id);
CREATE INDEX IF NOT EXISTS idx_promotion_results_season ON season_promotion_relegation_results(season_instance_id);
CREATE INDEX IF NOT EXISTS idx_promotion_results_team ON season_promotion_relegation_results(team_id);
