from typing import List, Optional
from arlo_db_tool.db.connection import get_db_connection
from arlo_db_tool.domain.calendar_social_role_draft import CalendarSocialDay


def load_calendar_social_days(
    db_path: Optional[str],
    calendar_system_id: str,
) -> List[CalendarSocialDay]:
    with get_db_connection(db_path) as conn:
        cursor = conn.execute(
            "SELECT id, order_index, name, social_role "
            "FROM calendar_week_days "
            "WHERE calendar_system_id = ? "
            "ORDER BY order_index ASC;",
            (str(calendar_system_id),),
        )
        rows = cursor.fetchall()

    return [
        CalendarSocialDay(
            id=str(row["id"]),
            order_index=int(row["order_index"]),
            name=str(row["name"]),
            social_role=str(row["social_role"]) if row["social_role"] is not None else None,
        )
        for row in rows
    ]
