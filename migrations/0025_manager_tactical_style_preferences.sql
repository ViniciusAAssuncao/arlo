ALTER TABLE manager_tactical_profiles ADD COLUMN passing_range_preference REAL NOT NULL DEFAULT 0.0;
ALTER TABLE manager_tactical_profiles ADD COLUMN aeriality_preference REAL NOT NULL DEFAULT 0.0;
ALTER TABLE manager_tactical_profiles ADD COLUMN structure_preference REAL NOT NULL DEFAULT 0.0;
ALTER TABLE manager_tactical_profiles ADD COLUMN physicality_preference REAL NOT NULL DEFAULT 0.5;
ALTER TABLE manager_tactical_profiles ADD COLUMN transition_pace_preference REAL NOT NULL DEFAULT 0.5;
ALTER TABLE manager_tactical_profiles ADD COLUMN press_block_shape_preference REAL NOT NULL DEFAULT 0.5;