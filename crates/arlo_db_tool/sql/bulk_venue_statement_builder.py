from typing import List
from arlo_db_tool.generation.bulk_venue_batch_generator import (
    build_venue_insert_statements,
)
from arlo_db_tool.generation.venue_draft import VenueDraft


def build_bulk_venue_statements(
    venues: List[VenueDraft],
    update_teams_home_venue: bool = False,
) -> List[str]:
    return build_venue_insert_statements(
        venues, update_teams_home_venue=update_teams_home_venue
    )


def generate_bulk_venue_sql(
    venues: List[VenueDraft],
    update_teams_home_venue: bool = False,
) -> str:
    stmts = build_bulk_venue_statements(
        venues, update_teams_home_venue=update_teams_home_venue
    )
    if not stmts:
        return ""
    all_lines = ["BEGIN TRANSACTION;"] + stmts + ["COMMIT;"]
    return "\n".join(all_lines)
