from typing import List
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft

def validate_promotion_relegation(draft: LeagueCalendarConfigDraft) -> List[str]:
    errors: List[str] = []
    existing_stage_indices = {s.stage_order_index for s in draft.stages}

    if draft.standings_stage_order_index not in existing_stage_indices:
        errors.append(
            f"standings_stage_order_index ({draft.standings_stage_order_index}) references non-existent stage"
        )

    promo_kind = draft.promotion_rule_kind
    if promo_kind == "Automatic":
        if draft.promotion_count is None or draft.promotion_count < 1:
            errors.append("promotion_rule_kind Automatic requires promotion_count >= 1")
    elif promo_kind == "PlayoffStage":
        if draft.promotion_count is None or draft.promotion_count < 1:
            errors.append("promotion_rule_kind PlayoffStage requires promotion_count >= 1")
        if draft.promotion_playoff_stage_order_index is None:
            errors.append("promotion_rule_kind PlayoffStage requires promotion_playoff_stage_order_index")
        elif draft.promotion_playoff_stage_order_index not in existing_stage_indices:
            errors.append(
                f"promotion_playoff_stage_order_index ({draft.promotion_playoff_stage_order_index}) references non-existent stage"
            )

    releg_kind = draft.relegation_rule_kind
    if releg_kind == "Automatic":
        if draft.relegation_count is None or draft.relegation_count < 1:
            errors.append("relegation_rule_kind Automatic requires relegation_count >= 1")
    elif releg_kind == "PlayoffStage":
        if draft.relegation_count is None or draft.relegation_count < 1:
            errors.append("relegation_rule_kind PlayoffStage requires relegation_count >= 1")
        if draft.relegation_playoff_stage_order_index is None:
            errors.append("relegation_rule_kind PlayoffStage requires relegation_playoff_stage_order_index")
        elif draft.relegation_playoff_stage_order_index not in existing_stage_indices:
            errors.append(
                f"relegation_playoff_stage_order_index ({draft.relegation_playoff_stage_order_index}) references non-existent stage"
            )

    return errors
