from typing import List
from arlo_db_tool.domain.tie_break_criterion_draft import TieBreakCriterionDraft
from arlo_db_tool.schema.field_spec import FieldType
from arlo_db_tool.sql.generic_delete_builder import build_delete_by_fk_statement
from arlo_db_tool.sql.value_formatting import format_sql_value

def build_tie_break_insert_statements(
    config_id: str,
    criteria: List[TieBreakCriterionDraft],
) -> List[str]:
    statements = []
    for criterion in criteria:
        id_val = format_sql_value(criterion.id, FieldType.UUID_PK)
        cfg_id_val = format_sql_value(config_id, FieldType.UUID_FK)
        order_val = format_sql_value(criterion.order_index, FieldType.INTEGER)
        kind_val = format_sql_value(criterion.criterion_kind, FieldType.ENUM)
        stmt = (
            f"INSERT INTO league_calendar_tie_break_criteria "
            f"(id, league_calendar_config_id, order_index, criterion_kind) "
            f"VALUES ({id_val}, {cfg_id_val}, {order_val}, {kind_val});"
        )
        statements.append(stmt)
    return statements

def build_tie_break_delete_all_statement(config_id: str) -> str:
    return build_delete_by_fk_statement(
        "league_calendar_tie_break_criteria",
        "league_calendar_config_id",
        config_id,
    )