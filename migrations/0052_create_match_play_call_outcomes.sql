CREATE TABLE match_play_call_outcomes (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    play_call_id TEXT NOT NULL REFERENCES play_calls(id),
    attempts INTEGER NOT NULL,
    successes INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_match_play_call_outcomes_match_call ON match_play_call_outcomes(match_id, play_call_id);