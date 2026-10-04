from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional
from arlo_db_tool.domain.schedule_block_draft import ScheduleBlockDraft
from arlo_db_tool.diff.id_keyed_collection_differ import diff_id_keyed_collection
from arlo_db_tool.diff.membership_set_differ import MembershipSetDiffResult, diff_membership_set
from arlo_db_tool.diff.scalar_field_differ import diff_scalar_fields

@dataclass
class ScheduleBlockUpdateDiff:
    block_id: str
    original: ScheduleBlockDraft
    current: ScheduleBlockDraft
    scalar_changes: Dict[str, Any] = field(default_factory=dict)
    pool_group_ids_diff: MembershipSetDiffResult = field(default_factory=MembershipSetDiffResult)

    @property
    def has_changes(self) -> bool:
        return bool(self.scalar_changes or self.pool_group_ids_diff.has_changes)

@dataclass
class ScheduleBlockCollectionDiffResult:
    added: List[ScheduleBlockDraft] = field(default_factory=list)
    updated: List[ScheduleBlockUpdateDiff] = field(default_factory=list)
    removed: List[ScheduleBlockDraft] = field(default_factory=list)

    @property
    def has_changes(self) -> bool:
        return bool(self.added or self.removed or any(u.has_changes for u in self.updated))

def diff_schedule_blocks(
    original_blocks: Optional[List[ScheduleBlockDraft]],
    current_blocks: Optional[List[ScheduleBlockDraft]],
) -> ScheduleBlockCollectionDiffResult:
    id_diff = diff_id_keyed_collection(original_blocks, current_blocks)
    scalar_fields = [
        "block_order_index",
        "block_kind",
        "group_a_id",
        "group_b_id",
        "mirrored",
        "pool_kind",
        "rounds_count",
    ]
    updated_diffs: List[ScheduleBlockUpdateDiff] = []

    for orig, curr in id_diff.matched:
        scalar_changes = diff_scalar_fields(orig, curr, scalar_fields)
        pool_groups_diff = diff_membership_set(orig.pool_group_ids, curr.pool_group_ids)
        item = ScheduleBlockUpdateDiff(
            block_id=curr.id,
            original=orig,
            current=curr,
            scalar_changes=scalar_changes,
            pool_group_ids_diff=pool_groups_diff,
        )
        if item.has_changes:
            updated_diffs.append(item)

    return ScheduleBlockCollectionDiffResult(
        added=id_diff.added,
        updated=updated_diffs,
        removed=id_diff.removed,
    )