CREATE TABLE IF NOT EXISTS match_player_performance (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    team_id TEXT NOT NULL REFERENCES teams(id),
    offensive_position TEXT NOT NULL,
    defensive_position TEXT NOT NULL,
    slot_role TEXT NOT NULL,
    performance_rating REAL NOT NULL,
    outcome_adjustment REAL NOT NULL,
    final_rating REAL NOT NULL,
    confidence REAL NOT NULL,
    seconds_played REAL NOT NULL,
    effective_opportunities INTEGER NOT NULL,
    execution REAL NOT NULL,
    production REAL NOT NULL,
    defense REAL NOT NULL,
    ball_security REAL NOT NULL,
    discipline REAL NOT NULL,
    high_impact REAL NOT NULL,
    model_version TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_match_player_performance_match_player ON match_player_performance(match_id, player_id);
CREATE INDEX IF NOT EXISTS idx_match_player_performance_player ON match_player_performance(player_id);
CREATE INDEX IF NOT EXISTS idx_match_player_performance_match ON match_player_performance(match_id);