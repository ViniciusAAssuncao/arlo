from dataclasses import dataclass, field
from typing import List, Optional, Sequence

@dataclass
class MembershipSetDiffResult:
    added: List[str] = field(default_factory=list)
    removed: List[str] = field(default_factory=list)

    @property
    def has_changes(self) -> bool:
        return bool(self.added or self.removed)

def diff_membership_set(
    original_ids: Optional[Sequence[str]],
    current_ids: Optional[Sequence[str]],
) -> MembershipSetDiffResult:
    orig_list = list(original_ids or [])
    curr_list = list(current_ids or [])

    orig_set = set(orig_list)
    curr_set = set(curr_list)

    added = [item for item in curr_list if item not in orig_set]
    removed = [item for item in orig_list if item not in curr_set]

    return MembershipSetDiffResult(added=added, removed=removed)