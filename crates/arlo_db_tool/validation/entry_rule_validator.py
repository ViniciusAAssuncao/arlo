from typing import List
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft

GROUP_REQUIRING_POOLS = {
    "GroupWinners",
    "GroupRunnersUp",
    "BestAtGroupPosition",
}

def validate_entry_rules(draft: LeagueCalendarConfigDraft) -> List[str]:
    errors: List[str] = []
    has_groups = len(draft.groups) > 0

    for stage in draft.stages:
        if not stage.entry_rule_pools:
            errors.append(f"stage {stage.stage_order_index} requires at least one entry rule pool")
            continue

        seen_pool_indices = set()
        pool_indices = []
        for pool in stage.entry_rule_pools:
            p_idx = pool.pool_order_index
            if p_idx in seen_pool_indices:
                errors.append(
                    f"stage {stage.stage_order_index} duplicate pool_order_index found: {p_idx}"
                )
            seen_pool_indices.add(p_idx)
            pool_indices.append(p_idx)

        sorted_pool_indices = sorted(pool_indices)
        for expected, actual in enumerate(sorted_pool_indices):
            if expected != actual:
                errors.append(
                    f"stage {stage.stage_order_index} pool_order_index must be sequential starting from 0, expected {expected} but got {actual}"
                )

        for pool in stage.entry_rule_pools:
            p_idx = pool.pool_order_index
            p_kind = pool.pool_kind

            if p_kind in GROUP_REQUIRING_POOLS and not has_groups:
                errors.append(
                    f"stage {stage.stage_order_index} pool {p_idx} ({p_kind}) requires at least one group defined in draft"
                )

            if p_kind in ("TopN", "BottomN"):
                if pool.count is None or pool.count < 1:
                    errors.append(
                        f"stage {stage.stage_order_index} pool {p_idx} ({p_kind}) requires count >= 1"
                    )
            elif p_kind == "BestAtGroupPosition":
                if pool.count is None or pool.count < 1:
                    errors.append(
                        f"stage {stage.stage_order_index} pool {p_idx} (BestAtGroupPosition) requires count >= 1"
                    )
                if pool.position_index is None or pool.position_index < 0:
                    errors.append(
                        f"stage {stage.stage_order_index} pool {p_idx} (BestAtGroupPosition) requires position_index >= 0"
                    )
            elif p_kind == "PositionRange":
                start = pool.range_start_position
                end = pool.range_end_position
                if start is None or start < 1:
                    errors.append(
                        f"stage {stage.stage_order_index} pool {p_idx} (PositionRange) requires range_start_position >= 1"
                    )
                if end is None:
                    errors.append(
                        f"stage {stage.stage_order_index} pool {p_idx} (PositionRange) requires range_end_position"
                    )
                elif start is not None and end < start:
                    errors.append(
                        f"stage {stage.stage_order_index} pool {p_idx} (PositionRange) requires range_end_position >= range_start_position"
                    )
            elif p_kind == "ExternalCompetitionWinner":
                if not pool.external_competition_id or not str(pool.external_competition_id).strip():
                    errors.append(
                        f"stage {stage.stage_order_index} pool {p_idx} (ExternalCompetitionWinner) requires external_competition_id"
                    )

    return errors
