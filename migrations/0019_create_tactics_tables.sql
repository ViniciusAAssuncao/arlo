CREATE TABLE team_tactical_profiles_snapshot (
    id TEXT PRIMARY KEY NOT NULL,
    team_id TEXT NOT NULL REFERENCES teams(id),
    name TEXT NOT NULL,
    is_active INTEGER NOT NULL,
    created_at_unix_seconds INTEGER NOT NULL
);

CREATE TABLE tactical_instruction_values_snapshot (
    id TEXT PRIMARY KEY NOT NULL,
    team_tactical_profile_id TEXT NOT NULL REFERENCES team_tactical_profiles(id) ON DELETE CASCADE,
    phase TEXT NOT NULL,
    instruction_key TEXT NOT NULL,
    value REAL NOT NULL
);

CREATE TABLE tactical_lineups_snapshot (
    id TEXT PRIMARY KEY NOT NULL,
    team_id TEXT NOT NULL REFERENCES teams(id),
    formation_id TEXT NOT NULL REFERENCES formations(id),
    name TEXT NOT NULL,
    created_at_unix_seconds INTEGER NOT NULL
);

CREATE TABLE tactical_lineup_slots_snapshot (
    id TEXT PRIMARY KEY NOT NULL,
    tactical_lineup_id TEXT NOT NULL REFERENCES tactical_lineups(id) ON DELETE CASCADE,
    slot_index INTEGER NOT NULL,
    player_id TEXT NOT NULL REFERENCES players(id),
    slot_role TEXT NOT NULL
);

CREATE UNIQUE INDEX idx_tactical_lineup_slots_lineup_slot_snapshot ON tactical_lineup_slots_snapshot(tactical_lineup_id, slot_index);
CREATE UNIQUE INDEX idx_tactical_instruction_values_profile_key_snapshot ON tactical_instruction_values_snapshot(team_tactical_profile_id, instruction_key);