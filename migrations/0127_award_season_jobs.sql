CREATE TABLE award_tie_breaks (
    award_definition_id TEXT NOT NULL REFERENCES award_definitions(id),
    priority INTEGER NOT NULL CHECK (priority > 0),
    metric_key TEXT NOT NULL,
    direction TEXT NOT NULL CHECK (direction IN ('Descending', 'Ascending')),
    PRIMARY KEY (award_definition_id, priority)
);

CREATE TABLE award_season_jobs (
    award_definition_id TEXT NOT NULL REFERENCES award_definitions(id),
    season_instance_id TEXT NOT NULL REFERENCES season_instances(id),
    status TEXT NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending', 'Completed', 'Unavailable')),
    reason TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    finished_at TEXT,
    PRIMARY KEY (award_definition_id, season_instance_id)
);

CREATE INDEX award_season_jobs_pending ON award_season_jobs(status, season_instance_id);
