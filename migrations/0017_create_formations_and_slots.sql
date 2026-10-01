CREATE TABLE formations (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE formation_slots (
    id TEXT PRIMARY KEY NOT NULL,
    formation_id TEXT NOT NULL REFERENCES formations(id) ON DELETE CASCADE,
    slot_index INTEGER NOT NULL,
    position TEXT NOT NULL,
    pitch_length_ratio REAL NOT NULL,
    pitch_width_ratio REAL NOT NULL,
    slot_role TEXT NOT NULL DEFAULT 'Standard'
);

CREATE UNIQUE INDEX idx_formation_slots_formation_slot_index ON formation_slots(formation_id, slot_index);
