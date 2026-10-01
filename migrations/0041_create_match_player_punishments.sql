CREATE TABLE match_player_punishments (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    yardage_loss_count INTEGER NOT NULL,
    loss_of_down_count INTEGER NOT NULL,
    loss_of_drive_count INTEGER NOT NULL,
    time_penalty_count INTEGER NOT NULL,
    expulsion_count INTEGER NOT NULL,
    invalidate_play_count INTEGER NOT NULL,
    total_yardage_loss_mirim REAL NOT NULL,
    total_loss_of_down_count INTEGER NOT NULL,
    total_time_penalty_seconds REAL NOT NULL,
    total_loss_of_drive_count INTEGER NOT NULL
);

CREATE UNIQUE INDEX idx_match_player_punishments_match_player ON match_player_punishments(match_id, player_id);
