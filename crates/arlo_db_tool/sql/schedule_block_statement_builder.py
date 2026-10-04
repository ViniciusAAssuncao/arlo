import uuid
from typing import Any, Dict, List, Optional
from arlo_db_tool.domain.schedule_block_draft import ScheduleBlockDraft
from arlo_db_tool.schema.field_spec import FieldType
from arlo_db_tool.sql.generic_delete_builder import (
    build_delete_by_fk_statement,
    build_delete_by_id_statement,
)
from arlo_db_tool.sql.value_formatting import format_sql_value

BLOCK_FIELD_TYPE_MAP = {
    "block_order_index": FieldType.INTEGER,
    "block_kind": FieldType.ENUM,
    "group_a_id": FieldType.UUID_FK,
    "group_b_id": FieldType.UUID_FK,
    "mirrored": FieldType.BOOLEAN,
    "pool_kind": FieldType.ENUM,
    "rounds_count": FieldType.INTEGER,
}

def build_schedule_block_pool_group_insert_statement(
    block_id: str,
    group_id: str,
    row_id: Optional[str] = None,
) -> str:
    pg_id = row_id if row_id is not None else str(uuid.uuid4())
    pg_id_val = format_sql_value(pg_id, FieldType.UUID_PK)
    block_id_val = format_sql_value(block_id, FieldType.UUID_FK)
    grp_id_val = format_sql_value(group_id, FieldType.UUID_FK)
    return (
        f"INSERT INTO schedule_block_pool_groups "
        f"(id, schedule_block_id, group_id) "
        f"VALUES ({pg_id_val}, {block_id_val}, {grp_id_val});"
    )

def build_schedule_block_pool_group_delete_statement(
    block_id: str,
    group_id: str,
) -> str:
    block_id_val = format_sql_value(block_id, FieldType.UUID_FK)
    grp_id_val = format_sql_value(group_id, FieldType.UUID_FK)
    return (
        f"DELETE FROM schedule_block_pool_groups "
        f"WHERE schedule_block_id = {block_id_val} AND group_id = {grp_id_val};"
    )

def build_schedule_block_statements(
    stage_id: str,
    schedule_blocks: List[ScheduleBlockDraft],
) -> List[str]:
    statements = []
    for block in schedule_blocks:
        id_val = format_sql_value(block.id, FieldType.UUID_PK)
        stage_id_val = format_sql_value(stage_id, FieldType.UUID_FK)
        order_val = format_sql_value(block.block_order_index, FieldType.INTEGER)
        kind_val = format_sql_value(block.block_kind, FieldType.ENUM)
        group_a_val = format_sql_value(block.group_a_id, FieldType.UUID_FK)
        group_b_val = format_sql_value(block.group_b_id, FieldType.UUID_FK)
        mirrored_val = format_sql_value(block.mirrored, FieldType.BOOLEAN) if block.mirrored is not None else "NULL"
        pool_kind_val = format_sql_value(block.pool_kind, FieldType.ENUM)
        rounds_val = format_sql_value(block.rounds_count, FieldType.INTEGER)

        stmt = (
            f"INSERT INTO league_calendar_stage_schedule_blocks "
            f"(id, league_calendar_stage_definition_id, block_order_index, block_kind, "
            f"group_a_id, group_b_id, mirrored, pool_kind, rounds_count) "
            f"VALUES ({id_val}, {stage_id_val}, {order_val}, {kind_val}, "
            f"{group_a_val}, {group_b_val}, {mirrored_val}, {pool_kind_val}, {rounds_val});"
        )
        statements.append(stmt)

        for group_id in block.pool_group_ids:
            statements.append(build_schedule_block_pool_group_insert_statement(block.id, group_id))

    return statements

def build_schedule_block_update_statement(
    block_id: str,
    scalar_changes: Dict[str, Any],
) -> str:
    set_clauses = []
    for key, val in scalar_changes.items():
        if key in BLOCK_FIELD_TYPE_MAP:
            ft = BLOCK_FIELD_TYPE_MAP[key]
            if ft == FieldType.BOOLEAN:
                formatted = "1" if bool(val) else ("0" if val is not None else "NULL")
            else:
                formatted = format_sql_value(val, ft)
            set_clauses.append(f"{key} = {formatted}")
    if not set_clauses:
        return ""
    id_val = format_sql_value(block_id, FieldType.UUID_PK)
    return f"UPDATE league_calendar_stage_schedule_blocks SET {', '.join(set_clauses)} WHERE id = {id_val};"

def build_schedule_block_delete_cascade_statements(block_id: str) -> List[str]:
    return [
        build_delete_by_fk_statement("schedule_block_pool_groups", "schedule_block_id", block_id),
        build_delete_by_id_statement("league_calendar_stage_schedule_blocks", block_id),
    ]

def build_schedule_blocks_delete_by_stage_statements(stage_id: str) -> List[str]:
    stage_id_val = format_sql_value(stage_id, FieldType.UUID_FK)
    return [
        f"DELETE FROM schedule_block_pool_groups WHERE schedule_block_id IN (SELECT id FROM league_calendar_stage_schedule_blocks WHERE league_calendar_stage_definition_id = {stage_id_val});",
        build_delete_by_fk_statement("league_calendar_stage_schedule_blocks", "league_calendar_stage_definition_id", stage_id),
    ]