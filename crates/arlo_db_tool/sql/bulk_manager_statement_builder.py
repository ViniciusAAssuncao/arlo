from typing import List
from arlo_db_tool.generation.manager_draft import ManagerDraft
from arlo_db_tool.schema.entities.manager import MANAGER_SPEC
from arlo_db_tool.sql.statement_builder import (
    build_attribute_insert_statements,
    build_composite_insert_statements,
)


def build_bulk_manager_statements(managers: List[ManagerDraft]) -> List[str]:
    statements: List[str] = []

    for manager in managers:
        manager_values = {
            "id": manager.id,
            "name": manager.name,
            "height_m": manager.height_m,
            "birthdate_unix_seconds": manager.birthdate_unix_seconds,
            "nationality_id": manager.nationality_id,
            "team_id": manager.team_id,
            "control_mode": manager.control_mode,
        }
        statements.extend(
            build_composite_insert_statements(
                MANAGER_SPEC, manager_values, shared_id=manager.id
            )
        )
        statements.extend(
            build_attribute_insert_statements(
                "manager_attributes",
                "manager_id",
                manager.id,
                manager.attributes,
            )
        )

    return statements


def generate_bulk_manager_sql(managers: List[ManagerDraft]) -> str:
    stmts = build_bulk_manager_statements(managers)
    if not stmts:
        return ""
    all_lines = ["BEGIN TRANSACTION;"] + stmts + ["COMMIT;"]
    return "\n".join(all_lines)
