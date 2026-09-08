CREATE TABLE manager_tactical_profiles (
    id TEXT PRIMARY KEY NOT NULL,
    manager_id TEXT NOT NULL UNIQUE REFERENCES managers(id) ON DELETE CASCADE,
    offensive_approach TEXT NOT NULL,
    defensive_approach TEXT NOT NULL,
    rotation_policy TEXT NOT NULL,
    artrine_dependency TEXT NOT NULL,
    flexibility_tendency REAL NOT NULL
);

CREATE TABLE manager_preferred_formations (
    id TEXT PRIMARY KEY NOT NULL,
    manager_tactical_profile_id TEXT NOT NULL REFERENCES manager_tactical_profiles(id) ON DELETE CASCADE,
    formation_id TEXT NOT NULL REFERENCES formations(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX idx_manager_preferred_formations_profile_formation ON manager_preferred_formations(manager_tactical_profile_id, formation_id);