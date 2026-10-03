CREATE TABLE preseason_projections (
    id TEXT PRIMARY KEY NOT NULL,
    season_instance_id TEXT NOT NULL UNIQUE REFERENCES season_instances(id),
    model_version INTEGER NOT NULL,
    calibration_id TEXT NOT NULL REFERENCES forecast_calibrations(id),
    generated_year INTEGER NOT NULL,
    generated_day_of_year INTEGER NOT NULL,
    simulation_count INTEGER NOT NULL,
    random_seed TEXT NOT NULL,
    power_seed_model_version INTEGER NOT NULL,
    historical_rating_sigma REAL NOT NULL,
    new_team_rating_sigma REAL NOT NULL,
    knockout_margin_mean REAL NOT NULL,
    goal_point_mean REAL,
    goal_point_rating_slope REAL
);

CREATE TABLE preseason_team_projections (
    projection_id TEXT NOT NULL REFERENCES preseason_projections(id) ON DELETE CASCADE,
    team_id TEXT NOT NULL,
    projected_table_position INTEGER,
    top_four_probability REAL,
    champion_probability REAL NOT NULL,
    runner_up_probability REAL NOT NULL,
    PRIMARY KEY (projection_id, team_id)
);

CREATE TABLE preseason_stage_team_projections (
    projection_id TEXT NOT NULL REFERENCES preseason_projections(id) ON DELETE CASCADE,
    stage_index INTEGER NOT NULL,
    team_id TEXT NOT NULL,
    reach_probability REAL NOT NULL,
    mean_position REAL,
    median_position INTEGER,
    modal_position INTEGER,
    position_probabilities_json TEXT NOT NULL,
    PRIMARY KEY (projection_id, stage_index, team_id)
);

CREATE TABLE preseason_round_team_projections (
    projection_id TEXT NOT NULL REFERENCES preseason_projections(id) ON DELETE CASCADE,
    stage_index INTEGER NOT NULL,
    round_index INTEGER NOT NULL,
    team_id TEXT NOT NULL,
    reach_probability REAL NOT NULL,
    PRIMARY KEY (projection_id, stage_index, round_index, team_id)
);

CREATE TABLE preseason_encounter_projections (
    projection_id TEXT NOT NULL REFERENCES preseason_projections(id) ON DELETE CASCADE,
    stage_index INTEGER NOT NULL,
    round_index INTEGER NOT NULL,
    first_team_id TEXT NOT NULL,
    second_team_id TEXT NOT NULL,
    probability REAL NOT NULL,
    PRIMARY KEY (projection_id, stage_index, round_index, first_team_id, second_team_id)
);
