DROP INDEX IF EXISTS idx_match_player_performance_match_player;
DROP INDEX IF EXISTS idx_match_player_performance_player;
DROP INDEX IF EXISTS idx_match_player_performance_match;

CREATE TABLE match_player_performance_new (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    team_id TEXT NOT NULL REFERENCES teams(id),
    offensive_position TEXT NOT NULL,
    defensive_position TEXT NOT NULL,
    slot_role TEXT NOT NULL,
    performance_rating REAL NOT NULL CHECK (performance_rating >= 0.0 AND performance_rating <= 10.0),
    outcome_adjustment REAL NOT NULL,
    final_rating REAL NOT NULL CHECK (final_rating >= 0.0 AND final_rating <= 10.0),
    confidence REAL NOT NULL CHECK (confidence >= 0.0 AND confidence <= 1.0),
    seconds_played REAL NOT NULL CHECK (seconds_played >= 0.0),
    effective_opportunities INTEGER NOT NULL CHECK (effective_opportunities >= 0 AND effective_opportunities <= 4294967295),
    execution_score REAL NOT NULL,
    production_score REAL NOT NULL,
    defense_score REAL NOT NULL,
    ball_security_score REAL NOT NULL,
    discipline_score REAL NOT NULL,
    high_impact_score REAL NOT NULL,
    model_version INTEGER NOT NULL CHECK (model_version >= 0 AND model_version <= 4294967295)
);

INSERT INTO match_player_performance_new (
    id,
    match_id,
    player_id,
    team_id,
    offensive_position,
    defensive_position,
    slot_role,
    performance_rating,
    outcome_adjustment,
    final_rating,
    confidence,
    seconds_played,
    effective_opportunities,
    execution_score,
    production_score,
    defense_score,
    ball_security_score,
    discipline_score,
    high_impact_score,
    model_version
)
SELECT
    id,
    match_id,
    player_id,
    team_id,
    offensive_position,
    defensive_position,
    slot_role,
    performance_rating,
    outcome_adjustment,
    final_rating,
    confidence,
    seconds_played,
    effective_opportunities,
    execution,
    production,
    defense,
    ball_security,
    discipline,
    high_impact,
    CAST(model_version AS INTEGER)
FROM match_player_performance;

DROP TABLE match_player_performance;

ALTER TABLE match_player_performance_new
RENAME TO match_player_performance;

CREATE UNIQUE INDEX IF NOT EXISTS idx_match_player_performance_match_player
ON match_player_performance(match_id, player_id);

CREATE INDEX IF NOT EXISTS idx_match_player_performance_player
ON match_player_performance(player_id);

CREATE INDEX IF NOT EXISTS idx_match_player_performance_match
ON match_player_performance(match_id);