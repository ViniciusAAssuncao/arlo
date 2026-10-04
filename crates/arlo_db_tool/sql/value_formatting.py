import math
from typing import Any, Optional
from uuid import UUID
from arlo_db_tool.schema.field_spec import FieldType

def format_sql_value(value: Any, field_type: Optional[FieldType] = None) -> str:
    if value is None:
        return "NULL"

    if isinstance(value, str) and not value.strip() and field_type != FieldType.TEXT:
        return "NULL"

    if field_type == FieldType.BOOLEAN or isinstance(value, bool):
        return "1" if bool(value) else "0"

    if field_type in (FieldType.INTEGER, FieldType.UNIX_TIMESTAMP):
        try:
            return str(int(value))
        except (ValueError, TypeError):
            return "NULL"

    if field_type == FieldType.REAL:
        try:
            val = float(value)
            if not math.isfinite(val):
                return "NULL"
            return f"{val:.6f}".rstrip("0").rstrip(".") if "." in f"{val:.6f}" else str(val)
        except (ValueError, TypeError):
            return "NULL"

    if isinstance(value, UUID):
        escaped = str(value).replace("'", "''")
        return f"'{escaped}'"

    if isinstance(value, (int, float)):
        if isinstance(value, float) and not math.isfinite(value):
            return "NULL"
        return str(value)

    escaped = str(value).replace("'", "''")
    return f"'{escaped}'"
