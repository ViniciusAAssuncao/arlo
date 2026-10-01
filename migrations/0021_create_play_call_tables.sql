CREATE TABLE play_calls (
    id TEXT PRIMARY KEY NOT NULL,
    team_id TEXT NOT NULL,
    tactical_lineup_id TEXT NOT NULL REFERENCES tactical_lineups(id),
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    counter_play_id TEXT REFERENCES play_calls(id),
    created_at_unix_seconds INTEGER NOT NULL
);

CREATE TABLE play_call_situational_parameters (
    id TEXT PRIMARY KEY NOT NULL,
    play_call_id TEXT NOT NULL REFERENCES play_calls(id) ON DELETE CASCADE,
    parameter_key TEXT NOT NULL,
    value REAL NOT NULL
);

CREATE TABLE play_call_decision_emphasis (
    id TEXT PRIMARY KEY NOT NULL,
    play_call_id TEXT NOT NULL REFERENCES play_calls(id) ON DELETE CASCADE,
    decision_kind TEXT NOT NULL,
    weight REAL NOT NULL
);

CREATE TABLE play_call_route_assignments (
    id TEXT PRIMARY KEY NOT NULL,
    play_call_id TEXT NOT NULL REFERENCES play_calls(id) ON DELETE CASCADE,
    slot_index INTEGER NOT NULL,
    has_route INTEGER NOT NULL,
    target_channel TEXT,
    depth_ratio REAL,
    break_ratio REAL,
    read_priority REAL,
    role_override TEXT
);

CREATE TABLE play_call_misdirection_links (
    id TEXT PRIMARY KEY NOT NULL,
    play_call_id TEXT NOT NULL UNIQUE REFERENCES play_calls(id) ON DELETE CASCADE,
    decoy_slot_index INTEGER NOT NULL,
    true_carrier_slot_index INTEGER NOT NULL,
    deception_intensity REAL NOT NULL
);

CREATE UNIQUE INDEX idx_play_call_situational_parameters_call_key ON play_call_situational_parameters(play_call_id, parameter_key);
CREATE UNIQUE INDEX idx_play_call_decision_emphasis_call_kind ON play_call_decision_emphasis(play_call_id, decision_kind);
CREATE UNIQUE INDEX idx_play_call_route_assignments_call_slot ON play_call_route_assignments(play_call_id, slot_index);
