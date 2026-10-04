import sqlite3
from typing import Any, Dict, Optional

def load_config_core(conn: sqlite3.Connection, config_id: str) -> Optional[Dict[str, Any]]:
    cursor = conn.execute(
        "SELECT id, competition_id, schedule_algorithm_kind, season_start_month_order_index, "
        "season_start_day_of_month, season_length_weeks, max_games_per_team_per_week, "
        "games_per_week_conflict_scope, postponement_strategy_kind, neutral_opener_enabled, "
        "neutral_opener_selection_strategy, spa_win_weight, spa_draw_weight, spa_loss_weight, "
        "spa_feo_k_factor, qta_home_win_weight, qta_away_win_weight, qta_home_draw_weight, "
        "qta_away_draw_weight, qta_home_loss_weight, qta_away_loss_weight, "
        "standings_stage_order_index, promotion_rule_kind, promotion_count, "
        "promotion_playoff_stage_order_index, promotion_target_league_id, "
        "relegation_rule_kind, relegation_count, relegation_playoff_stage_order_index, "
        "relegation_target_league_id FROM league_calendar_configs WHERE id = ? LIMIT 1;",
        (str(config_id),),
    )
    row = cursor.fetchone()
    if row is None:
        return None

    return {
        "id": str(row["id"]),
        "competition_id": str(row["competition_id"]),
        "algorithm": str(row["schedule_algorithm_kind"]),
        "season_start_month_order_index": int(row["season_start_month_order_index"]),
        "season_start_day_of_month": int(row["season_start_day_of_month"]),
        "season_length_weeks": int(row["season_length_weeks"]),
        "max_games_per_team_per_week": int(row["max_games_per_team_per_week"]),
        "games_per_week_conflict_scope": str(row["games_per_week_conflict_scope"]),
        "postponement_strategy_kind": str(row["postponement_strategy_kind"]),
        "neutral_opener_enabled": bool(row["neutral_opener_enabled"]),
        "neutral_opener_selection_strategy": (
            str(row["neutral_opener_selection_strategy"])
            if row["neutral_opener_selection_strategy"] is not None
            else None
        ),
        "spa_win_weight": float(row["spa_win_weight"]),
        "spa_draw_weight": float(row["spa_draw_weight"]),
        "spa_loss_weight": float(row["spa_loss_weight"]),
        "spa_feo_k_factor": float(row["spa_feo_k_factor"]),
        "qta_home_win_weight": float(row["qta_home_win_weight"]),
        "qta_away_win_weight": float(row["qta_away_win_weight"]),
        "qta_home_draw_weight": float(row["qta_home_draw_weight"]),
        "qta_away_draw_weight": float(row["qta_away_draw_weight"]),
        "qta_home_loss_weight": float(row["qta_home_loss_weight"]),
        "qta_away_loss_weight": float(row["qta_away_loss_weight"]),
        "standings_stage_order_index": int(row["standings_stage_order_index"]),
        "promotion_rule_kind": str(row["promotion_rule_kind"]),
        "promotion_count": (
            int(row["promotion_count"])
            if row["promotion_count"] is not None
            else None
        ),
        "promotion_playoff_stage_order_index": (
            int(row["promotion_playoff_stage_order_index"])
            if row["promotion_playoff_stage_order_index"] is not None
            else None
        ),
        "promotion_target_league_id": (
            str(row["promotion_target_league_id"])
            if row["promotion_target_league_id"] is not None
            else None
        ),
        "relegation_rule_kind": str(row["relegation_rule_kind"]),
        "relegation_count": (
            int(row["relegation_count"])
            if row["relegation_count"] is not None
            else None
        ),
        "relegation_playoff_stage_order_index": (
            int(row["relegation_playoff_stage_order_index"])
            if row["relegation_playoff_stage_order_index"] is not None
            else None
        ),
        "relegation_target_league_id": (
            str(row["relegation_target_league_id"])
            if row["relegation_target_league_id"] is not None
            else None
        ),
    }