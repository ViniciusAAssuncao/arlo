ALTER TABLE match_player_physical DROP COLUMN high_intensity_distance;
ALTER TABLE match_player_physical DROP COLUMN low_intensity_distance;
ALTER TABLE match_player_physical DROP COLUMN metabolic_energy_joules;
ALTER TABLE match_player_physical DROP COLUMN peak_speed_meters_per_sec;
ALTER TABLE match_player_physical DROP COLUMN distance_first_zone;
ALTER TABLE match_player_physical DROP COLUMN distance_second_zone;
ALTER TABLE match_player_physical DROP COLUMN distance_corridors;
ALTER TABLE match_player_physical DROP COLUMN distance_central;

ALTER TABLE match_turnovers DROP COLUMN point_x;
ALTER TABLE match_turnovers DROP COLUMN point_y;

ALTER TABLE match_kick_foul_awards DROP COLUMN spot_x_mirim;
ALTER TABLE match_kick_foul_awards DROP COLUMN spot_y_mirim;