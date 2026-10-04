from typing import List
from arlo_db_tool.domain.weekday_draft import WeekdayDraft
from arlo_db_tool.schema.field_spec import FieldType
from arlo_db_tool.sql.generic_delete_builder import build_delete_by_fk_statement
from arlo_db_tool.sql.value_formatting import format_sql_value

def build_weekday_insert_statements(
    config_id: str,
    weekdays: List[WeekdayDraft],
) -> List[str]:
    statements = []
    for weekday in weekdays:
        id_val = format_sql_value(weekday.id, FieldType.UUID_PK)
        cfg_id_val = format_sql_value(config_id, FieldType.UUID_FK)
        order_val = format_sql_value(weekday.weekday_order_index, FieldType.INTEGER)
        stmt = (
            f"INSERT INTO league_calendar_matchday_weekdays "
            f"(id, league_calendar_config_id, weekday_order_index) "
            f"VALUES ({id_val}, {cfg_id_val}, {order_val});"
        )
        statements.append(stmt)
    return statements

def build_weekday_delete_all_statement(config_id: str) -> str:
    return build_delete_by_fk_statement(
        "league_calendar_matchday_weekdays",
        "league_calendar_config_id",
        config_id,
    )