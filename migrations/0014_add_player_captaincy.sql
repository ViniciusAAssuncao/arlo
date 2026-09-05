ALTER TABLE players ADD COLUMN captaincy_role TEXT;

CREATE UNIQUE INDEX idx_players_team_captaincy ON players(team_id, captaincy_role) WHERE captaincy_role IS NOT NULL;