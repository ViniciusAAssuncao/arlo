CREATE TABLE match_player_receiving (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    targets INTEGER NOT NULL,
    receptions INTEGER NOT NULL,
    drops INTEGER NOT NULL,
    catch_rate REAL NOT NULL,
    drop_rate REAL NOT NULL,
    receiving_mirins REAL NOT NULL,
    run_after_catch_mirins REAL NOT NULL,
    longest_reception_mirim REAL NOT NULL,
    average_mirins_per_reception REAL NOT NULL,
    average_rac_per_reception REAL NOT NULL
);

CREATE UNIQUE INDEX idx_match_player_receiving_match_player ON match_player_receiving(match_id, player_id);
