from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional
from arlo_db_tool.domain.entry_rule_pool_draft import EntryRulePoolDraft
from arlo_db_tool.diff.id_keyed_collection_differ import diff_id_keyed_collection
from arlo_db_tool.diff.scalar_field_differ import diff_scalar_fields

@dataclass
class EntryRulePoolUpdateDiff:
    pool_id: str
    original: EntryRulePoolDraft
    current: EntryRulePoolDraft
    scalar_changes: Dict[str, Any] = field(default_factory=dict)

    @property
    def has_changes(self) -> bool:
        return bool(self.scalar_changes)

@dataclass
class EntryRulePoolCollectionDiffResult:
    added: List[EntryRulePoolDraft] = field(default_factory=list)
    updated: List[EntryRulePoolUpdateDiff] = field(default_factory=list)
    removed: List[EntryRulePoolDraft] = field(default_factory=list)

    @property
    def has_changes(self) -> bool:
        return bool(self.added or self.removed or any(u.has_changes for u in self.updated))

def diff_entry_rule_pools(
    original_pools: Optional[List[EntryRulePoolDraft]],
    current_pools: Optional[List[EntryRulePoolDraft]],
) -> EntryRulePoolCollectionDiffResult:
    id_diff = diff_id_keyed_collection(original_pools, current_pools)
    scalar_fields = [
        "pool_order_index",
        "pool_kind",
        "count",
        "position_index",
        "range_start_position",
        "range_end_position",
        "external_competition_id",
    ]
    updated_diffs: List[EntryRulePoolUpdateDiff] = []

    for orig, curr in id_diff.matched:
        scalar_changes = diff_scalar_fields(orig, curr, scalar_fields)
        item = EntryRulePoolUpdateDiff(
            pool_id=curr.id,
            original=orig,
            current=curr,
            scalar_changes=scalar_changes,
        )
        if item.has_changes:
            updated_diffs.append(item)

    return EntryRulePoolCollectionDiffResult(
        added=id_diff.added,
        updated=updated_diffs,
        removed=id_diff.removed,
    )