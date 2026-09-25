ALTER TABLE player_injury_history ADD COLUMN treatment_kind TEXT NOT NULL DEFAULT 'Conservative';
ALTER TABLE player_injury_history ADD COLUMN injury_extent TEXT CHECK (injury_extent IN ('Partial', 'Complete'));
