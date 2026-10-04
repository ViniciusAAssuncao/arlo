import sqlite3
from typing import List
import uuid
from arlo_db_tool.domain.weekday_draft import WeekdayDraft

def load_weekdays(conn: sqlite3.Connection, config_id: str) -> List[WeekdayDraft]:
    cursor = conn.execute(
        "SELECT id, weekday_order_index FROM league_calendar_matchday_weekdays "
        "WHERE league_calendar_config_id = ? ORDER BY weekday_order_index ASC;",
        (str(config_id),),
    )
    results = []
    for row in cursor.fetchall():
        row_id = str(row["id"]) if row["id"] else str(uuid.uuid4())
        idx = int(row["weekday_order_index"])
        results.append(WeekdayDraft(id=row_id, weekday_order_index=idx))
    return results

def load_weekday_indices(conn: sqlite3.Connection, config_id: str) -> List[int]:
    cursor = conn.execute(
        "SELECT weekday_order_index FROM league_calendar_matchday_weekdays "
        "WHERE league_calendar_config_id = ? ORDER BY weekday_order_index ASC;",
        (str(config_id),),
    )
    return [int(row["weekday_order_index"]) for row in cursor.fetchall()]