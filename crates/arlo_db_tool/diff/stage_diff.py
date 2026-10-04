from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional
from arlo_db_tool.domain.stage_draft import StageDraft
from arlo_db_tool.diff.entry_rule_pool_diff import (
    EntryRulePoolCollectionDiffResult,
    diff_entry_rule_pools,
)
from arlo_db_tool.diff.id_keyed_collection_differ import diff_id_keyed_collection
from arlo_db_tool.diff.scalar_field_differ import diff_scalar_fields
from arlo_db_tool.diff.schedule_block_diff import (
    ScheduleBlockCollectionDiffResult,
    diff_schedule_blocks,
)

@dataclass
class StageUpdateDiff:
    stage_id: str
    original: StageDraft
    current: StageDraft
    scalar_changes: Dict[str, Any] = field(default_factory=dict)
    entry_rule_pools_diff: EntryRulePoolCollectionDiffResult = field(
        default_factory=EntryRulePoolCollectionDiffResult
    )
    schedule_blocks_diff: ScheduleBlockCollectionDiffResult = field(
        default_factory=ScheduleBlockCollectionDiffResult
    )

    @property
    def has_changes(self) -> bool:
        return bool(
            self.scalar_changes
            or self.entry_rule_pools_diff.has_changes
            or self.schedule_blocks_diff.has_changes
        )

@dataclass
class StageCollectionDiffResult:
    added: List[StageDraft] = field(default_factory=list)
    updated: List[StageUpdateDiff] = field(default_factory=list)
    removed: List[StageDraft] = field(default_factory=list)

    @property
    def has_changes(self) -> bool:
        return bool(self.added or self.removed or any(u.has_changes for u in self.updated))

def diff_stages(
    original_stages: Optional[List[StageDraft]],
    current_stages: Optional[List[StageDraft]],
) -> StageCollectionDiffResult:
    id_diff = diff_id_keyed_collection(original_stages, current_stages)
    scalar_fields = ["stage_order_index", "stage_type", "leg_format"]
    updated_diffs: List[StageUpdateDiff] = []

    for orig, curr in id_diff.matched:
        scalar_changes = diff_scalar_fields(orig, curr, scalar_fields)
        pools_diff = diff_entry_rule_pools(orig.entry_rule_pools, curr.entry_rule_pools)
        blocks_diff = diff_schedule_blocks(orig.schedule_blocks, curr.schedule_blocks)
        item = StageUpdateDiff(
            stage_id=curr.id,
            original=orig,
            current=curr,
            scalar_changes=scalar_changes,
            entry_rule_pools_diff=pools_diff,
            schedule_blocks_diff=blocks_diff,
        )
        if item.has_changes:
            updated_diffs.append(item)

    return StageCollectionDiffResult(
        added=id_diff.added,
        updated=updated_diffs,
        removed=id_diff.removed,
    )