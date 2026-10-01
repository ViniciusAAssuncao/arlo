CREATE TABLE day_advancement_progress (
    save_uuid TEXT PRIMARY KEY REFERENCES save_calendar_states(save_uuid),
    previous_year INTEGER NOT NULL,
    previous_day_of_year INTEGER NOT NULL,
    target_year INTEGER NOT NULL,
    target_day_of_year INTEGER NOT NULL,
    matches_played_count INTEGER NOT NULL DEFAULT 0,
    phase TEXT NOT NULL CHECK (phase IN (
        'Started', 'RecoveryDone', 'EventsRunning', 'EventsDone', 'MatchesDone'
    ))
);
