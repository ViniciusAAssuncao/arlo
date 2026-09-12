CREATE TABLE match_time_calls (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    team_id TEXT NOT NULL REFERENCES teams(id),
    remaining_time_calls_after INTEGER NOT NULL,
    reason TEXT NOT NULL
);

CREATE TABLE match_challenges (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    team_id TEXT NOT NULL REFERENCES teams(id),
    call_kind TEXT NOT NULL,
    success INTEGER NOT NULL,
    remaining_challenges_after INTEGER NOT NULL
);

CREATE TABLE match_tactical_profile_activations (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    team_id TEXT NOT NULL REFERENCES teams(id),
    profile_id TEXT NOT NULL REFERENCES team_tactical_profiles(id),
    profile_name TEXT NOT NULL
);

CREATE TABLE match_play_call_selections (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    sequence_number INTEGER NOT NULL,
    period INTEGER NOT NULL,
    seconds_in_period REAL NOT NULL,
    team_id TEXT NOT NULL REFERENCES teams(id),
    play_call_id TEXT NOT NULL REFERENCES play_calls(id),
    play_call_name TEXT NOT NULL,
    category TEXT NOT NULL
);

CREATE INDEX idx_match_time_calls_match_seq ON match_time_calls(match_id, sequence_number);
CREATE INDEX idx_match_challenges_match_seq ON match_challenges(match_id, sequence_number);
CREATE INDEX idx_match_tactical_profile_activations_match_seq ON match_tactical_profile_activations(match_id, sequence_number);
CREATE INDEX idx_match_play_call_selections_match_seq ON match_play_call_selections(match_id, sequence_number);
