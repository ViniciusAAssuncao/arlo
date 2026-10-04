import uuid
from typing import Any, Dict, List, Optional
from arlo_db_tool.domain.competition_group_draft import CompetitionGroupDraft
from arlo_db_tool.schema.field_spec import FieldType
from arlo_db_tool.sql.generic_delete_builder import (
    build_delete_by_fk_statement,
    build_delete_by_id_statement,
)
from arlo_db_tool.sql.value_formatting import format_sql_value

GROUP_FIELD_TYPE_MAP = {
    "order_index": FieldType.INTEGER,
    "name": FieldType.TEXT,
}

def build_group_team_insert_statement(
    group_id: str,
    team_id: str,
    row_id: Optional[str] = None,
) -> str:
    team_row_id = row_id if row_id is not None else str(uuid.uuid4())
    team_row_id_val = format_sql_value(team_row_id, FieldType.UUID_PK)
    group_id_val = format_sql_value(group_id, FieldType.UUID_FK)
    team_id_val = format_sql_value(team_id, FieldType.UUID_FK)
    return (
        f"INSERT INTO competition_group_teams "
        f"(id, competition_group_id, team_id) "
        f"VALUES ({team_row_id_val}, {group_id_val}, {team_id_val});"
    )

def build_group_team_delete_statement(group_id: str, team_id: str) -> str:
    group_id_val = format_sql_value(group_id, FieldType.UUID_FK)
    team_id_val = format_sql_value(team_id, FieldType.UUID_FK)
    return (
        f"DELETE FROM competition_group_teams "
        f"WHERE competition_group_id = {group_id_val} AND team_id = {team_id_val};"
    )

def build_group_insert_statements(
    config_id: str,
    groups: List[CompetitionGroupDraft],
) -> List[str]:
    statements = []
    for group in groups:
        group_id_val = format_sql_value(group.id, FieldType.UUID_PK)
        cfg_id_val = format_sql_value(config_id, FieldType.UUID_FK)
        order_val = format_sql_value(group.order_index, FieldType.INTEGER)
        name_val = format_sql_value(group.name, FieldType.TEXT)
        group_stmt = (
            f"INSERT INTO competition_groups "
            f"(id, league_calendar_config_id, order_index, name) "
            f"VALUES ({group_id_val}, {cfg_id_val}, {order_val}, {name_val});"
        )
        statements.append(group_stmt)

        for team_id in group.team_ids:
            statements.append(build_group_team_insert_statement(group.id, team_id))

    return statements

def build_group_update_statement(
    group_id: str,
    scalar_changes: Dict[str, Any],
) -> str:
    set_clauses = []
    for key, val in scalar_changes.items():
        if key in GROUP_FIELD_TYPE_MAP:
            ft = GROUP_FIELD_TYPE_MAP[key]
            formatted = format_sql_value(val, ft)
            set_clauses.append(f"{key} = {formatted}")
    if not set_clauses:
        return ""
    id_val = format_sql_value(group_id, FieldType.UUID_PK)
    return f"UPDATE competition_groups SET {', '.join(set_clauses)} WHERE id = {id_val};"

def build_group_delete_cascade_statements(group_id: str) -> List[str]:
    return [
        build_delete_by_fk_statement("competition_group_teams", "competition_group_id", group_id),
        build_delete_by_id_statement("competition_groups", group_id),
    ]