from typing import List
from arlo_db_tool.generation.referee_draft import RefereeDraft
from arlo_db_tool.schema.entities.referee import REFEREE_SPEC
from arlo_db_tool.sql.statement_builder import (
    build_attribute_insert_statements,
    build_composite_insert_statements,
)


def build_bulk_referee_statements(referees: List[RefereeDraft]) -> List[str]:
    statements: List[str] = []

    for referee in referees:
        referee_values = {
            "id": referee.id,
            "name": referee.name,
            "height_m": referee.height_m,
            "birthdate_unix_seconds": referee.birthdate_unix_seconds,
            "nationality_id": referee.nationality_id,
            "primary_league_id": referee.primary_league_id,
            "tier": referee.tier,
        }
        statements.extend(
            build_composite_insert_statements(
                REFEREE_SPEC, referee_values, shared_id=referee.id
            )
        )
        statements.extend(
            build_attribute_insert_statements(
                "referee_attributes",
                "referee_id",
                referee.id,
                referee.attributes,
            )
        )

    return statements


def generate_bulk_referee_sql(referees: List[RefereeDraft]) -> str:
    stmts = build_bulk_referee_statements(referees)
    if not stmts:
        return ""
    all_lines = ["BEGIN TRANSACTION;"] + stmts + ["COMMIT;"]
    return "\n".join(all_lines)
