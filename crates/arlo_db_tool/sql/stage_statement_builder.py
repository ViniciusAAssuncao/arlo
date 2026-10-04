from typing import Any, Dict, List
from arlo_db_tool.domain.stage_draft import StageDraft
from arlo_db_tool.schema.field_spec import FieldType
from arlo_db_tool.sql.entry_rule_pool_statement_builder import (
    build_entry_rule_pool_statements,
    build_entry_rule_pools_delete_by_stage_statement,
)
from arlo_db_tool.sql.generic_delete_builder import (
    build_delete_by_id_statement,
)
from arlo_db_tool.sql.schedule_block_statement_builder import (
    build_schedule_block_statements,
    build_schedule_blocks_delete_by_stage_statements,
)
from arlo_db_tool.sql.value_formatting import format_sql_value

STAGE_FIELD_TYPE_MAP = {
    "stage_order_index": FieldType.INTEGER,
    "stage_type": FieldType.ENUM,
    "leg_format": FieldType.ENUM,
}

def build_stage_statements(
    config_id: str,
    stage: StageDraft,
) -> List[str]:
    statements = []
    id_val = format_sql_value(stage.id, FieldType.UUID_PK)
    cfg_id_val = format_sql_value(config_id, FieldType.UUID_FK)
    order_val = format_sql_value(stage.stage_order_index, FieldType.INTEGER)
    type_val = format_sql_value(stage.stage_type, FieldType.ENUM)
    leg_val = format_sql_value(stage.leg_format, FieldType.ENUM)

    stage_stmt = (
        f"INSERT INTO league_calendar_stage_definitions "
        f"(id, league_calendar_config_id, stage_order_index, stage_type, leg_format) "
        f"VALUES ({id_val}, {cfg_id_val}, {order_val}, {type_val}, {leg_val});"
    )
    statements.append(stage_stmt)

    statements.extend(build_entry_rule_pool_statements(stage.id, stage.entry_rule_pools))
    statements.extend(build_schedule_block_statements(stage.id, stage.schedule_blocks))

    return statements

def build_stage_update_statement(
    stage_id: str,
    scalar_changes: Dict[str, Any],
) -> str:
    set_clauses = []
    for key, val in scalar_changes.items():
        if key in STAGE_FIELD_TYPE_MAP:
            ft = STAGE_FIELD_TYPE_MAP[key]
            formatted = format_sql_value(val, ft)
            set_clauses.append(f"{key} = {formatted}")
    if not set_clauses:
        return ""
    id_val = format_sql_value(stage_id, FieldType.UUID_PK)
    return f"UPDATE league_calendar_stage_definitions SET {', '.join(set_clauses)} WHERE id = {id_val};"

def build_stage_delete_cascade_statements(stage_id: str) -> List[str]:
    statements = []
    statements.extend(build_schedule_blocks_delete_by_stage_statements(stage_id))
    statements.append(build_entry_rule_pools_delete_by_stage_statement(stage_id))
    statements.append(build_delete_by_id_statement("league_calendar_stage_definitions", stage_id))
    return statements