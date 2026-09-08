DROP TABLE IF EXISTS tactical_lineup_slots_snapshot;
DROP TABLE IF EXISTS tactical_lineups_snapshot;
DROP TABLE IF EXISTS tactical_instruction_values_snapshot;
DROP TABLE IF EXISTS team_tactical_profiles_snapshot;

CREATE TABLE team_tactical_profiles (
    id TEXT PRIMARY KEY NOT NULL,
    team_id TEXT NOT NULL REFERENCES teams(id),
    name TEXT NOT NULL,
    is_active INTEGER NOT NULL,
    created_at_unix_seconds INTEGER NOT NULL
);

CREATE TABLE tactical_instruction_values (
    id TEXT PRIMARY KEY NOT NULL,
    team_tactical_profile_id TEXT NOT NULL REFERENCES team_tactical_profiles(id) ON DELETE CASCADE,
    phase TEXT NOT NULL,
    instruction_key TEXT NOT NULL,
    value REAL NOT NULL
);

CREATE TABLE tactical_lineups (
    id TEXT PRIMARY KEY NOT NULL,
    team_id TEXT NOT NULL REFERENCES teams(id),
    formation_id TEXT NOT NULL REFERENCES formations(id),
    name TEXT NOT NULL,
    created_at_unix_seconds INTEGER NOT NULL
);

CREATE TABLE tactical_lineup_slots (
    id TEXT PRIMARY KEY NOT NULL,
    tactical_lineup_id TEXT NOT NULL REFERENCES tactical_lineups(id) ON DELETE CASCADE,
    slot_index INTEGER NOT NULL,
    player_id TEXT NOT NULL REFERENCES players(id),
    slot_role TEXT NOT NULL,
    marking_scheme TEXT,
    marking_target_position TEXT
);

CREATE TABLE tactical_lineup_slot_instructions (
    id TEXT PRIMARY KEY NOT NULL,
    tactical_lineup_slot_id TEXT NOT NULL REFERENCES tactical_lineup_slots(id) ON DELETE CASCADE,
    phase TEXT NOT NULL,
    instruction_key TEXT NOT NULL,
    value REAL NOT NULL
);

CREATE UNIQUE INDEX idx_tactical_lineup_slots_lineup_slot ON tactical_lineup_slots(tactical_lineup_id, slot_index);
CREATE UNIQUE INDEX idx_tactical_instruction_values_profile_key ON tactical_instruction_values(team_tactical_profile_id, instruction_key);
CREATE UNIQUE INDEX idx_tactical_lineup_slot_instructions_slot_key ON tactical_lineup_slot_instructions(tactical_lineup_slot_id, instruction_key);
