import argparse
import json
import sqlite3
from statistics import mean, median
from collections import Counter, defaultdict
from pathlib import Path


def rows(connection, query):
    return connection.execute(query).fetchall()


def load_participation(connection):
    return rows(
        connection,
        """
        SELECT selection.match_id, selection.team_id, selection.player_id,
               COALESCE(slot.position, 'Unknown') AS position,
               COALESCE(touches.total_touches, 0) AS touches,
               COALESCE(duels.total_duels, 0) AS duels,
               COALESCE(receiving.targets, 0) AS targets,
               COALESCE(shooting.attempts, 0) AS attempts,
               COALESCE(drives.total_drives, 0) AS drives
        FROM match_squad_selections AS selection
        LEFT JOIN match_team_lineup_usage AS lineup
          ON lineup.match_id = selection.match_id
         AND lineup.team_id = selection.team_id
        LEFT JOIN formation_slots AS slot
          ON slot.formation_id = lineup.formation_id
         AND slot.slot_index = selection.formation_slot_index
        LEFT JOIN match_player_touches AS touches
          ON touches.match_id = selection.match_id
         AND touches.player_id = selection.player_id
        LEFT JOIN match_player_duels AS duels
          ON duels.match_id = selection.match_id
         AND duels.player_id = selection.player_id
        LEFT JOIN match_player_receiving AS receiving
          ON receiving.match_id = selection.match_id
         AND receiving.player_id = selection.player_id
        LEFT JOIN match_player_scoring_attempts AS shooting
          ON shooting.match_id = selection.match_id
         AND shooting.player_id = selection.player_id
        LEFT JOIN match_player_drives AS drives
          ON drives.match_id = selection.match_id
         AND drives.player_id = selection.player_id
        WHERE selection.was_starter = 1
        """,
    )


def scoring_integrity(connection):
    matches = rows(
        connection,
        "SELECT matches.id, matches.home_team_id, matches.away_team_id, "
        "fixtures.home_score, fixtures.away_score "
        "FROM matches LEFT JOIN fixtures ON fixtures.id = matches.fixture_id",
    )
    scores = rows(
        connection,
        "SELECT match_id, team_id, goal_points, field_points, field_goals, total_points "
        "FROM match_team_scores",
    )
    plays = rows(
        connection,
        "SELECT match_id, team_id, scorer_id, sequence_number, period, seconds_in_period, "
        "play_type, points, scoring_post, drives_completed FROM match_scoring_plays "
        "ORDER BY match_id, sequence_number",
    )
    attempts = rows(
        connection,
        "SELECT match_id, player_id, goal_points_scored, field_points_scored, "
        "field_goals_scored, total_points_scored FROM match_player_scoring_attempts",
    )
    selected = {
        (row["match_id"], row["player_id"])
        for row in rows(
            connection,
            "SELECT match_id, player_id FROM match_squad_selections",
        )
    }
    by_team = defaultdict(list)
    by_match = defaultdict(list)
    by_scorer = defaultdict(list)
    issues = []
    counts = Counter()
    score_by_team = {(row["match_id"], row["team_id"]): row for row in scores}
    for match in matches:
        for side in ("home", "away"):
            team_id = match[f"{side}_team_id"]
            score = score_by_team.get((match["id"], team_id))
            if score is None:
                issues.append(
                    {
                        "kind": "missing_team_score",
                        "match_id": match["id"],
                        "team_id": team_id,
                    }
                )
            elif match[f"{side}_score"] is not None and (
                match[f"{side}_score"] != score["total_points"]
            ):
                issues.append(
                    {
                        "kind": "fixture_score_mismatch",
                        "match_id": match["id"],
                        "team_id": team_id,
                        "fixture_points": match[f"{side}_score"],
                        "team_score_points": score["total_points"],
                    }
                )
    for play in plays:
        by_team[(play["match_id"], play["team_id"])].append(play)
        by_match[play["match_id"]].append(play)
        by_scorer[(play["match_id"], play["scorer_id"])].append(play)
        counts[play["play_type"]] += 1
        expected_points = {
            "GoalPoint": 5,
            "FieldPoint": 3,
            "FieldGoal": 2 if play["scoring_post"] == "Goalpost" else 1,
            "MissedAttempt": 0,
        }.get(play["play_type"])
        if expected_points is None or expected_points != play["points"]:
            issues.append(
                {
                    "kind": "invalid_event_points",
                    "match_id": play["match_id"],
                    "sequence": play["sequence_number"],
                    "play_type": play["play_type"],
                    "points": play["points"],
                }
            )
        if (play["match_id"], play["scorer_id"]) not in selected:
            issues.append(
                {
                    "kind": "scorer_outside_squad",
                    "match_id": play["match_id"],
                    "sequence": play["sequence_number"],
                    "player_id": play["scorer_id"],
                }
            )
    for score in scores:
        key = (score["match_id"], score["team_id"])
        team_plays = by_team[key]
        event_counts = Counter(
            play["play_type"] for play in team_plays if play["points"] > 0
        )
        event_points = sum(play["points"] for play in team_plays)
        expected_counts = {
            "GoalPoint": score["goal_points"],
            "FieldPoint": score["field_points"],
            "FieldGoal": score["field_goals"],
        }
        if event_points != score["total_points"] or any(
            event_counts[kind] != value for kind, value in expected_counts.items()
        ):
            issues.append(
                {
                    "kind": "team_score_mismatch",
                    "match_id": score["match_id"],
                    "team_id": score["team_id"],
                    "stored_points": score["total_points"],
                    "event_points": event_points,
                    "stored_counts": expected_counts,
                    "event_counts": dict(event_counts),
                }
            )
    for match_id, match_plays in by_match.items():
        for index, play in enumerate(match_plays):
            if index:
                previous = match_plays[index - 1]
                if (
                    play["sequence_number"] <= previous["sequence_number"]
                    or (play["period"], play["seconds_in_period"])
                    < (previous["period"], previous["seconds_in_period"])
                ):
                    issues.append(
                        {
                            "kind": "scoring_sequence_out_of_order",
                            "match_id": match_id,
                            "sequence": play["sequence_number"],
                        }
                    )
            if (match_id, play["team_id"]) not in score_by_team:
                issues.append(
                    {
                        "kind": "scoring_event_without_team_score",
                        "match_id": match_id,
                        "sequence": play["sequence_number"],
                    }
                )
            if play["play_type"] == "GoalPoint" and (play["drives_completed"] or 0) > 0:
                following = match_plays[index + 1] if index + 1 < len(match_plays) else None
                valid_attempt = (
                    following is not None
                    and following["play_type"] in ("FieldGoal", "MissedAttempt")
                    and following["team_id"] == play["team_id"]
                    and following["period"] == play["period"]
                    and 0
                    <= following["seconds_in_period"] - play["seconds_in_period"]
                    <= 5.01
                )
                if not valid_attempt and play["seconds_in_period"] < 2700:
                    issues.append(
                        {
                            "kind": "regular_goal_point_without_bonus_attempt",
                            "match_id": match_id,
                            "sequence": play["sequence_number"],
                        }
                    )
            if play["play_type"] != "FieldGoal":
                continue
            previous = match_plays[index - 1] if index else None
            valid = (
                previous is not None
                and previous["play_type"] == "GoalPoint"
                and previous["drives_completed"] is not None
                and previous["drives_completed"] > 0
                and previous["team_id"] == play["team_id"]
                and previous["period"] == play["period"]
                and 0 <= play["seconds_in_period"] - previous["seconds_in_period"] <= 5.01
            )
            if not valid:
                issues.append(
                    {
                        "kind": "bonus_without_prior_regular_goal_point",
                        "match_id": match_id,
                        "sequence": play["sequence_number"],
                        "previous_scoring_sequence": (
                            previous["sequence_number"] if previous else None
                        ),
                    }
                )
    scorer_stats = {
        (row["match_id"], row["player_id"]): row for row in attempts
    }
    for key, scorer_plays in by_scorer.items():
        event_counts = Counter(
            play["play_type"] for play in scorer_plays if play["points"] > 0
        )
        event_points = sum(play["points"] for play in scorer_plays)
        if event_points == 0:
            continue
        stat = scorer_stats.get(key)
        if stat is None or (
            stat["goal_points_scored"] != event_counts["GoalPoint"]
            or stat["field_points_scored"] != event_counts["FieldPoint"]
            or stat["field_goals_scored"] != event_counts["FieldGoal"]
            or stat["total_points_scored"] != event_points
        ):
            issues.append(
                {
                    "kind": "player_scoring_mismatch",
                    "match_id": key[0],
                    "player_id": key[1],
                    "event_points": event_points,
                    "stored_points": (
                        stat["total_points_scored"] if stat is not None else None
                    ),
                }
            )
    by_team_points = defaultdict(lambda: Counter())
    for play in plays:
        if play["points"] > 0:
            by_team_points[(play["match_id"], play["team_id"])][
                play["scorer_id"]
            ] += play["points"]
    largest_scorer_shares = [
        max(player_points.values()) / sum(player_points.values())
        for player_points in by_team_points.values()
    ]
    scorers_per_team = [len(player_points) for player_points in by_team_points.values()]
    baseline = {
        "team_scores": len(scores),
        "mean_points_per_team": (
            mean(score["total_points"] for score in scores) if scores else None
        ),
        "median_points_per_team": (
            median(score["total_points"] for score in scores) if scores else None
        ),
        "mean_goal_points_per_team": (
            mean(score["goal_points"] for score in scores) if scores else None
        ),
        "mean_field_points_per_team": (
            mean(score["field_points"] for score in scores) if scores else None
        ),
        "mean_field_goals_per_team": (
            mean(score["field_goals"] for score in scores) if scores else None
        ),
        "mean_largest_scorer_point_share_per_team": (
            mean(largest_scorer_shares) if largest_scorer_shares else None
        ),
        "mean_scorers_per_team": (
            mean(scorers_per_team) if scorers_per_team else None
        ),
    }
    return counts, issues, baseline


def participation_summary(connection):
    starters = load_participation(connection)
    by_position = defaultdict(lambda: Counter())
    by_team = defaultdict(list)
    for player in starters:
        active = any(
            player[field] > 0
            for field in ("touches", "duels", "targets", "attempts", "drives")
        )
        position = by_position[player["position"]]
        position["starters"] += 1
        position["active"] += int(active)
        position["touches"] += player["touches"]
        position["duels"] += player["duels"]
        by_team[(player["match_id"], player["team_id"])].append(player)
    touch_shares = []
    for players in by_team.values():
        total = sum(player["touches"] for player in players)
        if total:
            touch_shares.append(max(player["touches"] for player in players) / total)
    return {
        "starter_appearances": len(starters),
        "zero_involvement": sum(
            all(
                player[field] == 0
                for field in ("touches", "duels", "targets", "attempts", "drives")
            )
            for player in starters
        ),
        "positions": {name: dict(counts) for name, counts in sorted(by_position.items())},
        "mean_largest_touch_share_per_team": (
            sum(touch_shares) / len(touch_shares) if touch_shares else None
        ),
    }


def audit(path):
    uri = f"file:{path.resolve().as_posix()}?mode=ro"
    with sqlite3.connect(uri, uri=True) as connection:
        connection.row_factory = sqlite3.Row
        matches = rows(connection, "SELECT id FROM matches")
        counts, issues, scores = scoring_integrity(connection)
        turnover = connection.execute(
            "SELECT COUNT(*) AS total, "
            "SUM(CASE WHEN recovering_player_id IS NULL THEN 1 ELSE 0 END) AS unnamed, "
            "SUM(CASE WHEN lost_by_player_id IS NULL THEN 1 ELSE 0 END) AS unnamed_loss "
            "FROM match_turnovers"
        ).fetchone()
        touches = connection.execute(
            "SELECT SUM(passes_attempted) AS passes_attempted, "
            "SUM(passes_received) AS passes_received, "
            "SUM(total_touches) AS total_touches FROM match_player_touches"
        ).fetchone()
        duel_count = connection.execute(
            "SELECT COALESCE(SUM(total_duels), 0) FROM match_player_duels"
        ).fetchone()[0]
        scoring_by_position = rows(
            connection,
            "SELECT COALESCE(slot.position, 'Unknown') AS position, "
            "play.play_type, COUNT(*) AS events "
            "FROM match_scoring_plays AS play "
            "JOIN match_squad_selections AS selection "
            "ON selection.match_id = play.match_id "
            "AND selection.player_id = play.scorer_id "
            "JOIN match_team_lineup_usage AS lineup "
            "ON lineup.match_id = selection.match_id "
            "AND lineup.team_id = selection.team_id "
            "LEFT JOIN formation_slots AS slot "
            "ON slot.formation_id = lineup.formation_id "
            "AND slot.slot_index = selection.formation_slot_index "
            "WHERE play.points > 0 "
            "GROUP BY position, play.play_type "
            "ORDER BY position, play.play_type",
        )
        return {
            "database": str(path.resolve()),
            "matches": len(matches),
            "scoring_events": dict(counts),
            "score_baseline": scores,
            "integrity_issues": issues,
            "participation": participation_summary(connection),
            "scoring_by_position": [dict(row) for row in scoring_by_position],
            "activity_totals": {
                "passes_attempted": touches["passes_attempted"] or 0,
                "passes_received": touches["passes_received"] or 0,
                "total_touches": touches["total_touches"] or 0,
                "duels": duel_count,
            },
            "turnovers": {
                "total": turnover["total"],
                "without_named_recoverer": turnover["unnamed"] or 0,
                "without_named_loser": turnover["unnamed_loss"] or 0,
            },
        }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("database", type=Path)
    args = parser.parse_args()
    print(json.dumps(audit(args.database), ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
