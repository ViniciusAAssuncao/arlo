from typing import List
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft

def validate_schedule_block_references(draft: LeagueCalendarConfigDraft) -> List[str]:
    errors: List[str] = []
    group_ids = {group.id for group in draft.groups if group.id}

    for stage in draft.stages:
        if not stage.schedule_blocks:
            continue

        seen_block_indices = set()
        block_indices = []
        for block in stage.schedule_blocks:
            b_idx = block.block_order_index
            if b_idx in seen_block_indices:
                errors.append(
                    f"stage {stage.stage_order_index} duplicate block_order_index found: {b_idx}"
                )
            seen_block_indices.add(b_idx)
            block_indices.append(b_idx)

        sorted_block_indices = sorted(block_indices)
        for expected, actual in enumerate(sorted_block_indices):
            if expected != actual:
                errors.append(
                    f"stage {stage.stage_order_index} schedule_blocks order_index must be sequential starting from 0, expected {expected} but got {actual}"
                )

        for block in stage.schedule_blocks:
            b_idx = block.block_order_index
            if block.block_kind == "GroupRoundRobin":
                target_gid = block.group_a_id
                if not target_gid or target_gid not in group_ids:
                    errors.append(
                        f"stage {stage.stage_order_index} block {b_idx} (GroupRoundRobin) references non-existent group_id: {target_gid}"
                    )
            elif block.block_kind == "CrossGroupPairing":
                g_a = block.group_a_id
                g_b = block.group_b_id
                if g_a and g_b and g_a == g_b:
                    errors.append(
                        f"stage {stage.stage_order_index} block {b_idx} (CrossGroupPairing) cannot pair group with itself: {g_a}"
                    )
                if not g_a or g_a not in group_ids:
                    errors.append(
                        f"stage {stage.stage_order_index} block {b_idx} (CrossGroupPairing) references non-existent group_a_id: {g_a}"
                    )
                if not g_b or g_b not in group_ids:
                    errors.append(
                        f"stage {stage.stage_order_index} block {b_idx} (CrossGroupPairing) references non-existent group_b_id: {g_b}"
                    )
            elif block.block_kind == "RandomPoolRounds":
                if block.rounds_count is None or block.rounds_count < 1:
                    errors.append(
                        f"stage {stage.stage_order_index} block {b_idx} (RandomPoolRounds) requires rounds_count >= 1"
                    )
                if block.pool_kind == "SpecificGroups":
                    if not block.pool_group_ids:
                        errors.append(
                            f"stage {stage.stage_order_index} block {b_idx} (RandomPoolRounds SpecificGroups) requires non-empty pool_group_ids"
                        )
                    for gid in block.pool_group_ids:
                        if gid not in group_ids:
                            errors.append(
                                f"stage {stage.stage_order_index} block {b_idx} (RandomPoolRounds) references non-existent pool group_id: {gid}"
                            )

    return errors
