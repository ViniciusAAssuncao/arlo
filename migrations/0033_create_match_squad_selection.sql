CREATE TABLE match_squad_selections (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    team_id TEXT NOT NULL REFERENCES teams(id),
    player_id TEXT NOT NULL REFERENCES players(id),
    was_starter INTEGER NOT NULL,
    formation_slot_index INTEGER,
    slot_role TEXT,
    was_used INTEGER NOT NULL,
    final_availability_status TEXT NOT NULL,
    final_suspended_remaining_seconds REAL
);

CREATE UNIQUE INDEX idx_match_squad_selections_match_player ON match_squad_selections(match_id, player_id);
