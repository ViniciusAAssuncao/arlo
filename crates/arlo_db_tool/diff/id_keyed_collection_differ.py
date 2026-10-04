from dataclasses import dataclass, field
from typing import Generic, List, Optional, Tuple, TypeVar

T = TypeVar("T")

@dataclass
class IdKeyedDiffResult(Generic[T]):
    added: List[T] = field(default_factory=list)
    matched: List[Tuple[T, T]] = field(default_factory=list)
    removed: List[T] = field(default_factory=list)

def diff_id_keyed_collection(
    original_items: Optional[List[T]],
    current_items: Optional[List[T]],
) -> IdKeyedDiffResult[T]:
    orig = original_items or []
    curr = current_items or []

    orig_map = {item.id: item for item in orig if hasattr(item, "id")}
    curr_map = {item.id: item for item in curr if hasattr(item, "id")}

    added = [item for item in curr if getattr(item, "id", None) not in orig_map]
    removed = [item for item in orig if getattr(item, "id", None) not in curr_map]
    matched = [
        (orig_map[item.id], item)
        for item in curr
        if getattr(item, "id", None) in orig_map
    ]

    return IdKeyedDiffResult(added=added, matched=matched, removed=removed)