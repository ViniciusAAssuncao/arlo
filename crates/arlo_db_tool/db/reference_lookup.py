import sqlite3
from typing import Any, Dict, List, Optional, Tuple
from arlo_db_tool.db.connection import get_db_connection

def get_table_columns(conn: sqlite3.Connection, table: str) -> List[str]:
    try:
        cursor = conn.execute(f"PRAGMA table_info({table});")
        return [row["name"] for row in cursor.fetchall()]
    except sqlite3.Error:
        return []

def get_options(
    db_path: Optional[str],
    table: str,
    id_col: str = "id",
    label_expr: Optional[str] = None,
    where: Optional[str] = None,
) -> List[Tuple[str, str]]:
    if not db_path:
        return []
    try:
        with get_db_connection(db_path) as conn:
            cols = get_table_columns(conn, table)
            if not cols:
                return []
            if id_col not in cols:
                id_col = cols[0]
            if label_expr is None or label_expr not in cols:
                for candidate in ["name", "display_name", "code", "key", "description"]:
                    if candidate in cols:
                        label_expr = candidate
                        break
                if label_expr is None:
                    label_expr = id_col

            query = f"SELECT {id_col} AS id, {label_expr} AS label FROM {table}"
            if where:
                query += f" WHERE {where}"
            query += f" ORDER BY {label_expr} ASC;"

            cursor = conn.execute(query)
            return [(str(row["id"]), str(row["label"])) for row in cursor.fetchall()]
    except Exception:
        return []

def get_record_by_id(
    db_path: Optional[str],
    table: str,
    id_col: str,
    id_val: Any,
) -> Optional[Dict[str, Any]]:
    if not db_path or id_val is None:
        return None
    try:
        with get_db_connection(db_path) as conn:
            cursor = conn.execute(f"SELECT * FROM {table} WHERE {id_col} = ? LIMIT 1;", (str(id_val),))
            row = cursor.fetchone()
            if row is None:
                return None
            return dict(row)
    except Exception:
        return None

def get_attribute_definitions(
    db_path: Optional[str],
    applies_to: str,
) -> List[Dict[str, Any]]:
    if not db_path:
        return []
    try:
        with get_db_connection(db_path) as conn:
            cols = get_table_columns(conn, "attribute_definitions")
            if not cols:
                return []
            cursor = conn.execute(
                "SELECT id, key, display_name, category, applies_to "
                "FROM attribute_definitions WHERE applies_to = ? "
                "ORDER BY category ASC, display_name ASC;",
                (applies_to,),
            )
            return [dict(row) for row in cursor.fetchall()]
    except Exception:
        return []

def get_entity_attributes(
    db_path: Optional[str],
    table_name: str,
    fk_col: str,
    entity_id: Any,
) -> Dict[str, Dict[str, Any]]:
    if not db_path or entity_id is None:
        return {}
    try:
        with get_db_connection(db_path) as conn:
            cols = get_table_columns(conn, table_name)
            if not cols:
                return {}
            cursor = conn.execute(
                f"SELECT attribute_definition_id, value FROM {table_name} WHERE {fk_col} = ?;",
                (str(entity_id),),
            )
            result = {}
            for row in cursor.fetchall():
                result[str(row["attribute_definition_id"])] = {
                    "value": int(row["value"]),
                }
            return result
    except Exception:
        return {}

def get_player_positions(
    db_path: Optional[str],
    player_id: Any,
) -> Dict[str, int]:
    if not db_path or player_id is None:
        return {}
    try:
        with get_db_connection(db_path) as conn:
            cursor = conn.execute(
                "SELECT position, proficiency FROM player_positions WHERE player_id = ?;",
                (str(player_id),),
            )
            return {str(row["position"]): int(row["proficiency"]) for row in cursor.fetchall()}
    except Exception:
        return {}

def get_manager_tactical_profile(
    db_path: Optional[str],
    manager_id: Any,
) -> Optional[Dict[str, Any]]:
    if not db_path or manager_id is None:
        return None
    try:
        with get_db_connection(db_path) as conn:
            cursor = conn.execute("SELECT * FROM manager_tactical_profiles WHERE manager_id = ? LIMIT 1;", (str(manager_id),))
            row = cursor.fetchone()
            if row:
                return dict(row)
    except Exception:
        pass
    return None

def get_manager_preferred_formations(
    db_path: Optional[str],
    profile_id: Any,
) -> List[str]:
    if not db_path or profile_id is None:
        return []
    try:
        with get_db_connection(db_path) as conn:
            cursor = conn.execute("SELECT formation_id FROM manager_preferred_formations WHERE manager_tactical_profile_id = ?;", (str(profile_id),))
            return [str(row["formation_id"]) for row in cursor.fetchall()]
    except Exception:
        pass
    return []

def get_calendar_options(db_path: Optional[str]) -> List[Tuple[str, str]]:
    if not db_path:
        return []
    try:
        with get_db_connection(db_path) as conn:
            cursor = conn.execute("SELECT name FROM sqlite_master WHERE type='table';")
            tables = [r["name"] for r in cursor.fetchall()]

            target_table = None
            for candidate in ["calendar_systems", "calendars", "calendar_definitions"]:
                if candidate in tables:
                    target_table = candidate
                    break

            if target_table is None:
                for tbl in tables:
                    tbl_lower = tbl.lower()
                    if "calendar" in tbl_lower and not any(k in tbl_lower for k in ["week", "day", "config", "stage", "matchday", "month"]):
                        target_table = tbl
                        break

            if not target_table:
                return []

            cols = get_table_columns(conn, target_table)
            id_col = "id" if "id" in cols else cols[0]
            name_col = "name"
            if "name" not in cols:
                for cand in ["display_name", "title", "label", "code", "key"]:
                    if cand in cols:
                        name_col = cand
                        break
                else:
                    name_col = id_col

            cursor = conn.execute(f"SELECT {id_col} AS id, {name_col} AS label FROM {target_table} ORDER BY {name_col} ASC;")
            return [(str(row["id"]), str(row["label"])) for row in cursor.fetchall()]
    except Exception:
        return []

def get_calendar_weekdays(db_path: Optional[str], calendar_id: Optional[str]) -> List[Tuple[int, str]]:
    if not db_path or not calendar_id:
        return []
    try:
        with get_db_connection(db_path) as conn:
            cursor = conn.execute("SELECT name FROM sqlite_master WHERE type='table';")
            tables = [r["name"] for r in cursor.fetchall()]

            for tbl in ["calendar_week_days", "calendar_weekdays", "calendar_days", "weekdays", "calendar_definition_weekdays"]:
                if tbl in tables:
                    cols = get_table_columns(conn, tbl)
                    order_col = "order_index" if "order_index" in cols else (
                        "weekday_order_index" if "weekday_order_index" in cols else (
                            "day_index" if "day_index" in cols else None
                        )
                    )
                    name_col = "name" if "name" in cols else (
                        "display_name" if "display_name" in cols else (
                            "label" if "label" in cols else order_col
                        )
                    )

                    fk_col = None
                    for candidate_fk in ["calendar_system_id", "calendar_id", "calendar_definition_id"]:
                        if candidate_fk in cols:
                            fk_col = candidate_fk
                            break

                    if order_col:
                        if fk_col:
                            cursor = conn.execute(
                                f"SELECT {order_col} AS idx, {name_col} AS label FROM {tbl} WHERE {fk_col} = ? ORDER BY {order_col} ASC;",
                                (str(calendar_id),),
                            )
                        else:
                            cursor = conn.execute(
                                f"SELECT {order_col} AS idx, {name_col} AS label FROM {tbl} ORDER BY {order_col} ASC;"
                            )
                        rows = cursor.fetchall()
                        if rows:
                            return [(int(r["idx"]), str(r["label"])) for r in rows]

            return []
    except Exception:
        return []

def get_calendar_months(db_path: Optional[str], calendar_id: Optional[str]) -> List[Tuple[int, str]]:
    if not db_path or not calendar_id:
        return []
    try:
        with get_db_connection(db_path) as conn:
            cursor = conn.execute("SELECT name FROM sqlite_master WHERE type='table';")
            tables = [r["name"] for r in cursor.fetchall()]

            for tbl in ["calendar_months", "calendar_definition_months", "months", "calendar_system_months"]:
                if tbl in tables:
                    cols = get_table_columns(conn, tbl)
                    order_col = "order_index" if "order_index" in cols else (
                        "month_order_index" if "month_order_index" in cols else (
                            "month_index" if "month_index" in cols else (
                                "month_number" if "month_number" in cols else None
                            )
                        )
                    )
                    name_col = "name" if "name" in cols else (
                        "display_name" if "display_name" in cols else (
                            "label" if "label" in cols else order_col
                        )
                    )

                    fk_col = None
                    for candidate_fk in ["calendar_system_id", "calendar_id", "calendar_definition_id"]:
                        if candidate_fk in cols:
                            fk_col = candidate_fk
                            break

                    if order_col:
                        if fk_col:
                            cursor = conn.execute(
                                f"SELECT {order_col} AS idx, {name_col} AS label FROM {tbl} WHERE {fk_col} = ? ORDER BY {order_col} ASC;",
                                (str(calendar_id),),
                            )
                        else:
                            cursor = conn.execute(
                                f"SELECT {order_col} AS idx, {name_col} AS label FROM {tbl} ORDER BY {order_col} ASC;"
                            )
                        rows = cursor.fetchall()
                        if rows:
                            return [(int(r["idx"]), str(r["label"])) for r in rows]

            return []
    except Exception:
        return []

def get_league_calendar_config_options(db_path: Optional[str]) -> List[Tuple[str, str]]:
    if not db_path:
        return []
    try:
        with get_db_connection(db_path) as conn:
            cols = get_table_columns(conn, "league_calendar_configs")
            if not cols:
                return []
            comp_cols = get_table_columns(conn, "competitions")
            if comp_cols:
                query = (
                    "SELECT lcc.id AS config_id, c.name AS competition_name "
                    "FROM league_calendar_configs lcc "
                    "LEFT JOIN competitions c ON c.id = lcc.competition_id "
                    "ORDER BY c.name ASC, lcc.id ASC;"
                )
                cursor = conn.execute(query)
                results = []
                for row in cursor.fetchall():
                    cfg_id = str(row["config_id"])
                    comp_name = str(row["competition_name"]) if row["competition_name"] else cfg_id
                    results.append((cfg_id, comp_name))
                return results
            else:
                cursor = conn.execute("SELECT id, competition_id FROM league_calendar_configs ORDER BY id ASC;")
                return [(str(row["id"]), str(row["competition_id"])) for row in cursor.fetchall()]
    except Exception:
        return []
