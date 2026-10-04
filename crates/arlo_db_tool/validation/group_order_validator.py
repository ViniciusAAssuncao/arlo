from typing import List
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft

def validate_group_order(draft: LeagueCalendarConfigDraft) -> List[str]:
    errors: List[str] = []
    if not draft.groups:
        return errors

    seen_indices = set()
    indices = []
    for group in draft.groups:
        idx = group.order_index
        if idx in seen_indices:
            errors.append(f"duplicate group order_index found: {idx}")
        seen_indices.add(idx)
        indices.append(idx)

    sorted_indices = sorted(indices)
    for expected, actual in enumerate(sorted_indices):
        if expected != actual:
            errors.append(
                f"groups order_index must be sequential starting from 0, expected {expected} but got {actual}"
            )

    seen_team_ids = set()
    for group in draft.groups:
        if not group.team_ids:
            errors.append(f"group '{group.name or group.order_index}' must have at least one team")
        for team_id in group.team_ids:
            if not str(team_id).strip():
                continue
            if team_id in seen_team_ids:
                errors.append(f"duplicate team_id '{team_id}' found across groups")
            seen_team_ids.add(team_id)

    return errors
