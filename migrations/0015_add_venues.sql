CREATE TABLE venues (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    kind TEXT NOT NULL,
    owner_team_id TEXT REFERENCES teams(id),
    country_id TEXT NOT NULL REFERENCES countries(id),
    capacity INTEGER,
    pitch_length_mirim REAL,
    pitch_width_mirim REAL
);

ALTER TABLE teams ADD COLUMN home_venue_id TEXT REFERENCES venues(id);
