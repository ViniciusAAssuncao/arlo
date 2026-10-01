CREATE TABLE match_referee_performance (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    referee_id TEXT NOT NULL REFERENCES referees(id),
    role TEXT NOT NULL,
    calls_made INTEGER NOT NULL,
    calls_correct INTEGER NOT NULL,
    calls_incorrect INTEGER NOT NULL,
    peace_referee_interventions INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_match_referee_performance_match_referee ON match_referee_performance(match_id, referee_id);
