CREATE TABLE match_referee_decisions (
    match_id TEXT NOT NULL,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    offending_player_id TEXT NOT NULL,
    offending_team_id TEXT NOT NULL,
    opposing_player_id TEXT NOT NULL,
    opposing_team_id TEXT NOT NULL,
    origin TEXT NOT NULL,
    fault_definition_id TEXT,
    factual_foul INTEGER NOT NULL,
    original_call INTEGER NOT NULL,
    peace_referee_intervened INTEGER NOT NULL,
    final_call INTEGER NOT NULL,
    PRIMARY KEY (match_id, sequence_number)
);

CREATE TABLE match_foul_punishments (
    match_id TEXT NOT NULL,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    offending_player_id TEXT NOT NULL,
    offending_team_id TEXT NOT NULL,
    fault_definition_id TEXT,
    kind TEXT NOT NULL,
    magnitude INTEGER,
    PRIMARY KEY (match_id, sequence_number)
);
