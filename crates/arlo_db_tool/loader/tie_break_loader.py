import sqlite3
from typing import List
import uuid
from arlo_db_tool.domain.tie_break_criterion_draft import TieBreakCriterionDraft

def load_tie_break_criteria(conn: sqlite3.Connection, config_id: str) -> List[TieBreakCriterionDraft]:
    cursor = conn.execute(
        "SELECT id, order_index, criterion_kind FROM league_calendar_tie_break_criteria "
        "WHERE league_calendar_config_id = ? ORDER BY order_index ASC;",
        (str(config_id),),
    )
    results = []
    for row in cursor.fetchall():
        row_id = str(row["id"]) if row["id"] else str(uuid.uuid4())
        order_idx = int(row["order_index"])
        kind = str(row["criterion_kind"])
        results.append(TieBreakCriterionDraft(id=row_id, order_index=order_idx, criterion_kind=kind))
    return results

def load_tie_break_criterion_kinds(conn: sqlite3.Connection, config_id: str) -> List[str]:
    cursor = conn.execute(
        "SELECT criterion_kind FROM league_calendar_tie_break_criteria "
        "WHERE league_calendar_config_id = ? ORDER BY order_index ASC;",
        (str(config_id),),
    )
    return [str(row["criterion_kind"]) for row in cursor.fetchall()]