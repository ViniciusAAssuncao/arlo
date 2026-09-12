CREATE TABLE match_player_physical (
    id TEXT PRIMARY KEY NOT NULL,
    match_id TEXT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(id),
    end_energy_level REAL NOT NULL,
    peak_anaerobic_depletion REAL NOT NULL,
    total_distance_covered REAL NOT NULL,
    high_intensity_distance REAL NOT NULL,
    low_intensity_distance REAL NOT NULL,
    metabolic_energy_joules REAL NOT NULL,
    peak_speed_meters_per_sec REAL NOT NULL,
    intra_match_recovery_amount REAL NOT NULL,
    distance_first_zone REAL NOT NULL,
    distance_second_zone REAL NOT NULL,
    distance_corridors REAL NOT NULL,
    distance_central REAL NOT NULL
);

CREATE UNIQUE INDEX idx_match_player_physical_match_player ON match_player_physical(match_id, player_id);
