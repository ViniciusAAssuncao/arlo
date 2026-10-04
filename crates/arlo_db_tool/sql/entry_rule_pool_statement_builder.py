from typing import Any, Dict, List
from arlo_db_tool.domain.entry_rule_pool_draft import EntryRulePoolDraft
from arlo_db_tool.schema.field_spec import FieldType
from arlo_db_tool.sql.generic_delete_builder import (
    build_delete_by_fk_statement,
    build_delete_by_id_statement,
)
from arlo_db_tool.sql.value_formatting import format_sql_value

POOL_FIELD_TYPE_MAP = {
    "pool_order_index": FieldType.INTEGER,
    "pool_kind": FieldType.ENUM,
    "count": FieldType.INTEGER,
    "position_index": FieldType.INTEGER,
    "range_start_position": FieldType.INTEGER,
    "range_end_position": FieldType.INTEGER,
    "external_competition_id": FieldType.UUID_FK,
}

def build_entry_rule_pool_insert_statement(
    stage_id: str,
    pool: EntryRulePoolDraft,
) -> str:
    id_val = format_sql_value(pool.id, FieldType.UUID_PK)
    stage_id_val = format_sql_value(stage_id, FieldType.UUID_FK)
    order_val = format_sql_value(pool.pool_order_index, FieldType.INTEGER)
    kind_val = format_sql_value(pool.pool_kind, FieldType.ENUM)
    count_val = format_sql_value(pool.count, FieldType.INTEGER)
    pos_val = format_sql_value(pool.position_index, FieldType.INTEGER)
    start_val = format_sql_value(pool.range_start_position, FieldType.INTEGER)
    end_val = format_sql_value(pool.range_end_position, FieldType.INTEGER)
    ext_comp_val = format_sql_value(pool.external_competition_id, FieldType.UUID_FK)

    return (
        f"INSERT INTO league_calendar_stage_entry_rule_pools "
        f"(id, league_calendar_stage_definition_id, pool_order_index, pool_kind, "
        f"count, position_index, range_start_position, range_end_position, external_competition_id) "
        f"VALUES ({id_val}, {stage_id_val}, {order_val}, {kind_val}, "
        f"{count_val}, {pos_val}, {start_val}, {end_val}, {ext_comp_val});"
    )

def build_entry_rule_pool_statements(
    stage_id: str,
    entry_rule_pools: List[EntryRulePoolDraft],
) -> List[str]:
    return [
        build_entry_rule_pool_insert_statement(stage_id, pool)
        for pool in entry_rule_pools
    ]

def build_entry_rule_pool_update_statement(
    pool_id: str,
    scalar_changes: Dict[str, Any],
) -> str:
    set_clauses = []
    for key, val in scalar_changes.items():
        if key in POOL_FIELD_TYPE_MAP:
            ft = POOL_FIELD_TYPE_MAP[key]
            formatted = format_sql_value(val, ft)
            set_clauses.append(f"{key} = {formatted}")
    if not set_clauses:
        return ""
    id_val = format_sql_value(pool_id, FieldType.UUID_PK)
    return f"UPDATE league_calendar_stage_entry_rule_pools SET {', '.join(set_clauses)} WHERE id = {id_val};"

def build_entry_rule_pool_delete_statement(pool_id: str) -> str:
    return build_delete_by_id_statement("league_calendar_stage_entry_rule_pools", pool_id)

def build_entry_rule_pools_delete_by_stage_statement(stage_id: str) -> str:
    return build_delete_by_fk_statement(
        "league_calendar_stage_entry_rule_pools",
        "league_calendar_stage_definition_id",
        stage_id,
    )