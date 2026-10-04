from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional
from arlo_db_tool.domain.competition_group_draft import CompetitionGroupDraft
from arlo_db_tool.diff.id_keyed_collection_differ import diff_id_keyed_collection
from arlo_db_tool.diff.membership_set_differ import MembershipSetDiffResult, diff_membership_set
from arlo_db_tool.diff.scalar_field_differ import diff_scalar_fields

@dataclass
class GroupUpdateDiff:
    group_id: str
    original: CompetitionGroupDraft
    current: CompetitionGroupDraft
    scalar_changes: Dict[str, Any] = field(default_factory=dict)
    team_ids_diff: MembershipSetDiffResult = field(default_factory=MembershipSetDiffResult)

    @property
    def has_changes(self) -> bool:
        return bool(self.scalar_changes or self.team_ids_diff.has_changes)

@dataclass
class GroupCollectionDiffResult:
    added: List[CompetitionGroupDraft] = field(default_factory=list)
    updated: List[GroupUpdateDiff] = field(default_factory=list)
    removed: List[CompetitionGroupDraft] = field(default_factory=list)

    @property
    def has_changes(self) -> bool:
        return bool(self.added or self.removed or any(u.has_changes for u in self.updated))

def diff_groups(
    original_groups: Optional[List[CompetitionGroupDraft]],
    current_groups: Optional[List[CompetitionGroupDraft]],
) -> GroupCollectionDiffResult:
    id_diff = diff_id_keyed_collection(original_groups, current_groups)
    group_scalar_fields = ["order_index", "name"]
    updated_diffs: List[GroupUpdateDiff] = []

    for orig, curr in id_diff.matched:
        scalar_changes = diff_scalar_fields(orig, curr, group_scalar_fields)
        teams_diff = diff_membership_set(orig.team_ids, curr.team_ids)
        diff_item = GroupUpdateDiff(
            group_id=curr.id,
            original=orig,
            current=curr,
            scalar_changes=scalar_changes,
            team_ids_diff=teams_diff,
        )
        if diff_item.has_changes:
            updated_diffs.append(diff_item)

    return GroupCollectionDiffResult(
        added=id_diff.added,
        updated=updated_diffs,
        removed=id_diff.removed,
    )