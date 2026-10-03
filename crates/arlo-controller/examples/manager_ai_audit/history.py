import argparse
import collections
import hashlib
import json
import math
import sqlite3
from pathlib import Path


def audit(path):
    path = path.resolve()
    stat = path.stat()
    with path.open("rb") as handle:
        digest = hashlib.file_digest(handle, "sha256").hexdigest()
    result = {"source": str(path), "bytes": stat.st_size,
              "mtime_ns": stat.st_mtime_ns, "sha256": digest}
    with sqlite3.connect(path.as_uri() + "?mode=ro", uri=True) as connection:
        connection.row_factory = sqlite3.Row
        tables = {row[0] for row in connection.execute("SELECT name FROM sqlite_master WHERE type='table'")}
        result["integrity"] = [row[0] for row in connection.execute("PRAGMA integrity_check")]
        result["foreign_key_violations"] = [list(row) for row in connection.execute("PRAGMA foreign_key_check")]
        result["counts"] = {name: connection.execute(f"SELECT COUNT(*) FROM {name}").fetchone()[0]
                            for name in sorted(tables) if name.startswith("match_") or name in ["matches", "fixtures"]}
        result["substitution_reasons"] = dict(connection.execute("SELECT reason, COUNT(*) FROM match_substitutions GROUP BY reason"))
        result["manager_counter_mismatches"] = connection.execute(
            "SELECT COUNT(*) FROM match_manager_decisions d WHERE d.substitutions_made <> "
            "(SELECT COUNT(*) FROM match_substitutions s WHERE s.match_id=d.match_id AND s.team_id=d.team_id)").fetchone()[0]
        result["starter_count_mismatches"] = connection.execute(
            "SELECT COUNT(*) FROM (SELECT match_id, team_id FROM match_squad_selections GROUP BY match_id, team_id HAVING SUM(was_starter)<>14)").fetchone()[0]
        result["substitution_roster_mismatches"] = connection.execute(
            "SELECT COUNT(*) FROM match_substitutions s WHERE player_out_id=player_in_id OR "
            "NOT EXISTS (SELECT 1 FROM match_squad_selections q WHERE q.match_id=s.match_id AND q.team_id=s.team_id AND q.player_id=s.player_out_id) OR "
            "NOT EXISTS (SELECT 1 FROM match_squad_selections q WHERE q.match_id=s.match_id AND q.team_id=s.team_id AND q.player_id=s.player_in_id)").fetchone()[0]
        result["reason_counter_mismatches"] = connection.execute(
            "SELECT COUNT(*) FROM match_manager_substitutions_by_reason d WHERE d.substitutions_count <> "
            "(SELECT COUNT(*) FROM match_substitutions s WHERE s.match_id=d.match_id AND s.team_id=d.team_id AND s.reason=d.reason)").fetchone()[0]
        result["invalid_performance_rows"] = connection.execute(
            "SELECT COUNT(*) FROM match_player_performance WHERE performance_rating NOT BETWEEN 0 AND 10 OR final_rating NOT BETWEEN 0 AND 10 "
            "OR confidence NOT BETWEEN 0 AND 1 OR seconds_played < 0 OR effective_opportunities < 0").fetchone()[0]
        exits = set()
        result["historical_reentries"] = 0
        for row in connection.execute("SELECT * FROM match_substitutions ORDER BY match_id, sequence_number"):
            result["historical_reentries"] += (row["match_id"], row["player_in_id"]) in exits
            exits.add((row["match_id"], row["player_out_id"]))
        if "match_prepared_plans" in tables:
            plans = {}
            groups = collections.defaultdict(list)
            result["invalid_plan_snapshots"] = 0
            for row in connection.execute("SELECT * FROM match_prepared_plans"):
                plan, profile = json.loads(row["plan_json"]), json.loads(row["profile_json"])
                layout = plan["layout"]
                assignments = layout["lineup"]["assignments"]
                ids = {a["player_id"] for a in assignments}
                mandatory = {pos: [a["player_id"] for a in assignments if a["position"] == pos]
                             for pos in ["Goalguard", "Artrine", "Passer"]}
                valid = len(assignments) == len(ids) == len(layout["formation"]["slots"]) == 14
                valid &= {a["formation_slot_index"] for a in assignments} == set(range(14))
                valid &= all(a["position"] == layout["formation"]["slots"][a["formation_slot_index"]]["offensive_position"] for a in assignments)
                valid &= all(len(value) == 1 for value in mandatory.values())
                valid &= row["plan_id"] == plan["id"] and plan["profile_id"] == profile["id"]
                valid &= row["team_id"] == profile["team_id"] == layout["lineup"]["team_id"]
                valid &= all(sum(a["slot_role"] == role for a in assignments) <= maximum
                             for role, maximum in [("FalseArtrine", 1), ("Launcher", 1), ("Safeguard", 1), ("Kicker", 1), ("Blocker", 3)])
                valid &= finite(profile["instructions"])
                result["invalid_plan_snapshots"] += not valid
                key = (row["match_id"], row["team_id"], row["plan_id"])
                plans[key] = plan
                groups[key[:2]].append((ids, mandatory))
            result["plan_membership_mismatches"] = sum(any(value != group[0] for value in group) for group in groups.values())
            result["repertoire_distribution"] = dict(sorted(collections.Counter(len(group) for group in groups.values()).items()))
            result["invalid_plan_activations"] = 0
            for row in connection.execute("SELECT * FROM match_tactical_plan_activations"):
                plan = plans.get((row["match_id"], row["team_id"], row["plan_id"]))
                assignments = json.loads(row["assignments_json"])
                valid = plan is not None and len(assignments) == len({a["player_id"] for a in assignments}) == 14
                if plan:
                    valid &= row["formation_id"] == plan["layout"]["formation"]["id"] and row["profile_id"] == plan["profile_id"]
                valid &= row["total_elapsed_seconds"] >= row["seconds_in_period"] >= 0
                valid &= {a["formation_slot_index"] for a in assignments} == set(range(14))
                squad = {value[0] for value in connection.execute("SELECT player_id FROM match_squad_selections WHERE match_id=? AND team_id=?", (row["match_id"], row["team_id"]))}
                valid &= all(a["player_id"] in squad for a in assignments)
                result["invalid_plan_activations"] += not valid
        if result["integrity"] != ["ok"] or result["foreign_key_violations"]:
            raise ValueError(f"Invalid database: {path}")
        issues = {key: value for key, value in result.items() if (key.endswith("mismatches") or key.startswith("invalid_")) and value}
        result["issues"] = issues
    connection.close()
    return result


def finite(value):
    if isinstance(value, dict):
        return all(finite(item) for item in value.values())
    if isinstance(value, list):
        return all(finite(item) for item in value)
    return not isinstance(value, float) or math.isfinite(value)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("saves", nargs="+")
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    results = [audit(Path(path)) for path in args.saves]
    Path(args.output).write_text(json.dumps(results, indent=2), encoding="utf-8")
    for result in results:
        print(Path(result["source"]).name, result["counts"]["matches"], result["substitution_reasons"], "issues", result["issues"])
    raise SystemExit(int(any(result["issues"] for result in results)))
