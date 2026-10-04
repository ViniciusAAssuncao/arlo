import sqlite3
from typing import Dict, List, Sequence, Tuple


def fetch_ids(conn: sqlite3.Connection, query: str, params: Tuple = ()) -> List[str]:
    cursor = conn.execute(query, params)
    return [str(row[0]) for row in cursor.fetchall() if row[0] is not None]


def fetch_ids_in(
    conn: sqlite3.Connection,
    table: str,
    id_col: str,
    fk_col: str,
    fk_values: Sequence[str],
) -> List[str]:
    if not fk_values:
        return []
    results: List[str] = []
    chunk_size = 500
    for i in range(0, len(fk_values), chunk_size):
        chunk = fk_values[i : i + chunk_size]
        placeholders = ",".join(["?"] * len(chunk))
        query = f"SELECT {id_col} FROM {table} WHERE {fk_col} IN ({placeholders});"
        cursor = conn.execute(query, list(chunk))
        for row in cursor.fetchall():
            if row[0] is not None:
                results.append(str(row[0]))
    return results


def merge_resolved_maps(*maps: Dict[str, List[str]]) -> Dict[str, List[str]]:
    merged: Dict[str, List[str]] = {}
    for m in maps:
        for table, ids in m.items():
            if table not in merged:
                merged[table] = []
            for item in ids:
                if item not in merged[table]:
                    merged[table].append(item)
    return merged


def _resolve_player_children(
    conn: sqlite3.Connection, player_ids: List[str]
) -> Dict[str, List[str]]:
    if not player_ids:
        return {}
    attr_ids = fetch_ids_in(conn, "player_attributes", "player_id", "player_id", player_ids)
    pos_ids = fetch_ids_in(conn, "player_positions", "player_id", "player_id", player_ids)
    return {
        "player_attributes": list(set(attr_ids)),
        "player_positions": list(set(pos_ids)),
        "players": player_ids,
    }


def _resolve_manager_children(
    conn: sqlite3.Connection, manager_ids: List[str]
) -> Dict[str, List[str]]:
    if not manager_ids:
        return {}
    attr_ids = fetch_ids_in(conn, "manager_attributes", "manager_id", "manager_id", manager_ids)
    profile_ids = fetch_ids_in(
        conn, "manager_tactical_profiles", "id", "manager_id", manager_ids
    )
    formation_ids = fetch_ids_in(
        conn,
        "manager_preferred_formations",
        "id",
        "manager_tactical_profile_id",
        profile_ids,
    )
    return {
        "manager_preferred_formations": formation_ids,
        "manager_tactical_profiles": profile_ids,
        "manager_attributes": list(set(attr_ids)),
        "managers": manager_ids,
        "persons": manager_ids,
    }


def _resolve_referee_children(
    conn: sqlite3.Connection, referee_ids: List[str]
) -> Dict[str, List[str]]:
    if not referee_ids:
        return {}
    attr_ids = fetch_ids_in(conn, "referee_attributes", "referee_id", "referee_id", referee_ids)
    return {
        "referee_attributes": list(set(attr_ids)),
        "referees": referee_ids,
        "persons": referee_ids,
    }


def _resolve_calendar_config_children(
    conn: sqlite3.Connection, config_ids: List[str]
) -> Dict[str, List[str]]:
    if not config_ids:
        return {}
    weekday_ids = fetch_ids_in(
        conn,
        "league_calendar_matchday_weekdays",
        "id",
        "league_calendar_config_id",
        config_ids,
    )
    criterion_ids = fetch_ids_in(
        conn,
        "league_calendar_tie_break_criteria",
        "id",
        "league_calendar_config_id",
        config_ids,
    )
    group_ids = fetch_ids_in(
        conn, "competition_groups", "id", "league_calendar_config_id", config_ids
    )
    group_team_ids = fetch_ids_in(
        conn, "competition_group_teams", "id", "competition_group_id", group_ids
    )

    stage_ids = fetch_ids_in(
        conn,
        "league_calendar_stage_definitions",
        "id",
        "league_calendar_config_id",
        config_ids,
    )
    pool_ids = fetch_ids_in(
        conn,
        "league_calendar_stage_entry_rule_pools",
        "id",
        "league_calendar_stage_definition_id",
        stage_ids,
    )
    block_ids = fetch_ids_in(
        conn,
        "league_calendar_stage_schedule_blocks",
        "id",
        "league_calendar_stage_definition_id",
        stage_ids,
    )
    pool_group_ids = fetch_ids_in(
        conn,
        "schedule_block_pool_groups",
        "id",
        "schedule_block_id",
        block_ids,
    )

    return {
        "schedule_block_pool_groups": pool_group_ids,
        "league_calendar_stage_schedule_blocks": block_ids,
        "league_calendar_stage_entry_rule_pools": pool_ids,
        "league_calendar_stage_definitions": stage_ids,
        "competition_group_teams": group_team_ids,
        "competition_groups": group_ids,
        "league_calendar_tie_break_criteria": criterion_ids,
        "league_calendar_matchday_weekdays": weekday_ids,
    }


def resolve_players_of_team(
    conn: sqlite3.Connection, team_id: str
) -> Dict[str, List[str]]:
    player_ids = fetch_ids(
        conn, "SELECT id FROM players WHERE team_id = ?;", (str(team_id),)
    )
    return _resolve_player_children(conn, player_ids)


def resolve_players_by_ids(
    conn: sqlite3.Connection, player_ids: List[str]
) -> Dict[str, List[str]]:
    return _resolve_player_children(conn, player_ids)


def resolve_players_of_nationality(
    conn: sqlite3.Connection, country_id: str
) -> Dict[str, List[str]]:
    player_ids = fetch_ids(
        conn, "SELECT id FROM players WHERE nationality_id = ?;", (str(country_id),)
    )
    return _resolve_player_children(conn, player_ids)


def resolve_all_players(conn: sqlite3.Connection) -> Dict[str, List[str]]:
    player_ids = fetch_ids(conn, "SELECT id FROM players;")
    return _resolve_player_children(conn, player_ids)


def resolve_managers_of_team(
    conn: sqlite3.Connection, team_id: str
) -> Dict[str, List[str]]:
    manager_ids = fetch_ids(
        conn, "SELECT id FROM managers WHERE team_id = ?;", (str(team_id),)
    )
    return _resolve_manager_children(conn, manager_ids)


def resolve_all_managers(conn: sqlite3.Connection) -> Dict[str, List[str]]:
    manager_ids = fetch_ids(conn, "SELECT id FROM managers;")
    return _resolve_manager_children(conn, manager_ids)


def resolve_referees_of_league(
    conn: sqlite3.Connection, league_id: str
) -> Dict[str, List[str]]:
    referee_ids = fetch_ids(
        conn,
        "SELECT id FROM referees WHERE primary_league_id = ?;",
        (str(league_id),),
    )
    return _resolve_referee_children(conn, referee_ids)


def resolve_all_referees(conn: sqlite3.Connection) -> Dict[str, List[str]]:
    referee_ids = fetch_ids(conn, "SELECT id FROM referees;")
    return _resolve_referee_children(conn, referee_ids)


def resolve_teams(
    conn: sqlite3.Connection, team_ids: List[str]
) -> Dict[str, List[str]]:
    if not team_ids:
        return {}
    player_ids = fetch_ids_in(conn, "players", "id", "team_id", team_ids)
    players_map = _resolve_player_children(conn, player_ids)

    manager_ids = fetch_ids_in(conn, "managers", "id", "team_id", team_ids)
    managers_map = _resolve_manager_children(conn, manager_ids)

    group_team_ids = fetch_ids_in(
        conn, "competition_group_teams", "id", "team_id", team_ids
    )
    title_ids = fetch_ids_in(conn, "titles", "id", "winner_team_id", team_ids)
    venue_ids = fetch_ids_in(conn, "venues", "id", "owner_team_id", team_ids)

    direct_map = {
        "competition_group_teams": group_team_ids,
        "titles": title_ids,
        "venues": venue_ids,
        "teams": list(team_ids),
    }

    return merge_resolved_maps(players_map, managers_map, direct_map)


def resolve_team(conn: sqlite3.Connection, team_id: str) -> Dict[str, List[str]]:
    return resolve_teams(conn, [str(team_id)])


def resolve_teams_of_league(
    conn: sqlite3.Connection, league_id: str
) -> Dict[str, List[str]]:
    team_ids = fetch_ids(
        conn, "SELECT id FROM teams WHERE league_id = ?;", (str(league_id),)
    )
    return resolve_teams(conn, team_ids)


def resolve_all_teams(conn: sqlite3.Connection) -> Dict[str, List[str]]:
    team_ids = fetch_ids(conn, "SELECT id FROM teams;")
    return resolve_teams(conn, team_ids)


def resolve_competition(
    conn: sqlite3.Connection,
    competition_id: str,
    include_teams: bool = False,
    include_referees: bool = True,
) -> Dict[str, List[str]]:
    comp_id = str(competition_id)
    config_ids = fetch_ids(
        conn,
        "SELECT id FROM league_calendar_configs WHERE competition_id = ?;",
        (comp_id,),
    )
    rule_ids = fetch_ids(
        conn, "SELECT id FROM rules WHERE competition_id = ?;", (comp_id,)
    )
    title_ids = fetch_ids(
        conn, "SELECT id FROM titles WHERE competition_id = ?;", (comp_id,)
    )
    league_ids = fetch_ids(
        conn, "SELECT competition_id FROM leagues WHERE competition_id = ?;", (comp_id,)
    )

    cal_map = _resolve_calendar_config_children(conn, config_ids)

    direct_map = {
        "league_calendar_configs": config_ids,
        "rules": rule_ids,
        "titles": title_ids,
        "leagues": league_ids,
        "competitions": [comp_id],
    }

    maps = [cal_map, direct_map]

    if include_referees:
        maps.append(resolve_referees_of_league(conn, comp_id))

    if include_teams:
        maps.append(resolve_teams_of_league(conn, comp_id))

    return merge_resolved_maps(*maps)


def resolve_all_competitions(
    conn: sqlite3.Connection,
    include_teams: bool = False,
    include_referees: bool = True,
) -> Dict[str, List[str]]:
    comp_ids = fetch_ids(conn, "SELECT id FROM competitions;")
    maps = [
        resolve_competition(
            conn,
            cid,
            include_teams=include_teams,
            include_referees=include_referees,
        )
        for cid in comp_ids
    ]
    return merge_resolved_maps(*maps)


def resolve_country(
    conn: sqlite3.Connection, country_id: str
) -> Dict[str, List[str]]:
    c_id = str(country_id)
    venue_ids = fetch_ids(
        conn, "SELECT id FROM venues WHERE country_id = ?;", (c_id,)
    )
    team_ids = fetch_ids(
        conn, "SELECT id FROM teams WHERE country_id = ?;", (c_id,)
    )
    teams_map = resolve_teams(conn, team_ids)

    comp_ids = fetch_ids(
        conn, "SELECT id FROM competitions WHERE country_id = ?;", (c_id,)
    )
    comp_maps = [
        resolve_competition(conn, cid, include_teams=True, include_referees=True)
        for cid in comp_ids
    ]

    nat_players_map = resolve_players_of_nationality(conn, c_id)

    person_ids = fetch_ids(
        conn, "SELECT id FROM persons WHERE nationality_id = ?;", (c_id,)
    )
    manager_ids_in_persons = fetch_ids_in(conn, "managers", "id", "id", person_ids)
    referee_ids_in_persons = fetch_ids_in(conn, "referees", "id", "id", person_ids)

    mgr_map = _resolve_manager_children(conn, manager_ids_in_persons)
    ref_map = _resolve_referee_children(conn, referee_ids_in_persons)

    direct_map = {
        "venues": venue_ids,
        "persons": person_ids,
        "countries": [c_id],
    }

    return merge_resolved_maps(
        teams_map, nat_players_map, mgr_map, ref_map, direct_map, *comp_maps
    )


def resolve_federation(
    conn: sqlite3.Connection, federation_id: str
) -> Dict[str, List[str]]:
    all_fed_ids = [str(federation_id)]
    to_explore = [str(federation_id)]
    while to_explore:
        curr = to_explore.pop(0)
        children = fetch_ids(
            conn,
            "SELECT id FROM federations WHERE parent_federation_id = ?;",
            (curr,),
        )
        for ch in children:
            if ch not in all_fed_ids:
                all_fed_ids.append(ch)
                to_explore.append(ch)

    comp_ids = fetch_ids_in(conn, "competitions", "id", "federation_id", all_fed_ids)
    title_ids = fetch_ids_in(conn, "titles", "id", "winner_federation_id", all_fed_ids)

    maps = []
    for c_id in comp_ids:
        maps.append(
            resolve_competition(conn, c_id, include_teams=True, include_referees=True)
        )

    direct_map = {
        "titles": title_ids,
        "federations": all_fed_ids,
    }
    return merge_resolved_maps(direct_map, *maps)