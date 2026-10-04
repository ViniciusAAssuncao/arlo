from typing import List
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft

def validate_stage_order(draft: LeagueCalendarConfigDraft) -> List[str]:
    errors: List[str] = []
    if not draft.stages:
        errors.append("stages must not be empty")
        return errors

    seen_indices = set()
    indices = []
    for stage in draft.stages:
        idx = stage.stage_order_index
        if idx in seen_indices:
            errors.append(f"duplicate stage_order_index found: {idx}")
        seen_indices.add(idx)
        indices.append(idx)

    sorted_indices = sorted(indices)
    for expected, actual in enumerate(sorted_indices):
        if expected != actual:
            errors.append(
                f"stages order_index must be sequential starting from 0, expected {expected} but got {actual}"
            )

    for stage in draft.stages:
        st_type = stage.stage_type
        if st_type == "KnockoutBracket":
            if not stage.leg_format or not str(stage.leg_format).strip():
                errors.append(
                    f"stage {stage.stage_order_index} (KnockoutBracket) requires leg_format"
                )
            if stage.schedule_blocks:
                errors.append(
                    f"stage {stage.stage_order_index} (KnockoutBracket) must not have schedule_blocks"
                )
        elif st_type == "GroupedCompetitionTable":
            if not stage.schedule_blocks:
                errors.append(
                    f"stage {stage.stage_order_index} (GroupedCompetitionTable) requires non-empty schedule_blocks"
                )
            if stage.leg_format and str(stage.leg_format).strip():
                errors.append(
                    f"stage {stage.stage_order_index} (GroupedCompetitionTable) must not have leg_format"
                )
        elif st_type == "RoundRobinTable":
            if stage.leg_format and str(stage.leg_format).strip():
                errors.append(
                    f"stage {stage.stage_order_index} (RoundRobinTable) must not have leg_format"
                )
            if stage.schedule_blocks:
                errors.append(
                    f"stage {stage.stage_order_index} (RoundRobinTable) must not have schedule_blocks"
                )
        else:
            errors.append(f"stage {stage.stage_order_index} has invalid stage_type: {st_type}")

    return errors