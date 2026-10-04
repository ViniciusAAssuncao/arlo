from typing import List, Sequence
from arlo_db_tool.domain.attendance_reference_draft import AttendanceReferenceSuggestion
from arlo_db_tool.schema.field_spec import FieldType
from arlo_db_tool.sql.value_formatting import format_sql_value


def build_attendance_reference_update_statements(
    suggestions: Sequence[AttendanceReferenceSuggestion],
    overwrite_existing: bool,
) -> List[str]:
    statements: List[str] = []
    for suggestion in suggestions:
        has_complete_current = (
            suggestion.current_min is not None
            and suggestion.current_max is not None
        )
        if has_complete_current and not overwrite_existing:
            continue

        team_id = format_sql_value(suggestion.team_id, FieldType.UUID_PK)
        minimum = format_sql_value(suggestion.suggested_min, FieldType.INTEGER)
        maximum = format_sql_value(suggestion.suggested_max, FieldType.INTEGER)
        statements.append(
            "UPDATE teams "
            f"SET min_attendance = {minimum}, max_attendance = {maximum} "
            f"WHERE id = {team_id};"
        )
    return statements
