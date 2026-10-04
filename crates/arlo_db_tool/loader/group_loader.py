import sqlite3
from typing import List
from arlo_db_tool.domain.competition_group_draft import CompetitionGroupDraft

def load_competition_groups(conn: sqlite3.Connection, config_id: str) -> List[CompetitionGroupDraft]:
    cursor = conn.execute(
        "SELECT id, order_index, name FROM competition_groups "
        "WHERE league_calendar_config_id = ? ORDER BY order_index ASC;",
        (str(config_id),),
    )
    group_rows = cursor.fetchall()

    results = []
    for row in group_rows:
        group_id = str(row["id"])
        order_idx = int(row["order_index"])
        name = str(row["name"])

        teams_cursor = conn.execute(
            "SELECT team_id FROM competition_group_teams "
            "WHERE competition_group_id = ? ORDER BY rowid ASC;",
            (group_id,),
        )
        team_ids = [str(t_row["team_id"]) for t_row in teams_cursor.fetchall()]

        results.append(
            CompetitionGroupDraft(
                id=group_id,
                order_index=order_idx,
                name=name,
                team_ids=team_ids,
            )
        )
    return results

load_groups = load_competition_groups