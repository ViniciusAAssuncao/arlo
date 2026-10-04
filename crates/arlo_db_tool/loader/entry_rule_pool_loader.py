import sqlite3
from typing import List
from arlo_db_tool.domain.entry_rule_pool_draft import EntryRulePoolDraft

def load_entry_rule_pools(conn: sqlite3.Connection, stage_id: str) -> List[EntryRulePoolDraft]:
    cursor = conn.execute(
        "SELECT id, pool_order_index, pool_kind, count, position_index, "
        "range_start_position, range_end_position, external_competition_id "
        "FROM league_calendar_stage_entry_rule_pools "
        "WHERE league_calendar_stage_definition_id = ? ORDER BY pool_order_index ASC;",
        (str(stage_id),),
    )
    results = []
    for row in cursor.fetchall():
        results.append(
            EntryRulePoolDraft(
                id=str(row["id"]),
                pool_order_index=int(row["pool_order_index"]),
                pool_kind=str(row["pool_kind"]),
                count=int(row["count"]) if row["count"] is not None else None,
                position_index=int(row["position_index"]) if row["position_index"] is not None else None,
                range_start_position=int(row["range_start_position"]) if row["range_start_position"] is not None else None,
                range_end_position=int(row["range_end_position"]) if row["range_end_position"] is not None else None,
                external_competition_id=str(row["external_competition_id"]) if row["external_competition_id"] is not None else None,
            )
        )
    return results