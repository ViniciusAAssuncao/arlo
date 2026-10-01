CREATE TABLE match_fouls (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    offending_player_id TEXT NOT NULL REFERENCES players(id),
    offending_team_id TEXT NOT NULL REFERENCES teams(id),
    opposing_player_id TEXT NOT NULL REFERENCES players(id),
    opposing_team_id TEXT NOT NULL REFERENCES teams(id),
    origin TEXT NOT NULL,
    original_call_correct INTEGER NOT NULL,
    peace_referee_intervened INTEGER NOT NULL,
    fault_definition_id TEXT REFERENCES fault_definitions(id),
    punishment_kind TEXT,
    punishment_magnitude INTEGER
);

CREATE INDEX idx_match_fouls_match_seq ON match_fouls(match_id, sequence_number);
