from typing import Dict, List
from arlo_db_tool.deletion.cascade_graph import (
    TABLE_DELETION_ORDER,
    get_table_primary_key,
)


def build_cascade_delete_statements(resolved_ids: Dict[str, List[str]]) -> List[str]:
    statements: List[str] = []

    for table in TABLE_DELETION_ORDER:
        ids = resolved_ids.get(table, [])
        if not ids:
            continue

        pk_col = get_table_primary_key(table)
        chunk_size = 500

        for i in range(0, len(ids), chunk_size):
            chunk = ids[i : i + chunk_size]
            if len(chunk) == 1:
                escaped = str(chunk[0]).replace("'", "''")
                statements.append(f"DELETE FROM {table} WHERE {pk_col} = '{escaped}';")
            else:
                formatted_ids = ", ".join(f"'{str(x).replace('\'', '\'\'')}'" for x in chunk)
                statements.append(f"DELETE FROM {table} WHERE {pk_col} IN ({formatted_ids});")

    return statements


def generate_cascade_delete_sql(resolved_ids: Dict[str, List[str]]) -> str:
    stmts = build_cascade_delete_statements(resolved_ids)
    if not stmts:
        return ""
    all_lines = ["BEGIN TRANSACTION;"] + stmts + ["COMMIT;"]
    return "\n".join(all_lines)
