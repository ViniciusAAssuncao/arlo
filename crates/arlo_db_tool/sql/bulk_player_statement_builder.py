from typing import List
from arlo_db_tool.generation.player_draft import PlayerDraft
from arlo_db_tool.schema.entities.player import PLAYER_SPEC
from arlo_db_tool.schema.entities.player_position import PLAYER_POSITION_SPEC
from arlo_db_tool.sql.statement_builder import (
    build_attribute_insert_statements,
    build_insert_statement,
)


def build_bulk_player_statements(players: List[PlayerDraft]) -> List[str]:
    statements: List[str] = []

    for player in players:
        player_values = {
            "id": player.id,
            "name": player.name,
            "height_m": player.height_m,
            "birthdate_unix_seconds": player.birthdate_unix_seconds,
            "nationality_id": player.nationality_id,
            "team_id": player.team_id,
            "squad_number": player.squad_number,
            "captaincy_role": player.captaincy_role,
        }
        statements.append(build_insert_statement(PLAYER_SPEC, player_values))

        for pos in player.positions:
            pos_values = {
                "player_id": player.id,
                "position": pos.position,
                "proficiency": pos.proficiency,
            }
            statements.append(build_insert_statement(PLAYER_POSITION_SPEC, pos_values))

        statements.extend(
            build_attribute_insert_statements(
                "player_attributes",
                "player_id",
                player.id,
                player.attributes,
            )
        )

    return statements


def generate_bulk_player_sql(players: List[PlayerDraft]) -> str:
    stmts = build_bulk_player_statements(players)
    if not stmts:
        return ""
    all_lines = ["BEGIN TRANSACTION;"] + stmts + ["COMMIT;"]
    return "\n".join(all_lines)