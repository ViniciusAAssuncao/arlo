ALTER TABLE award_definitions ADD COLUMN announcement_month_order_index INTEGER CHECK (announcement_month_order_index >= 0);
ALTER TABLE award_definitions ADD COLUMN announcement_day_of_month INTEGER CHECK (announcement_day_of_month > 0);

CREATE TABLE award_announcement_jobs (
    award_definition_id TEXT NOT NULL REFERENCES award_definitions(id),
    announcement_year INTEGER NOT NULL,
    scope_id TEXT NOT NULL DEFAULT '',
    status TEXT NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending', 'Completed', 'Unavailable')),
    reason TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    finished_at TEXT,
    PRIMARY KEY (award_definition_id, announcement_year, scope_id)
);

CREATE INDEX award_announcement_jobs_pending ON award_announcement_jobs(status, announcement_year);

CREATE TABLE award_announcement_sources (
    award_instance_id TEXT NOT NULL REFERENCES award_instances(id),
    season_instance_id TEXT NOT NULL REFERENCES season_instances(id),
    competition_id TEXT NOT NULL REFERENCES competitions(id),
    start_year INTEGER NOT NULL,
    start_day_of_year INTEGER NOT NULL,
    end_year INTEGER NOT NULL,
    end_day_of_year INTEGER NOT NULL,
    PRIMARY KEY (award_instance_id, season_instance_id)
);
