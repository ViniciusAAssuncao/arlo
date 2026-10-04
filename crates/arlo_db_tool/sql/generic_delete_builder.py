from typing import Any
from arlo_db_tool.schema.field_spec import FieldType
from arlo_db_tool.sql.value_formatting import format_sql_value


def build_delete_by_id_statement(
    table_name: str,
    id_value: Any,
    id_column: str = "id",
) -> str:
    id_val = format_sql_value(id_value, FieldType.UUID_PK)
    return f"DELETE FROM {table_name} WHERE {id_column} = {id_val};"


def build_delete_by_fk_statement(
    table_name: str,
    fk_column: str,
    fk_value: Any,
) -> str:
    fk_val = format_sql_value(fk_value, FieldType.UUID_FK)
    return f"DELETE FROM {table_name} WHERE {fk_column} = {fk_val};"
