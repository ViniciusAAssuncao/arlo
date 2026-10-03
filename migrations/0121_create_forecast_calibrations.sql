CREATE TABLE forecast_calibrations (
    id TEXT PRIMARY KEY NOT NULL,
    season_instance_id TEXT NOT NULL REFERENCES season_instances(id),
    model_version INTEGER NOT NULL,
    generated_year INTEGER NOT NULL,
    generated_day_of_year INTEGER NOT NULL,
    cutoff_year INTEGER NOT NULL,
    cutoff_day_of_year INTEGER NOT NULL,
    scope TEXT NOT NULL,
    sample_count INTEGER NOT NULL,
    home_advantage REAL NOT NULL,
    draw_propensity REAL NOT NULL,
    temperature REAL NOT NULL,
    rating_scale REAL NOT NULL,
    UNIQUE (season_instance_id, model_version)
);

CREATE INDEX idx_forecast_calibrations_season ON forecast_calibrations (season_instance_id, model_version);

CREATE INDEX idx_fixtures_forecast_history ON fixtures (status, scheduled_year DESC, scheduled_day_of_year DESC);
