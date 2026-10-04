ALTER TABLE award_definitions ADD COLUMN result_kind TEXT NOT NULL DEFAULT 'SingleWinner' CHECK (result_kind IN ('SingleWinner', 'Roster'));

CREATE TABLE award_selection_profiles (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE
);

CREATE TABLE award_selection_profile_criteria (
    profile_id TEXT NOT NULL REFERENCES award_selection_profiles(id),
    key TEXT NOT NULL,
    weight REAL NOT NULL CHECK (weight >= 0),
    normalization_scope TEXT NOT NULL CHECK (normalization_scope IN ('Global', 'Position', 'PositionFamily', 'CandidatePool', 'Competition')),
    PRIMARY KEY (profile_id, key)
);

CREATE TABLE award_roster_slots (
    award_definition_id TEXT NOT NULL REFERENCES award_definitions(id),
    slot_index INTEGER NOT NULL CHECK (slot_index > 0),
    position_code TEXT NOT NULL,
    selection_group TEXT,
    slot_role TEXT,
    selection_profile_id TEXT REFERENCES award_selection_profiles(id),
    PRIMARY KEY (award_definition_id, slot_index)
);

CREATE TABLE award_roster_results (
    award_instance_id TEXT NOT NULL REFERENCES award_instances(id),
    slot_index INTEGER NOT NULL CHECK (slot_index > 0),
    position_code TEXT NOT NULL,
    selection_group TEXT,
    slot_role TEXT,
    subject_kind TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    utility REAL NOT NULL,
    selection_score REAL NOT NULL,
    PRIMARY KEY (award_instance_id, slot_index),
    UNIQUE (award_instance_id, subject_kind, subject_id)
);

CREATE INDEX award_roster_results_subject ON award_roster_results(subject_kind, subject_id);

CREATE TABLE award_roster_candidates (
    award_instance_id TEXT NOT NULL REFERENCES award_instances(id),
    slot_index INTEGER NOT NULL,
    subject_kind TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    utility REAL NOT NULL,
    selection_score REAL NOT NULL,
    final_rank INTEGER NOT NULL CHECK (final_rank > 0),
    PRIMARY KEY (award_instance_id, slot_index, subject_kind, subject_id),
    UNIQUE (award_instance_id, slot_index, final_rank)
);

CREATE TABLE award_roster_electorates (
    award_instance_id TEXT NOT NULL REFERENCES award_instances(id),
    slot_index INTEGER NOT NULL,
    group_code TEXT NOT NULL,
    voter_count INTEGER NOT NULL,
    result_weight REAL NOT NULL,
    PRIMARY KEY (award_instance_id, slot_index, group_code)
);

CREATE TABLE award_roster_vote_tallies (
    award_instance_id TEXT NOT NULL,
    slot_index INTEGER NOT NULL,
    group_code TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    points INTEGER NOT NULL,
    first_place_votes INTEGER NOT NULL,
    PRIMARY KEY (award_instance_id, slot_index, group_code, subject_id),
    FOREIGN KEY (award_instance_id, slot_index, group_code) REFERENCES award_roster_electorates(award_instance_id, slot_index, group_code)
);

CREATE TABLE award_global_cycle_jobs (
    award_definition_id TEXT NOT NULL REFERENCES award_definitions(id),
    reference_year INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending', 'Completed', 'Unavailable')),
    reason TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    finished_at TEXT,
    PRIMARY KEY (award_definition_id, reference_year)
);

CREATE INDEX award_global_cycle_jobs_pending ON award_global_cycle_jobs(status, reference_year);

CREATE TABLE award_global_cycle_sources (
    award_instance_id TEXT NOT NULL REFERENCES award_instances(id),
    season_instance_id TEXT NOT NULL REFERENCES season_instances(id),
    competition_id TEXT NOT NULL REFERENCES competitions(id),
    PRIMARY KEY (award_instance_id, season_instance_id)
);
