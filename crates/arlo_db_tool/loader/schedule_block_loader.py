import sqlite3
from typing import List
from arlo_db_tool.domain.schedule_block_draft import ScheduleBlockDraft

def load_schedule_blocks(conn: sqlite3.Connection, stage_id: str) -> List[ScheduleBlockDraft]:
    cursor = conn.execute(
        "SELECT id, block_order_index, block_kind, group_a_id, group_b_id, "
        "mirrored, pool_kind, rounds_count "
        "FROM league_calendar_stage_schedule_blocks "
        "WHERE league_calendar_stage_definition_id = ? ORDER BY block_order_index ASC;",
        (str(stage_id),),
    )
    block_rows = cursor.fetchall()

    results = []
    for row in block_rows:
        block_id = str(row["id"])
        mirrored_val = row["mirrored"]
        if mirrored_val is not None:
            mirrored_bool = bool(mirrored_val)
        else:
            mirrored_bool = None

        pg_cursor = conn.execute(
            "SELECT group_id FROM schedule_block_pool_groups "
            "WHERE schedule_block_id = ? ORDER BY rowid ASC;",
            (block_id,),
        )
        pool_group_ids = [str(pg_row["group_id"]) for pg_row in pg_cursor.fetchall()]

        results.append(
            ScheduleBlockDraft(
                id=block_id,
                block_order_index=int(row["block_order_index"]),
                block_kind=str(row["block_kind"]),
                group_a_id=str(row["group_a_id"]) if row["group_a_id"] is not None else None,
                group_b_id=str(row["group_b_id"]) if row["group_b_id"] is not None else None,
                mirrored=mirrored_bool,
                pool_kind=str(row["pool_kind"]) if row["pool_kind"] is not None else None,
                rounds_count=int(row["rounds_count"]) if row["rounds_count"] is not None else None,
                pool_group_ids=pool_group_ids,
            )
        )
    return results