CREATE TABLE award_organizations (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT,
    country_id TEXT REFERENCES countries(id),
    continent_id TEXT REFERENCES continents(id),
    league_id TEXT UNIQUE REFERENCES leagues(competition_id)
);

CREATE TABLE award_definitions (
    id TEXT PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    short_name TEXT,
    organizer_id TEXT REFERENCES award_organizations(id),
    organizer_policy TEXT NOT NULL CHECK (organizer_policy IN ('DefinedOrganization', 'LeagueCommittee')),
    recipient_kind TEXT NOT NULL CHECK (recipient_kind IN ('Player', 'Team', 'Manager', 'Referee', 'Person', 'Federation')),
    prestige REAL NOT NULL CHECK (prestige >= 0),
    scope_kind TEXT NOT NULL CHECK (scope_kind IN ('Global', 'Competition', 'Team', 'Player')),
    trigger_policy TEXT NOT NULL,
    evaluation_window TEXT NOT NULL,
    minimum_matches INTEGER NOT NULL DEFAULT 0 CHECK (minimum_matches >= 0),
    nomination_limit INTEGER CHECK (nomination_limit > 0),
    selection_kind TEXT NOT NULL CHECK (selection_kind IN ('Utility', 'RankedVoting')),
    selection_temperature REAL CHECK (selection_temperature >= 0),
    active INTEGER NOT NULL DEFAULT 1 CHECK (active IN (0, 1))
);

CREATE TABLE award_criteria (
    award_definition_id TEXT NOT NULL REFERENCES award_definitions(id),
    key TEXT NOT NULL,
    weight REAL NOT NULL CHECK (weight >= 0),
    normalization_scope TEXT NOT NULL CHECK (normalization_scope IN ('Global', 'Position', 'PositionFamily', 'CandidatePool', 'Competition')),
    PRIMARY KEY (award_definition_id, key)
);

CREATE TABLE award_voter_groups (
    award_definition_id TEXT NOT NULL REFERENCES award_definitions(id),
    code TEXT NOT NULL,
    voter_count INTEGER NOT NULL CHECK (voter_count > 0),
    result_weight REAL NOT NULL CHECK (result_weight > 0),
    PRIMARY KEY (award_definition_id, code)
);

CREATE TABLE award_voter_preferences (
    award_definition_id TEXT NOT NULL,
    group_code TEXT NOT NULL,
    criterion_key TEXT NOT NULL,
    multiplier REAL NOT NULL,
    PRIMARY KEY (award_definition_id, group_code, criterion_key),
    FOREIGN KEY (award_definition_id, group_code) REFERENCES award_voter_groups(award_definition_id, code),
    FOREIGN KEY (award_definition_id, criterion_key) REFERENCES award_criteria(award_definition_id, key)
);

CREATE TABLE award_ballot_points (
    award_definition_id TEXT NOT NULL REFERENCES award_definitions(id),
    rank INTEGER NOT NULL CHECK (rank > 0),
    points INTEGER NOT NULL CHECK (points >= 0),
    PRIMARY KEY (award_definition_id, rank)
);

CREATE TABLE award_eligibility_positions (
    award_definition_id TEXT NOT NULL REFERENCES award_definitions(id),
    position_code TEXT NOT NULL,
    PRIMARY KEY (award_definition_id, position_code)
);

CREATE TABLE award_eligibility_countries (
    award_definition_id TEXT NOT NULL REFERENCES award_definitions(id),
    country_id TEXT NOT NULL REFERENCES countries(id),
    PRIMARY KEY (award_definition_id, country_id)
);

CREATE TABLE award_eligibility_continents (
    award_definition_id TEXT NOT NULL REFERENCES award_definitions(id),
    continent_id TEXT NOT NULL REFERENCES continents(id),
    PRIMARY KEY (award_definition_id, continent_id)
);

CREATE TABLE award_eligibility_competitions (
    award_definition_id TEXT NOT NULL REFERENCES award_definitions(id),
    competition_id TEXT NOT NULL REFERENCES competitions(id),
    PRIMARY KEY (award_definition_id, competition_id)
);

CREATE TABLE award_instances (
    id TEXT PRIMARY KEY,
    award_definition_id TEXT NOT NULL REFERENCES award_definitions(id),
    period_key TEXT NOT NULL,
    scope_id TEXT,
    seed TEXT NOT NULL,
    selection_model_version INTEGER NOT NULL CHECK (selection_model_version > 0),
    definition_snapshot TEXT NOT NULL,
    organizer_id TEXT REFERENCES award_organizations(id),
    organizer_league_id TEXT REFERENCES leagues(competition_id),
    resolved_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (award_definition_id, period_key, scope_id)
);

CREATE UNIQUE INDEX award_instance_global_key ON award_instances (award_definition_id, period_key) WHERE scope_id IS NULL;

CREATE TABLE award_nominees (
    award_instance_id TEXT NOT NULL REFERENCES award_instances(id),
    subject_kind TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    utility REAL NOT NULL,
    selection_score REAL NOT NULL,
    final_rank INTEGER NOT NULL CHECK (final_rank > 0),
    PRIMARY KEY (award_instance_id, subject_kind, subject_id),
    UNIQUE (award_instance_id, final_rank)
);

CREATE TABLE award_electorate_snapshots (
    award_instance_id TEXT NOT NULL REFERENCES award_instances(id),
    group_code TEXT NOT NULL,
    voter_count INTEGER NOT NULL,
    result_weight REAL NOT NULL,
    PRIMARY KEY (award_instance_id, group_code)
);

CREATE TABLE award_vote_tallies (
    award_instance_id TEXT NOT NULL REFERENCES award_instances(id),
    group_code TEXT NOT NULL,
    subject_id TEXT NOT NULL,
    points INTEGER NOT NULL,
    first_place_votes INTEGER NOT NULL,
    PRIMARY KEY (award_instance_id, group_code, subject_id),
    FOREIGN KEY (award_instance_id, group_code) REFERENCES award_electorate_snapshots(award_instance_id, group_code)
);

CREATE TABLE award_results (
    award_instance_id TEXT PRIMARY KEY REFERENCES award_instances(id),
    subject_kind TEXT NOT NULL,
    subject_id TEXT NOT NULL
);

INSERT INTO award_definitions (
    id, code, name, short_name, organizer_id, organizer_policy, recipient_kind, prestige, scope_kind,
    trigger_policy, evaluation_window, minimum_matches, nomination_limit, selection_kind, selection_temperature, active
) VALUES (
    'b6926416-75c1-498e-813f-413962269c6f', 'match-mvp', 'Most Valuable Player of the Match', 'MVP',
    NULL, 'LeagueCommittee', 'Player', 1.0, 'Competition',
    '"MatchCompleted"', '"CurrentMatch"', 1, NULL, 'Utility', 0.12, 1
);

INSERT INTO award_criteria (award_definition_id, key, weight, normalization_scope) VALUES
    ('b6926416-75c1-498e-813f-413962269c6f', 'performance_rating', 0.35, 'Position'),
    ('b6926416-75c1-498e-813f-413962269c6f', 'high_impact', 0.30, 'CandidatePool'),
    ('b6926416-75c1-498e-813f-413962269c6f', 'production', 0.15, 'Position'),
    ('b6926416-75c1-498e-813f-413962269c6f', 'defense', 0.15, 'Position'),
    ('b6926416-75c1-498e-813f-413962269c6f', 'discipline', 0.05, 'CandidatePool');
