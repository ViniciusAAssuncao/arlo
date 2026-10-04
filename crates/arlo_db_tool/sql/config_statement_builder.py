import time
from typing import Any, Dict, Optional
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft
from arlo_db_tool.schema.field_spec import FieldType
from arlo_db_tool.sql.generic_delete_builder import build_delete_by_id_statement
from arlo_db_tool.sql.value_formatting import format_sql_value

CONFIG_FIELD_TYPE_MAP = {
    "competition_id": FieldType.UUID_FK,
    "algorithm": FieldType.ENUM,
    "schedule_algorithm_kind": FieldType.ENUM,
    "season_start_month_order_index": FieldType.INTEGER,
    "season_start_day_of_month": FieldType.INTEGER,
    "season_length_weeks": FieldType.INTEGER,
    "max_games_per_team_per_week": FieldType.INTEGER,
    "games_per_week_conflict_scope": FieldType.ENUM,
    "postponement_strategy_kind": FieldType.ENUM,
    "neutral_opener_enabled": FieldType.BOOLEAN,
    "neutral_opener_selection_strategy": FieldType.ENUM,
    "spa_win_weight": FieldType.REAL,
    "spa_draw_weight": FieldType.REAL,
    "spa_loss_weight": FieldType.REAL,
    "spa_feo_k_factor": FieldType.REAL,
    "qta_home_win_weight": FieldType.REAL,
    "qta_away_win_weight": FieldType.REAL,
    "qta_home_draw_weight": FieldType.REAL,
    "qta_away_draw_weight": FieldType.REAL,
    "qta_home_loss_weight": FieldType.REAL,
    "qta_away_loss_weight": FieldType.REAL,
    "standings_stage_order_index": FieldType.INTEGER,
    "promotion_rule_kind": FieldType.ENUM,
    "promotion_count": FieldType.INTEGER,
    "promotion_playoff_stage_order_index": FieldType.INTEGER,
    "promotion_target_league_id": FieldType.UUID_FK,
    "relegation_rule_kind": FieldType.ENUM,
    "relegation_count": FieldType.INTEGER,
    "relegation_playoff_stage_order_index": FieldType.INTEGER,
    "relegation_target_league_id": FieldType.UUID_FK,
}

CONFIG_COLUMN_MAP = {
    "algorithm": "schedule_algorithm_kind",
}

def build_config_insert_statement(
    draft: LeagueCalendarConfigDraft,
    created_at_unix_seconds: Optional[int] = None,
) -> str:
    timestamp = created_at_unix_seconds if created_at_unix_seconds is not None else int(time.time())
    columns = [
        "id",
        "competition_id",
        "schedule_algorithm_kind",
        "season_start_month_order_index",
        "season_start_day_of_month",
        "season_length_weeks",
        "max_games_per_team_per_week",
        "games_per_week_conflict_scope",
        "postponement_strategy_kind",
        "neutral_opener_enabled",
        "neutral_opener_selection_strategy",
        "spa_win_weight",
        "spa_draw_weight",
        "spa_loss_weight",
        "spa_feo_k_factor",
        "qta_home_win_weight",
        "qta_away_win_weight",
        "qta_home_draw_weight",
        "qta_away_draw_weight",
        "qta_home_loss_weight",
        "qta_away_loss_weight",
        "standings_stage_order_index",
        "promotion_rule_kind",
        "promotion_count",
        "promotion_playoff_stage_order_index",
        "promotion_target_league_id",
        "relegation_rule_kind",
        "relegation_count",
        "relegation_playoff_stage_order_index",
        "relegation_target_league_id",
        "created_at_unix_seconds",
    ]
    values = [
        format_sql_value(draft.id, FieldType.UUID_PK),
        format_sql_value(draft.competition_id, FieldType.UUID_FK),
        format_sql_value(draft.algorithm, FieldType.ENUM),
        format_sql_value(draft.season_start_month_order_index, FieldType.INTEGER),
        format_sql_value(draft.season_start_day_of_month, FieldType.INTEGER),
        format_sql_value(draft.season_length_weeks, FieldType.INTEGER),
        format_sql_value(draft.max_games_per_team_per_week, FieldType.INTEGER),
        format_sql_value(draft.games_per_week_conflict_scope, FieldType.ENUM),
        format_sql_value(draft.postponement_strategy_kind, FieldType.ENUM),
        format_sql_value(1 if draft.neutral_opener_enabled else 0, FieldType.INTEGER),
        format_sql_value(draft.neutral_opener_selection_strategy, FieldType.ENUM),
        format_sql_value(draft.spa_win_weight, FieldType.REAL),
        format_sql_value(draft.spa_draw_weight, FieldType.REAL),
        format_sql_value(draft.spa_loss_weight, FieldType.REAL),
        format_sql_value(draft.spa_feo_k_factor, FieldType.REAL),
        format_sql_value(draft.qta_home_win_weight, FieldType.REAL),
        format_sql_value(draft.qta_away_win_weight, FieldType.REAL),
        format_sql_value(draft.qta_home_draw_weight, FieldType.REAL),
        format_sql_value(draft.qta_away_draw_weight, FieldType.REAL),
        format_sql_value(draft.qta_home_loss_weight, FieldType.REAL),
        format_sql_value(draft.qta_away_loss_weight, FieldType.REAL),
        format_sql_value(draft.standings_stage_order_index, FieldType.INTEGER),
        format_sql_value(draft.promotion_rule_kind, FieldType.ENUM),
        format_sql_value(draft.promotion_count, FieldType.INTEGER),
        format_sql_value(draft.promotion_playoff_stage_order_index, FieldType.INTEGER),
        format_sql_value(draft.promotion_target_league_id, FieldType.UUID_FK),
        format_sql_value(draft.relegation_rule_kind, FieldType.ENUM),
        format_sql_value(draft.relegation_count, FieldType.INTEGER),
        format_sql_value(draft.relegation_playoff_stage_order_index, FieldType.INTEGER),
        format_sql_value(draft.relegation_target_league_id, FieldType.UUID_FK),
        format_sql_value(timestamp, FieldType.UNIX_TIMESTAMP),
    ]
    cols_str = ", ".join(columns)
    vals_str = ", ".join(values)
    return f"INSERT INTO league_calendar_configs ({cols_str}) VALUES ({vals_str});"

def build_config_update_statement(
    config_id: str,
    changed_fields: Dict[str, Any],
) -> str:
    set_clauses = []
    for key, val in changed_fields.items():
        col_name = CONFIG_COLUMN_MAP.get(key, key)
        if key in CONFIG_FIELD_TYPE_MAP:
            ft = CONFIG_FIELD_TYPE_MAP[key]
            if ft == FieldType.BOOLEAN:
                formatted = "1" if bool(val) else "0"
            else:
                formatted = format_sql_value(val, ft)
            set_clauses.append(f"{col_name} = {formatted}")
        elif col_name in CONFIG_FIELD_TYPE_MAP:
            ft = CONFIG_FIELD_TYPE_MAP[col_name]
            if ft == FieldType.BOOLEAN:
                formatted = "1" if bool(val) else "0"
            else:
                formatted = format_sql_value(val, ft)
            set_clauses.append(f"{col_name} = {formatted}")
    if not set_clauses:
        return ""
    id_val = format_sql_value(config_id, FieldType.UUID_PK)
    return f"UPDATE league_calendar_configs SET {', '.join(set_clauses)} WHERE id = {id_val};"

def build_config_delete_statement(config_id: str) -> str:
    return build_delete_by_id_statement("league_calendar_configs", config_id)