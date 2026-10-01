CREATE TABLE match_player_duels (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    total_duels INTEGER NOT NULL,
    total_wins INTEGER NOT NULL,
    total_losses INTEGER NOT NULL,
    win_rate REAL NOT NULL,
    attacker_duels INTEGER NOT NULL,
    attacker_wins INTEGER NOT NULL,
    attacker_losses INTEGER NOT NULL,
    attacker_win_rate REAL NOT NULL,
    defender_duels INTEGER NOT NULL,
    defender_wins INTEGER NOT NULL,
    defender_losses INTEGER NOT NULL,
    defender_win_rate REAL NOT NULL
);

CREATE TABLE match_player_duels_by_kind (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    duel_kind TEXT NOT NULL,
    total INTEGER NOT NULL,
    wins INTEGER NOT NULL,
    losses INTEGER NOT NULL,
    as_attacker_wins INTEGER NOT NULL,
    as_attacker_losses INTEGER NOT NULL,
    as_defender_wins INTEGER NOT NULL,
    as_defender_losses INTEGER NOT NULL,
    win_rate REAL NOT NULL,
    attacker_win_rate REAL NOT NULL,
    defender_win_rate REAL NOT NULL
);

CREATE UNIQUE INDEX idx_match_player_duels_match_player ON match_player_duels(match_id, player_id);
CREATE UNIQUE INDEX idx_match_player_duels_by_kind_match_player_kind ON match_player_duels_by_kind(match_id, player_id, duel_kind);
