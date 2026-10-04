import sqlite3
from typing import List
from arlo_db_tool.domain.stage_draft import StageDraft
from arlo_db_tool.loader.entry_rule_pool_loader import load_entry_rule_pools
from arlo_db_tool.loader.schedule_block_loader import load_schedule_blocks

def load_stages(conn: sqlite3.Connection, config_id: str) -> List[StageDraft]:
    cursor = conn.execute(
        "SELECT id, stage_order_index, stage_type, leg_format "
        "FROM league_calendar_stage_definitions "
        "WHERE league_calendar_config_id = ? ORDER BY stage_order_index ASC;",
        (str(config_id),),
    )
    stage_rows = cursor.fetchall()

    results = []
    for row in stage_rows:
        stage_id = str(row["id"])
        order_idx = int(row["stage_order_index"])
        stage_type = str(row["stage_type"])
        leg_format = str(row["leg_format"]) if row["leg_format"] is not None else None

        pools = load_entry_rule_pools(conn, stage_id)
        blocks = load_schedule_blocks(conn, stage_id)

        results.append(
            StageDraft(
                id=stage_id,
                stage_order_index=order_idx,
                stage_type=stage_type,
                leg_format=leg_format,
                entry_rule_pools=pools,
                schedule_blocks=blocks,
            )
        )
    return results