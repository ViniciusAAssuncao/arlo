from typing import Dict, List, Optional, Sequence
from arlo_db_tool.domain.calendar_social_role_draft import CalendarSocialDay
from arlo_db_tool.schema.field_spec import FieldType
from arlo_db_tool.sql.value_formatting import format_sql_value

ALLOWED_SOCIAL_ROLES = {"workday", "rest_day"}


def build_calendar_social_role_updates(
    days: Sequence[CalendarSocialDay],
    roles_by_id: Dict[str, Optional[str]],
) -> List[str]:
    statements: List[str] = []
    for day in days:
        role = roles_by_id.get(day.id)
        if role not in ALLOWED_SOCIAL_ROLES:
            role = None
        if role == day.social_role:
            continue

        role_sql = format_sql_value(role, FieldType.ENUM)
        day_id = format_sql_value(day.id, FieldType.UUID_PK)
        statements.append(
            f"UPDATE calendar_week_days SET social_role = {role_sql} WHERE id = {day_id};"
        )
    return statements
