import argparse
import json
import pathlib

from audit_support import compare, compare_captures, normalize_migrations, read_database, snapshot


def logical_order(connection, table, columns):
    names = {column[1] for column in columns}
    primary = [column[1] for column in sorted(columns, key=lambda c: c[5]) if column[5]]
    if primary:
        return primary
    for index in connection.execute(f'PRAGMA index_list("{table}")'):
        if not index[2]:
            continue
        fields = [row[2] for row in connection.execute(f'PRAGMA index_info("{index[1]}")')]
        if fields and all(field in names for field in fields):
            return fields
    return [
        column[1] for column in columns
        if "REAL" not in column[2].upper() and not column[1].endswith("_json")
    ]


def compare_databases(before, after, captures):
    identifiers = [
        json.loads(path.read_text(encoding="utf-8"))["match_id"]
        for path in sorted(pathlib.Path(captures).glob("*.input.json"))
    ]
    if not identifiers:
        raise ValueError("No captured fixtures")
    placeholders = ",".join("?" for _ in identifiers)
    tables = {}
    with read_database(before) as left, read_database(after) as right:
        for table, in left.execute("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"):
            columns = list(left.execute(f'PRAGMA table_info("{table}")'))
            names = [column[1] for column in columns]
            identifier = "id" if table == "matches" else "match_id"
            if identifier not in names:
                continue
            columns = [column for column in columns if (column[1] != "id" or table == "matches") and column[1] != "created_at_unix_seconds"]
            fields = ",".join(f'"{column[1]}"' for column in columns)
            order = ",".join(f'"{name}"' for name in logical_order(left, table, columns))
            query = f'SELECT {fields} FROM "{table}" WHERE "{identifier}" IN ({placeholders})'
            if order:
                query += " ORDER BY " + order
            before_rows = left.execute(query, identifiers).fetchall()
            after_rows = right.execute(query, identifiers).fetchall()
            if len(before_rows) != len(after_rows):
                raise ValueError(f"Row count differs: {table}: {len(before_rows)} vs {len(after_rows)}")
            for index, (a, b) in enumerate(zip(before_rows, after_rows)):
                for column, av, bv in zip(columns, a, b):
                    if column[1].endswith("_json") and av is not None and bv is not None:
                        av, bv = json.loads(av), json.loads(bv)
                    if not compare(av, bv):
                        raise ValueError(f"Persisted value differs: {table}, row {index}, column {column[1]}: {av!r} vs {bv!r}")
            tables[table] = len(before_rows)
            import os
            import sys
            if os.environ.get("ARLO_PERF_PROGRESS"):
                print(f"{table}: {len(before_rows)} equal rows", file=sys.stderr, flush=True)
        integrity = right.execute("PRAGMA integrity_check").fetchall()
        foreign_keys = right.execute("PRAGMA foreign_key_check").fetchall()
        if integrity != [("ok",)] or foreign_keys:
            raise ValueError(f"Database integrity: {integrity}, foreign keys: {foreign_keys[:5]}")
    return {"matches": len(identifiers), "tables": tables, "equal_rows": sum(tables.values()), "integrity": "ok", "foreign_keys": "ok"}


def summarize(path):
    import statistics

    reports = json.loads(pathlib.Path(path).read_text(encoding="utf-8"))
    fixtures = reports["fixtures"]
    values = [value for fixture in fixtures for value in fixture.get("simulation_ms", [])]

    def distribution(items):
        items = sorted(items)
        return {
            "n": len(items), "mean": statistics.mean(items), "median": statistics.median(items),
            "p95": items[min(len(items) - 1, int(len(items) * 0.95))], "total": sum(items),
        } if items else None

    result = {"fixtures": len(fixtures), "simulation_ms": distribution(values)}
    result["preparation_ms"] = distribution([f["preparation_ms"] for f in fixtures if "preparation_ms" in f])
    for key in ("write_ms", "finish_ms"):
        result[key] = distribution([p[key] for f in fixtures for p in f.get("persistence", [])])
    for key in ("allocation_count", "allocated_bytes"):
        result[key] = distribution([f[key] for f in fixtures if key in f])
    if fixtures and "profile" in fixtures[0]:
        result["profile_means"] = {
            key: statistics.mean(f["profile"][key] for f in fixtures)
            for key in fixtures[0]["profile"]
        }
    return result


parser = argparse.ArgumentParser()
parser.add_argument("command", choices=["snapshot", "normalize-migrations", "compare-captures", "compare-databases", "summarize"])
parser.add_argument("paths", nargs="+")
arguments = parser.parse_args()
commands = {
    "snapshot": snapshot,
    "normalize-migrations": normalize_migrations,
    "compare-captures": compare_captures,
    "compare-databases": compare_databases,
    "summarize": summarize,
}
print(json.dumps(commands[arguments.command](*arguments.paths), indent=2, ensure_ascii=False))
