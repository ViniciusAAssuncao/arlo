from typing import List, Optional
from arlo_db_tool.db.connection import get_db_connection
from arlo_db_tool.domain.attendance_reference_draft import AttendanceReferenceTeam


def load_league_attendance_teams(
    db_path: Optional[str],
    league_id: str,
) -> List[AttendanceReferenceTeam]:
    with get_db_connection(db_path) as conn:
        cursor = conn.execute(
            "SELECT t.id, t.name, t.prestige, t.min_attendance, t.max_attendance, "
            "v.capacity AS venue_capacity, v.name AS venue_name "
            "FROM teams t "
            "LEFT JOIN venues v ON v.id = t.home_venue_id "
            "WHERE t.league_id = ? "
            "ORDER BY t.prestige DESC, t.name ASC;",
            (str(league_id),),
        )
        rows = cursor.fetchall()

    return [
        AttendanceReferenceTeam(
            id=str(row["id"]),
            name=str(row["name"]),
            prestige=int(row["prestige"]),
            capacity=int(row["venue_capacity"]) if row["venue_capacity"] is not None else None,
            venue_name=str(row["venue_name"]) if row["venue_name"] is not None else None,
            min_attendance=int(row["min_attendance"]) if row["min_attendance"] is not None else None,
            max_attendance=int(row["max_attendance"]) if row["max_attendance"] is not None else None,
        )
        for row in rows
    ]
