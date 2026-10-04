import sqlite3
from typing import Optional
from arlo_db_tool.db.connection import get_db_connection
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft
from arlo_db_tool.loader.config_core_loader import load_config_core
from arlo_db_tool.loader.group_loader import load_competition_groups
from arlo_db_tool.loader.stage_loader import load_stages
from arlo_db_tool.loader.tie_break_loader import load_tie_break_criteria
from arlo_db_tool.loader.weekday_loader import load_weekdays

def load_league_calendar_config_draft(
    conn: sqlite3.Connection,
    config_id: str,
) -> Optional[LeagueCalendarConfigDraft]:
    core = load_config_core(conn, config_id)
    if core is None:
        return None

    weekdays = load_weekdays(conn, config_id)
    tie_break_criteria = load_tie_break_criteria(conn, config_id)
    groups = load_competition_groups(conn, config_id)
    stages = load_stages(conn, config_id)

    draft = LeagueCalendarConfigDraft(
        id=core["id"],
        competition_id=core["competition_id"],
        algorithm=core["algorithm"],
        season_start_month_order_index=core["season_start_month_order_index"],
        season_start_day_of_month=core["season_start_day_of_month"],
        season_length_weeks=core["season_length_weeks"],
        max_games_per_team_per_week=core["max_games_per_team_per_week"],
        games_per_week_conflict_scope=core["games_per_week_conflict_scope"],
        postponement_strategy_kind=core["postponement_strategy_kind"],
        neutral_opener_enabled=core["neutral_opener_enabled"],
        neutral_opener_selection_strategy=core["neutral_opener_selection_strategy"],
        spa_win_weight=core["spa_win_weight"],
        spa_draw_weight=core["spa_draw_weight"],
        spa_loss_weight=core["spa_loss_weight"],
        spa_feo_k_factor=core["spa_feo_k_factor"],
        qta_home_win_weight=core["qta_home_win_weight"],
        qta_away_win_weight=core["qta_away_win_weight"],
        qta_home_draw_weight=core["qta_home_draw_weight"],
        qta_away_draw_weight=core["qta_away_draw_weight"],
        qta_home_loss_weight=core["qta_home_loss_weight"],
        qta_away_loss_weight=core["qta_away_loss_weight"],
        standings_stage_order_index=core["standings_stage_order_index"],
        promotion_rule_kind=core["promotion_rule_kind"],
        promotion_count=core["promotion_count"],
        promotion_playoff_stage_order_index=core["promotion_playoff_stage_order_index"],
        promotion_target_league_id=core["promotion_target_league_id"],
        relegation_rule_kind=core["relegation_rule_kind"],
        relegation_count=core["relegation_count"],
        relegation_playoff_stage_order_index=core["relegation_playoff_stage_order_index"],
        relegation_target_league_id=core["relegation_target_league_id"],
        weekdays=weekdays,
        tie_break_criteria=tie_break_criteria,
        groups=groups,
        stages=stages,
    )
    return draft

def load_league_calendar_config_draft_from_db(
    db_path: Optional[str],
    config_id: str,
) -> Optional[LeagueCalendarConfigDraft]:
    if not db_path:
        return None
    try:
        with get_db_connection(db_path) as conn:
            return load_league_calendar_config_draft(conn, config_id)
    except Exception:
        return None