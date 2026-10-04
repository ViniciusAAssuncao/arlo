ALTER TABLE award_definitions ADD COLUMN minimum_competition_prestige INTEGER CHECK (minimum_competition_prestige >= 0);
ALTER TABLE award_definitions ADD COLUMN announcement_delay_days INTEGER NOT NULL DEFAULT 0 CHECK (announcement_delay_days >= 0);
ALTER TABLE award_definitions ADD COLUMN dynamic_roster_size INTEGER CHECK (dynamic_roster_size > 0);
ALTER TABLE award_definitions ADD COLUMN dynamic_minimum_position_seconds REAL CHECK (dynamic_minimum_position_seconds >= 0);
ALTER TABLE award_definitions ADD COLUMN dynamic_minimum_position_candidates INTEGER CHECK (dynamic_minimum_position_candidates > 0);
ALTER TABLE award_definitions ADD COLUMN dynamic_minimum_utility REAL CHECK (dynamic_minimum_utility >= 0);

CREATE TABLE match_player_position_seconds (
    match_id TEXT NOT NULL REFERENCES matches(id),
    player_id TEXT NOT NULL REFERENCES players(id),
    position_code TEXT NOT NULL,
    seconds_played REAL NOT NULL CHECK (seconds_played >= 0),
    PRIMARY KEY (match_id, player_id, position_code)
);

CREATE INDEX match_player_position_seconds_player ON match_player_position_seconds(player_id, position_code);

CREATE TABLE award_roster_instance_slots (
    award_instance_id TEXT NOT NULL REFERENCES award_instances(id),
    slot_index INTEGER NOT NULL CHECK (slot_index > 0),
    position_code TEXT NOT NULL,
    usage_score REAL NOT NULL CHECK (usage_score >= 0),
    evidence_score REAL NOT NULL CHECK (evidence_score >= 0),
    PRIMARY KEY (award_instance_id, slot_index)
);

CREATE TABLE award_dynamic_position_profiles (
    award_definition_id TEXT NOT NULL REFERENCES award_definitions(id),
    position_code TEXT NOT NULL,
    selection_profile_id TEXT NOT NULL REFERENCES award_selection_profiles(id),
    PRIMARY KEY (award_definition_id, position_code)
);

ALTER TABLE award_global_cycle_jobs ADD COLUMN ready_year INTEGER;
ALTER TABLE award_global_cycle_jobs ADD COLUMN ready_day_of_year INTEGER;
