ALTER TABLE teams ADD COLUMN min_attendance INTEGER CHECK (min_attendance IS NULL OR min_attendance >= 0);
ALTER TABLE teams ADD COLUMN max_attendance INTEGER CHECK (max_attendance IS NULL OR max_attendance >= 0);

CREATE TRIGGER teams_attendance_references_insert
BEFORE INSERT ON teams
WHEN (NEW.min_attendance IS NULL) != (NEW.max_attendance IS NULL)
    OR NEW.min_attendance > NEW.max_attendance
BEGIN
    SELECT RAISE(ABORT, 'invalid attendance references');
END;

CREATE TRIGGER teams_attendance_references_update
BEFORE UPDATE OF min_attendance, max_attendance ON teams
WHEN (NEW.min_attendance IS NULL) != (NEW.max_attendance IS NULL)
    OR NEW.min_attendance > NEW.max_attendance
BEGIN
    SELECT RAISE(ABORT, 'invalid attendance references');
END;
